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
use generators::{bootstrap, docs, readme};
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
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    Readme { name: String },
    Docs { name: String },
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
        Commands::Init { name, path } => bootstrap::bootstrap(&path, &name)?,
        Commands::Readme { name } => println!("{}", readme::generate(&name)),
        Commands::Docs { name } => println!("{}", docs::generate(&name)),
    }
    Ok(())
}
