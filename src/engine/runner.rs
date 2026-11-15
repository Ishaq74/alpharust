use crate::parser::Rule;
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Violation {
    pub file: String,
    pub line: usize,
    pub rule: String,
    pub message: String,
    pub severity: String,
}

fn matches_apply(apply: &str, path: &Path) -> bool {
    if apply.trim() == "*" {
        return true;
    }
    let s = path.to_string_lossy().to_lowercase();
    for part in apply.split(',') {
        let part = part.trim().to_lowercase();
        if let Some(ext) = part.strip_prefix("*.") {
            if s.ends_with(&format!(".{}", ext)) {
                return true;
            }
        } else if !part.is_empty() && s.contains(&part) {
            return true;
        }
    }
    false
}

pub fn run(rules: &[Rule], target: &Path) -> anyhow::Result<Vec<Violation>> {
    let mut violations = Vec::new();

    for entry in WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.components().any(|c| c.as_os_str() == "target" || c.as_os_str() == ".git") {
            continue;
        }

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for rule in rules {
            if !matches_apply(&rule.apply, path) {
                continue;
            }
            let Some(pat) = &rule.pattern else { continue };
            if pat.is_empty() {
                continue;
            }
            let re = match Regex::new(pat) {
                Ok(r) => r,
                Err(_) => continue,
            };
            for (i, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    violations.push(Violation {
                        file: path.display().to_string(),
                        line: i + 1,
                        rule: rule.name.clone(),
                        message: rule.name.clone(),
                        severity: rule.severity.clone(),
                    });
                }
            }
        }
    }
    Ok(violations)
}
