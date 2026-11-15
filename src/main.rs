use clap::Parser;
use std::path::Path;

mod parser;
mod engine;
mod checkers;
mod generators;

use engine::runner;
use engine::reporter;
use parser::Rule;
use generators::bootstrap;

#[derive(Parser)]
#[command(name = "alpharust")]
#[command(about = "AlphaRust — Fondation intégrale")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Lint un projet
    Lint {
        #[arg(default_value = ".")]
        path: String,
    },
    /// Lister les règles
    List,
    /// Initialiser un projet
    Init {
        name: String,
        #[arg(default_value = ".")]
        path: String,
    },
    /// Générer un README
    Readme {
        name: String,
    },
}

fn load_rules() -> Vec<Rule> {
    let mut rules = Vec::new();
    let rules_dir = Path::new("rules");
    if rules_dir.exists() {
        for entry in walkdir::WalkDir::new(rules_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map(|s| s == "md").unwrap_or(false) {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(rule) = Rule::from_markdown(entry.path(), &content) {
                        rules.push(rule);
                    }
                }
            }
        }
    }
    rules
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lint { path } => {
            let rules = load_rules();
            let violations = runner::run(&rules, Path::new(&path));
            reporter::report(&violations);
            if !violations.is_empty() {
                std::process::exit(1);
            }
        }
        Commands::List => {
            let rules = load_rules();
            println!("{} règles chargées", rules.len());
            for r in rules {
                println!("  {} [{}] ({})", r.name, r.severity, r.apply);
            }
        }
        Commands::Init { name, path } => {
            if let Err(e) = bootstrap::bootstrap(Path::new(&path), &name) {
                eprintln!("Erreur: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Readme { name } => {
            println!("{}", generators::readme::generate(&name));
        }
    }
}
