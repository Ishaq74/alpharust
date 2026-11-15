use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub rules: Vec<String>,
}

impl Profile {
    pub fn load(name: &str) -> Result<Self> {
        let path = Path::new("profiles").join(format!("{}.toml", name));
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("loading profile {}", name))?;
        toml::from_str(&content).context("parsing profile")
    }

    pub fn list() -> Result<Vec<String>> {
        let mut names = Vec::new();
        let dir = Path::new("profiles");
        if dir.exists() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                if let Some(stem) = entry.path().file_stem().and_then(|s| s.to_str()) {
                    names.push(stem.to_string());
                }
            }
        }
        Ok(names)
    }

    pub fn resolve_rules(&self) -> Result<Vec<crate::parser::Rule>> {
        let all = crate::parser::Rule::load_all()?;
        let mut selected = Vec::new();
        for pattern in &self.rules {
            if let Some(prefix) = pattern.strip_suffix("/*") {
                for r in &all {
                    if r.path.to_string_lossy().contains(prefix) {
                        selected.push(r.clone());
                    }
                }
            } else {
                for r in &all {
                    if r.name == *pattern {
                        selected.push(r.clone());
                    }
                }
            }
        }
        Ok(selected)
    }
}
