use crate::parser::Rule;
use anyhow::Result;
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Violation {
    pub file: String,
    pub line: usize,
    pub rule: String,
    pub message: String,
    pub severity: String,
}

pub fn run(rules: &[Rule], target: &Path) -> Result<Vec<Violation>> {
    let mut violations = Vec::new();

    for entry in WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.components().any(|c| c.as_os_str() == "target") {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for rule in rules {
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
