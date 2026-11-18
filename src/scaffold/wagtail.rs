//! Wagtail : venv + `pip install wagtail` + `wagtail start` officiel, puis couche AlphaRust.

use super::exec::{Cmd, Cwd};
use super::tools::{self, ToolStatus};
use super::{DesignTarget, Internal, Phase, Request, Step};
use crate::generators::Vars;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WagtailOptions {
    /// Paquets pip supplémentaires (ex. `psycopg[binary]`, `django-debug-toolbar`).
    pub packages: Vec<String>,
    /// `manage.py migrate` après la création.
    pub migrate: bool,
    /// Catalogue de composants du design kit partagé (application Django `ds`, balises `{% ds_… %}`, `/components/`).
    pub ui: bool,
    /// Palette de marque (exige `ui`) : même `themes/theme.css` qu'Astro, donc même rendu.
    pub theme: Option<super::theme::ThemeColors>,
}

impl Default for WagtailOptions {
    fn default() -> Self {
        Self {
            packages: vec![],
            migrate: true,
            ui: false,
            theme: None,
        }
    }
}

/// Un nom de paquet pip : lettres, chiffres, `-_.`, extras `[x]` et contraintes simples.
pub fn valid_pip_spec(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 80
        && !s.starts_with('-')
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.[],<>=~!".contains(c))
}

impl WagtailOptions {
    pub fn validate(&self) -> Result<()> {
        for p in &self.packages {
            if !valid_pip_spec(p) {
                bail!("paquet pip invalide : `{p}`");
            }
        }
        if self.theme.is_some() && !self.ui {
            bail!("--theme-* s'applique au design kit : ajoutez --ui css");
        }
        if let Some(t) = &self.theme {
            t.validate()?;
        }
        Ok(())
    }
}

fn python() -> &'static str {
    if tools::which("python3").is_some() || tools::which("python").is_none() {
        "python3"
    } else {
        "python"
    }
}

fn venv_bin(name: &str) -> String {
    if cfg!(windows) {
        format!("./.venv/Scripts/{name}")
    } else {
        format!("./.venv/bin/{name}")
    }
}

pub fn tools(_req: &Request, _o: &WagtailOptions) -> Vec<ToolStatus> {
    let mut v = vec![tools::detect(&tools::PYTHON, true)];
    if let Some(py) = tools::PYTHON.programs.iter().find_map(|p| tools::which(p)) {
        let ok = tools::python_venv_available(&py);
        v.push(ToolStatus {
            id: "venv".into(),
            label: "python -m venv (venv + ensurepip)".into(),
            required: true,
            found: true,
            path: None,
            version: Some(if ok {
                "disponible".into()
            } else {
                "indisponible".into()
            }),
            min: None,
            ok,
            hint: "Installer le module venv : `apt install python3-venv` (Debian/Ubuntu)".into(),
        });
    }
    v
}

pub fn steps(req: &Request, o: &WagtailOptions) -> Vec<Step> {
    let vars = Vars::new(&req.name).expect("nom validé");
    let py = venv_bin("python");
    let mut steps = vec![
        Step::internal(
            "prepare",
            Phase::Prepare,
            "Contrôle de la destination",
            Internal::Prepare,
        ),
        Step::internal(
            "mkdir",
            Phase::Prepare,
            "Créer le dossier du projet",
            Internal::Mkdir,
        ),
        Step::run(
            "venv",
            Phase::Scaffold,
            "Créer l'environnement virtuel (python -m venv .venv)",
            Cmd::new(python(), &["-m", "venv", ".venv"], Cwd::Project),
        ),
    ];
    let mut pip = vec![
        "-m".to_string(),
        "pip".into(),
        "install".into(),
        "wagtail".into(),
    ];
    pip.extend(o.packages.iter().cloned());
    steps.push(Step::run(
        "pip-wagtail",
        Phase::Install,
        "Installer Wagtail (pip install wagtail)",
        Cmd::owned(&py, pip, Cwd::Project),
    ));
    steps.push(Step::run(
        "start",
        Phase::Scaffold,
        &format!("Créer le projet avec wagtail start {}", vars.pkg),
        Cmd::owned(
            &venv_bin("wagtail"),
            vec!["start".into(), vars.pkg.clone(), ".".into()],
            Cwd::Project,
        ),
    ));
    if !o.packages.is_empty() {
        steps.push(Step::internal(
            "requirements",
            Phase::Configure,
            "Ajouter les paquets supplémentaires à requirements.txt",
            Internal::Requirements(o.packages.clone()),
        ));
    }
    steps.push(Step::run(
        "pip-requirements",
        Phase::Install,
        "Installer requirements.txt",
        Cmd::new(
            &py,
            &["-m", "pip", "install", "-r", "requirements.txt"],
            Cwd::Project,
        ),
    ));
    steps.push(Step::internal(
        "layer",
        Phase::Layer,
        "Appliquer la couche AlphaRust (config qualité, docs, tests, .gitignore)",
        Internal::Layer(if o.ui {
            vec![
                "common",
                "wagtail",
                "ui-django",
                "wagtail-ds",
                "site-wagtail",
            ]
        } else {
            vec!["common", "wagtail"]
        }),
    ));
    if o.ui {
        steps.push(Step::internal(
            "design-kit",
            Phase::Layer,
            "Copier le design kit (ds/static/ds)",
            Internal::DesignKit(DesignTarget::Wagtail),
        ));
        if let Some(t) = &o.theme {
            steps.push(Step::internal(
                "theme",
                Phase::Layer,
                "Écrire la palette de marque (ds/static/ds/css/themes/theme.css, contrôle WCAG)",
                Internal::DesignTheme(t.clone(), DesignTarget::Wagtail),
            ));
        }
        steps.push(Step::internal(
            "design-wire",
            Phase::Configure,
            "Brancher l'application ds (INSTALLED_APPS, /components/, feuille et script du kit)",
            Internal::WagtailDesign,
        ));
    }
    if o.ui {
        // Migrations des applications du site (layout, site_settings, flex, service, blog, contact, home) :
        // générées par Django pour la version de Wagtail installée.
        steps.push(Step::run(
            "makemigrations",
            Phase::Configure,
            "Créer les migrations du site (manage.py makemigrations)",
            Cmd::new(
                &py,
                &["manage.py", "makemigrations", "--no-input"],
                Cwd::Project,
            ),
        ));
    }
    if o.migrate {
        steps.push(Step::run(
            "migrate",
            Phase::Configure,
            "Migrer la base (manage.py migrate)",
            Cmd::new(&py, &["manage.py", "migrate", "--no-input"], Cwd::Project),
        ));
        if o.ui {
            steps.push(Step::run(
                "seed-site",
                Phase::Configure,
                "Créer les pages, menus et réglages du site (manage.py seed_site)",
                Cmd::new(&py, &["manage.py", "seed_site"], Cwd::Project),
            ));
        }
    }
    steps.push(Step::internal(
        "freeze",
        Phase::Configure,
        "Figer les versions installées (requirements-lock.txt)",
        Internal::Freeze,
    ));
    steps.push(Step::internal(
        "manifest",
        Phase::Configure,
        "Écrire le manifeste du projet",
        Internal::WriteManifest,
    ));
    if req.verify {
        steps.push(Step::run(
            "check",
            Phase::Verify,
            "Contrôles Django (manage.py check)",
            Cmd::new(&py, &["manage.py", "check"], Cwd::Project),
        ));
        steps.push(Step::run(
            "migrations",
            Phase::Verify,
            "Migrations à jour (makemigrations --check)",
            Cmd::new(
                &py,
                &["manage.py", "makemigrations", "--check", "--dry-run"],
                Cwd::Project,
            ),
        ));
        steps.push(Step::run(
            "test",
            Phase::Verify,
            "Lancer les tests (manage.py test)",
            Cmd::new(&py, &["manage.py", "test"], Cwd::Project),
        ));
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
    fn plan_uses_wagtail_start_in_a_venv() {
        let r = Request::new(Framework::Wagtail, "Mon-CMS");
        let o = WagtailOptions::default();
        let s = steps(&r, &o);
        let ids: Vec<&str> = s.iter().map(|s| s.id).collect();
        assert_eq!(
            ids,
            [
                "prepare",
                "mkdir",
                "venv",
                "pip-wagtail",
                "start",
                "pip-requirements",
                "layer",
                "migrate",
                "freeze",
                "manifest",
                "check",
                "migrations",
                "test",
                "lint",
                "git"
            ]
        );
        let start = s.iter().find(|s| s.id == "start").unwrap();
        match &start.action {
            Action::Run(c) => {
                assert!(c.program.ends_with("wagtail"));
                assert_eq!(c.args, ["start", "mon_cms", "."]);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn extra_packages_are_validated_and_installed() {
        assert!(valid_pip_spec("psycopg[binary]"));
        assert!(valid_pip_spec("django>=5"));
        for bad in ["", "-r evil.txt", "a b", "x;rm -rf /", "$(id)", "../x"] {
            assert!(!valid_pip_spec(bad), "{bad}");
        }
        let o = WagtailOptions {
            packages: vec!["psycopg[binary]".into()],
            migrate: false,
            ui: false,
            theme: None,
        };
        let mut r = Request::new(Framework::Wagtail, "x");
        r.options = Options::Wagtail(o.clone());
        let s = steps(&r, &o);
        assert!(s.iter().any(|s| s.id == "requirements"));
        assert!(!s.iter().any(|s| s.id == "migrate"));
        let pip = s.iter().find(|s| s.id == "pip-wagtail").unwrap();
        match &pip.action {
            Action::Run(c) => assert!(c.args.contains(&"psycopg[binary]".to_string())),
            _ => panic!(),
        }
    }

    #[test]
    fn install_is_mandatory() {
        let mut r = Request::new(Framework::Wagtail, "x");
        r.install = false;
        assert!(r.validate().is_err());
    }
}
