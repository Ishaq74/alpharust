//! Échafaudage de projets avec les **outils officiels** (create-astro, composer create-project,
//! `wagtail start`, `cargo new`), puis couche de templates AlphaRust, puis vérification.
//!
//! Pipeline : `preflight → plan (commandes exactes, aperçu) → exécution streamée → couche → vérification → rapport`.
//! La même [`Request`] sert la CLI (flags et prompts), la TUI et la GUI.

pub mod astro;
pub mod exec;
pub mod internal;
pub mod laravel;
pub mod layer;
pub mod rust;
pub mod tools;
pub mod wagtail;

use crate::generators::validate_name;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

pub use exec::{Cmd, Cwd};
pub use tools::ToolStatus;

/// Nom du fichier qui marque un projet généré par AlphaRust (et en garde la configuration).
pub const MANIFEST: &str = ".alpharust-project.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Framework {
    Astro,
    Laravel,
    Wagtail,
    Rust,
}

impl Framework {
    pub const ALL: [Framework; 4] = [
        Framework::Astro,
        Framework::Laravel,
        Framework::Wagtail,
        Framework::Rust,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Framework::Astro => "astro",
            Framework::Laravel => "laravel",
            Framework::Wagtail => "wagtail",
            Framework::Rust => "rust",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Framework::Astro => "Astro",
            Framework::Laravel => "Laravel",
            Framework::Wagtail => "Wagtail (Django)",
            Framework::Rust => "Rust",
        }
    }

    pub fn official_tool(self) -> &'static str {
        match self {
            Framework::Astro => "create-astro",
            Framework::Laravel => "composer create-project laravel/laravel",
            Framework::Wagtail => "wagtail start",
            Framework::Rust => "cargo new",
        }
    }

    pub fn parse(s: &str) -> Option<Framework> {
        match s.to_lowercase().as_str() {
            "astro" => Some(Framework::Astro),
            "laravel" => Some(Framework::Laravel),
            "wagtail" | "django" => Some(Framework::Wagtail),
            "rust" | "bootstrap" | "init" => Some(Framework::Rust),
            _ => None,
        }
    }
}

/// Options propres à chaque framework (désérialisées à plat depuis la GUI : `{"framework":"astro","template":…}`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "framework", rename_all = "lowercase")]
pub enum Options {
    Astro(astro::AstroOptions),
    Laravel(laravel::LaravelOptions),
    Wagtail(wagtail::WagtailOptions),
    Rust(rust::RustOptions),
}

impl Options {
    pub fn default_for(f: Framework) -> Options {
        match f {
            Framework::Astro => Options::Astro(Default::default()),
            Framework::Laravel => Options::Laravel(Default::default()),
            Framework::Wagtail => Options::Wagtail(Default::default()),
            Framework::Rust => Options::Rust(Default::default()),
        }
    }

    pub fn framework(&self) -> Framework {
        match self {
            Options::Astro(_) => Framework::Astro,
            Options::Laravel(_) => Framework::Laravel,
            Options::Wagtail(_) => Framework::Wagtail,
            Options::Rust(_) => Framework::Rust,
        }
    }
}

fn yes() -> bool {
    true
}

/// Demande de génération.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    /// Nom du projet (un seul segment : lettres, chiffres, `-`, `_`, `.`).
    pub name: String,
    /// Dossier du projet (défaut : `sites/<name>`).
    #[serde(default)]
    pub path: Option<PathBuf>,
    /// Remplacer un projet AlphaRust existant (jamais un dossier étranger).
    #[serde(default)]
    pub force: bool,
    /// `git init` + commit initial.
    #[serde(default = "yes")]
    pub git: bool,
    /// Installer les dépendances (`npm install`, `composer install`, `pip install`…).
    #[serde(default = "yes")]
    pub install: bool,
    /// Vérifier le résultat (build, tests, checks) puis lancer le lint AlphaRust.
    #[serde(default = "yes")]
    pub verify: bool,
    #[serde(flatten)]
    pub options: Options,
}

impl Request {
    pub fn new(framework: Framework, name: &str) -> Request {
        Request {
            name: name.to_string(),
            path: None,
            force: false,
            git: true,
            install: true,
            verify: true,
            options: Options::default_for(framework),
        }
    }

    pub fn framework(&self) -> Framework {
        self.options.framework()
    }

    /// Dossier du projet.
    pub fn target(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_else(|| crate::generators::default_target(&self.name))
    }

    /// Nom du dossier du projet (dernier segment de la destination).
    pub fn folder(&self) -> String {
        self.target()
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.name.clone())
    }

    /// Valide la demande (nom, destination, options) sans rien écrire.
    pub fn validate(&self) -> Result<()> {
        validate_name(&self.name)?;
        validate_name(&self.folder())?;
        match &self.options {
            Options::Astro(o) => o.validate(self.install)?,
            Options::Laravel(o) => o.validate()?,
            Options::Wagtail(o) => {
                o.validate()?;
                if !self.install {
                    bail!("Wagtail exige l'installation (`wagtail start` a besoin de wagtail dans le venv) : retirer --no-install");
                }
            }
            Options::Rust(o) => o.validate()?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Prepare,
    Scaffold,
    Install,
    Configure,
    Layer,
    Verify,
    Finish,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::Prepare => "préparation",
            Phase::Scaffold => "outil officiel",
            Phase::Install => "installation",
            Phase::Configure => "configuration",
            Phase::Layer => "couche AlphaRust",
            Phase::Verify => "vérification",
            Phase::Finish => "finalisation",
        }
    }
}

/// Actions internes (sans sous-processus).
#[derive(Debug, Clone, PartialEq)]
pub enum Internal {
    /// Contrôle la destination ; supprime un ancien projet AlphaRust si `force`.
    Prepare,
    /// Crée le dossier du projet.
    Mkdir,
    /// Applique des couches de templates.
    Layer(Vec<&'static str>),
    AstroTsconfig(astro::TsLevel),
    AstroSite(String),
    PackageScripts(Vec<(&'static str, &'static str)>),
    LaravelEnv(laravel::Database),
    LaravelRoutes,
    WriteManifest,
    GitInit,
    Lint,
    Freeze,
    /// Ajoute des paquets à requirements.txt (Wagtail).
    Requirements(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Run(Cmd),
    Do(Internal),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub id: &'static str,
    pub phase: Phase,
    pub label: String,
    pub action: Action,
    /// Un échec arrête la génération (sinon : avertissement).
    pub fatal: bool,
}

impl Step {
    pub fn run(id: &'static str, phase: Phase, label: &str, cmd: Cmd) -> Step {
        Step {
            id,
            phase,
            label: label.into(),
            action: Action::Run(cmd),
            fatal: true,
        }
    }
    pub fn internal(id: &'static str, phase: Phase, label: &str, what: Internal) -> Step {
        Step {
            id,
            phase,
            label: label.into(),
            action: Action::Do(what),
            fatal: true,
        }
    }
    pub fn soft(mut self) -> Step {
        self.fatal = false;
        self
    }
    pub fn command(&self) -> Option<String> {
        match &self.action {
            Action::Run(c) => Some(c.display()),
            Action::Do(_) => None,
        }
    }
}

/// Plan d'exécution : exactement ce qui sera lancé.
#[derive(Debug, Clone)]
pub struct Plan {
    pub framework: Framework,
    pub target: PathBuf,
    pub steps: Vec<Step>,
}

/// Version sérialisable d'une étape (aperçu).
#[derive(Debug, Clone, Serialize)]
pub struct StepInfo {
    pub id: String,
    pub phase: Phase,
    pub phase_label: &'static str,
    pub label: String,
    pub command: Option<String>,
    pub cwd: Option<Cwd>,
    pub fatal: bool,
}

impl Plan {
    pub fn infos(&self) -> Vec<StepInfo> {
        self.steps
            .iter()
            .map(|s| StepInfo {
                id: s.id.to_string(),
                phase: s.phase,
                phase_label: s.phase.label(),
                label: s.label.clone(),
                command: s.command(),
                cwd: match &s.action {
                    Action::Run(c) => Some(c.cwd),
                    _ => None,
                },
                fatal: s.fatal,
            })
            .collect()
    }

    /// Aperçu texte : une ligne par étape avec sa commande exacte.
    pub fn preview(&self) -> String {
        let mut out = String::new();
        let parent = self
            .target
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        out.push_str(&format!("Projet : {}\n", self.target.display()));
        for (i, s) in self.steps.iter().enumerate() {
            out.push_str(&format!(
                "{:>2}. [{}] {}\n",
                i + 1,
                s.phase.label(),
                s.label
            ));
            if let Action::Run(c) = &s.action {
                let dir = match c.cwd {
                    Cwd::Parent => parent.clone(),
                    Cwd::Project => self.target.display().to_string(),
                };
                out.push_str(&format!("      $ {}   (dans {})\n", c.display(), dir));
            }
        }
        out
    }
}

/// Construit le plan d'une demande (sans effet de bord).
pub fn plan(req: &Request) -> Result<Plan> {
    req.validate()?;
    let target = req.target();
    let steps = match &req.options {
        Options::Astro(o) => astro::steps(req, o),
        Options::Laravel(o) => laravel::steps(req, o),
        Options::Wagtail(o) => wagtail::steps(req, o),
        Options::Rust(o) => rust::steps(req, o),
    };
    Ok(Plan {
        framework: req.framework(),
        target,
        steps,
    })
}

/// Outils nécessaires (ou recommandés) pour cette demande.
pub fn preflight(req: &Request) -> Vec<ToolStatus> {
    let mut v = match &req.options {
        Options::Astro(o) => astro::tools(req, o),
        Options::Laravel(o) => laravel::tools(req, o),
        Options::Wagtail(o) => wagtail::tools(req, o),
        Options::Rust(o) => rust::tools(req, o),
    };
    if req.git {
        v.push(tools::detect(&tools::GIT, true));
    }
    v
}

/// Erreur lisible si un outil requis manque ou est trop ancien.
pub fn preflight_check(tools: &[ToolStatus]) -> Result<()> {
    let bad: Vec<&ToolStatus> = tools.iter().filter(|t| t.required && !t.ok).collect();
    if bad.is_empty() {
        return Ok(());
    }
    let mut msg = String::from("outils requis manquants ou trop anciens :\n");
    for t in bad {
        let state = if !t.found {
            "introuvable".to_string()
        } else {
            format!(
                "version {} < {} requise",
                t.version.clone().unwrap_or_else(|| "?".into()),
                t.min.clone().unwrap_or_default()
            )
        };
        msg.push_str(&format!("  ✗ {} — {}\n    → {}\n", t.label, state, t.hint));
    }
    msg.push_str("Lancer `alpharust doctor` pour revérifier.");
    bail!(msg)
}

/// Événements émis pendant l'exécution (CLI : affichage ; GUI : SSE ; TUI : journal).
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Preflight {
        tools: Vec<ToolStatus>,
    },
    Plan {
        target: String,
        steps: Vec<StepInfo>,
    },
    StepStart {
        index: usize,
        total: usize,
        id: String,
        label: String,
        phase: Phase,
        command: Option<String>,
    },
    Log {
        index: usize,
        line: String,
    },
    StepEnd {
        index: usize,
        id: String,
        ok: bool,
        fatal: bool,
        millis: u128,
        note: Option<String>,
    },
    Done {
        report: Report,
    },
    Failed {
        message: String,
        step: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct StepResult {
    pub id: String,
    pub label: String,
    pub phase: Phase,
    pub ok: bool,
    pub fatal: bool,
    pub millis: u128,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct LintSummary {
    pub files: usize,
    pub rules: usize,
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
    pub profile: Option<String>,
    /// Premières violations (`fichier:ligne règle message`).
    pub sample: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub framework: Framework,
    pub name: String,
    pub target: String,
    pub ok: bool,
    pub steps: Vec<StepResult>,
    pub files: Vec<String>,
    pub lint: Option<LintSummary>,
    pub tools: Vec<ToolStatus>,
    pub next_steps: Vec<String>,
    pub elapsed_ms: u128,
}

impl Report {
    pub fn summary(&self) -> String {
        let failed: Vec<&StepResult> = self.steps.iter().filter(|s| !s.ok).collect();
        let mut s = format!(
            "✓ Projet {} « {} » prêt dans {} ({} fichiers, {:.1}s)",
            self.framework.label(),
            self.name,
            self.target,
            self.files.len(),
            self.elapsed_ms as f64 / 1000.0
        );
        if !failed.is_empty() {
            s.push_str(&format!(
                "\n⚠ {} étape(s) non bloquante(s) en échec : {}",
                failed.len(),
                failed
                    .iter()
                    .map(|f| f.label.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        s
    }
}

/// Exécute le plan. `sink` reçoit chaque événement ; `cancel` interrompt la commande en cours.
pub fn execute(req: &Request, sink: &mut dyn FnMut(Event), cancel: &AtomicBool) -> Result<Report> {
    let started = Instant::now();
    let plan = match plan(req) {
        Ok(p) => p,
        Err(e) => {
            sink(Event::Failed {
                message: e.to_string(),
                step: None,
            });
            return Err(e);
        }
    };
    let tools = preflight(req);
    sink(Event::Preflight {
        tools: tools.clone(),
    });
    if let Err(e) = preflight_check(&tools) {
        sink(Event::Failed {
            message: e.to_string(),
            step: Some("preflight".into()),
        });
        return Err(e);
    }
    sink(Event::Plan {
        target: plan.target.display().to_string(),
        steps: plan.infos(),
    });

    let mut state = internal::State::new(req, &plan);
    let total = plan.steps.len();
    let mut results: Vec<StepResult> = Vec::new();
    for (index, step) in plan.steps.iter().enumerate() {
        if cancel.load(Ordering::Relaxed) {
            let msg = "génération annulée".to_string();
            sink(Event::Failed {
                message: msg.clone(),
                step: Some(step.id.to_string()),
            });
            bail!(msg);
        }
        sink(Event::StepStart {
            index,
            total,
            id: step.id.to_string(),
            label: step.label.clone(),
            phase: step.phase,
            command: step.command(),
        });
        let t0 = Instant::now();
        let outcome = internal::run_step(req, &plan, step, &mut state, cancel, &mut |line| {
            sink(Event::Log {
                index,
                line: line.to_string(),
            })
        });
        let millis = t0.elapsed().as_millis();
        let (ok, note) = match outcome {
            Ok(note) => (true, note),
            Err(e) => (false, Some(e.to_string())),
        };
        sink(Event::StepEnd {
            index,
            id: step.id.to_string(),
            ok,
            fatal: step.fatal,
            millis,
            note: note.clone(),
        });
        results.push(StepResult {
            id: step.id.to_string(),
            label: step.label.clone(),
            phase: step.phase,
            ok,
            fatal: step.fatal,
            millis,
            note: note.clone(),
        });
        if !ok && step.fatal {
            let msg = format!(
                "étape « {} » en échec : {}",
                step.label,
                note.unwrap_or_default()
            );
            sink(Event::Failed {
                message: msg.clone(),
                step: Some(step.id.to_string()),
            });
            bail!(msg);
        }
    }

    let report = Report {
        framework: plan.framework,
        name: req.name.clone(),
        target: plan.target.display().to_string(),
        ok: results.iter().all(|r| r.ok || !r.fatal),
        steps: results,
        files: internal::list_files(&plan.target),
        lint: state.lint.take(),
        tools,
        next_steps: next_steps(req, &plan.target),
        elapsed_ms: started.elapsed().as_millis(),
    };
    sink(Event::Done {
        report: report.clone(),
    });
    Ok(report)
}

/// Exécute sans retour d'événements (tests, usage programmatique).
pub fn execute_quiet(req: &Request) -> Result<Report> {
    execute(req, &mut |_| {}, &AtomicBool::new(false))
}

pub fn next_steps(req: &Request, target: &Path) -> Vec<String> {
    let dir = target.display();
    match &req.options {
        Options::Astro(o) => {
            let pm = o.package_manager;
            let mut v = vec![format!("cd {dir}")];
            if !req.install {
                v.push(pm.install_cmd());
            }
            v.push(format!("{} dev   # http://localhost:4321", pm.run_prefix()));
            v
        }
        Options::Laravel(o) => {
            let mut v = vec![
                format!("cd {dir}"),
                "php artisan serve   # http://localhost:8000".into(),
            ];
            if o.starter.has_frontend() {
                v.insert(1, "npm run dev   # (dans un second terminal)".into());
            }
            v
        }
        Options::Wagtail(_) => vec![
            format!("cd {dir}"),
            ". .venv/bin/activate".into(),
            "python manage.py createsuperuser".into(),
            "python manage.py runserver   # http://localhost:8000".into(),
        ],
        Options::Rust(_) => vec![format!("cd {dir}"), "cargo run".into(), "cargo test".into()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_deserializes_flat_from_gui_json() {
        let r: Request = serde_json::from_str(
            r#"{"framework":"astro","name":"demo","package_manager":"pnpm","integrations":["tailwind"],"template":"minimal"}"#,
        )
        .unwrap();
        assert_eq!(r.framework(), Framework::Astro);
        assert!(r.git && r.install && r.verify);
        match r.options {
            Options::Astro(o) => {
                assert_eq!(o.package_manager, astro::PackageManager::Pnpm);
                assert_eq!(o.integrations, vec!["tailwind"]);
            }
            _ => panic!(),
        }
        // options absentes => valeurs par défaut
        let r: Request = serde_json::from_str(r#"{"framework":"laravel","name":"x"}"#).unwrap();
        assert_eq!(r.framework(), Framework::Laravel);
        let r: Request = serde_json::from_str(r#"{"framework":"wagtail","name":"x"}"#).unwrap();
        assert_eq!(r.framework(), Framework::Wagtail);
        let r: Request = serde_json::from_str(r#"{"framework":"rust","name":"x"}"#).unwrap();
        assert_eq!(r.framework(), Framework::Rust);
        assert!(serde_json::from_str::<Request>(r#"{"framework":"cobol","name":"x"}"#).is_err());
    }

    #[test]
    fn invalid_names_and_options_are_rejected_before_any_effect() {
        for bad in ["../x", "a/b", "", ".x", "a b"] {
            assert!(
                Request::new(Framework::Astro, bad).validate().is_err(),
                "{bad}"
            );
        }
        let mut r = Request::new(Framework::Astro, "ok");
        assert!(r.validate().is_ok());
        if let Options::Astro(o) = &mut r.options {
            o.integrations = vec!["sitemap".into()];
        }
        assert!(r.validate().is_err(), "sitemap exige une URL de site");
    }

    #[test]
    fn plan_preview_shows_exact_commands() {
        let r = Request::new(Framework::Astro, "demo");
        let p = plan(&r).unwrap();
        let text = p.preview();
        assert!(text.contains("create astro@latest demo"), "{text}");
        assert!(text.contains("build"), "{text}");
        assert!(p.steps.iter().any(|s| s.id == "create"));
    }

    #[test]
    fn preflight_error_is_actionable() {
        let t = ToolStatus {
            id: "php".into(),
            label: "PHP".into(),
            required: true,
            found: false,
            path: None,
            version: None,
            min: Some("8.3.0".into()),
            ok: false,
            hint: "Installer PHP".into(),
        };
        let e = preflight_check(std::slice::from_ref(&t))
            .unwrap_err()
            .to_string();
        assert!(e.contains("PHP") && e.contains("introuvable") && e.contains("Installer PHP"));
        let mut ok = t;
        ok.ok = true;
        assert!(preflight_check(&[ok]).is_ok());
    }
}
