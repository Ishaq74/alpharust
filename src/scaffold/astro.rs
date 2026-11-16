//! Astro : `create-astro` officiel, `astro add` pour les intégrations, puis couche AlphaRust.

use super::exec::{Cmd, Cwd};
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
        }
    }
}

impl AstroOptions {
    pub fn has(&self, name: &str) -> bool {
        self.integrations.iter().any(|i| i == name)
    }

    pub fn validate(&self, install: bool) -> Result<()> {
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

    /// Drapeaux de rendu de la couche de templates.
    pub fn flags(&self, install: bool) -> Vec<(&'static str, bool)> {
        vec![
            ("tailwind", self.has("tailwind")),
            ("mdx", self.has("mdx")),
            ("sitemap", self.has("sitemap")),
            ("react", self.has("react")),
            ("vue", self.has("vue")),
            ("svelte", self.has("svelte")),
            ("official", self.template != Template::Alpharust),
            ("typecheck", self.typecheck && install),
            ("site", self.site.is_some()),
        ]
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

    let to_add: Vec<String> = o
        .integrations
        .iter()
        .filter(|i| !(o.template == Template::Blog && (*i == "mdx" || *i == "sitemap")))
        .cloned()
        .collect();
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
        steps.push(Step::internal(
            "scripts",
            Phase::Configure,
            "Ajouter le script `check` (astro check)",
            Internal::PackageScripts(vec![("check", "astro check")]),
        ));
    }

    steps.push(Step::internal(
        "layer",
        Phase::Layer,
        "Appliquer la couche AlphaRust (structure, thème, docs, config qualité)",
        Internal::Layer(vec!["common", "astro"]),
    ));
    if let Some(site) = &o.site {
        steps.push(Step::internal(
            "site",
            Phase::Configure,
            "Renseigner `site` dans astro.config.mjs",
            Internal::AstroSite(site.clone()),
        ));
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
}
