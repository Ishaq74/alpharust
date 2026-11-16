//! Utilitaires partagés des générateurs : validation de nom, variables dérivées, générateurs de texte
//! (`readme`, `docs`, `checks`). L'échafaudage des projets (Astro, Laravel, Wagtail, Rust) vit dans
//! [`crate::scaffold`] : il s'appuie sur les outils officiels.

pub mod checks;
pub mod docs;
pub mod readme;

use anyhow::{bail, Result};
use std::path::PathBuf;

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
}
