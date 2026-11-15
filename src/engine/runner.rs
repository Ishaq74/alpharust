use crate::engine::{check_file, Violation};
use crate::parser::Rule;
use std::path::Path;
use walkdir::WalkDir;

fn matches_apply(apply: &str, path: &Path) -> bool {
    if apply.trim() == "*" {
        return true;
    }
    let s = path.to_string_lossy().to_lowercase();
    for part in apply.split(',') {
        let part = part.trim().to_lowercase();
        if let Some(ext) = part.strip_prefix("*.") {
            if s.ends_with(&format!".{ext}")) {
                return true;
            }
        } else if !part.is_empty() && s.contains(&part) {
            return true;
        }
    }
    false
}

fn is_presence_rule(rule: &Rule) -> bool {
    rule.pattern.as_ref().map(|p| p.is_empty()).unwrap_or(true)
}

fn check_presence(rule: &Rule, root: &Path) -> Vec<Violation> {
    let mut violations = Vec::new();
    let keywords: Vec<String> = match rule.name.as_str() {
        "migrations-required" => vec!["migrations".into()],
        "required-readme" | "has-installation" | "has-usage" | "has-license" => vec!["readme".into()],
        "api-docs-present" => vec!["api".into(), "docs".into(), "swagger".into(), "openapi".into()],
        "unit-tests-required" => vec!["test".into(), "tests".into(), "spec".into()],
        _ => vec![rule.name.replace('-', "_")],
    };
    let found = WalkDir::new(root)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .any(|e| {
            let name = e.file_name().to_string_lossy().to_lowercase();
            keywords.iter().any(|k| name.contains(k))
        });
    if !found {
        violations.push(Violation {
            file: root.display().to_string(),
            line: 0,
            rule: rule.name clone(),
            severity: rule.severity clone(),
            message: format!("présence non satisfaite : aucun fichier lié à {} trouvé", rule.name),
        });
    }
    violations
}

pub fn run(rules: &Vec<Rule>, root: &Path) -> anyhow::Result<Vec<Violation>> {
    let mut violations = Vec::new();
    for rule in rules {
        if is_presence_rule(rule) {
            violations.extend(check_presence(rule, root));
            continue;
        }
        for entry in WalkDir::new(root).max_depth(8).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let s = path.to_string_lossy();
            if s.contains("/.git/") || s.contains("/target/") {
                continue;
            }
            if !matches_apply(&rule.apply, path) {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(path) {
                violations.extend(check_file(rule, path, &content));
            }
        }
    }
    Ok(violations)
}
