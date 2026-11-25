//! Export SARIF 2.1.0 (spécification OASIS).

use crate::engine::runner::Violation;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct SarifLog {
    #[serde(rename = "$schema")]
    schema: &'static str,
    version: &'static str,
    runs: Vec<SarifRun>,
}

#[derive(Serialize)]
struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

#[derive(Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifDriver {
    name: &'static str,
    version: &'static str,
    information_uri: &'static str,
    rules: Vec<SarifRule>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifRule {
    id: String,
    name: String,
    default_configuration: SarifConfiguration,
}

#[derive(Serialize)]
struct SarifConfiguration {
    level: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifResult {
    rule_id: String,
    rule_index: usize,
    level: &'static str,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
struct SarifMessage {
    text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifLocation {
    physical_location: SarifPhysicalLocation,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifPhysicalLocation {
    artifact_location: SarifArtifactLocation,
    region: SarifRegion,
}

#[derive(Serialize)]
struct SarifArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifRegion {
    start_line: usize,
}

/// Niveau SARIF pour une sévérité AlphaRust (`info` → `note`).
fn level(severity: &str) -> &'static str {
    match severity {
        "error" => "error",
        "warning" => "warning",
        _ => "note",
    }
}

fn uri(file: &str) -> String {
    file.trim_start_matches("./").replace('\\', "/")
}

pub fn to_sarif(violations: &[Violation]) -> String {
    // Une entrée `rules[]` par règle effectivement signalée ; `ruleIndex` y renvoie.
    let mut index: BTreeMap<&str, usize> = BTreeMap::new();
    let mut rules = Vec::new();
    for v in violations {
        if !index.contains_key(v.rule_id.as_str()) {
            index.insert(&v.rule_id, rules.len());
            rules.push(SarifRule {
                id: v.rule_id.clone(),
                name: v.rule.clone(),
                default_configuration: SarifConfiguration {
                    level: level(&v.severity),
                },
            });
        }
    }
    let results = violations
        .iter()
        .map(|v| SarifResult {
            rule_id: v.rule_id.clone(),
            rule_index: index[v.rule_id.as_str()],
            level: level(&v.severity),
            message: SarifMessage {
                text: v.message.clone(),
            },
            locations: vec![SarifLocation {
                physical_location: SarifPhysicalLocation {
                    artifact_location: SarifArtifactLocation { uri: uri(&v.file) },
                    region: SarifRegion { start_line: v.line },
                },
            }],
        })
        .collect();

    let log = SarifLog {
        // alpharust-ignore: no-hardcoded-urls
        schema: "https://json.schemastore.org/sarif-2.1.0.json",
        version: "2.1.0",
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "alpharust",
                    version: env!("CARGO_PKG_VERSION"),
                    // alpharust-ignore: no-hardcoded-urls
                    information_uri: "https://github.com/Ishaq74/alpharust",
                    rules,
                },
            },
            results,
        }],
    };
    serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(rule: &str, sev: &str, line: usize) -> Violation {
        Violation {
            file: "./src\\a.rs".into(),
            line,
            rule: rule.into(),
            rule_id: format!("core/{rule}"),
            message: "m".into(),
            severity: sev.into(),
        }
    }

    #[test]
    fn valid_sarif_structure() {
        let out = to_sarif(&[v("a", "error", 3), v("b", "info", 4), v("a", "error", 9)]);
        let json: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(json["version"], "2.1.0");
        let run = &json["runs"][0];
        assert_eq!(run["tool"]["driver"]["name"], "alpharust");
        assert_eq!(run["tool"]["driver"]["rules"].as_array().unwrap().len(), 2);
        let results = run["results"].as_array().unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0]["ruleId"], "core/a");
        assert_eq!(results[0]["ruleIndex"], 0);
        assert_eq!(results[1]["level"], "note");
        assert_eq!(results[2]["ruleIndex"], 0);
        let loc = &results[0]["locations"][0]["physicalLocation"];
        assert_eq!(loc["artifactLocation"]["uri"], "src/a.rs");
        assert_eq!(loc["region"]["startLine"], 3);
    }

    #[test]
    fn empty_results_are_valid() {
        let json: serde_json::Value = serde_json::from_str(&to_sarif(&[])).unwrap();
        assert!(json["runs"][0]["results"].as_array().unwrap().is_empty());
    }
}
