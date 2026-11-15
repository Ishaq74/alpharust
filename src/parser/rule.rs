use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub name: String,
    pub apply: String,
    pub severity: String,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(skip)]
    pub description: String,
    #[serde(skip)]
    pub path: PathBuf,
}

impl Rule {
    pub fn load_all() -> Result<Vec<Self>> {
        let mut rules = Vec::new();
        let root = Path::new("rules");
        if !root.exists() {
            return Ok(rules);
        }
        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(rule) = Self::from_file(path) {
                    rules.push(rule);
                }
            }
        }
        Ok(rules)
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.display()))?;
        Self::from_markdown(path, &content)
    }

    pub fn from_markdown(path: &Path, content: &str) -> Result<Self> {
        let (front, body) = split_frontmatter(content)?;
        let mut rule: Rule = serde_yaml::from_str(&front)
            .with_context(|| format!("parsing frontmatter in {}", path.display()))?;
        rule.description = body.trim().to_string();
        rule.path = path.to_path_buf();
        Ok(rule)
    }
}

fn split_frontmatter(content: &str) -> Result<(String, String)> {
    let content = content.trim_start();
    if !content.starts_with("---") {
        anyhow::bail!("missing frontmatter");
    }
    let rest = &content[3..];
    let end = rest
        .find("\n---")
        .ok_or_else(|| anyhow::anyhow!("unclosed frontmatter"))?;
    let front = rest[..end].trim().to_string();
    let body = rest[end + 4..].to_string();
    Ok((front, body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_rule() {
        let md = r#"---
name: test-rule
apply: *
severity: error
pattern: "foo"
---

Description here.
"#;
        let rule = Rule::from_markdown(Path::new("test.md"), md).unwrap();
        assert_eq!(rule.name, "test-rule");
        assert_eq!(rule.severity, "error");
        assert!(rule.pattern.is_some());
    }
}
