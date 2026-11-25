use crate::glob::glob_match_loose;
use crate::parser::Rule;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProfileFile {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub description: String,
    /// Motifs : id exact (`core/no-eval`), glob (`domains/database/*`) ou nom de règle.
    pub rule_names: Vec<String>,
}

impl Profile {
    pub fn list() -> Result<Vec<String>> {
        let root = crate::paths::profiles_dir();
        let mut names = Vec::new();
        if root.exists() {
            for entry in fs::read_dir(&root)? {
                let path = entry?.path();
                if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        names.push(stem.to_string());
                    }
                }
            }
        }
        names.sort();
        Ok(names)
    }

    pub fn load(name: &str) -> Result<Self> {
        if name.is_empty() || name.contains(['/', '\\', '.']) {
            bail!("nom de profil invalide : `{name}`");
        }
        let path = crate::paths::profiles_dir().join(format!("{name}.toml"));
        if !path.exists() {
            bail!(
                "profil `{}` introuvable (disponibles : {})",
                name,
                Self::list().unwrap_or_default().join(", ")
            );
        }
        let content =
            fs::read_to_string(&path).with_context(|| format!("lecture du profil {name}"))?;
        let pf: ProfileFile =
            toml::from_str(&content).with_context(|| format!("profil {name} invalide"))?;
        Ok(Profile {
            name: if pf.name.is_empty() {
                name.to_string()
            } else {
                pf.name
            },
            description: pf.description,
            rule_names: pf.rules,
        })
    }

    pub fn matches(&self, rule: &Rule) -> bool {
        self.rule_names
            .iter()
            .any(|p| glob_match_loose(p, &rule.id) || glob_match_loose(p, &rule.name))
    }

    /// Règles du profil. Un profil sans motif sélectionne toutes les règles.
    pub fn select(&self, all: Vec<Rule>) -> Vec<Rule> {
        if self.rule_names.is_empty() {
            return all;
        }
        all.into_iter().filter(|r| self.matches(r)).collect()
    }

    pub fn resolve_rules(&self) -> Result<Vec<Rule>> {
        Ok(self.select(Rule::load_all()?))
    }

    /// Motifs qui ne correspondent à aucune règle (faute de frappe probable).
    pub fn unmatched_patterns(&self, all: &[Rule]) -> Vec<String> {
        self.rule_names
            .iter()
            .filter(|p| {
                !all.iter()
                    .any(|r| glob_match_loose(p, &r.id) || glob_match_loose(p, &r.name))
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_profiles_has_core() {
        let names = Profile::list().unwrap_or_default();
        assert!(names.iter().any(|n| n == "core"));
    }

    #[test]
    fn profiles_select_real_rules() {
        let all = Rule::load_all().unwrap();
        for name in Profile::list().unwrap() {
            let p = Profile::load(&name).unwrap();
            let selected = p.select(all.clone());
            assert!(
                !selected.is_empty(),
                "le profil {name} ne sélectionne aucune règle"
            );
            assert!(
                p.unmatched_patterns(&all).is_empty(),
                "motifs inutilisés dans {}: {:?}",
                name,
                p.unmatched_patterns(&all)
            );
        }
    }

    #[test]
    fn strict_is_a_superset_of_core() {
        let all = Rule::load_all().unwrap();
        let core = Profile::load("core").unwrap().select(all.clone());
        let strict = Profile::load("strict").unwrap().select(all);
        for r in &core {
            assert!(
                strict.iter().any(|s| s.id == r.id),
                "{} absent de strict",
                r.id
            );
        }
        assert!(strict.len() > core.len());
    }

    #[test]
    fn rejects_unknown_and_path_like_names() {
        assert!(Profile::load("nope").is_err());
        assert!(Profile::load("../Cargo").is_err());
    }
}
