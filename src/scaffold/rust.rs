//! Rust : `cargo new` officiel, `cargo add` pour les dépendances, puis couche AlphaRust (tests, CI, docs).

use super::exec::{Cmd, Cwd};
use super::tools::{self, ToolStatus};
use super::{Internal, Phase, Request, Step};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CrateKind {
    #[default]
    Bin,
    Lib,
}

/// Dépendances proposées (nom, fonctionnalités à activer).
pub const DEPS: &[(&str, &[&str])] = &[
    ("clap", &["derive"]),
    ("anyhow", &[]),
    ("thiserror", &[]),
    ("serde", &["derive"]),
    ("serde_json", &[]),
    ("tokio", &["full"]),
    ("tracing", &[]),
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RustOptions {
    pub kind: CrateKind,
    /// `2021` ou `2024` (défaut : celui de cargo).
    pub edition: Option<String>,
    pub deps: Vec<String>,
}

impl Default for RustOptions {
    fn default() -> Self {
        Self {
            kind: CrateKind::Bin,
            edition: None,
            deps: vec![],
        }
    }
}

impl RustOptions {
    pub fn validate(&self) -> Result<()> {
        if let Some(e) = &self.edition {
            if !["2018", "2021", "2024"].contains(&e.as_str()) {
                bail!("édition inconnue `{e}` (2018, 2021 ou 2024)");
            }
        }
        for d in &self.deps {
            if !DEPS.iter().any(|(n, _)| n == d) {
                bail!(
                    "dépendance inconnue `{}` (disponibles : {})",
                    d,
                    DEPS.iter().map(|(n, _)| *n).collect::<Vec<_>>().join(", ")
                );
            }
        }
        Ok(())
    }
}

pub fn tools(_req: &Request, _o: &RustOptions) -> Vec<ToolStatus> {
    vec![
        tools::detect(&tools::CARGO, true),
        ToolStatus {
            id: "clippy".into(),
            label: "clippy".into(),
            required: false,
            ..tools::detect(
                &tools::ToolSpec {
                    id: "cargo-clippy",
                    label: "clippy",
                    programs: &["cargo-clippy"],
                    version_args: &["--version"],
                    min: None,
                    hint: "rustup component add clippy",
                },
                false,
            )
        },
    ]
}

pub fn steps(req: &Request, o: &RustOptions) -> Vec<Step> {
    let mut args = vec![
        "new".to_string(),
        req.folder(),
        "--vcs".into(),
        "none".into(),
        if o.kind == CrateKind::Lib {
            "--lib"
        } else {
            "--bin"
        }
        .into(),
    ];
    if let Some(e) = &o.edition {
        args.push("--edition".into());
        args.push(e.clone());
    }
    let mut steps = vec![
        Step::internal(
            "prepare",
            Phase::Prepare,
            "Contrôle de la destination",
            Internal::Prepare,
        ),
        Step::run(
            "create",
            Phase::Scaffold,
            "Créer le crate avec cargo new",
            Cmd::owned("cargo", args, Cwd::Parent),
        ),
    ];
    for d in &o.deps {
        let feats = DEPS
            .iter()
            .find(|(n, _)| n == d)
            .map(|(_, f)| *f)
            .unwrap_or(&[]);
        let mut a = vec!["add".to_string(), d.clone()];
        if !feats.is_empty() {
            a.push("--features".into());
            a.push(feats.join(","));
        }
        steps.push(Step::run(
            "deps",
            Phase::Install,
            &format!("Ajouter {d} (cargo add)"),
            Cmd::owned("cargo", a, Cwd::Project),
        ));
    }
    steps.push(Step::internal(
        "layer",
        Phase::Layer,
        "Appliquer la couche AlphaRust (tests, CI, docs, config qualité)",
        Internal::Layer(vec!["common", "rust"]),
    ));
    steps.push(Step::internal(
        "manifest",
        Phase::Configure,
        "Écrire le manifeste du projet",
        Internal::WriteManifest,
    ));
    if req.verify {
        steps.push(Step::run(
            "test",
            Phase::Verify,
            "Lancer les tests (cargo test)",
            Cmd::new("cargo", &["test"], Cwd::Project),
        ));
        steps.push(
            Step::run(
                "fmt",
                Phase::Verify,
                "Format (cargo fmt --check)",
                Cmd::new("cargo", &["fmt", "--check"], Cwd::Project),
            )
            .soft(),
        );
        steps.push(
            Step::run(
                "clippy",
                Phase::Verify,
                "Clippy (-D warnings)",
                Cmd::new(
                    "cargo",
                    &["clippy", "--all-targets", "--", "-D", "warnings"],
                    Cwd::Project,
                ),
            )
            .soft(),
        );
        steps.push(
            Step::internal(
                "lint",
                Phase::Verify,
                "Lint AlphaRust du projet généré",
                Internal::Lint,
            )
            .soft(),
        );
    }
    if req.git {
        steps.push(
            Step::internal(
                "git",
                Phase::Finish,
                "Dépôt Git et commit initial",
                Internal::GitInit,
            )
            .soft(),
        );
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scaffold::{Action, Framework, Options};

    #[test]
    fn cargo_new_flags_and_deps() {
        let o = RustOptions {
            kind: CrateKind::Lib,
            edition: Some("2021".into()),
            deps: vec!["clap".into(), "anyhow".into()],
        };
        let mut r = Request::new(Framework::Rust, "demo");
        r.options = Options::Rust(o.clone());
        let s = steps(&r, &o);
        let create = match &s.iter().find(|s| s.id == "create").unwrap().action {
            Action::Run(c) => c.display(),
            _ => panic!(),
        };
        assert_eq!(create, "cargo new demo --vcs none --lib --edition 2021");
        let adds: Vec<String> = s
            .iter()
            .filter(|s| s.id == "deps")
            .map(|s| s.command().unwrap())
            .collect();
        assert_eq!(
            adds,
            ["cargo add clap --features derive", "cargo add anyhow"]
        );
    }

    #[test]
    fn validation() {
        assert!(RustOptions {
            edition: Some("1999".into()),
            ..Default::default()
        }
        .validate()
        .is_err());
        assert!(RustOptions {
            deps: vec!["left-pad".into()],
            ..Default::default()
        }
        .validate()
        .is_err());
        assert!(RustOptions::default().validate().is_ok());
    }
}
