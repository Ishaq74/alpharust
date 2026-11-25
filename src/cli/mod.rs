//! Interface en ligne de commande.

use crate::engine::{self, composer, reporter, validate};
use crate::generators::{checks, docs, readme};
use crate::parser::{cache, Rule};
use crate::profiles::Profile;
use crate::scaffold::astro::{AstroOptions, PackageManager, Template, TsLevel};
use crate::scaffold::laravel::{Database, LaravelOptions, Starter};
use crate::scaffold::platform::{Db, UiKit};
use crate::scaffold::rust::{CrateKind, RustOptions};
use crate::scaffold::theme::ThemeColors;
use crate::scaffold::wagtail::WagtailOptions;
use crate::scaffold::{self, Event, Framework, Options, Request};
use crate::{gui, paths, sarif, tui};
use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::io::Write;
use std::path::PathBuf;

mod progress;
mod wizard;

#[derive(Parser)]
#[command(
    name = "alpharust",
    version,
    about = "AlphaRust — règles de qualité, générateurs de projets, CLI + TUI + GUI web",
    long_about = None
)]
struct Cli {
    /// Ne pas utiliser le cache des règles (.alpharust-cache/)
    #[arg(long, global = true)]
    no_cache: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lint un chemin avec les règles AlphaRust
    Lint {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(
            short,
            long,
            help = "Profil à utiliser (défaut: .alpharust.toml, sinon toutes les règles)"
        )]
        profile: Option<String>,
        #[arg(long, help = "Sortie au format SARIF 2.1.0", conflicts_with = "json")]
        sarif: bool,
        #[arg(long, help = "Sortie JSON")]
        json: bool,
        #[arg(
            long,
            value_enum,
            help = "Sévérité minimale qui fait échouer la commande (défaut: error)"
        )]
        fail_on: Option<Severity>,
    },
    /// Lister les règles, les profils ou les checkers
    List {
        #[arg(long, help = "Lister les profils au lieu des règles")]
        profiles: bool,
        #[arg(
            long,
            help = "Lister les checkers sémantiques",
            conflicts_with = "profiles"
        )]
        checkers: bool,
    },
    /// Valider toutes les règles, exemples et profils
    CheckRules,
    /// Assistant interactif : questions guidées, aperçu des commandes, génération
    New {
        /// Nom du projet (sinon demandé)
        name: Option<String>,
        /// Framework (sinon demandé)
        #[arg(value_enum, short, long)]
        framework: Option<GeneratorKind>,
    },
    /// Générer un projet avec les outils officiels (create-astro, composer, wagtail start, cargo new)
    Generate {
        #[arg(value_enum)]
        framework: GeneratorKind,
        name: String,
        #[command(flatten)]
        opts: GenOpts,
    },
    /// Lister les projets générés (état, briques, tâches disponibles)
    Projects {
        #[arg(long, help = "Dossier à parcourir (défaut : sites/)")]
        dir: Option<PathBuf>,
        #[arg(long, help = "Sortie JSON")]
        json: bool,
    },
    /// Exécuter une tâche sur un projet généré : install, dev, build, check, test, lint, update
    Run {
        /// Nom (dans sites/) ou chemin du projet
        project: String,
        /// install | dev | build | check | test | lint | update
        task: String,
        #[arg(long, help = "Dossier des projets (défaut : sites/)")]
        dir: Option<PathBuf>,
        #[arg(long, help = "Événements en JSON (une ligne par événement)")]
        json: bool,
        #[arg(short, long, help = "Afficher toutes les lignes de journal")]
        verbose: bool,
    },
    /// Lister les préréglages de génération (`generate astro <nom> --preset …`)
    Presets {
        #[arg(long, help = "Sortie JSON")]
        json: bool,
    },
    /// Initialiser un projet Rust (alias de `generate rust`)
    Init {
        name: String,
        #[command(flatten)]
        opts: GenOpts,
    },
    /// Vérifier les outils installés (node, npm/pnpm/yarn/bun, php, composer, python, cargo, git)
    Doctor {
        #[arg(value_enum, short, long)]
        framework: Option<GeneratorKind>,
    },
    /// Générer un README
    Readme { name: String },
    /// Générer de la documentation
    Docs { name: String },
    /// Générer une checklist Markdown des règles d'un profil
    Checks {
        #[arg(short, long)]
        profile: Option<String>,
    },
    /// Gérer le cache des règles compilées
    Cache {
        #[arg(value_enum)]
        action: CacheAction,
    },
    /// Mode TUI interactif
    Tui,
    /// Mode GUI web
    Gui {
        #[arg(short, long, default_value_t = gui::DEFAULT_PORT)]
        port: u16,
    },
}

#[derive(ValueEnum, Clone, Copy)]
enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    fn as_str(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }
}

#[derive(ValueEnum, Clone, Copy)]
enum GeneratorKind {
    Astro,
    Laravel,
    Wagtail,
    Rust,
}

impl GeneratorKind {
    fn framework(self) -> Framework {
        match self {
            GeneratorKind::Astro => Framework::Astro,
            GeneratorKind::Laravel => Framework::Laravel,
            GeneratorKind::Wagtail => Framework::Wagtail,
            GeneratorKind::Rust => Framework::Rust,
        }
    }
}

/// Options de génération (communes + propres à chaque framework).
#[derive(clap::Args, Clone, Default)]
struct GenOpts {
    #[arg(short, long, help = "Dossier du projet (défaut : sites/<name>)")]
    path: Option<PathBuf>,
    #[arg(
        long,
        help = "Remplacer un projet AlphaRust existant (jamais un dossier étranger)"
    )]
    force: bool,
    #[arg(long, help = "Ne pas installer les dépendances")]
    no_install: bool,
    #[arg(long, help = "Ne pas vérifier le résultat (build, tests, lint)")]
    no_verify: bool,
    #[arg(long, help = "Pas de dépôt Git")]
    no_git: bool,
    #[arg(long, help = "Afficher le plan exact (commandes) sans rien exécuter")]
    dry_run: bool,
    #[arg(long, help = "Événements en JSON (une ligne par événement) sur stdout")]
    json: bool,
    #[arg(short, long, help = "Afficher toutes les lignes de journal")]
    verbose: bool,
    // — Astro —
    #[arg(
        long,
        value_name = "ID",
        help = "[astro] préréglage de briques (voir `alpharust presets`)"
    )]
    preset: Option<String>,
    #[arg(
        long,
        value_name = "npm|pnpm|yarn|bun",
        help = "[astro] gestionnaire de paquets"
    )]
    pm: Option<String>,
    #[arg(
        long,
        value_name = "alpharust|minimal|basics|blog",
        help = "[astro] template"
    )]
    template: Option<String>,
    #[arg(
        long = "ts",
        value_name = "base|strict|strictest",
        help = "[astro] niveau TypeScript"
    )]
    typescript: Option<String>,
    #[arg(
        short = 'i',
        long = "integration",
        value_delimiter = ',',
        help = "[astro] intégrations : tailwind, mdx, sitemap, react, vue, svelte"
    )]
    integrations: Vec<String>,
    #[arg(
        long,
        help = "[astro] URL publique du site (requise par sitemap et blog)"
    )]
    site: Option<String>,
    #[arg(long, help = "[astro] ne pas installer @astrojs/check")]
    no_typecheck: bool,
    #[arg(
        long,
        value_name = "none|starwind|concordia|css",
        help = "kit d'interface — astro : starwind (Tailwind) ou concordia (CSS) ; laravel, wagtail : css (design kit CSS) ou starwind (Tailwind + classes Starwind), même catalogue et même site"
    )]
    ui: Option<String>,
    #[arg(
        long,
        value_name = "#rrggbb",
        help = "[astro ; laravel, wagtail avec --ui css|starwind] couleur primaire de marque (WCAG)"
    )]
    theme_primary: Option<String>,
    #[arg(
        long,
        value_name = "#rrggbb",
        help = "[astro ; laravel, wagtail avec --ui css|starwind] couleur secondaire de marque"
    )]
    theme_secondary: Option<String>,
    #[arg(
        long,
        value_name = "#rrggbb",
        help = "[astro ; laravel, wagtail avec --ui css|starwind] couleur d'accent de marque"
    )]
    theme_accent: Option<String>,
    #[arg(
        long = "css-variant",
        value_delimiter = ',',
        help = "[astro, ui concordia] variantes : initial, retro, modern, futuristic (défaut : toutes)"
    )]
    css_variants: Vec<String>,
    #[arg(
        long,
        value_name = "URL",
        help = "[astro, db postgres] URL PostgreSQL de développement (applique les migrations)"
    )]
    db_url: Option<String>,
    #[arg(
        long,
        help = "comptes du site : [astro] Better Auth (exige --db) ; [laravel] Fortify ; [wagtail] auth Django (exigent --ui)"
    )]
    auth: bool,
    #[arg(
        long,
        value_name = "JEU",
        help = "[site vitrine] jeu d'icônes Iconify : mdi (défaut), lucide, ou tout jeu Iconify (tabler, ph…) ; drapeaux Circle Flags"
    )]
    icons: Option<String>,
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "LANGUES",
        help = "langues : fr, en, es, ar (RTL) — site vitrine (fr toujours, défaut fr,en) ; [astro] sans --ui : routage i18n (la première est par défaut)"
    )]
    langs: Vec<String>,
    #[arg(
        long = "auth-plugin",
        value_delimiter = ',',
        help = "[astro] plugins Better Auth : username, admin, organization"
    )]
    auth_plugins: Vec<String>,
    #[arg(
        long,
        value_delimiter = ',',
        help = "[astro] déprécié : alias de --langs (la première langue est celle par défaut)"
    )]
    i18n: Vec<String>,
    #[arg(long, help = "[astro] Dockerfile + docker-compose")]
    docker: bool,
    #[arg(long, help = "[astro] workflow GitHub Actions")]
    ci: bool,
    // — Laravel —
    #[arg(
        long,
        value_name = "none|breeze-blade|breeze-livewire|breeze-react|breeze-vue|breeze-api",
        help = "[laravel] kit de démarrage"
    )]
    starter: Option<String>,
    #[arg(long, help = "[laravel] Breeze en mode sombre")]
    dark: bool,
    #[arg(long, help = "[laravel] Pest au lieu de PHPUnit (Breeze)")]
    pest: bool,
    #[arg(
        long,
        value_name = "sqlite|postgres|mysql|none",
        help = "base de données — [laravel] sqlite (défaut), postgres (pilote pgsql), mysql ; [astro] base Drizzle : none (défaut), sqlite, postgres"
    )]
    db: Option<String>,
    #[arg(long, help = "[laravel] npm install && npm run build même sans kit")]
    frontend: bool,
    // — Wagtail —
    #[arg(
        long = "package",
        value_delimiter = ',',
        help = "[wagtail] paquets pip supplémentaires"
    )]
    packages: Vec<String>,
    #[arg(long, help = "[wagtail] ne pas exécuter migrate")]
    no_migrate: bool,
    // — Rust —
    #[arg(long, value_name = "bin|lib", help = "[rust] type de crate")]
    kind: Option<String>,
    #[arg(long, value_name = "2018|2021|2024", help = "[rust] édition")]
    edition: Option<String>,
    #[arg(
        long = "dep",
        value_delimiter = ',',
        help = "[rust] dépendances (cargo add) : clap, anyhow, serde…"
    )]
    deps: Vec<String>,
}

fn parse_opt<T>(
    label: &str,
    v: &Option<String>,
    f: impl Fn(&str) -> Option<T>,
    choices: &str,
) -> Result<Option<T>> {
    match v {
        None => Ok(None),
        Some(s) => f(s).map(Some).ok_or_else(|| {
            anyhow::anyhow!("valeur invalide pour {label} : `{s}` (attendu : {choices})")
        }),
    }
}

/// `--ui` pour Laravel et Wagtail : `css` (design kit en CSS pur), `starwind` (même catalogue, Tailwind + classes
/// Starwind) ou `none`. Renvoie (catalogue, starwind).
fn parse_catalog_ui(v: &Option<String>) -> Result<(bool, bool)> {
    match v.as_deref() {
        None | Some("none") => Ok((false, false)),
        Some("css") | Some("concordia") => Ok((true, false)),
        Some("starwind") => Ok((true, true)),
        Some(other) => {
            bail!("valeur invalide pour --ui : `{other}` (attendu pour laravel et wagtail : none, css, starwind)")
        }
    }
}

/// Palette de marque depuis `--theme-primary/secondary/accent` (Astro, et design kit Laravel/Wagtail).
fn theme_from(o: &GenOpts) -> Option<ThemeColors> {
    if o.theme_primary.is_none() && o.theme_secondary.is_none() && o.theme_accent.is_none() {
        return None;
    }
    let mut t = ThemeColors::default();
    if let Some(c) = &o.theme_primary {
        t.primary = c.clone();
    }
    if let Some(c) = &o.theme_secondary {
        t.secondary = c.clone();
    }
    if let Some(c) = &o.theme_accent {
        t.accent = c.clone();
    }
    Some(t)
}

/// Construit la demande depuis les flags ; refuse les options d'un autre framework.
fn request_from(framework: Framework, name: &str, o: &GenOpts) -> Result<Request> {
    let mut foreign: Vec<&str> = Vec::new();
    let astro_set = o.pm.is_some()
        || o.preset.is_some()
        || o.template.is_some()
        || o.typescript.is_some()
        || !o.integrations.is_empty()
        || o.site.is_some()
        || o.no_typecheck
        || (o.ui.is_some() && !matches!(framework, Framework::Laravel | Framework::Wagtail))
        || ((o.theme_primary.is_some() || o.theme_secondary.is_some() || o.theme_accent.is_some())
            && !matches!(framework, Framework::Laravel | Framework::Wagtail))
        || !o.css_variants.is_empty()
        || o.db_url.is_some()
        || (o.auth && !matches!(framework, Framework::Laravel | Framework::Wagtail))
        || (o.icons.is_some() && !matches!(framework, Framework::Laravel | Framework::Wagtail))
        || (!o.langs.is_empty() && framework == Framework::Rust)
        || !o.auth_plugins.is_empty()
        || !o.i18n.is_empty()
        || o.docker
        || o.ci;
    let laravel_set = o.starter.is_some() || o.dark || o.pest || o.frontend;
    let wagtail_set = !o.packages.is_empty() || o.no_migrate;
    let rust_set = o.kind.is_some() || o.edition.is_some() || !o.deps.is_empty();
    if astro_set && framework != Framework::Astro {
        foreign.push("astro (--pm, --template, --ts, --integration, --site, --ui, --auth, --i18n, --docker…)");
    }
    if laravel_set && framework != Framework::Laravel {
        foreign.push("laravel (--starter, --dark, --pest, --db, --frontend)");
    }
    if wagtail_set && framework != Framework::Wagtail {
        foreign.push("wagtail (--package, --no-migrate)");
    }
    if o.db.is_some() && !matches!(framework, Framework::Astro | Framework::Laravel) {
        foreign.push("astro/laravel (--db)");
    }
    if rust_set && framework != Framework::Rust {
        foreign.push("rust (--kind, --edition, --dep)");
    }
    if !foreign.is_empty() {
        bail!(
            "options propres à un autre framework que {} : {}",
            framework.id(),
            foreign.join(" ; ")
        );
    }
    let mut req = Request::new(framework, name);
    req.path = o.path.clone();
    req.force = o.force;
    req.install = !o.no_install;
    req.verify = !o.no_verify;
    req.git = !o.no_git;
    req.options = match framework {
        Framework::Astro => {
            let mut a = AstroOptions::default();
            if let Some(v) =
                parse_opt("--pm", &o.pm, PackageManager::parse, "npm, pnpm, yarn, bun")?
            {
                a.package_manager = v;
            }
            if let Some(v) = parse_opt(
                "--template",
                &o.template,
                Template::parse,
                "alpharust, minimal, basics, blog",
            )? {
                a.template = v;
            }
            if let Some(v) = parse_opt(
                "--ts",
                &o.typescript,
                TsLevel::parse,
                "base, strict, strictest",
            )? {
                a.typescript = v;
            }
            a.integrations = o.integrations.clone();
            a.site = o.site.clone();
            a.typecheck = !o.no_typecheck;
            if let Some(v) = parse_opt("--ui", &o.ui, UiKit::parse, "none, starwind, concordia")? {
                a.ui = v;
            }
            if let Some(v) = parse_opt("--db", &o.db, Db::parse, "none, postgres, sqlite")? {
                a.database = v;
            }
            a.db_url = o.db_url.clone();
            a.auth = o.auth;
            a.icons = o.icons.clone();
            a.langs = o.langs.clone();
            a.auth_plugins = o.auth_plugins.clone();
            a.i18n = o.i18n.clone();
            a.css_variants = o.css_variants.clone();
            a.theme = theme_from(o);
            a.docker = o.docker;
            a.ci = o.ci;
            if let Some(id) = &o.preset {
                let preset = scaffold::platform::Preset::find(id).ok_or_else(|| {
                    anyhow::anyhow!(
                        "préréglage inconnu `{id}` (disponibles : {})",
                        scaffold::platform::PRESETS
                            .iter()
                            .map(|p| p.id)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })?;
                preset.apply(&mut a);
            }
            // `--langs` est l'option des langues pour les 3 frameworks : sans site vitrine (pas de --ui), elle
            // règle le routage i18n de la plateforme Astro (l'ancien `--i18n`, gardé comme alias déprécié).
            if !a.vitrine() && a.i18n.is_empty() && !a.langs.is_empty() {
                a.i18n = std::mem::take(&mut a.langs);
            }
            if !o.i18n.is_empty() {
                eprintln!("avertissement : --i18n est déprécié, utilisez --langs (mêmes valeurs)");
            }
            Options::Astro(a)
        }
        Framework::Laravel => {
            let mut l = LaravelOptions::default();
            if let Some(v) = parse_opt(
                "--starter",
                &o.starter,
                Starter::parse,
                "none, breeze-blade, breeze-livewire, breeze-react, breeze-vue, breeze-api",
            )? {
                l.starter = v;
            }
            if let Some(v) = parse_opt(
                "--db",
                &o.db,
                Database::parse,
                "sqlite, postgres (pgsql), mysql",
            )? {
                l.database = v;
            }
            l.dark = o.dark;
            l.pest = o.pest;
            l.frontend = o.frontend;
            (l.ui, l.starwind) = parse_catalog_ui(&o.ui)?;
            l.theme = theme_from(o);
            l.auth = o.auth;
            l.icons = o.icons.clone();
            l.langs = o.langs.clone();
            l.validate()?;
            Options::Laravel(l)
        }
        Framework::Wagtail => {
            let (ui, starwind) = parse_catalog_ui(&o.ui)?;
            let w = WagtailOptions {
                packages: o.packages.clone(),
                migrate: !o.no_migrate,
                ui,
                starwind,
                theme: theme_from(o),
                auth: o.auth,
                icons: o.icons.clone(),
                langs: o.langs.clone(),
            };
            w.validate()?;
            Options::Wagtail(w)
        }
        Framework::Rust => {
            let kind = match o.kind.as_deref() {
                None | Some("bin") => CrateKind::Bin,
                Some("lib") => CrateKind::Lib,
                Some(other) => {
                    bail!("valeur invalide pour --kind : `{other}` (attendu : bin, lib)")
                }
            };
            Options::Rust(RustOptions {
                kind,
                edition: o.edition.clone(),
                deps: o.deps.clone(),
            })
        }
    };
    req.validate()?;
    Ok(req)
}

/// Aperçu (dry-run) ou exécution d'une demande. Renvoie le code de sortie.
fn run_scaffold(req: &Request, dry_run: bool, json: bool, verbose: bool) -> Result<i32> {
    let plan = scaffold::plan(req)?;
    if dry_run {
        let tools = scaffold::preflight(req);
        if json {
            emit(&format!(
                "{}\n",
                serde_json::to_string(&serde_json::json!({
                    "tools": tools,
                    "steps": plan.infos(),
                    "target": plan.target.display().to_string(),
                }))?
            ));
        } else {
            let printer = progress::Printer::new(false);
            emit(&format!(
                "Outils détectés\n{}\n{}",
                printer.tools(&tools),
                plan.preview()
            ));
        }
        let ok = scaffold::preflight_check(&tools).is_ok();
        if !ok && !json {
            emit("\n⚠ Des outils requis manquent : la génération échouerait (voir ci-dessus).\n");
        }
        return Ok(if ok { 0 } else { 1 });
    }
    let cancel = std::sync::atomic::AtomicBool::new(false);
    // le projet n'est "partiel" que si une étape après le contrôle de la destination a démarré
    let touched = std::cell::Cell::new(false);
    let result = if json {
        scaffold::execute(
            req,
            &mut |ev: Event| {
                if matches!(&ev, Event::StepStart { index, .. } if *index > 0) {
                    touched.set(true);
                }
                if let Ok(line) = serde_json::to_string(&ev) {
                    emit(&format!("{line}\n"));
                }
            },
            &cancel,
        )
    } else {
        let mut printer = progress::Printer::new(verbose);
        scaffold::execute(
            req,
            &mut |ev: Event| {
                if matches!(&ev, Event::StepStart { index, .. } if *index > 0) {
                    touched.set(true);
                }
                printer.handle(&ev);
            },
            &cancel,
        )
    };
    match result {
        Ok(report) => {
            if !json {
                let mut out = format!("\n{}\n", report.summary());
                if let Some(l) = &report.lint {
                    out.push_str(&format!(
                        "Lint AlphaRust : {} erreur(s), {} avertissement(s), {} info(s) sur {} fichier(s)\n",
                        l.errors, l.warnings, l.infos, l.files
                    ));
                }
                out.push_str("\nSuite :\n");
                for s in &report.next_steps {
                    out.push_str(&format!("  {s}\n"));
                }
                emit(&out);
            }
            Ok(0)
        }
        Err(_) => {
            // le message a déjà été affiché par l'événement `Failed`
            if !json && touched.get() {
                eprintln!(
                    "Le projet partiel reste dans {} pour analyse (relancer avec --force pour le remplacer).",
                    plan.target.display()
                );
            }
            Ok(1)
        }
    }
}

fn doctor(framework: Option<Framework>) -> i32 {
    let frameworks: Vec<Framework> = framework
        .map(|f| vec![f])
        .unwrap_or_else(|| Framework::ALL.to_vec());
    let printer = progress::Printer::new(false);
    let mut all_ok = true;
    for f in frameworks {
        let req = Request::new(f, "doctor");
        let tools = scaffold::preflight(&req);
        let ready = scaffold::preflight_check(&tools).is_ok();
        all_ok &= ready;
        emit(&format!(
            "{} — {} ({})\n{}\n",
            f.label(),
            if ready { "prêt" } else { "outils manquants" },
            f.official_tool(),
            printer.tools(&tools)
        ));
    }
    if all_ok {
        0
    } else if framework.is_some() {
        1
    } else {
        0
    }
}

#[derive(ValueEnum, Clone, Copy)]
enum CacheAction {
    Status,
    Clear,
}

/// Écrit sur stdout ; un tube fermé (`| head`) termine proprement.
fn emit(text: &str) {
    let mut out = std::io::stdout().lock();
    if out
        .write_all(text.as_bytes())
        .and_then(|_| out.flush())
        .is_err()
    {
        std::process::exit(0);
    }
}

/// Exécute la CLI et renvoie le code de sortie.
pub async fn run() -> Result<i32> {
    let cli = Cli::parse();
    if cli.no_cache {
        std::env::set_var("ALPHARUST_NO_CACHE", "1");
    }

    match cli.command {
        Commands::Lint {
            path,
            profile,
            sarif: as_sarif,
            json,
            fail_on,
        } => {
            let report = engine::lint(&path, profile.as_deref())?;
            if as_sarif {
                emit(&format!("{}\n", sarif::to_sarif(&report.violations)));
            } else if json {
                emit(&format!(
                    "{}\n",
                    serde_json::to_string_pretty(&report.violations)?
                ));
            } else {
                emit(&reporter::render(&report.violations));
                eprintln!(
                    "{} fichier(s) analysé(s), {} règle(s) actives{}",
                    report.files_scanned,
                    report.rules_used,
                    report
                        .profile
                        .as_ref()
                        .map(|p| format!(", profil {p}"))
                        .unwrap_or_default()
                );
            }
            Ok(report.exit_code(fail_on.map(Severity::as_str)))
        }
        Commands::List { profiles, checkers } => {
            let mut out = String::new();
            if profiles {
                let all = Rule::load_all()?;
                for name in Profile::list()? {
                    let p = Profile::load(&name)?;
                    out.push_str(&format!(
                        "{:<12} {:>3} règles  {}\n",
                        name,
                        p.select(all.clone()).len(),
                        p.description
                    ));
                }
            } else if checkers {
                for (name, desc) in crate::checkers::catalog() {
                    out.push_str(&format!("{name:<22} {desc}\n"));
                }
            } else {
                let rules = Rule::load_all()?;
                out.push_str(&format!("{} règles\n", rules.len()));
                for r in &rules {
                    out.push_str(&format!(
                        "  {:<44} {:<8} {:<7} {}\n",
                        r.id,
                        r.severity,
                        r.kind(),
                        r.apply
                    ));
                }
            }
            emit(&out);
            Ok(0)
        }
        Commands::CheckRules => {
            let report = validate::check_all();
            emit(&format!(
                "{} règle(s), {} profil(s), {} exemple(s) vérifiés\n",
                report.rules, report.profiles, report.examples
            ));
            if report.problems.is_empty() {
                emit("✓ Toutes les règles, exemples et profils sont valides\n");
                Ok(0)
            } else {
                for p in &report.problems {
                    emit(&format!("✗ {p}\n"));
                }
                Ok(1)
            }
        }
        Commands::New { name, framework } => {
            use std::io::IsTerminal;
            if !std::io::stdin().is_terminal() {
                bail!("`alpharust new` est interactif et nécessite un terminal : utiliser `alpharust generate <framework> <nom> [options]`");
            }
            let req = wizard::ask(name, framework.map(GeneratorKind::framework))?;
            if !wizard::confirm_plan(&req)? {
                emit("Annulé.\n");
                return Ok(0);
            }
            run_scaffold(&req, false, false, false)
        }
        Commands::Generate {
            framework,
            name,
            opts,
        } => {
            let req = request_from(framework.framework(), &name, &opts)?;
            run_scaffold(&req, opts.dry_run, opts.json, opts.verbose)
        }
        Commands::Init { name, opts } => {
            let req = request_from(Framework::Rust, &name, &opts)?;
            run_scaffold(&req, opts.dry_run, opts.json, opts.verbose)
        }
        Commands::Projects { dir, json } => {
            let root = dir.unwrap_or_else(crate::paths::sites_dir);
            Ok(projects_command(&root, json))
        }
        Commands::Run {
            project,
            task,
            dir,
            json,
            verbose,
        } => {
            let root = dir.unwrap_or_else(crate::paths::sites_dir);
            run_project_task(&root, &project, &task, json, verbose)
        }
        Commands::Presets { json } => Ok(presets_command(json)),
        Commands::Doctor { framework } => Ok(doctor(framework.map(GeneratorKind::framework))),
        Commands::Readme { name } => {
            crate::generators::validate_name(&name)?;
            emit(&readme::generate(&name));
            Ok(0)
        }
        Commands::Docs { name } => {
            crate::generators::validate_name(&name)?;
            emit(&docs::generate(&name));
            Ok(0)
        }
        Commands::Checks { profile } => {
            let (title, rules) = match &profile {
                Some(p) => (format!("profil {p}"), Profile::load(p)?.resolve_rules()?),
                None => ("toutes les règles".to_string(), Rule::load_all()?),
            };
            emit(&checks::generate(&title, &composer::compose(rules)));
            Ok(0)
        }
        Commands::Cache { action } => {
            match action {
                CacheAction::Status => {
                    let (exists, size) = cache::status();
                    emit(&format!(
                        "cache : {} ({}) — {}\n",
                        cache::cache_file().display(),
                        if exists {
                            format!("{size} octets")
                        } else {
                            "absent".into()
                        },
                        if std::env::var_os("ALPHARUST_NO_CACHE").is_some() {
                            "désactivé"
                        } else {
                            "actif"
                        }
                    ));
                }
                CacheAction::Clear => {
                    let removed = cache::clear();
                    emit(if removed {
                        "✓ Cache supprimé\n"
                    } else {
                        "Aucun cache à supprimer\n"
                    });
                }
            }
            Ok(0)
        }
        Commands::Tui => {
            tui::run()?;
            Ok(0)
        }
        Commands::Gui { port } => {
            if !paths::rules_dir().is_dir() {
                bail!("dossier rules/ introuvable (définir ALPHARUST_HOME)");
            }
            gui::run(port).await?;
            Ok(0)
        }
    }
}

fn projects_command(root: &std::path::Path, json: bool) -> i32 {
    let list = scaffold::projects::list(root);
    if json {
        emit(&format!(
            "{}\n",
            serde_json::to_string_pretty(&list).unwrap_or_default()
        ));
        return 0;
    }
    if list.is_empty() {
        emit(&format!(
            "Aucun projet AlphaRust dans {} (créez-en un : alpharust generate astro mon-site)\n",
            root.display()
        ));
        return 0;
    }
    let mut out = format!("Projets dans {}\n\n", root.display());
    for p in &list {
        let git = p.git.as_ref().map_or(String::new(), |g| {
            format!(
                " · git {}{}",
                g.branch,
                if g.dirty { " (modifié)" } else { "" }
            )
        });
        out.push_str(&format!(
            "  {} — {}{}{}\n",
            p.name,
            p.framework.label(),
            p.package_manager
                .as_deref()
                .map_or(String::new(), |pm| format!(" · {pm}")),
            git
        ));
        out.push_str(&format!(
            "      {}\n",
            if p.installed {
                "dépendances installées"
            } else {
                "dépendances non installées"
            }
        ));
        if !p.features.is_empty() {
            out.push_str(&format!("      briques : {}\n", p.features.join(", ")));
        }
        out.push_str(&format!(
            "      tâches : {}\n",
            p.tasks.iter().map(|t| t.id).collect::<Vec<_>>().join(", ")
        ));
    }
    out.push_str("\nLancer une tâche : alpharust run <projet> <tâche>\n");
    emit(&out);
    0
}

fn run_project_task(
    root: &std::path::Path,
    project: &str,
    task: &str,
    json: bool,
    verbose: bool,
) -> Result<i32> {
    let task = scaffold::projects::Task::parse(task).ok_or_else(|| {
        anyhow::anyhow!(
            "tâche inconnue `{task}` (disponibles : {})",
            scaffold::projects::Task::ALL
                .iter()
                .map(|t| t.id())
                .collect::<Vec<_>>()
                .join(", ")
        )
    })?;
    let p = scaffold::projects::resolve(root, project)?;
    let cancel = std::sync::atomic::AtomicBool::new(false);
    let result = if json {
        scaffold::projects::run(
            &p,
            task,
            &mut |ev: Event| {
                if let Ok(line) = serde_json::to_string(&ev) {
                    emit(&format!("{line}\n"));
                }
            },
            &cancel,
        )
    } else {
        let mut printer = progress::Printer::new(verbose || task.is_long_running());
        scaffold::projects::run(&p, task, &mut |ev: Event| printer.handle(&ev), &cancel)
    };
    match result {
        Ok(report) => {
            if !json {
                if let Some(l) = &report.lint {
                    emit(&format!(
                        "\nLint AlphaRust : {} erreur(s), {} avertissement(s), {} info(s) sur {} fichier(s)\n",
                        l.errors, l.warnings, l.infos, l.files
                    ));
                }
            }
            Ok(0)
        }
        // l'échec a déjà été affiché (événement `Failed`)
        Err(_) => Ok(1),
    }
}

fn presets_command(json: bool) -> i32 {
    let list = scaffold::platform::PRESETS;
    if json {
        let v: Vec<_> = list
            .iter()
            .map(|p| serde_json::json!({"id": p.id, "label": p.label, "description": p.description, "options": p.flags}))
            .collect();
        emit(&format!(
            "{}\n",
            serde_json::to_string_pretty(&v).unwrap_or_default()
        ));
        return 0;
    }
    let mut out =
        String::from("Préréglages Astro (alpharust generate astro <nom> --preset <id>)\n\n");
    for p in list {
        out.push_str(&format!("  {:<12} {}\n", p.id, p.label));
        out.push_str(&format!("               {}\n", p.description));
        out.push_str(&format!("               équivaut à : {}\n\n", p.flags));
    }
    out.push_str(
        "Les options explicites (--db, --ui, --i18n…) complètent ou remplacent le préréglage.\n",
    );
    emit(&out);
    0
}
