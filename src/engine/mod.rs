//! Moteur : composition des règles, exécution, rapports.
//!
//! [`lint`] est le point d'entrée partagé par la CLI, la TUI et la GUI.

pub mod composer;
pub mod reporter;
pub mod runner;
pub mod validate;

use crate::config::Config;
use crate::parser::Rule;
use crate::profiles::Profile;
use anyhow::Result;
use std::path::Path;

pub use runner::Violation;

/// Résultat complet d'un lint.
#[derive(Debug, Default)]
pub struct LintReport {
    pub violations: Vec<Violation>,
    pub rules_used: usize,
    pub files_scanned: usize,
    pub profile: Option<String>,
    pub config: Config,
}

impl LintReport {
    pub fn count(&self, severity: &str) -> usize {
        self.violations
            .iter()
            .filter(|v| v.severity == severity)
            .count()
    }

    /// Code de sortie : 1 si une violation atteint le seuil `fail_on` (défaut : error).
    pub fn exit_code(&self, fail_on: Option<&str>) -> i32 {
        let threshold = fail_on
            .or(self.config.fail_on.as_deref())
            .unwrap_or("error");
        let rank = |s: &str| match s {
            "error" => 3,
            "warning" => 2,
            _ => 1,
        };
        i32::from(
            self.violations
                .iter()
                .any(|v| rank(&v.severity) >= rank(threshold)),
        )
    }
}

/// Lint `path` : config (`.alpharust.toml` du dossier analysé, sinon du dossier courant),
/// profil (argument > config), règles composées, exécution.
pub fn lint(path: &Path, profile: Option<&str>) -> Result<LintReport> {
    let config = if path.is_dir() && path.join(crate::config::CONFIG_FILE).exists() {
        Config::load_from(path)?
    } else {
        Config::load()?
    };
    let profile_name = profile
        .map(str::to_string)
        .or_else(|| config.profile.clone());
    let rules = match &profile_name {
        Some(p) => Profile::load(p)?.resolve_rules()?,
        None => Rule::load_all()?,
    };
    let rules = composer::compose(config.apply(rules));
    let report = runner::run_with(
        &rules,
        path,
        &runner::Options {
            exclude: config.exclude.clone(),
        },
    )?;
    Ok(LintReport {
        violations: report.violations,
        rules_used: rules.len(),
        files_scanned: report.files_scanned,
        profile: profile_name,
        config,
    })
}
