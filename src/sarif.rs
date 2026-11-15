use crate::engine::runner::Violation;
use serde::Serialize;

#[derive(Serialize)]
struct SarifLog {
    version: String,
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
struct SarifDriver {
    name: String,
}

#[derive(Serialize)]
struct SarifResult {
    ruleId: String,
    level: String,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
}

#[derive(Serialize)]
struct SarifMessage {
    text: String,
}

#[derive(Serialize)]
struct SarifLocation {
    physicalLocation: SarifPhysicalLocation,
}

#[derive(Serialize)]
struct SarifPhysicalLocation {
    artifactLocation: SarifArtifactLocation,
    region: SarifRegion,
}

#[derive(Serialize)]
struct SarifArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
struct SarifRegion {
    startLine: usize,
}

pub fn to_sarif(violations: &[Violation]) -> String {
    let results = violations
        .iter()
        .map(|v| SarifResult {
            ruleId: v.rule.clone(),
            level: if v.severity == "error" { "error".into() } else { "warning".into() },
            message: SarifMessage { text: v.message.clone() },
            locations: vec![SarifLocation {
                physicalLocation: SarifPhysicalLocation {
                    artifactLocation: SarifArtifactLocation { uri: v.file.clone() },
                    region: SarifRegion { startLine: v.line },
                },
            }],
        })
        .collect();

    let log = SarifLog {
        version: "2.1.0".into(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver { name: "alpharust".into() },
            },
            results,
        }],
    };

    serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".into())
}
