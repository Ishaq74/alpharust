use clap::Parser;
use std::path::Path;

mod parser;
mod engine;
mod checkers;

use engine::runner;
use engine::reporter;
use parser::Rule;

#[derive(Parser)]
#[command(name = "alpharust")]
#[command(about = "AlphaRust — Fondation intégrale")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Lint {
        #[arg(default_value = ".")]
        path: String,
    },
    List,
}

fn load_rules() -> Vec<Rule> {
    let mut rules = Vec::new();
    let rules_dir = Path::new("rules");
    if rules_dir.exists() {
        for entry in walkdir::WalkDir::new(rules_dir) {
            if let Ok(e) = entry {
                if e.path().extension().map(|s| s == "md").unwrap_or(false) {
                    if let Ok(content) = std::fs::read_to_string(e.path()) {
                        if let Ok(rule) = Rule::from_markdown(e.path(), &content) {
                            rules.push(rule);
                        }
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
            for r in rules {
                println!("{} ({}) - {}", r.name, r.severity, r.apply);
            }
        }
    }
}
