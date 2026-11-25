//! Langues du site vitrine (`--langs`), communes à Astro, Laravel et Wagtail.
//!
//! La liste vient de `tools/dsgen/vitrine/locales.json`, écrit par `gen_ds.py` depuis `vitrine/locales/<code>.py` :
//! ajouter un fichier de langue à la spec l'ajoute ici sans toucher au code Rust. Chaque langue choisie active le
//! drapeau de couche `lang_<code>` (fichiers et blocs propres à la langue) ; `multilingual` vaut vrai dès deux langues.

use anyhow::{bail, Result};
use std::sync::OnceLock;

const LOCALES_JSON: &str = include_str!("../../tools/dsgen/vitrine/locales.json");

/// Langues par défaut quand `--langs` est absent.
pub const DEFAULT_LANGS: &[&str] = &["fr", "en"];

#[derive(Debug)]
pub struct Lang {
    pub code: String,
    pub name: String,
    pub rtl: bool,
    flag: String,
}

struct Catalog {
    default: String,
    langs: Vec<Lang>,
}

fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let v: serde_json::Value =
            serde_json::from_str(LOCALES_JSON).expect("tools/dsgen/vitrine/locales.json invalide");
        let langs = v["locales"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|l| {
                        let code = l["code"].as_str().unwrap_or_default().to_string();
                        Lang {
                            flag: format!("lang_{code}"),
                            name: l["name"].as_str().unwrap_or_default().to_string(),
                            rtl: l["dir"].as_str() == Some("rtl"),
                            code,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();
        Catalog {
            default: v["default"].as_str().unwrap_or("fr").to_string(),
            langs,
        }
    })
}

/// Toutes les langues disponibles, dans l'ordre du sélecteur.
pub fn all() -> &'static [Lang] {
    &catalog().langs
}

/// Langue par défaut (servie sans préfixe ; langue source de la spec).
pub fn default_lang() -> &'static str {
    &catalog().default
}

pub fn find(code: &str) -> Option<&'static Lang> {
    all().iter().find(|l| l.code == code)
}

/// Langues retenues : celles demandées (ou fr,en), dans l'ordre du sélecteur.
pub fn resolve(langs: &[String]) -> Vec<&'static Lang> {
    let wanted: Vec<&str> = if langs.is_empty() {
        DEFAULT_LANGS.to_vec()
    } else {
        langs.iter().map(String::as_str).collect()
    };
    all()
        .iter()
        .filter(|l| wanted.contains(&l.code.as_str()))
        .collect()
}

/// Codes retenus (ordre du sélecteur).
pub fn codes(langs: &[String]) -> Vec<String> {
    resolve(langs).iter().map(|l| l.code.clone()).collect()
}

/// Contrôle `--langs` : langues connues, sans doublon, langue par défaut incluse.
pub fn validate(langs: &[String]) -> Result<()> {
    let available = || {
        all()
            .iter()
            .map(|l| l.code.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let mut seen = std::collections::BTreeSet::new();
    for l in langs {
        if find(l).is_none() {
            bail!(
                "--langs : langue inconnue `{l}` (disponibles : {})",
                available()
            );
        }
        if !seen.insert(l) {
            bail!("--langs : langue `{l}` en double");
        }
    }
    if !langs.is_empty() && !langs.iter().any(|l| l == default_lang()) {
        bail!(
            "--langs : la langue par défaut `{}` (langue source du contenu, servie sans préfixe) doit figurer dans la liste",
            default_lang()
        );
    }
    Ok(())
}

/// Drapeaux de couche : `lang_<code>` pour chaque langue disponible, `multilingual` et `rtl` (une langue RTL choisie).
pub fn flags(langs: &[String]) -> Vec<(&'static str, bool)> {
    let chosen = resolve(langs);
    let mut v: Vec<(&'static str, bool)> = all()
        .iter()
        .map(|l| (l.flag.as_str(), chosen.iter().any(|c| c.code == l.code)))
        .collect();
    v.push(("multilingual", chosen.len() > 1));
    v.push(("rtl", chosen.iter().any(|l| l.rtl)));
    v
}

/// Réglage Django `LANGUAGES` (Wagtail) des langues retenues.
pub fn django_languages(langs: &[String]) -> String {
    resolve(langs)
        .iter()
        .map(|l| format!("(\"{}\", \"{}\")", l.code, l.name))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(std::string::ToString::to_string).collect()
    }

    #[test]
    fn catalog_lists_the_spec_languages() {
        let codes: Vec<_> = all().iter().map(|l| l.code.as_str()).collect();
        assert_eq!(codes, ["fr", "en", "es", "ar"]);
        assert!(find("ar").unwrap().rtl && !find("es").unwrap().rtl);
        assert_eq!(default_lang(), "fr");
    }

    #[test]
    fn defaults_and_order_follow_the_switcher() {
        assert_eq!(codes(&[]), ["fr", "en"]);
        assert_eq!(codes(&s(&["ar", "fr", "es"])), ["fr", "es", "ar"]);
        let f: std::collections::HashMap<_, _> = flags(&s(&["fr", "ar"])).into_iter().collect();
        assert!(f["lang_ar"] && !f["lang_en"] && f["multilingual"] && f["rtl"]);
        let f: std::collections::HashMap<_, _> = flags(&s(&["fr"])).into_iter().collect();
        assert!(!f["multilingual"] && !f["rtl"]);
    }

    #[test]
    fn validation_rejects_unknown_duplicate_or_missing_default() {
        assert!(validate(&s(&["fr", "en", "es", "ar"])).is_ok());
        assert!(validate(&[]).is_ok());
        assert!(validate(&s(&["fr", "xx"]))
            .unwrap_err()
            .to_string()
            .contains("xx"));
        assert!(validate(&s(&["fr", "fr"]))
            .unwrap_err()
            .to_string()
            .contains("double"));
        assert!(validate(&s(&["en", "es"]))
            .unwrap_err()
            .to_string()
            .contains("`fr`"));
    }

    #[test]
    fn django_languages_lists_names() {
        assert_eq!(
            django_languages(&s(&["fr", "ar"])),
            "(\"fr\", \"Français\"), (\"ar\", \"العربية\")"
        );
    }

    #[test]
    fn every_language_has_its_layer_files() {
        // un fichier de langue par framework (sauf la langue par défaut, toujours présente)
        for l in all() {
            for (layer, path) in [
                ("site-laravel", format!("lang/{}/site.php.tpl", l.code)),
                (
                    "site-astro",
                    format!("src/data/locales/{}.json.tpl", l.code),
                ),
            ] {
                assert!(
                    super::super::layer::files(layer)
                        .iter()
                        .any(|(p, _)| *p == path),
                    "{layer}/{path} manquant (lancer python3 tools/dsgen/gen_ds.py)"
                );
            }
        }
    }
}
