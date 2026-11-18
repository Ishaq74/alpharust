//! Orchestration des projets générés : inventaire, état et tâches (installer, construire, vérifier,
//! tester, linter, mettre à jour, lancer) à partir du manifeste `.alpharust-project.json`.
//!
//! Chaque tâche appelle l'outil natif du projet (gestionnaire de paquets, composer, cargo…) : AlphaRust
//! ne réimplémente rien, il choisit la bonne commande d'après le manifeste et diffuse la sortie.

use super::exec::{self, Cmd, Cwd};
use super::{Event, Framework, LintSummary, Phase, Report, StepInfo, StepResult, MANIFEST};
use anyhow::{anyhow, bail, Result};
use serde::Serialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Tâches disponibles sur un projet généré.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Task {
    Install,
    Dev,
    Build,
    Check,
    Test,
    Lint,
    Update,
}

impl Task {
    pub const ALL: [Task; 7] = [
        Task::Install,
        Task::Dev,
        Task::Build,
        Task::Check,
        Task::Test,
        Task::Lint,
        Task::Update,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Task::Install => "install",
            Task::Dev => "dev",
            Task::Build => "build",
            Task::Check => "check",
            Task::Test => "test",
            Task::Lint => "lint",
            Task::Update => "update",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Task::Install => "Installer les dépendances",
            Task::Dev => "Lancer le serveur de développement",
            Task::Build => "Construire",
            Task::Check => "Vérifier (types, configuration)",
            Task::Test => "Lancer les tests",
            Task::Lint => "Linter avec AlphaRust",
            Task::Update => "Mettre à jour les dépendances",
        }
    }

    pub fn parse(s: &str) -> Option<Task> {
        Self::ALL.into_iter().find(|t| t.id() == s.to_lowercase())
    }

    /// Tâche de longue durée (ne se termine pas seule) : réservée à la CLI.
    pub fn is_long_running(self) -> bool {
        self == Task::Dev
    }
}

/// Un projet généré par AlphaRust.
#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub framework: Framework,
    pub created_at: Option<String>,
    pub package_manager: Option<String>,
    /// Briques choisies à la génération (UI, base de données, authentification, langues…).
    pub features: Vec<String>,
    /// Dépendances installées (`node_modules`, `vendor`, `.venv`, `target`).
    pub installed: bool,
    pub git: Option<GitState>,
    pub tasks: Vec<TaskInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GitState {
    pub branch: String,
    pub dirty: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskInfo {
    pub id: &'static str,
    pub label: &'static str,
    pub command: String,
    /// Disponible dans la GUI (hors serveur de développement).
    pub interactive: bool,
}

fn read_manifest(dir: &Path) -> Option<Value> {
    std::fs::read_to_string(dir.join(MANIFEST))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
}

/// Options du framework : le manifeste aplatit la requête (`request.ui`…) ; l'ancien
/// format les imbriquait sous `request.options`.
fn options(manifest: &Value) -> &Value {
    let nested = &manifest["request"]["options"];
    if nested.is_object() {
        nested
    } else {
        &manifest["request"]
    }
}

/// Résumé lisible des options du manifeste.
fn features(framework: Framework, o: &Value) -> Vec<String> {
    let mut f = Vec::new();
    let s = |k: &str| o[k].as_str().filter(|v| !v.is_empty() && *v != "none");
    let list = |k: &str| -> Vec<String> {
        o[k].as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    };
    match framework {
        Framework::Astro => {
            if let Some(t) = s("template") {
                f.push(format!("template {t}"));
            }
            if let Some(ui) = s("ui") {
                f.push(format!("UI {ui}"));
            }
            if let Some(db) = s("database") {
                f.push(format!("base {db}"));
            }
            if o["auth"].as_bool() == Some(true) {
                let plugins = list("auth_plugins");
                f.push(if plugins.is_empty() {
                    "auth".to_string()
                } else {
                    format!("auth ({})", plugins.join(", "))
                });
            }
            let i18n = list("i18n");
            if !i18n.is_empty() {
                f.push(format!("i18n {}", i18n.join("/")));
            }
            for i in list("integrations") {
                f.push(i);
            }
            if o["docker"].as_bool() == Some(true) {
                f.push("docker".into());
            }
            if o["ci"].as_bool() == Some(true) {
                f.push("CI".into());
            }
        }
        Framework::Laravel => {
            if let Some(s) = s("starter") {
                f.push(format!("starter {s}"));
            }
            if let Some(d) = s("database") {
                f.push(format!("base {d}"));
            }
            if o["pest"].as_bool() == Some(true) {
                f.push("Pest".into());
            }
            site_features(o, &mut f);
        }
        Framework::Wagtail => {
            for p in list("packages") {
                f.push(p);
            }
            site_features(o, &mut f);
        }
        Framework::Rust => {
            if let Some(k) = s("kind") {
                f.push(k.to_string());
            }
        }
    }
    f
}

/// Site vitrine Laravel / Wagtail : kit, comptes, jeu d'icônes.
fn site_features(o: &Value, f: &mut Vec<String>) {
    if o["ui"].as_bool() == Some(true) {
        f.push(if o["starwind"].as_bool() == Some(true) {
            "UI starwind".into()
        } else {
            "UI css".into()
        });
    }
    if o["auth"].as_bool() == Some(true) {
        f.push("auth".into());
    }
    if let Some(icons) = o["icons"].as_str() {
        f.push(format!("icônes {icons}"));
    }
}

fn git_state(dir: &Path) -> Option<GitState> {
    if !dir.join(".git").exists() {
        return None;
    }
    let git = |args: &[&str]| -> Option<String> {
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .ok()?;
        out.status
            .success()
            .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
    };
    Some(GitState {
        branch: git(&["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default(),
        dirty: git(&["status", "--porcelain"]).is_some_and(|s| !s.is_empty()),
    })
}

fn is_installed(framework: Framework, dir: &Path) -> bool {
    match framework {
        Framework::Astro => dir.join("node_modules").is_dir(),
        Framework::Laravel => dir.join("vendor").is_dir(),
        Framework::Wagtail => dir.join(".venv").is_dir(),
        Framework::Rust => dir.join("target").is_dir(),
    }
}

fn has_script(dir: &Path, script: &str) -> bool {
    std::fs::read_to_string(dir.join("package.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .is_some_and(|v| v["scripts"].get(script).is_some())
}

fn pm_of(manifest: &Value) -> String {
    options(manifest)["package_manager"]
        .as_str()
        .unwrap_or("npm")
        .to_string()
}

fn pm_run(pm: &str, script: &str) -> Cmd {
    if pm == "npm" {
        Cmd::new("npm", &["run", script], Cwd::Project)
    } else {
        Cmd::new(pm, &["run", script], Cwd::Project)
    }
}

/// Commande native d'une tâche ; `None` pour `lint` (moteur interne) ; erreur si la tâche n'existe pas.
pub fn command(p: &Project, task: Task) -> Result<Option<Cmd>> {
    let dir = PathBuf::from(&p.path);
    let manifest = read_manifest(&dir).ok_or_else(|| anyhow!("manifeste introuvable"))?;
    if task == Task::Lint {
        return Ok(None);
    }
    let unsupported = || {
        anyhow!(
            "la tâche `{}` n'existe pas pour un projet {}",
            task.id(),
            p.framework.label()
        )
    };
    let cmd = match p.framework {
        Framework::Astro => {
            let pm = pm_of(&manifest);
            match task {
                Task::Install => Cmd::new(&pm, &["install"], Cwd::Project),
                Task::Update => match pm.as_str() {
                    "yarn" => Cmd::new("yarn", &["upgrade"], Cwd::Project),
                    other => Cmd::new(other, &["update"], Cwd::Project),
                },
                Task::Dev | Task::Build => pm_run(&pm, task.id()),
                Task::Check | Task::Test => {
                    if !has_script(&dir, task.id()) {
                        bail!(
                            "ce projet n'a pas de script `{}` dans package.json",
                            task.id()
                        );
                    }
                    pm_run(&pm, task.id())
                }
                Task::Lint => unreachable!(),
            }
        }
        Framework::Laravel => match task {
            Task::Install => Cmd::new("composer", &["install"], Cwd::Project),
            Task::Update => Cmd::new("composer", &["update"], Cwd::Project),
            Task::Dev => Cmd::new("php", &["artisan", "serve"], Cwd::Project),
            Task::Build => {
                if !has_script(&dir, "build") {
                    return Err(unsupported());
                }
                pm_run("npm", "build")
            }
            Task::Check => Cmd::new(
                "composer",
                &["validate", "--no-check-publish"],
                Cwd::Project,
            ),
            Task::Test => Cmd::new("php", &["artisan", "test"], Cwd::Project),
            Task::Lint => unreachable!(),
        },
        Framework::Wagtail => {
            let py = if cfg!(windows) {
                "./.venv/Scripts/python"
            } else {
                "./.venv/bin/python"
            };
            match task {
                Task::Install => Cmd::new(
                    py,
                    &["-m", "pip", "install", "-r", "requirements.txt"],
                    Cwd::Project,
                ),
                Task::Dev => Cmd::new(py, &["manage.py", "runserver"], Cwd::Project),
                Task::Check => Cmd::new(py, &["manage.py", "check"], Cwd::Project),
                Task::Test => Cmd::new(py, &["manage.py", "test"], Cwd::Project),
                Task::Build => Cmd::new(
                    py,
                    &["manage.py", "collectstatic", "--noinput"],
                    Cwd::Project,
                ),
                Task::Update => Cmd::new(
                    py,
                    &["-m", "pip", "list", "--outdated", "--format=columns"],
                    Cwd::Project,
                ),
                Task::Lint => unreachable!(),
            }
        }
        Framework::Rust => match task {
            Task::Install => Cmd::new("cargo", &["fetch"], Cwd::Project),
            Task::Dev => Cmd::new("cargo", &["run"], Cwd::Project),
            Task::Build => Cmd::new("cargo", &["build"], Cwd::Project),
            Task::Check => Cmd::new("cargo", &["check", "--all-targets"], Cwd::Project),
            Task::Test => Cmd::new("cargo", &["test"], Cwd::Project),
            Task::Update => Cmd::new("cargo", &["update"], Cwd::Project),
            Task::Lint => unreachable!(),
        },
    };
    Ok(Some(cmd))
}

/// Lit un dossier de projet ; `None` si ce n'est pas un projet AlphaRust.
pub fn inspect(dir: &Path) -> Option<Project> {
    let manifest = read_manifest(dir)?;
    let framework = Framework::parse(manifest["framework"].as_str()?)?;
    let name = dir.file_name()?.to_string_lossy().into_owned();
    let mut p = Project {
        name,
        path: dir.display().to_string(),
        framework,
        created_at: manifest["created_at"].as_str().map(String::from),
        package_manager: (framework == Framework::Astro).then(|| pm_of(&manifest)),
        features: features(framework, options(&manifest)),
        installed: is_installed(framework, dir),
        git: git_state(dir),
        tasks: Vec::new(),
    };
    p.tasks = Task::ALL
        .into_iter()
        .filter_map(|t| {
            let command = match command(&p, t) {
                Ok(Some(c)) => c.display(),
                Ok(None) => "alpharust lint".to_string(),
                Err(_) => return None,
            };
            Some(TaskInfo {
                id: t.id(),
                label: t.label(),
                command,
                interactive: !t.is_long_running(),
            })
        })
        .collect();
    Some(p)
}

/// Tous les projets AlphaRust d'un dossier (`sites/` par défaut), triés par nom.
pub fn list(root: &Path) -> Vec<Project> {
    let mut out: Vec<Project> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter_map(|e| inspect(&e.path()))
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Un projet par nom (dans `root`) ou par chemin.
pub fn resolve(root: &Path, name_or_path: &str) -> Result<Project> {
    let as_path = PathBuf::from(name_or_path);
    let candidates = [root.join(name_or_path), as_path];
    for c in candidates {
        if c.is_dir() {
            if let Some(p) = inspect(&c.canonicalize().unwrap_or(c)) {
                return Ok(p);
            }
        }
    }
    bail!(
        "projet « {name_or_path} » introuvable (ni dans {} ni comme chemin AlphaRust avec {MANIFEST})",
        root.display()
    )
}

/// Exécute une tâche en émettant les mêmes événements qu'une génération (CLI, GUI, TUI).
pub fn run(
    p: &Project,
    task: Task,
    sink: &mut dyn FnMut(Event),
    cancel: &AtomicBool,
) -> Result<Report> {
    let started = Instant::now();
    let dir = PathBuf::from(&p.path);
    let cmd = command(p, task)?;
    let command_text = cmd
        .as_ref()
        .map(|c| c.display())
        .unwrap_or_else(|| "alpharust lint".into());
    let info = StepInfo {
        id: task.id().to_string(),
        label: task.label().to_string(),
        phase: Phase::Verify,
        phase_label: Phase::Verify.label(),
        command: Some(command_text.clone()),
        cwd: cmd.as_ref().map(|c| c.cwd),
        fatal: true,
    };
    sink(Event::Plan {
        target: p.path.clone(),
        steps: vec![info],
    });
    sink(Event::StepStart {
        index: 0,
        total: 1,
        id: task.id().to_string(),
        label: task.label().to_string(),
        phase: Phase::Verify,
        command: Some(command_text),
    });
    let t0 = Instant::now();
    let mut lint_summary: Option<LintSummary> = None;
    let outcome: Result<Option<String>> = match &cmd {
        Some(c) => {
            let timeout = if task.is_long_running() {
                std::time::Duration::from_secs(60 * 60 * 24)
            } else {
                exec::step_timeout()
            };
            exec::run(&c.program, &c.args, &dir, cancel, timeout, &mut |line| {
                sink(Event::Log {
                    index: 0,
                    line: line.to_string(),
                })
            })
            .map_err(|e| anyhow!("{} : {e}", c.program))
            .and_then(|o| {
                if o.success {
                    Ok(None)
                } else if o.cancelled {
                    Err(anyhow!("tâche annulée"))
                } else if o.timed_out {
                    Err(anyhow!("délai dépassé"))
                } else {
                    Err(anyhow!(
                        "`{}` a échoué (code {})",
                        c.display(),
                        o.code.map_or("?".into(), |c| c.to_string())
                    ))
                }
            })
        }
        None => crate::engine::lint(&dir, None).map(|r| {
            let (mut e, mut w, mut i) = (0, 0, 0);
            let mut sample = Vec::new();
            for v in &r.violations {
                match v.severity.as_str() {
                    "error" => e += 1,
                    "warning" => w += 1,
                    _ => i += 1,
                }
                if sample.len() < 12 {
                    sample.push(format!("{}:{} [{}] {}", v.file, v.line, v.rule, v.message));
                }
                sink(Event::Log {
                    index: 0,
                    line: format!("{}:{} [{}] {}", v.file, v.line, v.rule, v.message),
                });
            }
            lint_summary = Some(LintSummary {
                files: r.files_scanned,
                rules: r.rules_used,
                errors: e,
                warnings: w,
                infos: i,
                profile: r.profile.clone(),
                sample,
            });
            Some(format!("{e} erreur(s), {w} avertissement(s), {i} info(s)"))
        }),
    };
    let millis = t0.elapsed().as_millis();
    let lint_failed = lint_summary.as_ref().is_some_and(|l| l.errors > 0);
    let (ok, note) = match outcome {
        Ok(note) => (!lint_failed, note),
        Err(e) => (false, Some(e.to_string())),
    };
    sink(Event::StepEnd {
        index: 0,
        id: task.id().to_string(),
        ok,
        fatal: true,
        millis,
        note: note.clone(),
    });
    if !ok {
        let msg = format!(
            "{} : {}",
            task.label(),
            note.clone().unwrap_or_else(|| "échec".into())
        );
        sink(Event::Failed {
            message: msg.clone(),
            step: Some(task.id().to_string()),
        });
        if cancel.load(Ordering::Relaxed) {
            bail!("tâche annulée");
        }
        bail!(msg);
    }
    let report = Report {
        framework: p.framework,
        name: p.name.clone(),
        target: p.path.clone(),
        ok: true,
        steps: vec![StepResult {
            id: task.id().to_string(),
            label: task.label().to_string(),
            phase: Phase::Verify,
            ok: true,
            fatal: true,
            millis,
            note,
        }],
        files: Vec::new(),
        lint: lint_summary,
        tools: Vec::new(),
        next_steps: Vec::new(),
        elapsed_ms: started.elapsed().as_millis(),
    };
    sink(Event::Done {
        report: report.clone(),
    });
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(dir: &Path, framework: &str, options: Value) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(
            dir.join(MANIFEST),
            serde_json::json!({
                "framework": framework,
                "created_at": "2026-11-16T10:00:00Z",
                "request": options
            })
            .to_string(),
        )
        .unwrap();
    }

    #[test]
    fn lists_only_alpharust_projects_sorted() {
        let root = tempfile::tempdir().unwrap();
        project(&root.path().join("zeta"), "rust", serde_json::json!({}));
        project(
            &root.path().join("alpha"),
            "astro",
            serde_json::json!({"package_manager":"pnpm","ui":"starwind","database":"sqlite","auth":true,"auth_plugins":["admin"],"i18n":["fr","en"]}),
        );
        std::fs::create_dir_all(root.path().join("étranger")).unwrap();
        let l = list(root.path());
        assert_eq!(
            l.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(),
            ["alpha", "zeta"]
        );
        let a = &l[0];
        assert_eq!(a.package_manager.as_deref(), Some("pnpm"));
        assert!(a.features.contains(&"UI starwind".to_string()));
        assert!(a.features.contains(&"auth (admin)".to_string()));
        assert!(a.features.contains(&"i18n fr/en".to_string()));
        assert!(!a.installed);
    }

    #[test]
    fn astro_commands_follow_the_package_manager_and_scripts() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("site");
        project(&dir, "astro", serde_json::json!({"package_manager":"pnpm"}));
        std::fs::write(
            dir.join("package.json"),
            r#"{"scripts":{"build":"astro build","check":"astro check"}}"#,
        )
        .unwrap();
        let p = inspect(&dir).unwrap();
        let show = |t| command(&p, t).unwrap().map(|c| c.display());
        assert_eq!(show(Task::Build).unwrap().as_str(), "pnpm run build");
        assert_eq!(show(Task::Install).unwrap().as_str(), "pnpm install");
        assert_eq!(show(Task::Update).unwrap().as_str(), "pnpm update");
        assert_eq!(show(Task::Lint), None, "le lint est interne");
        assert!(command(&p, Task::Test).is_err(), "pas de script test");
        let ids: Vec<&str> = p.tasks.iter().map(|t| t.id).collect();
        assert!(ids.contains(&"check") && !ids.contains(&"test"));
        assert!(!p.tasks.iter().find(|t| t.id == "dev").unwrap().interactive);
    }

    #[test]
    fn rust_and_laravel_tasks() {
        let root = tempfile::tempdir().unwrap();
        project(&root.path().join("r"), "rust", serde_json::json!({}));
        project(&root.path().join("l"), "laravel", serde_json::json!({}));
        let r = resolve(root.path(), "r").unwrap();
        assert_eq!(
            command(&r, Task::Test).unwrap().unwrap().display(),
            "cargo test"
        );
        let l = resolve(root.path(), "l").unwrap();
        assert_eq!(
            command(&l, Task::Test).unwrap().unwrap().display(),
            "php artisan test"
        );
        assert!(command(&l, Task::Build).is_err(), "pas de script build");
        assert!(resolve(root.path(), "inconnu").is_err());
    }

    #[test]
    fn running_a_task_streams_events_and_reports_failure() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("r");
        project(&dir, "rust", serde_json::json!({}));
        let p = inspect(&dir).unwrap();
        // lint interne : aucun fichier, donc succès
        let mut events = Vec::new();
        let rep = run(
            &p,
            Task::Lint,
            &mut |e| events.push(e),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert!(rep.ok && rep.lint.is_some());
        assert!(matches!(events.first(), Some(Event::Plan { .. })));
        assert!(matches!(events.last(), Some(Event::Done { .. })));
        // cargo check sans Cargo.toml : échec propre
        let mut events = Vec::new();
        let err = run(
            &p,
            Task::Check,
            &mut |e| events.push(e),
            &AtomicBool::new(false),
        )
        .unwrap_err();
        assert!(err.to_string().contains("échoué") || err.to_string().contains("cargo"));
        assert!(matches!(events.last(), Some(Event::Failed { .. })));
    }
}
