use anyhow::{Context, Result};
use serde::de::IgnoredAny;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, deserialize)]
struct ProfileFile {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    rules: Vec<String>,
    #[serde(flatten)]
    _extra: HashMap<String, IgnoredAny>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub rule_names: Vec<String>,
}

impl Profile {
    pub fn list() -> Result<Vec<String>> {
        let root = Path::new("profiles");
        let mut names = Vec::new();
        if root.exists() {
            for entry in fs::read_dir(root)? {
                let entry = entry?;
                let path = entry.path();
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
        let path = Path::new("profiles").join(format!("{}.toml", name));
        let content = fs::read_to_string(&path)
            .with_context(|| format!("profil {} introuvable", name));
        let pf: ProfileFile = toml::from_str(&content?)
            .with_context(|| format!("parsing du profil {})", name));
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

    pub fn resolve_rules(&self) -> Result<Vec<crate::parser::Rule>> {
        let all = crate::parser::Rule::load_all?;
        let all = all?;
        let wanted: HashSet<String> = self.rule_names.iter().cloned().collect();
        if wanted.is_empty() {
            return Ok(all);
        }
        Ok(all.into_iter().filter(|r| wanted.contains(&r.name)).collect())
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
}
