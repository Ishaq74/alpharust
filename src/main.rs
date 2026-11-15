use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod parser;
mod engine;
mod generators;
mod profiles;
mod config;
mod sarif;

use config::Config;
use engine::{composer, reporter, runner};
use generators::{astro, bootstrap, docs, laravel, readme, wagtail};
use parser::Rule;
use profiles::Profile;

#[derive(Parser)]
#[command(name = "alpharust", version, about = "AlphaRust — Fondation intégrale")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Lint {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long)]
        profile: Option<String>,
        #[arg(long)]
        sarif: bool,
    },
    List {
        #[arg(long)]
        profiles: bool,
    },
    Init {
        name: String,
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    Generate {
        #[arg(value_enum)]
        kind: GeneratorKind,
        name: String,
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    Readme { name: String },
    Docs { name: String },
}

#[derive(clap::ValueEnum, Clone)]
enum GeneratorKind {
    Wagtail,
    Astro,
    Laravel,
}

fn default_sites_path(name: &str) -> PathBuf {
    PathBuf::from("sites").join(name)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load().unwrap_or_default();

    match cli.command {
        Commands::Lint { path, profile, sarif } => {
            let profile_name = profile.or(config.profile);
            let mut rules = if let Some(p) = profile_name {
                Profile::load(&p)?.resolve_rules()?
            } else {
                Rule::load_all()?
            };
            rules = composer::compose(rules);
            let violations = runner::run(&rules, &path)?;
            if sarif {
                println!("{}", sarif::to_sarif(&violations));
            } else {
                reporter::report(&violations);
            }
            if violations.iter().any(|v| v.severity == "error") {
                std::process::exit(1);
            }
        }
        Commands::List { profiles } => {
            if profiles {
                for p in Profile::list()? {
                    println!("{}", p);
                }
            } else {
                let rules = Rule::load_all()?;
                println!("{} règles", rules.len());
                for r in &rules {
                    println!("  {:<36} [{}] {}", r.name, r.severity, r.apply);
                }
            }
        }
        Commands::Init { name, path } => {
            let target = path.unwrap_or_else(|| default_sites_path(&name));
            bootstrap::bootstrap(&target, &name)?;
        }
        Commands::Generate { kind, name, path } => {
            let target = path.unwrap_or_else(|| default_sites_path(&name));
            match kind {
                GeneratorKind::Wagtail => wagtail::generate(&target, &name)?,
                GeneratorKind::Astro => astro::generate(&target, &name)?,
                GeneratorKind::Laravel => laravel::generate(&target, &name)?,
            }
        }
        Commands::Readme { name } => println!("{}", readme::generate(&name)),
        Commands::Docs { name } => println!("{}", docs::generate(&name)),
    }
    Ok(())
}
