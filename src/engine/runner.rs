use crate::parser::Rule;
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

pub struct Violation {
    pub file: String,
    pub line: usize,
    pub rule: String,
    pub message: String,
}

pub fn run(rules: &[Rule], target: &Path) -> Vec<Violation> {
    let mut violations = Vec::new();

    for entry in WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() { continue; }
        if let Ok(content) = std::fs::read_to_string(path) {
            for rule in rules {
                if let Some(pat) = &rule.pattern {
                    if let Ok(re) = Regex::new(pat) {
                        for (i, line) in content.lines().enumerate() {
                            if re.is_match(line) {
                                violations.push(Violation {
                                    file: path.display().to_string(),
                                    line: i + 1,
                                    rule: rule.name.clone(),
                                    message: format!("Violation of {}", rule.name),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    violations
}
