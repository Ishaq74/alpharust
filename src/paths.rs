//! Résolution du répertoire racine AlphaRust (celui qui contient `rules/`).
//!
//! Ordre de recherche :
//! 1. variable d'environnement `ALPHARUST_HOME` ;
//! 2. le répertoire courant, puis ses parents ;
//! 3. les parents de l'exécutable (ex. `target/release/alpharust` → racine du dépôt) ;
//! 4. à défaut, le répertoire courant.
//!
//! Un binaire installé seul (release GitHub, `cargo install`) n'a pas de `rules/` à côté de lui : les règles et
//! profils embarqués à la compilation sont alors extraits une fois dans un dossier temporaire versionné.

use std::path::{Path, PathBuf};

fn has_rules(dir: &Path) -> bool {
    dir.join("rules").is_dir()
}

/// Racine du dépôt AlphaRust (là où vivent `rules/`, `profiles/` et `sites/`).
pub fn root() -> PathBuf {
    if let Some(home) = std::env::var_os("ALPHARUST_HOME") {
        return PathBuf::from(home);
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    for dir in cwd.ancestors() {
        if has_rules(dir) {
            return dir.to_path_buf();
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        for dir in exe.ancestors().skip(1).take(5) {
            if has_rules(dir) {
                return dir.to_path_buf();
            }
        }
    }
    cwd
}

static EMBEDDED_RULES: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/rules");
static EMBEDDED_PROFILES: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/profiles");

/// Copie embarquée de `rules/` et `profiles/`, extraite au premier besoin (`<temp>/alpharust-<version>`).
fn embedded_home() -> PathBuf {
    let home = std::env::temp_dir().join(concat!("alpharust-", env!("CARGO_PKG_VERSION")));
    let marker = home.join(".complete");
    if !marker.is_file() {
        let staging = home.with_extension(format!("tmp-{}", std::process::id()));
        let ok = std::fs::create_dir_all(staging.join("rules")).is_ok()
            && std::fs::create_dir_all(staging.join("profiles")).is_ok()
            && EMBEDDED_RULES.extract(staging.join("rules")).is_ok()
            && EMBEDDED_PROFILES.extract(staging.join("profiles")).is_ok()
            && std::fs::write(staging.join(".complete"), "").is_ok();
        if ok && std::fs::rename(&staging, &home).is_err() {
            // un autre processus a gagné la course : sa copie est identique
            let _ = std::fs::remove_dir_all(&staging);
        }
    }
    home
}

fn data_root() -> PathBuf {
    let root = root();
    if has_rules(&root) {
        root
    } else {
        embedded_home()
    }
}

pub fn rules_dir() -> PathBuf {
    data_root().join("rules")
}

pub fn profiles_dir() -> PathBuf {
    data_root().join("profiles")
}

pub fn sites_dir() -> PathBuf {
    root().join("sites")
}

pub fn cache_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("ALPHARUST_CACHE_DIR") {
        return PathBuf::from(dir);
    }
    root().join(".alpharust-cache")
}

#[cfg(test)]
mod tests {
    #[test]
    fn embedded_rules_and_profiles_extract_completely() {
        let home = super::embedded_home();
        assert!(home.join(".complete").is_file());
        let count = |d: &str| {
            walk(&home.join(d))
                .into_iter()
                .filter(|p| p.extension().is_some_and(|e| e == "md" || e == "toml"))
                .count()
        };
        assert!(
            count("rules") >= 90,
            "règles extraites : {}",
            count("rules")
        );
        assert!(count("profiles") >= 5);
    }

    fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
        let mut out = vec![];
        for e in std::fs::read_dir(dir).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                out.extend(walk(&p));
            } else {
                out.push(p);
            }
        }
        out
    }
}
