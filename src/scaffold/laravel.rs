//! Laravel : `composer create-project laravel/laravel`, kit de démarrage Breeze optionnel, puis couche AlphaRust.

use super::exec::{Cmd, Cwd};
use super::tools::{self, ToolStatus};
use super::{DesignTarget, Internal, Phase, Request, Step};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Starter {
    #[default]
    None,
    BreezeBlade,
    BreezeLivewire,
    BreezeReact,
    BreezeVue,
    BreezeApi,
}

impl Starter {
    pub const ALL: [Starter; 6] = [
        Starter::None,
        Starter::BreezeBlade,
        Starter::BreezeLivewire,
        Starter::BreezeReact,
        Starter::BreezeVue,
        Starter::BreezeApi,
    ];
    pub fn id(self) -> &'static str {
        match self {
            Starter::None => "none",
            Starter::BreezeBlade => "breeze-blade",
            Starter::BreezeLivewire => "breeze-livewire",
            Starter::BreezeReact => "breeze-react",
            Starter::BreezeVue => "breeze-vue",
            Starter::BreezeApi => "breeze-api",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Starter::None => "Aucun (squelette Laravel)",
            Starter::BreezeBlade => "Breeze · Blade",
            Starter::BreezeLivewire => "Breeze · Livewire",
            Starter::BreezeReact => "Breeze · React (Inertia)",
            Starter::BreezeVue => "Breeze · Vue (Inertia)",
            Starter::BreezeApi => "Breeze · API",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|t| t.id() == s.to_lowercase())
    }
    /// Pile passée à `breeze:install`.
    fn stack(self) -> Option<&'static str> {
        match self {
            Starter::None => None,
            Starter::BreezeBlade => Some("blade"),
            Starter::BreezeLivewire => Some("livewire"),
            Starter::BreezeReact => Some("react"),
            Starter::BreezeVue => Some("vue"),
            Starter::BreezeApi => Some("api"),
        }
    }
    /// Le kit installe/compile des assets (Node requis).
    pub fn has_frontend(self) -> bool {
        !matches!(self, Starter::None | Starter::BreezeApi)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Database {
    #[default]
    Sqlite,
    Mysql,
    Pgsql,
}

impl Database {
    pub const ALL: [Database; 3] = [Database::Sqlite, Database::Mysql, Database::Pgsql];
    pub fn id(self) -> &'static str {
        match self {
            Database::Sqlite => "sqlite",
            Database::Mysql => "mysql",
            Database::Pgsql => "pgsql",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|t| t.id() == s.to_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LaravelOptions {
    pub starter: Starter,
    /// Mode sombre du kit Breeze.
    pub dark: bool,
    /// Pest au lieu de PHPUnit (kit Breeze uniquement).
    pub pest: bool,
    pub database: Database,
    /// `npm install && npm run build` même sans kit de démarrage.
    pub frontend: bool,
    /// Catalogue de composants du design kit partagé (Blade, CSS pur, Vite + Tailwind v4) et page `/components`.
    pub ui: bool,
    /// Kit Tailwind + Starwind (`--ui starwind`) au lieu du kit CSS : mêmes composants, classes des atomes d'atomic.
    pub starwind: bool,
    /// Palette de marque (exige `ui`) : même `themes/theme.css` qu'Astro, donc même rendu.
    pub theme: Option<super::theme::ThemeColors>,
}

impl Default for LaravelOptions {
    fn default() -> Self {
        Self {
            starter: Starter::None,
            dark: false,
            pest: false,
            database: Database::Sqlite,
            frontend: false,
            ui: false,
            starwind: false,
            theme: None,
        }
    }
}

impl LaravelOptions {
    pub fn validate(&self) -> Result<()> {
        if self.starter == Starter::None && (self.dark || self.pest) {
            anyhow::bail!("--dark et --pest nécessitent un kit de démarrage (--starter breeze-…)");
        }
        if self.dark && !self.starter.has_frontend() {
            anyhow::bail!("--dark n'a pas de sens avec le kit API");
        }
        if self.theme.is_some() && !self.ui {
            anyhow::bail!("--theme-* s'applique au design kit : ajoutez --ui css ou --ui starwind");
        }
        if let Some(t) = &self.theme {
            t.validate()?;
        }
        if self.ui && self.starter != Starter::None {
            anyhow::bail!(
                "--ui css s'applique au squelette Laravel seul (--starter none) : \
                 Breeze impose déjà ses propres vues et feuilles de style"
            );
        }
        Ok(())
    }

    fn needs_node(&self) -> bool {
        self.starter.has_frontend() || self.frontend || self.ui
    }

    pub fn flags(&self) -> Vec<(&'static str, bool)> {
        vec![
            ("breeze", self.starter != Starter::None),
            ("sqlite", self.database == Database::Sqlite),
            ("ui", self.ui),
            ("starwind", self.ui && self.starwind),
        ]
    }
}

pub fn tools(_req: &Request, o: &LaravelOptions) -> Vec<ToolStatus> {
    let mut v = vec![
        tools::detect(&tools::PHP, true),
        tools::detect(&tools::COMPOSER, true),
    ];
    // extensions PHP indispensables à Laravel
    let exts = tools::php_extensions();
    if !exts.is_empty() {
        let mut needed = vec!["mbstring", "openssl", "tokenizer", "ctype", "xml", "curl"];
        needed.push(match o.database {
            Database::Sqlite => "pdo_sqlite",
            Database::Mysql => "pdo_mysql",
            Database::Pgsql => "pdo_pgsql",
        });
        let missing: Vec<&str> = needed
            .into_iter()
            .filter(|n| !exts.iter().any(|e| e == n))
            .collect();
        v.push(ToolStatus {
            id: "php-extensions".into(),
            label: "Extensions PHP".into(),
            required: true,
            found: true,
            path: None,
            version: Some(if missing.is_empty() {
                "mbstring, openssl, tokenizer, ctype, xml, curl, pdo".into()
            } else {
                format!("manquantes : {}", missing.join(", "))
            }),
            min: None,
            ok: missing.is_empty(),
            hint: "Activer les extensions PHP manquantes (php.ini, `apt install php-<ext>`)".into(),
        });
    }
    if o.needs_node() {
        v.push(tools::detect(&tools::NODE, true));
        v.push(tools::detect(&tools::NPM, true));
    }
    v
}

pub fn steps(req: &Request, o: &LaravelOptions) -> Vec<Step> {
    let artisan = |id: &'static str, phase, label: &str, args: &[&str]| {
        let mut a = vec!["artisan"];
        a.extend_from_slice(args);
        Step::run(id, phase, label, Cmd::new("php", &a, Cwd::Project))
    };
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
            "Créer le projet avec composer create-project laravel/laravel",
            Cmd::new(
                "composer",
                &[
                    "create-project",
                    "laravel/laravel",
                    &req.folder(),
                    "--prefer-dist",
                    "--no-interaction",
                ],
                Cwd::Parent,
            ),
        ),
    ];
    if let Some(stack) = o.starter.stack() {
        steps.push(Step::run(
            "starter-require",
            Phase::Install,
            "Installer laravel/breeze",
            Cmd::new(
                "composer",
                &["require", "laravel/breeze", "--dev", "--no-interaction"],
                Cwd::Project,
            ),
        ));
        let mut a = vec!["artisan", "breeze:install", stack];
        if o.dark && o.starter.has_frontend() {
            a.push("--dark");
        }
        if o.pest {
            a.push("--pest");
        }
        a.push("--no-interaction");
        steps.push(Step::run(
            "starter",
            Phase::Install,
            &format!("Installer le kit {}", o.starter.label()),
            Cmd::new("php", &a, Cwd::Project),
        ));
    } else if o.frontend || o.ui {
        steps.push(Step::run(
            "npm-install",
            Phase::Install,
            "Installer les dépendances front (npm install)",
            Cmd::new("npm", &["install"], Cwd::Project),
        ));
        if !o.ui {
            steps.push(Step::run(
                "npm-build",
                Phase::Install,
                "Compiler les assets (npm run build)",
                Cmd::new("npm", &["run", "build"], Cwd::Project),
            ));
        }
    }
    if o.database != Database::Sqlite {
        steps.push(Step::internal(
            "env",
            Phase::Configure,
            &format!(
                "Base de données : {} (.env et .env.example)",
                o.database.id()
            ),
            Internal::LaravelEnv(o.database),
        ));
    }
    steps.push(Step::internal(
        "layer",
        Phase::Layer,
        "Appliquer la couche AlphaRust (pages, modèle, migration, tests, docs)",
        Internal::Layer(if o.ui {
            let mut v = vec![
                "common",
                "laravel",
                "ui-blade",
                "laravel-ds",
                "site-laravel",
            ];
            if o.starwind {
                v.push("laravel-starwind");
            }
            v
        } else {
            vec!["common", "laravel"]
        }),
    ));
    if o.ui {
        steps.push(Step::internal(
            "design-kit",
            Phase::Layer,
            "Copier le design kit (resources/css/design, resources/js/design)",
            Internal::DesignKit(DesignTarget::Laravel),
        ));
        if let Some(t) = &o.theme {
            steps.push(Step::internal(
                "theme",
                Phase::Layer,
                "Écrire la palette de marque (resources/css/design/themes/theme.css, contrôle WCAG)",
                Internal::DesignTheme(t.clone(), DesignTarget::Laravel),
            ));
        }
        steps.push(Step::run(
            "npm-build",
            Phase::Install,
            "Compiler les assets du design kit (npm run build : Vite + Tailwind v4)",
            Cmd::new("npm", &["run", "build"], Cwd::Project),
        ));
    }
    steps.push(Step::internal(
        "routes",
        Phase::Configure,
        "Brancher routes/pages.php dans routes/web.php",
        Internal::LaravelRoutes,
    ));
    steps.push(
        Step::run(
            "pint",
            Phase::Configure,
            "Formater le code avec Laravel Pint",
            Cmd::new("./vendor/bin/pint", &["--quiet"], Cwd::Project),
        )
        .soft(),
    );
    if o.database == Database::Sqlite {
        steps.push(artisan(
            "migrate",
            Phase::Configure,
            "Migrer la base (table pages)",
            &["migrate", "--force"],
        ));
        steps.push(artisan(
            "seed",
            Phase::Configure,
            "Insérer les pages d'exemple",
            &["db:seed", "--class=PageSeeder", "--force"],
        ));
        if o.ui {
            steps.push(artisan(
                "seed-site",
                Phase::Configure,
                "Insérer les services et articles du site (SiteSeeder)",
                &["db:seed", "--class=SiteSeeder", "--force"],
            ));
        }
    }
    steps.push(Step::internal(
        "manifest",
        Phase::Configure,
        "Écrire le manifeste du projet",
        Internal::WriteManifest,
    ));
    if req.verify {
        steps.push(artisan(
            "test",
            Phase::Verify,
            "Lancer les tests (php artisan test)",
            &["test"],
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
    #[test]
    fn starwind_kit_adds_its_layer_and_flag() {
        let o = LaravelOptions {
            ui: true,
            starwind: true,
            ..Default::default()
        };
        assert!(o.flags().contains(&("starwind", true)));
        let mut r = Request::new(super::super::Framework::Laravel, "x");
        r.options = super::super::Options::Laravel(o.clone());
        let s = steps(&r, &o);
        let layer = s.iter().find(|s| s.id == "layer").unwrap();
        match &layer.action {
            super::super::Action::Do(Internal::Layer(v)) => {
                assert_eq!(v.last(), Some(&"laravel-starwind"))
            }
            _ => panic!(),
        }
    }

    use super::*;
    use crate::scaffold::{Action, Framework, Options};

    fn steps_with(f: impl FnOnce(&mut LaravelOptions)) -> Vec<Step> {
        let mut o = LaravelOptions::default();
        f(&mut o);
        let mut r = Request::new(Framework::Laravel, "demo");
        r.options = Options::Laravel(o.clone());
        steps(&r, &o)
    }

    fn display(steps: &[Step], id: &str) -> String {
        match &steps.iter().find(|s| s.id == id).unwrap().action {
            Action::Run(c) => c.display(),
            _ => panic!(),
        }
    }

    #[test]
    fn default_plan() {
        let s = steps_with(|_| {});
        assert_eq!(
            display(&s, "create"),
            "composer create-project laravel/laravel demo --prefer-dist --no-interaction"
        );
        let ids: Vec<&str> = s.iter().map(|s| s.id).collect();
        assert_eq!(
            ids,
            [
                "prepare", "create", "layer", "routes", "pint", "migrate", "seed", "manifest",
                "test", "lint", "git"
            ]
        );
        assert_eq!(display(&s, "test"), "php artisan test");
    }

    #[test]
    fn breeze_stack_flags() {
        let s = steps_with(|o| {
            o.starter = Starter::BreezeLivewire;
            o.dark = true;
            o.pest = true;
        });
        assert_eq!(
            display(&s, "starter-require"),
            "composer require laravel/breeze --dev --no-interaction"
        );
        assert_eq!(
            display(&s, "starter"),
            "php artisan breeze:install livewire --dark --pest --no-interaction"
        );
        let s = steps_with(|o| o.starter = Starter::BreezeApi);
        assert_eq!(
            display(&s, "starter"),
            "php artisan breeze:install api --no-interaction"
        );
    }

    #[test]
    fn non_sqlite_skips_migration_and_patches_env() {
        let s = steps_with(|o| o.database = Database::Mysql);
        let ids: Vec<&str> = s.iter().map(|s| s.id).collect();
        assert!(ids.contains(&"env") && !ids.contains(&"migrate") && !ids.contains(&"seed"));
    }

    #[test]
    fn option_validation() {
        assert!(LaravelOptions {
            dark: true,
            ..Default::default()
        }
        .validate()
        .is_err());
        assert!(LaravelOptions {
            pest: true,
            ..Default::default()
        }
        .validate()
        .is_err());
        assert!(LaravelOptions {
            starter: Starter::BreezeApi,
            dark: true,
            ..Default::default()
        }
        .validate()
        .is_err());
        assert!(LaravelOptions {
            starter: Starter::BreezeReact,
            dark: true,
            pest: true,
            ..Default::default()
        }
        .validate()
        .is_ok());
        assert_eq!(Starter::parse("breeze-vue"), Some(Starter::BreezeVue));
    }
}
