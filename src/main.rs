use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod parser;
mod engine;
mod generators;
mod profiles;

use engine::runner;
use engine::reporter;
use generators::bootstrap;
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
    Readme {
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lint { path, profile } => {
            let rules = if let Some(p) = profile {
                Profile::load(&p)?.resolve_rules()?
            } else {
                Rule::load_all()?
            };
            let violations = runner::run(&rules, &path)?;
            reporter::report(&violations);
            if !violations.is_empty() {
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
                    println!("  {:<30} [{}] {}", r.name, r.severity, r.apply);
                }
            }
        }
        Commands::Init { name, path } => {
            bootstrap::bootstrap(&path, &name)?;
        }
        Commands::Readme { name } => {
            println!("{}", generators::readme::generate(&name));
        }
    }
    Ok(())
}
