//! Cohérence des manifestes de dépendances (package.json, Cargo.toml, composer.json, requirements.txt).

use super::{FileCtx, Finding};
use std::collections::BTreeMap;

fn line_for(content: &str, needle: &str) -> usize {
    content
        .lines()
        .position(|l| l.contains(needle))
        .map_or(1, |i| i + 1)
}

fn is_wildcard(v: &str) -> bool {
    let v = v.trim();
    v == "*" || v.eq_ignore_ascii_case("latest") || v.is_empty()
}

pub fn consistency(ctx: &FileCtx) -> Vec<Finding> {
    let name = ctx
        .path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    match name.as_str() {
        "package.json" => json_manifest(
            ctx.content,
            &[
                "dependencies",
                "devDependencies",
                "peerDependencies",
                "optionalDependencies",
            ],
            &["devDependencies"],
        ),
        "composer.json" => {
            json_manifest(ctx.content, &["require", "require-dev"], &["require-dev"])
        }
        "cargo.toml" => cargo(ctx.content),
        "requirements.txt" => requirements(ctx.content),
        _ => Vec::new(),
    }
}

fn json_manifest(content: &str, sections: &[&str], dev: &[&str]) -> Vec<Finding> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(content) else {
        return vec![Finding::new(1, "manifeste JSON invalide")];
    };
    let mut out = Vec::new();
    let mut seen: BTreeMap<String, (String, String)> = BTreeMap::new();
    for section in sections {
        let Some(map) = value.get(section).and_then(|v| v.as_object()) else {
            continue;
        };
        for (dep, ver) in map {
            let ver = ver.as_str().unwrap_or("").to_string();
            if is_wildcard(&ver) && !dep.starts_with("php") && !dep.starts_with("ext-") {
                out.push(Finding::new(
                    line_for(content, &format!("\"{}\"", dep)),
                    format!("`{}` : version non contrainte (`{}`)", dep, ver),
                ));
            }
            if let Some((prev_section, prev_ver)) = seen.get(dep) {
                let involves_dev = dev.contains(section) || dev.contains(&prev_section.as_str());
                if involves_dev || prev_ver != &ver {
                    out.push(Finding::new(
                        line_for(content, &format!("\"{}\"", dep)),
                        format!(
                            "`{}` déclaré dans `{}` ({}) et `{}` ({})",
                            dep, prev_section, prev_ver, section, ver
                        ),
                    ));
                }
            } else {
                seen.insert(dep.clone(), (section.to_string(), ver));
            }
        }
    }
    out
}

fn cargo(content: &str) -> Vec<Finding> {
    let Ok(value) = content.parse::<toml::Table>() else {
        return vec![Finding::new(1, "Cargo.toml invalide")];
    };
    let mut out = Vec::new();
    let mut seen: BTreeMap<String, (String, String)> = BTreeMap::new();
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let Some(table) = value.get(section).and_then(|v| v.as_table()) else {
            continue;
        };
        for (dep, spec) in table {
            let ver = match spec {
                toml::Value::String(s) => s.clone(),
                toml::Value::Table(t) => t
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                _ => String::new(),
            };
            let has_source = spec.as_table().is_some_and(|t| {
                t.contains_key("path") || t.contains_key("git") || t.contains_key("workspace")
            });
            if !has_source && is_wildcard(&ver) {
                out.push(Finding::new(
                    line_for(content, dep),
                    format!("`{}` : version joker ou absente", dep),
                ));
            }
            if let Some((prev_section, prev_ver)) = seen.get(dep) {
                if prev_ver != &ver {
                    out.push(Finding::new(
                        line_for(content, dep),
                        format!(
                            "`{}` : versions divergentes entre [{}] ({}) et [{}] ({})",
                            dep, prev_section, prev_ver, section, ver
                        ),
                    ));
                }
            } else {
                seen.insert(dep.clone(), (section.to_string(), ver));
            }
        }
    }
    out
}

fn requirements(content: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    for (i, line) in content.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() || line.starts_with('-') {
            continue;
        }
        let split = line
            .find(['=', '<', '>', '~', '!', ';', ' '])
            .unwrap_or(line.len());
        let pkg = line[..split]
            .split('[')
            .next()
            .unwrap_or("")
            .to_lowercase()
            .replace('_', "-");
        let spec = line[split..].trim().to_string();
        if let Some(prev) = seen.get(&pkg) {
            out.push(Finding::new(
                i + 1,
                format!("`{}` déclaré deux fois ({} puis {})", pkg, prev, spec),
            ));
        } else {
            seen.insert(pkg, spec);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::Args;
    use std::collections::BTreeMap;
    use std::path::Path;

    fn run(file: &str, content: &str) -> Vec<Finding> {
        let args = BTreeMap::new();
        consistency(&FileCtx {
            path: Path::new(file),
            content,
            args: Args(&args),
        })
    }

    #[test]
    fn package_json_wildcards_and_duplicates() {
        let json = "{\n \"dependencies\": {\"a\": \"*\", \"b\": \"^1.0.0\"},\n \"devDependencies\": {\"b\": \"^1.0.0\"}\n}";
        let f = run("package.json", json);
        assert_eq!(f.len(), 2);
        assert!(f[0].message.contains("version non contrainte"));
        assert!(f[1].message.contains("devDependencies"));
    }

    #[test]
    fn clean_package_json() {
        let json = "{\"dependencies\": {\"a\": \"^1\"}, \"devDependencies\": {\"b\": \"^2\"}}";
        assert!(run("package.json", json).is_empty());
    }

    #[test]
    fn cargo_wildcard_and_divergence() {
        let toml = "[dependencies]\nserde = \"1\"\nfoo = \"*\"\nlocal = { path = \"../x\" }\n[dev-dependencies]\nserde = \"2\"\n";
        let f = run("Cargo.toml", toml);
        assert_eq!(f.len(), 2);
    }

    #[test]
    fn composer_ignores_php_constraint() {
        let json = "{\"require\": {\"php\": \"*\", \"laravel/framework\": \"^11.0\"}}";
        assert!(run("composer.json", json).is_empty());
    }

    #[test]
    fn requirements_duplicates() {
        let f = run("requirements.txt", "Django>=5\nwagtail>=6\ndjango<6\n# c\n");
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].line, 3);
    }

    #[test]
    fn other_files_are_ignored() {
        assert!(run("main.rs", "*").is_empty());
    }
}
