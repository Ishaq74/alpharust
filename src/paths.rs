//! Résolution du répertoire racine AlphaRust (celui qui contient `rules/`).
//!
//! Ordre de recherche :
//! 1. variable d'environnement `ALPHARUST_HOME` ;
//! 2. le répertoire courant, puis ses parents ;
//! 3. les parents de l'exécutable (ex. `target/release/alpharust` → racine du dépôt) ;
//! 4. à défaut, le répertoire courant.

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

pub fn rules_dir() -> PathBuf {
    root().join("rules")
}

pub fn profiles_dir() -> PathBuf {
    root().join("profiles")
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
