//! Générateurs de projets.
//!
//! Les fichiers générés vivent dans `templates/<type>/…` (extension `.tpl`) et sont
//! embarqués dans le binaire à la compilation. Les marqueurs `@@name@@`, `@@slug@@`,
//! `@@pkg@@`, `@@title@@` et `@@upper@@` sont remplacés au moment de l'écriture, y compris
//! dans les chemins.

pub mod astro;
pub mod bootstrap;
pub mod checks;
pub mod docs;
pub mod laravel;
pub mod readme;
pub mod smoke;
pub mod wagtail;

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Embarque `templates/<chemin>` (relatif à la racine du crate).
#[macro_export]
macro_rules! tpl {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/", $path))
    };
}

/// Fichier de template : (chemin cible avec marqueurs, contenu avec marqueurs).
pub type TemplateFile<'a> = (&'a str, &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Bootstrap,
    Wagtail,
    Astro,
    Laravel,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Bootstrap => "Rust (init)",
            Kind::Wagtail => "Wagtail",
            Kind::Astro => "Astro",
            Kind::Laravel => "Laravel",
        }
    }

    pub fn parse(s: &str) -> Option<Kind> {
        match s.to_lowercase().as_str() {
            "bootstrap" | "init" | "rust" => Some(Kind::Bootstrap),
            "wagtail" => Some(Kind::Wagtail),
            "astro" => Some(Kind::Astro),
            "laravel" => Some(Kind::Laravel),
            _ => None,
        }
    }
}

/// Variables de substitution dérivées du nom du projet.
#[derive(Debug, Clone)]
pub struct Vars {
    pub name: String,
    pub slug: String,
    pub pkg: String,
    pub title: String,
    pub upper: String,
}

impl Vars {
    pub fn new(name: &str) -> Result<Self> {
        validate_name(name)?;
        let slug: String = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .to_string();
        let mut pkg = slug.replace('-', "_");
        if pkg.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            pkg = format!("site_{}", pkg);
        }
        let title = slug
            .split('-')
            .filter(|w| !w.is_empty())
            .map(|w| {
                let mut c = w.chars();
                c.next()
                    .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>()
            .join(" ");
        Ok(Self {
            name: name.to_string(),
            upper: pkg.to_uppercase(),
            slug,
            pkg,
            title,
        })
    }

    pub fn render(&self, text: &str) -> String {
        text.replace("@@name@@", &self.name)
            .replace("@@slug@@", &self.slug)
            .replace("@@pkg@@", &self.pkg)
            .replace("@@title@@", &self.title)
            .replace("@@upper@@", &self.upper)
    }
}

/// Un nom de projet est un seul segment de chemin : lettres, chiffres, `-`, `_`, `.`
/// (jamais de `..`, de séparateur ni de point initial).
pub fn validate_name(name: &str) -> Result<()> {
    let ok = !name.is_empty()
        && name.len() <= 64
        && !name.starts_with(['.', '-'])
        && !name.contains("..")
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && name.chars().any(|c| c.is_ascii_alphanumeric());
    if !ok {
        bail!(
            "nom de projet invalide `{}` : utiliser des lettres, chiffres, `-`, `_` ou `.` (64 max, sans `..`)",
            name
        );
    }
    Ok(())
}

/// Résultat d'une génération.
#[derive(Debug, Clone)]
pub struct Generated {
    pub kind: Kind,
    pub target: PathBuf,
    /// Fichiers créés (chemins relatifs, triés).
    pub files: Vec<String>,
}

impl Generated {
    pub fn summary(&self) -> String {
        format!(
            "✓ Projet {} généré dans {} ({} fichiers)",
            self.kind.label(),
            self.target.display(),
            self.files.len()
        )
    }
}

/// Écrit les fichiers d'un générateur. Refuse d'écrire dans un dossier non vide sauf `force`.
pub fn write_files(
    kind: Kind,
    target: &Path,
    name: &str,
    files: &[TemplateFile],
    force: bool,
) -> Result<Generated> {
    let vars = Vars::new(name)?;
    if target.exists() {
        let non_empty = target.is_dir()
            && fs::read_dir(target)
                .map(|mut d| d.next().is_some())
                .unwrap_or(true);
        if (non_empty || !target.is_dir()) && !force {
            bail!(
                "{} existe déjà et n'est pas vide (utiliser --force pour écraser les fichiers générés)",
                target.display()
            );
        }
    }
    let mut written = Vec::new();
    for (rel, content) in files {
        let rel = vars.render(rel);
        let dest = target.join(&rel);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("création de {}", parent.display()))?;
        }
        fs::write(&dest, vars.render(content))
            .with_context(|| format!("écriture de {}", dest.display()))?;
        #[cfg(unix)]
        if rel == "artisan" || rel == "manage.py" {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&dest, fs::Permissions::from_mode(0o755));
        }
        written.push(rel);
    }
    written.sort();
    Ok(Generated {
        kind,
        target: target.to_path_buf(),
        files: written,
    })
}

/// Comme [`write_files`], en ajoutant `docs/index.md` (documentation de départ du projet).
pub fn write_with_docs(
    kind: Kind,
    target: &Path,
    name: &str,
    files: &[TemplateFile],
    force: bool,
) -> Result<Generated> {
    let docs_md = docs::generate(name);
    let mut all: Vec<TemplateFile> = files.to_vec();
    all.push(("docs/index.md", &docs_md));
    write_files(kind, target, name, &all, force)
}

/// Point d'entrée commun CLI / TUI / GUI.
pub fn generate(kind: Kind, target: &Path, name: &str, force: bool) -> Result<Generated> {
    match kind {
        Kind::Bootstrap => bootstrap::generate(target, name, force),
        Kind::Wagtail => wagtail::generate(target, name, force),
        Kind::Astro => astro::generate(target, name, force),
        Kind::Laravel => laravel::generate(target, name, force),
    }
}

/// Destination par défaut : `<racine>/sites/<name>`.
pub fn default_target(name: &str) -> PathBuf {
    crate::paths::sites_dir().join(name)
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn names_are_validated() {
        for ok in ["mon-site", "site_1", "a", "My.Site", "9lives"] {
            assert!(validate_name(ok).is_ok(), "{}", ok);
        }
        for bad in [
            "", "..", "../x", "a/b", "a\\b", ".hidden", "-x", "a b", "é", "---", "a..b",
        ] {
            assert!(validate_name(bad).is_err(), "{}", bad);
        }
    }

    #[test]
    fn vars_derivation() {
        let v = Vars::new("Mon-Site_2").unwrap();
        assert_eq!(v.slug, "mon-site-2");
        assert_eq!(v.pkg, "mon_site_2");
        assert_eq!(v.title, "Mon Site 2");
        assert_eq!(Vars::new("9lives").unwrap().pkg, "site_9lives");
        assert_eq!(
            v.render("@@pkg@@/@@slug@@/@@title@@"),
            "mon_site_2/mon-site-2/Mon Site 2"
        );
    }

    #[test]
    fn refuses_non_empty_target_without_force() {
        let dir = tempfile::tempdir().unwrap();
        let t = dir.path().join("x");
        let files: &[TemplateFile] = &[("a/b.txt", "hello @@name@@")];
        let g = write_files(Kind::Astro, &t, "x", files, false).unwrap();
        assert_eq!(g.files, vec!["a/b.txt"]);
        assert_eq!(fs::read_to_string(t.join("a/b.txt")).unwrap(), "hello x");
        assert!(write_files(Kind::Astro, &t, "x", files, false).is_err());
        assert!(write_files(Kind::Astro, &t, "x", files, true).is_ok());
    }
}
