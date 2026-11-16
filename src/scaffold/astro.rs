//! Astro : `create-astro` officiel, `astro add` pour les intégrations, puis couche AlphaRust.

use super::exec::{Cmd, Cwd};
use super::platform::{self, Db, UiKit};
use super::theme::{self, ThemeColors};
use super::tools::{self, ToolStatus};
use super::{Internal, Phase, Request, Step};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    #[default]
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

impl PackageManager {
    pub const ALL: [PackageManager; 4] = [
        PackageManager::Npm,
        PackageManager::Pnpm,
        PackageManager::Yarn,
        PackageManager::Bun,
    ];

    pub fn id(self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
            PackageManager::Bun => "bun",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.id() == s.to_lowercase())
    }

    pub fn install_cmd(self) -> String {
        format!("{} install", self.id())
    }

    /// Préfixe d'exécution d'un script (`npm run`, `pnpm`, `yarn`, `bun run`).
    pub fn run_prefix(self) -> String {
        match self {
            PackageManager::Npm => "npm run".into(),
            PackageManager::Bun => "bun run".into(),
            p => p.id().to_string(),
        }
    }

    fn spec(self) -> &'static tools::ToolSpec {
        match self {
            PackageManager::Npm => &tools::NPM,
            PackageManager::Pnpm => &tools::PNPM,
            PackageManager::Yarn => &tools::YARN,
            PackageManager::Bun => &tools::BUN,
        }
    }

    /// `<pm> create astro@latest <dir> <flags…>` (chaque gestionnaire a sa syntaxe de passage d'arguments).
    fn create(self, folder: &str, flags: Vec<String>) -> Cmd {
        let mut args: Vec<String> = Vec::new();
        match self {
            PackageManager::Npm => {
                args.extend(["create", "astro@latest", folder, "--"].map(String::from));
            }
            PackageManager::Pnpm => {
                args.extend(["create", "astro@latest", folder].map(String::from));
            }
            PackageManager::Yarn => {
                args.extend(["create", "astro", folder].map(String::from));
            }
            PackageManager::Bun => {
                args.extend(["create", "astro@latest", folder].map(String::from));
            }
        }
        args.extend(flags);
        Cmd::owned(self.id(), args, Cwd::Parent)
    }

    /// `<pm> astro add <integrations…> --yes`
    fn astro_add(self, integrations: &[String]) -> Cmd {
        let mut args: Vec<String> = match self {
            PackageManager::Npm => vec!["exec".into(), "--".into(), "astro".into()],
            PackageManager::Pnpm | PackageManager::Yarn => vec!["astro".into()],
            PackageManager::Bun => vec!["x".into(), "astro".into()],
        };
        args.push("add".into());
        args.extend(integrations.iter().cloned());
        args.push("--yes".into());
        Cmd::owned(self.id(), args, Cwd::Project)
    }

    fn add_dev(self, pkgs: &[&str]) -> Cmd {
        let mut args: Vec<String> = match self {
            PackageManager::Npm => vec!["install".into(), "-D".into()],
            PackageManager::Pnpm | PackageManager::Yarn | PackageManager::Bun => {
                vec!["add".into(), "-D".into()]
            }
        };
        args.extend(pkgs.iter().map(|s| s.to_string()));
        Cmd::owned(self.id(), args, Cwd::Project)
    }

    fn add_dep(self, pkgs: &[String]) -> Cmd {
        let mut args: Vec<String> = match self {
            PackageManager::Npm => vec!["install".into()],
            _ => vec!["add".into()],
        };
        args.extend(pkgs.iter().cloned());
        Cmd::owned(self.id(), args, Cwd::Project)
    }

    /// Exécute un paquet sans l'installer (`npx`, `pnpm dlx`, `yarn dlx`, `bunx`) dans le projet.
    fn dlx(self, package: &str, args: &[String]) -> Cmd {
        let mut a: Vec<String> = match self {
            PackageManager::Npm => vec!["exec".into(), "--yes".into(), "--".into()],
            PackageManager::Pnpm | PackageManager::Yarn => vec!["dlx".into()],
            PackageManager::Bun => vec!["x".into()],
        };
        a.push(package.to_string());
        a.extend(args.iter().cloned());
        Cmd::owned(self.id(), a, Cwd::Project)
    }

    fn script(self, name: &str) -> Cmd {
        Cmd::owned(self.id(), vec!["run".into(), name.into()], Cwd::Project)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Template {
    /// `minimal` officiel + structure AlphaRust (layout, composants, contenu, thème clair/sombre).
    #[default]
    Alpharust,
    Minimal,
    Basics,
    Blog,
}

impl Template {
    pub const ALL: [Template; 4] = [
        Template::Alpharust,
        Template::Minimal,
        Template::Basics,
        Template::Blog,
    ];
    pub fn id(self) -> &'static str {
        match self {
            Template::Alpharust => "alpharust",
            Template::Minimal => "minimal",
            Template::Basics => "basics",
            Template::Blog => "blog",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|t| t.id() == s.to_lowercase())
    }
    /// Nom du template pour `create-astro`.
    fn official(self) -> &'static str {
        match self {
            Template::Alpharust | Template::Minimal => "minimal",
            Template::Basics => "basics",
            Template::Blog => "blog",
        }
    }
}

/// Niveau TypeScript = preset officiel `astro/tsconfigs/*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TsLevel {
    /// Permissif (`astro/tsconfigs/base`).
    Base,
    #[default]
    Strict,
    Strictest,
}

impl TsLevel {
    pub const ALL: [TsLevel; 3] = [TsLevel::Base, TsLevel::Strict, TsLevel::Strictest];
    pub fn id(self) -> &'static str {
        match self {
            TsLevel::Base => "base",
            TsLevel::Strict => "strict",
            TsLevel::Strictest => "strictest",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "relaxed" => Some(TsLevel::Base),
            other => Self::ALL.into_iter().find(|t| t.id() == other),
        }
    }
}

/// Intégrations installables via `astro add`.
pub const INTEGRATIONS: &[(&str, &str)] = &[
    ("tailwind", "Tailwind CSS (plugin Vite officiel)"),
    ("mdx", "Contenu MDX"),
    ("sitemap", "Sitemap XML (exige l'URL du site)"),
    ("react", "Îlots React"),
    ("vue", "Îlots Vue"),
    ("svelte", "Îlots Svelte"),
];

/// `astro check` ne gère pas encore TypeScript 7 : on épingle la branche 6.
const TYPESCRIPT_SPEC: &str = "typescript@^6";

const UI_FRAMEWORKS: &[&str] = &["react", "vue", "svelte"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AstroOptions {
    pub package_manager: PackageManager,
    pub template: Template,
    pub typescript: TsLevel,
    pub integrations: Vec<String>,
    /// URL publique du site (`https://…`), requise par `sitemap`.
    pub site: Option<String>,
    /// Installe `@astrojs/check` + `typescript` et vérifie avec `astro check`.
    pub typecheck: bool,
    /// Kit d'interface : Starwind (Tailwind) ou système CSS Concordia.
    pub ui: UiKit,
    /// Base de données Drizzle (PostgreSQL ou SQLite).
    pub database: Db,
    /// URL PostgreSQL de développement ; si elle est fournie, les migrations sont appliquées.
    pub db_url: Option<String>,
    /// Authentification Better Auth (exige une base de données).
    pub auth: bool,
    /// Plugins Better Auth : username, admin, organization.
    pub auth_plugins: Vec<String>,
    /// Langues (la première est la langue par défaut) : fr, en, es, ar.
    pub i18n: Vec<String>,
    /// Couleurs de marque (tous les kits), contrôlées WCAG, écrites dans `src/styles/themes/theme.css`.
    pub theme: Option<ThemeColors>,
    /// Variantes de composants CSS à inclure : initial (toujours), retro, modern, futuristic.
    pub css_variants: Vec<String>,
    /// Dockerfile + docker-compose.
    pub docker: bool,
    /// Workflow GitHub Actions (check, build, tests).
    pub ci: bool,
}

impl Default for AstroOptions {
    fn default() -> Self {
        Self {
            package_manager: PackageManager::Npm,
            template: Template::Alpharust,
            typescript: TsLevel::Strict,
            integrations: vec![],
            site: None,
            typecheck: true,
            ui: UiKit::None,
            database: Db::None,
            db_url: None,
            auth: false,
            auth_plugins: vec![],
            i18n: vec![],
            theme: None,
            css_variants: vec![],
            docker: false,
            ci: false,
        }
    }
}

impl AstroOptions {
    pub fn has(&self, name: &str) -> bool {
        self.integrations.iter().any(|i| i == name)
    }

    /// Rendu côté serveur (adaptateur Node) : base de données ou authentification.
    pub fn ssr(&self) -> bool {
        self.database != Db::None || self.auth || self.docker
    }

    /// Une brique « plateforme » est demandée (exige le template `alpharust`).
    pub fn platform(&self) -> bool {
        self.ui != UiKit::None || self.database != Db::None || self.auth || !self.i18n.is_empty()
    }

    /// Le projet généré embarque des tests (vitest).
    pub fn has_tests(&self) -> bool {
        self.database != Db::None || !self.i18n.is_empty()
    }

    /// Variante CSS incluse ? (liste vide = les quatre variantes de Concordia).
    pub fn css_variant(&self, name: &str) -> bool {
        if self.css_variants.is_empty() {
            true
        } else {
            name == "initial" || self.css_variants.iter().any(|v| v == name)
        }
    }

    /// Le site embarque un catalogue de messages (authentification ou plusieurs langues).
    pub fn has_messages(&self) -> bool {
        self.auth || !self.i18n.is_empty()
    }

    /// Langues du catalogue : celles demandées, ou le français seul pour l'authentification.
    pub fn locales(&self) -> Vec<String> {
        if !self.i18n.is_empty() {
            self.i18n.clone()
        } else if self.auth {
            vec!["fr".to_string()]
        } else {
            Vec::new()
        }
    }

    pub fn default_locale(&self) -> Option<&str> {
        self.i18n
            .first()
            .map(String::as_str)
            .or(if self.auth { Some("fr") } else { None })
    }

    /// Libellé du kit de design (README et docs du projet généré).
    pub fn kit_label(&self) -> &'static str {
        match (self.ui, self.has("tailwind")) {
            (UiKit::Starwind, _) => "Tailwind + Starwind UI",
            (UiKit::Concordia, _) => "CSS Concordia",
            (UiKit::None, true) => "Tailwind",
            (UiKit::None, false) => "CSS simple",
        }
    }

    /// Moteur CSS chargé en tête de `global.css`.
    pub fn engine_label(&self) -> &'static str {
        match (self.ui, self.has("tailwind")) {
            (UiKit::Starwind, _) => "`starwind.css` (Tailwind 4)",
            (UiKit::None, true) => "`tailwindcss`",
            _ => "aucun",
        }
    }

    /// Préfixe des variables de marque : `color` (jetons Concordia) ou `brand` (autres kits).
    pub fn theme_prefix(&self) -> &'static str {
        if self.ui == UiKit::Concordia {
            "color"
        } else {
            "brand"
        }
    }

    /// Intégrations passées à `astro add` : choix de l'utilisateur, Tailwind pour Starwind, adaptateur Node pour le SSR.
    pub fn astro_add_list(&self) -> Vec<String> {
        let mut v: Vec<String> = self
            .integrations
            .iter()
            .filter(|i| !(self.template == Template::Blog && (*i == "mdx" || *i == "sitemap")))
            .cloned()
            .collect();
        if self.ui == UiKit::Starwind && !v.iter().any(|i| i == "tailwind") {
            v.insert(0, "tailwind".into());
        }
        if self.ssr() {
            v.push("node".into());
        }
        v
    }

    pub fn validate(&self, install: bool) -> Result<()> {
        self.validate_platform(install)?;
        for i in &self.integrations {
            if !INTEGRATIONS.iter().any(|(n, _)| n == i) {
                bail!(
                    "intégration inconnue `{}` (disponibles : {})",
                    i,
                    INTEGRATIONS
                        .iter()
                        .map(|(n, _)| *n)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
        let mut seen = std::collections::BTreeSet::new();
        for i in &self.integrations {
            if !seen.insert(i) {
                bail!("intégration `{}` en double", i);
            }
        }
        let ui: Vec<&String> = self
            .integrations
            .iter()
            .filter(|i| UI_FRAMEWORKS.contains(&i.as_str()))
            .collect();
        if ui.len() > 1 {
            bail!("un seul framework d'îlots à la fois (react, vue ou svelte) : {ui:?}");
        }
        if !self.integrations.is_empty() && !install {
            bail!("les intégrations (`astro add`) exigent l'installation des dépendances (retirer --no-install)");
        }
        if self.typecheck && !install {
            // sans installation, pas de vérification de types : on l'ignore silencieusement
        }
        if self.has("sitemap") || self.template == Template::Blog {
            match &self.site {
                Some(s) if s.starts_with("https://") || s.starts_with("http://") => {
                    if s.chars()
                        .any(|c| c.is_whitespace() || c == '\'' || c == '"' || c == '\\')
                    {
                        bail!("URL du site invalide : `{s}`");
                    }
                }
                _ if self.has("sitemap") => {
                    bail!(
                        // alpharust-ignore: no-hardcoded-urls
                        "l'intégration `sitemap` exige l'URL du site (--site https://exemple.com)"
                    )
                }
                _ => {}
            }
        }
        if let Some(s) = &self.site {
            if !(s.starts_with("https://") || s.starts_with("http://"))
                || s.chars()
                    .any(|c| c.is_whitespace() || c == '\'' || c == '"' || c == '\\')
            {
                // alpharust-ignore: no-hardcoded-urls
                bail!("URL du site invalide : `{s}` (attendu : https://exemple.com)");
            }
        }
        Ok(())
    }

    fn validate_platform(&self, install: bool) -> Result<()> {
        if self.platform() && self.template != Template::Alpharust {
            bail!("les briques plateforme (--ui, --db, --auth, --i18n) exigent le template `alpharust` (structure complète)");
        }
        if (self.ui != UiKit::None || self.database != Db::None || self.auth) && !install {
            bail!("l'interface, la base de données et l'authentification exigent l'installation des dépendances (retirer --no-install)");
        }
        if self.docker && (!install || self.package_manager == PackageManager::Bun) {
            bail!("le Dockerfile généré cible npm, pnpm ou yarn avec installation des dépendances (image node:24)");
        }
        if self.ui == UiKit::Concordia && self.has("tailwind") {
            bail!("le kit `concordia` est un système CSS sans Tailwind : retirer l'intégration tailwind ou choisir `--ui starwind`");
        }
        if !self.css_variants.is_empty() && self.ui != UiKit::Concordia {
            bail!("--css-variant n'a de sens qu'avec `--ui concordia`");
        }
        if self.theme.is_some() && self.template != Template::Alpharust {
            bail!("--theme-* exige le template `alpharust` (le design system AlphaRust)");
        }
        if let Some(t) = &self.theme {
            t.validate()?;
        }
        for v in &self.css_variants {
            if !theme::VARIANTS.contains(&v.as_str()) {
                bail!(
                    "variante CSS inconnue `{v}` (disponibles : {})",
                    theme::VARIANTS.join(", ")
                );
            }
        }
        if self.auth && self.database == Db::None {
            bail!(
                "l'authentification (Better Auth) exige une base de données (--db postgres|sqlite)"
            );
        }
        if !self.auth_plugins.is_empty() && !self.auth {
            bail!("les plugins d'authentification exigent --auth");
        }
        let mut seen = std::collections::BTreeSet::new();
        for p in &self.auth_plugins {
            if !platform::AUTH_PLUGINS.iter().any(|(n, _)| n == p) {
                bail!(
                    "plugin d'authentification inconnu `{p}` (disponibles : {})",
                    platform::AUTH_PLUGINS
                        .iter()
                        .map(|(n, _)| *n)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            if !seen.insert(p) {
                bail!("plugin `{p}` en double");
            }
        }
        if !self.i18n.is_empty() {
            if self.i18n.len() < 2 {
                bail!("i18n demande au moins deux langues (ex. fr,en)");
            }
            let mut seen = std::collections::BTreeSet::new();
            for l in &self.i18n {
                if !platform::is_locale(l) {
                    bail!(
                        "langue inconnue `{l}` (disponibles : {})",
                        platform::LOCALES
                            .iter()
                            .map(|(c, _)| *c)
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                if !seen.insert(l) {
                    bail!("langue `{l}` en double");
                }
            }
        }
        if let Some(u) = &self.db_url {
            if self.database != Db::Postgres {
                bail!("--db-url ne concerne que PostgreSQL");
            }
            let ok_scheme = u.starts_with("postgres://") || u.starts_with("postgresql://");
            let bad = u.chars().any(|c| {
                c.is_whitespace() || matches!(c, '\'' | '"' | '\\' | '$' | '`' | '#' | '\n')
            });
            if !ok_scheme || bad {
                bail!("URL PostgreSQL invalide (attendu : schéma postgresql://, identifiants, hôte, port et base, sans espaces ni guillemets)");
            }
        }
        Ok(())
    }

    /// Drapeaux de rendu de la couche de templates.
    pub fn flags(&self, install: bool) -> Vec<(&'static str, bool)> {
        let starwind = self.ui == UiKit::Starwind;
        let mut v = vec![
            ("starwind", starwind),
            ("concordia", self.ui == UiKit::Concordia),
            ("db", self.database != Db::None),
            ("postgres", self.database == Db::Postgres),
            ("sqlite", self.database == Db::Sqlite),
            ("auth", self.auth),
            ("audit", self.auth),
            (
                "plugin_username",
                self.auth_plugins.iter().any(|p| p == "username"),
            ),
            (
                "plugin_admin",
                self.auth_plugins.iter().any(|p| p == "admin"),
            ),
            (
                "plugin_organization",
                self.auth_plugins.iter().any(|p| p == "organization"),
            ),
            ("i18n", !self.i18n.is_empty()),
            ("platform", self.platform()),
            ("ssr", self.ssr()),
            ("docker", self.docker),
            ("ci", self.ci),
            ("theme", self.theme.is_some()),
            ("css_retro", self.css_variant("retro")),
            ("css_modern", self.css_variant("modern")),
            ("css_futuristic", self.css_variant("futuristic")),
            ("tests", self.has_tests() && install),
        ];
        let locales = self.locales();
        v.push(("messages", self.has_messages()));
        for (code, _) in platform::LOCALES {
            let on = locales.iter().any(|l| l == code);
            v.push((
                match *code {
                    "fr" => "i18n_fr",
                    "en" => "i18n_en",
                    "es" => "i18n_es",
                    _ => "i18n_ar",
                },
                on,
            ));
        }
        v.extend(vec![
            ("tailwind", self.has("tailwind") || starwind),
            ("mdx", self.has("mdx")),
            ("sitemap", self.has("sitemap")),
            ("react", self.has("react")),
            ("vue", self.has("vue")),
            ("svelte", self.has("svelte")),
            ("official", self.template != Template::Alpharust),
            ("typecheck", self.typecheck && install),
            ("site", self.site.is_some()),
        ]);
        v
    }
}

pub fn tools(req: &Request, o: &AstroOptions) -> Vec<ToolStatus> {
    let mut v = vec![tools::detect(&tools::NODE, true)];
    v.push(tools::detect(o.package_manager.spec(), true));
    if o.package_manager != PackageManager::Npm {
        // create-astro appelle `npx`/`npm` pour certaines opérations : npm reste recommandé
        v.push(tools::detect(&tools::NPM, false));
    }
    let _ = req;
    v
}

pub fn steps(req: &Request, o: &AstroOptions) -> Vec<Step> {
    let pm = o.package_manager;
    let mut steps = vec![Step::internal(
        "prepare",
        Phase::Prepare,
        "Contrôle de la destination",
        Internal::Prepare,
    )];

    let mut flags = vec![
        "--template".to_string(),
        o.template.official().to_string(),
        if req.install {
            "--install"
        } else {
            "--no-install"
        }
        .to_string(),
        "--no-git".to_string(),
        "--no-ai".to_string(),
        "--yes".to_string(),
        "--skip-houston".to_string(),
    ];
    flags.retain(|f| !f.is_empty());
    steps.push(Step::run(
        "create",
        Phase::Scaffold,
        &format!(
            "Créer le projet avec create-astro (template {})",
            o.template.official()
        ),
        pm.create(&req.folder(), flags),
    ));

    steps.push(Step::internal(
        "tsconfig",
        Phase::Configure,
        &format!("TypeScript : preset astro/tsconfigs/{}", o.typescript.id()),
        Internal::AstroTsconfig(o.typescript),
    ));

    let to_add = o.astro_add_list();
    if !to_add.is_empty() && req.install {
        steps.push(Step::run(
            "integrations",
            Phase::Install,
            &format!("Ajouter les intégrations : {}", to_add.join(", ")),
            pm.astro_add(&to_add),
        ));
    }
    if o.typecheck && req.install {
        steps.push(Step::run(
            "typecheck-deps",
            Phase::Install,
            "Installer @astrojs/check et typescript 6",
            pm.add_dev(&["@astrojs/check", TYPESCRIPT_SPEC]),
        ));
    }

    // ── Briques plateforme : outils officiels d'abord (starwind, paquets), glue ensuite ──
    if req.install && o.ui == UiKit::Starwind {
        steps.push(Step::run(
            "starwind-init",
            Phase::Install,
            "Initialiser Starwind UI (starwind init)",
            pm.dlx("starwind@latest", &["init".into(), "--defaults".into()]),
        ));
        steps.push(Step::internal(
            "starwind-config",
            Phase::Configure,
            "Configurer Starwind comme atomic (composants dans src/components/atoms, utilitaires dans src/lib/starwind)",
            Internal::StarwindConfig,
        ));
        steps.push(Step::run(
            "atoms-deps",
            Phase::Install,
            "Installer les dépendances des composants atomic (tailwind-variants, @tabler/icons, embla-carousel, motion, astro-icon)",
            pm.add_dep(&[
                "tailwind-variants".to_string(),
                "@tabler/icons".to_string(),
                "embla-carousel".to_string(),
                "motion".to_string(),
                "astro-icon".to_string(),
                "@iconify-json/mdi".to_string(),
            ]),
        ));
    }
    if req.install && o.ui == UiKit::Concordia {
        steps.push(Step::run(
            "icons-deps",
            Phase::Install,
            "Installer astro-icon et les icônes MDI (composants Concordia)",
            pm.add_dep(&["astro-icon".to_string(), "@iconify-json/mdi".to_string()]),
        ));
    }
    if req.install && (o.database != Db::None || o.has_tests()) {
        if o.database != Db::None {
            let mut deps: Vec<String> = vec![
                "dotenv".into(),
                "drizzle-orm".into(),
                o.database.driver().into(),
            ];
            if o.auth {
                deps.push("better-auth".into());
            }
            steps.push(Step::run(
                "platform-deps",
                Phase::Install,
                &format!("Installer {}", deps.join(", ")),
                pm.add_dep(&deps),
            ));
        }
        let mut dev: Vec<&str> = vec!["vitest"];
        if o.database != Db::None {
            dev.extend(["drizzle-kit", o.database.driver_types()]);
        }
        steps.push(Step::run(
            "platform-dev-deps",
            Phase::Install,
            &format!("Installer (dev) {}", dev.join(", ")),
            pm.add_dev(&dev),
        ));
    }

    let mut scripts: Vec<(&'static str, &'static str)> = vec![];
    if o.typecheck && req.install {
        scripts.push(("check", "astro check"));
    }
    if o.database != Db::None {
        scripts.extend([
            ("db:generate", "drizzle-kit generate"),
            ("db:migrate", "drizzle-kit migrate"),
            ("db:studio", "drizzle-kit studio"),
        ]);
    }
    if o.has_tests() {
        scripts.extend([("test", "vitest run"), ("test:watch", "vitest")]);
    }
    if o.theme.is_some() {
        scripts.push(("theme:check", "node scripts/theme-check.mjs"));
    }
    if !scripts.is_empty() && req.install {
        steps.push(Step::internal(
            "scripts",
            Phase::Configure,
            &format!(
                "Ajouter les scripts : {}",
                scripts
                    .iter()
                    .map(|(k, _)| *k)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Internal::PackageScripts(scripts),
        ));
    }

    let mut layers = vec!["common", "astro"];
    if o.platform() || o.docker || o.ci {
        layers.push("platform");
    }
    if o.ui == UiKit::Concordia {
        layers.push("ui-concordia");
    }
    if o.ui == UiKit::Starwind {
        layers.push("ui-atomic");
    }
    steps.push(Step::internal(
        "layer",
        Phase::Layer,
        "Appliquer la couche AlphaRust (structure, thème, docs, config qualité)",
        Internal::Layer(layers),
    ));
    if let Some(site) = &o.site {
        steps.push(Step::internal(
            "site",
            Phase::Configure,
            "Renseigner `site` dans astro.config.mjs",
            Internal::AstroSite(site.clone()),
        ));
    }
    if o.platform() {
        steps.push(Step::internal(
            "paths",
            Phase::Configure,
            "Alias de chemins TypeScript (@lib, @database, @components…)",
            Internal::AstroPaths,
        ));
        steps.push(Step::internal(
            "astro-config",
            Phase::Configure,
            "Configurer astro.config (rendu serveur, i18n, sécurité)",
            Internal::AstroConfig {
                ssr: o.ssr(),
                locales: o.i18n.clone(),
                icons: o.ui != UiKit::None,
            },
        ));
    }
    if let Some(t) = &o.theme {
        steps.push(Step::internal(
            "theme",
            Phase::Configure,
            "Générer src/styles/themes/theme.css (contrôle WCAG)",
            Internal::CssTheme(t.clone(), o.theme_prefix()),
        ));
    }
    if req.install && o.auth {
        steps.push(Step::run(
            "auth-schema",
            Phase::Install,
            "Générer le schéma Better Auth (auth generate → Drizzle)",
            pm.dlx(
                "auth@latest",
                &[
                    "generate".into(),
                    "--adapter".into(),
                    "drizzle".into(),
                    "--dialect".into(),
                    o.database.dialect().into(),
                    "--config".into(),
                    "src/lib/auth.ts".into(),
                    "--output".into(),
                    "src/database/schemas/auth.schema.ts".into(),
                    "--yes".into(),
                ],
            ),
        ));
    }
    if req.install && o.database != Db::None {
        steps.push(Step::run(
            "db-generate",
            Phase::Configure,
            "Générer les migrations SQL (drizzle-kit generate)",
            pm.script("db:generate"),
        ));
        if o.database == Db::Sqlite {
            steps.push(Step::run(
                "db-migrate",
                Phase::Configure,
                "Appliquer les migrations (drizzle-kit migrate)",
                pm.script("db:migrate"),
            ));
        } else if o.db_url.is_some() {
            steps.push(
                Step::run(
                    "db-migrate",
                    Phase::Configure,
                    "Appliquer les migrations sur PostgreSQL (drizzle-kit migrate)",
                    pm.script("db:migrate"),
                )
                .soft(),
            );
        }
    }
    steps.push(Step::internal(
        "manifest",
        Phase::Configure,
        "Écrire le manifeste du projet",
        Internal::WriteManifest,
    ));

    if req.verify && req.install {
        if o.typecheck {
            steps.push(Step::run(
                "check",
                Phase::Verify,
                "Vérifier les types (astro check)",
                pm.script("check"),
            ));
        }
        steps.push(Step::run(
            "build",
            Phase::Verify,
            "Construire le site (astro build)",
            pm.script("build"),
        ));
        if o.has_tests() {
            steps.push(Step::run(
                "test",
                Phase::Verify,
                "Lancer les tests du projet (vitest)",
                pm.script("test"),
            ));
        }
    }
    if req.verify {
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

    fn req(f: impl FnOnce(&mut AstroOptions, &mut Request)) -> Request {
        let mut r = Request::new(Framework::Astro, "demo");
        let mut o = AstroOptions::default();
        f(&mut o, &mut r);
        r.options = Options::Astro(o);
        r
    }

    fn cmd_of<'a>(steps: &'a [Step], id: &str) -> &'a Cmd {
        match &steps.iter().find(|s| s.id == id).unwrap().action {
            Action::Run(c) => c,
            _ => panic!("pas une commande"),
        }
    }

    fn steps_for(r: &Request) -> Vec<Step> {
        match &r.options {
            Options::Astro(o) => steps(r, o),
            _ => unreachable!(),
        }
    }

    #[test]
    fn default_plan_uses_official_tools_in_order() {
        let s = steps_for(&req(|_, _| {}));
        let ids: Vec<&str> = s.iter().map(|s| s.id).collect();
        assert_eq!(
            ids,
            [
                "prepare",
                "create",
                "tsconfig",
                "typecheck-deps",
                "scripts",
                "layer",
                "manifest",
                "check",
                "build",
                "lint",
                "git"
            ]
        );
        let c = cmd_of(&s, "create");
        assert_eq!(c.program, "npm");
        assert_eq!(
            c.args.join(" "),
            "create astro@latest demo -- --template minimal --install --no-git --no-ai --yes --skip-houston"
        );
        assert_eq!(c.cwd, Cwd::Parent);
        assert_eq!(cmd_of(&s, "build").display(), "npm run build");
    }

    #[test]
    fn package_manager_changes_every_command() {
        let r = req(|o, _| {
            o.package_manager = PackageManager::Pnpm;
            o.integrations = vec!["tailwind".into(), "mdx".into()];
        });
        let s = steps_for(&r);
        assert_eq!(
            cmd_of(&s, "create").display(),
            "pnpm create astro@latest demo --template minimal --install --no-git --no-ai --yes --skip-houston"
        );
        assert_eq!(
            cmd_of(&s, "integrations").display(),
            "pnpm astro add tailwind mdx --yes"
        );
        assert_eq!(
            cmd_of(&s, "typecheck-deps").display(),
            "pnpm add -D @astrojs/check typescript@^6"
        );
        assert_eq!(cmd_of(&s, "build").display(), "pnpm run build");

        let r = req(|o, _| {
            o.package_manager = PackageManager::Npm;
            o.integrations = vec!["react".into()];
        });
        let s = steps_for(&r);
        assert_eq!(
            cmd_of(&s, "integrations").display(),
            "npm exec -- astro add react --yes"
        );
        assert_eq!(
            cmd_of(&s, "typecheck-deps").display(),
            "npm install -D @astrojs/check typescript@^6"
        );
    }

    #[test]
    fn no_install_skips_install_and_verification_steps() {
        let s = steps_for(&req(|_, r| r.install = false));
        let ids: Vec<&str> = s.iter().map(|s| s.id).collect();
        assert!(
            !ids.contains(&"build") && !ids.contains(&"check") && !ids.contains(&"typecheck-deps")
        );
        assert!(cmd_of(&s, "create")
            .args
            .contains(&"--no-install".to_string()));
        assert!(ids.contains(&"lint"));
    }

    #[test]
    fn blog_template_does_not_re_add_its_own_integrations() {
        let r = req(|o, _| {
            o.template = Template::Blog;
            // alpharust-ignore: no-hardcoded-urls
            o.site = Some("https://exemple.com".into());
            o.integrations = vec!["mdx".into(), "sitemap".into(), "tailwind".into()];
        });
        let s = steps_for(&r);
        assert_eq!(
            cmd_of(&s, "integrations").display(),
            "npm exec -- astro add tailwind --yes"
        );
    }

    #[test]
    fn validation_rules() {
        let o = |f: &dyn Fn(&mut AstroOptions)| {
            let mut o = AstroOptions::default();
            f(&mut o);
            o.validate(true)
        };
        assert!(o(&|o| o.integrations = vec!["nope".into()]).is_err());
        assert!(o(&|o| o.integrations = vec!["react".into(), "vue".into()]).is_err());
        assert!(o(&|o| o.integrations = vec!["mdx".into(), "mdx".into()]).is_err());
        assert!(o(&|o| o.integrations = vec!["sitemap".into()]).is_err());
        assert!(o(&|o| {
            o.integrations = vec!["sitemap".into()];
            // alpharust-ignore: no-hardcoded-urls
            o.site = Some("https://exemple.com".into());
        })
        .is_ok());
        assert!(o(&|o| o.site = Some("javascript:alert(1)".into())).is_err());
        // alpharust-ignore: no-hardcoded-urls
        assert!(o(&|o| o.site = Some("https://a.com/'x".into())).is_err());
        assert!(AstroOptions {
            integrations: vec!["mdx".into()],
            ..Default::default()
        }
        .validate(false)
        .is_err());
        assert_eq!(TsLevel::parse("relaxed"), Some(TsLevel::Base));
        assert_eq!(PackageManager::parse("PNPM"), Some(PackageManager::Pnpm));
    }
    fn platform_req(f: impl FnOnce(&mut AstroOptions)) -> Request {
        req(|o, _| {
            o.template = Template::Alpharust;
            f(o);
        })
    }

    fn ids(r: &Request) -> Vec<&'static str> {
        steps_for(r).iter().map(|s| s.id).collect()
    }

    #[test]
    fn platform_plan_orders_official_tools_before_glue() {
        let r = platform_req(|o| {
            o.ui = UiKit::Starwind;
            o.database = Db::Sqlite;
            o.auth = true;
        });
        let order = ids(&r);
        let pos = |id: &str| {
            order
                .iter()
                .position(|x| *x == id)
                .unwrap_or_else(|| panic!("{id}"))
        };
        assert!(pos("starwind-init") < pos("starwind-config"));
        assert!(pos("starwind-config") < pos("atoms-deps"));
        assert!(pos("atoms-deps") < pos("platform-deps"));
        assert!(
            !order.contains(&"starwind-add"),
            "les composants viennent du calque atomic, pas de `starwind add`"
        );
        assert!(pos("platform-dev-deps") < pos("layer"));
        assert!(
            pos("layer") < pos("auth-schema"),
            "le schéma Better Auth lit auth.ts posé par la couche"
        );
        assert!(pos("auth-schema") < pos("db-generate"));
        assert!(pos("db-generate") < pos("db-migrate"));
        assert!(pos("db-migrate") < pos("check"));
        let s = steps_for(&r);
        assert!(cmd_of(&s, "starwind-init")
            .display()
            .contains("starwind@latest init --defaults"));
        assert!(cmd_of(&s, "auth-schema")
            .display()
            .contains("auth@latest generate --adapter drizzle --dialect sqlite"));
    }

    #[test]
    fn platform_flags_and_locales() {
        let r = platform_req(|o| {
            o.database = Db::Sqlite;
            o.auth = true;
        });
        let Options::Astro(o) = &r.options else {
            unreachable!()
        };
        assert_eq!(
            o.locales(),
            vec!["fr".to_string()],
            "auth seule : catalogue français"
        );
        let flags: std::collections::HashMap<_, _> = o.flags(true).into_iter().collect();
        assert!(
            flags["messages"]
                && flags["i18n_fr"]
                && !flags["i18n"]
                && flags["ssr"]
                && flags["audit"]
        );

        let r = platform_req(|o| o.i18n = vec!["en".into(), "ar".into()]);
        let Options::Astro(o) = &r.options else {
            unreachable!()
        };
        assert_eq!(o.default_locale(), Some("en"));
        let flags: std::collections::HashMap<_, _> = o.flags(true).into_iter().collect();
        assert!(flags["i18n_en"] && flags["i18n_ar"] && !flags["i18n_fr"] && !flags["ssr"]);
    }

    #[test]
    fn platform_validation_rejects_incoherent_choices() {
        let err = |f: fn(&mut AstroOptions), install: bool| {
            let r = platform_req(f);
            match &r.options {
                Options::Astro(o) => o.validate(install).unwrap_err().to_string(),
                _ => unreachable!(),
            }
        };
        assert!(err(|o| o.auth = true, true).contains("base de données"));
        assert!(err(|o| o.database = Db::Sqlite, false).contains("installation"));
        assert!(err(|o| o.ui = UiKit::Concordia, false).contains("installation"));
        assert!(err(|o| o.i18n = vec!["fr".into()], true).contains("deux langues"));
        assert!(err(|o| o.i18n = vec!["xx".into(), "fr".into()], true).contains("xx"));
        assert!(err(
            |o| {
                o.ui = UiKit::Concordia;
                o.integrations = vec!["tailwind".into()];
            },
            true
        )
        .contains("Tailwind"));
        let r = req(|o, _| {
            o.template = Template::Minimal;
            o.auth = true;
        });
        let Options::Astro(o) = &r.options else {
            unreachable!()
        };
        assert!(o
            .validate(true)
            .unwrap_err()
            .to_string()
            .contains("alpharust"));
    }

    #[test]
    fn concordia_theme_step_and_layers() {
        let r = platform_req(|o| {
            o.ui = UiKit::Concordia;
            o.theme = Some(ThemeColors::default());
        });
        let s = steps_for(&r);
        assert!(s.iter().any(|x| x.id == "icons-deps"));
        assert!(s.iter().any(|x| x.id == "theme"));
        match &s.iter().find(|x| x.id == "layer").unwrap().action {
            Action::Do(Internal::Layer(l)) => {
                assert_eq!(l, &vec!["common", "astro", "platform", "ui-concordia"]);
            }
            _ => panic!("couche attendue"),
        }
    }
}
