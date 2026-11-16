//! Interface en ligne de commande.

use crate::engine::{self, composer, reporter, validate};
use crate::generators::{self, checks, docs, readme, Kind};
use crate::parser::{cache, Rule};
use crate::profiles::Profile;
use crate::{gui, paths, sarif, tui};
use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::io::Write;
use std::path::PathBuf;

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
    /// Initialiser un projet Rust minimal dans sites/
    Init {
        name: String,
        #[arg(short, long, help = "Chemin de destination (défaut: sites/<name>)")]
        path: Option<PathBuf>,
        #[arg(long, help = "Écraser un dossier existant non vide")]
        force: bool,
    },
    /// Générer un projet framework dans sites/
    Generate {
        #[arg(value_enum)]
        kind: GeneratorKind,
        name: String,
        #[arg(short, long, help = "Chemin de destination (défaut: sites/<name>)")]
        path: Option<PathBuf>,
        #[arg(long, help = "Écraser un dossier existant non vide")]
        force: bool,
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
    Wagtail,
    Astro,
    Laravel,
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

fn next_steps(kind: Kind, target: &std::path::Path) -> String {
    let dir = target.display();
    match kind {
        Kind::Bootstrap => format!("Suite : cd {dir} && cargo test && cargo run"),
        Kind::Wagtail => format!(
            "Suite : cd {dir} && python -m venv .venv && . .venv/bin/activate && pip install -r requirements.txt && python manage.py migrate && python manage.py runserver"
        ),
        Kind::Astro => format!("Suite : cd {dir} && npm install && npm run dev"),
        Kind::Laravel => format!(
            "Suite : cd {dir} && composer install && cp .env.example .env && php artisan key:generate && touch database/database.sqlite && php artisan migrate --seed && php artisan serve"
        ),
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
                        .map(|p| format!(", profil {}", p))
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
                    out.push_str(&format!("{:<22} {}\n", name, desc));
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
                    emit(&format!("✗ {}\n", p));
                }
                Ok(1)
            }
        }
        Commands::Init { name, path, force } => {
            let target = path.unwrap_or_else(|| generators::default_target(&name));
            let g = generators::generate(Kind::Bootstrap, &target, &name, force)?;
            emit(&format!(
                "{}\n{}\n",
                g.summary(),
                next_steps(g.kind, &g.target)
            ));
            Ok(0)
        }
        Commands::Generate {
            kind,
            name,
            path,
            force,
        } => {
            let kind = match kind {
                GeneratorKind::Wagtail => Kind::Wagtail,
                GeneratorKind::Astro => Kind::Astro,
                GeneratorKind::Laravel => Kind::Laravel,
            };
            let target = path.unwrap_or_else(|| generators::default_target(&name));
            let g = generators::generate(kind, &target, &name, force)?;
            emit(&format!(
                "{}\n{}\n",
                g.summary(),
                next_steps(g.kind, &g.target)
            ));
            Ok(0)
        }
        Commands::Readme { name } => {
            generators::validate_name(&name)?;
            emit(&readme::generate(&name));
            Ok(0)
        }
        Commands::Docs { name } => {
            generators::validate_name(&name)?;
            emit(&docs::generate(&name));
            Ok(0)
        }
        Commands::Checks { profile } => {
            let (title, rules) = match &profile {
                Some(p) => (format!("profil {}", p), Profile::load(p)?.resolve_rules()?),
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
                            format!("{} octets", size)
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
