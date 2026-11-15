use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub name: String,
    pub apply: String,
    pub severity: String,
    pub pattern: Option<String>,
    pub description: String,
    pub path: String,
}

impl Rule {
    pub fn from_markdown(path: &Path, content: &str) -> anyhow::Result<Self> {
        let (front, body) = split_frontmatter(content)?;
        let fm: Frontmatter = serde_yaml::from_str(&front)?;
        Ok(Rule {
            name: fm.name,
            apply: fm.apply,
            severity: fm.severity,
            pattern: fm.pattern,
            description: body.trim().to_string(),
            path: path.display().to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    name: String,
    apply: String,
    severity: String,
    pattern: Option<String>,
}

fn split_frontmatter(content: &str) -> anyhow::Result<(String, String)> {
    let content = content.trim();
    if !content.starts_with("---") {
        anyhow::bail!("Missing frontmatter");
    }
    let rest = &content[3..];
    if let Some(end) = rest.find("\n---") {
        let front = &rest[..end];
        let body = &rest[end + 4..];
        Ok((front.trim().to_string(), body.to_string()))
    } else {
        anyhow::bail!("Unclosed frontmatter")
    }
}
