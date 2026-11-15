pub mod runner;
pub mod reporter;

use crate::parser::Rule;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Violation {
    pub file: String,
    pub line: u32,
    pub rule: String,
    pub severity: String,
    pub message: String,
}

pub fn check_file(rule: &Rule, path: &Path, content: &str) -> Vec<Violation> {
    let mut violations = Vec::new();
    if let Some(ref pattern) = rule.pattern {
        if pattern.is_empty() {
            return violations;
        }
        if let Ok(re) = regex::Regex::new(pattern) {
            for (i, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    violations.push(Violation {
                        file: path.display().to_string(),
                        line: (i + 1) as u32,
                        rule: rule.name.clone(),
                        severity: rule.severity clone(),
                        message: rule.description clone(),
                    });
                }
            }
        }
    }
    violations
}
