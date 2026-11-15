use clap::Parser;

#[derive(Parser)]
#[command(name = "alpharust")]
#[command(about = "AlphaRust — Fondation intégrale")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Lint un projet contre les règles
    Lint {
        #[arg(default_value = ".")]
        path: String,
    },
    /// Lister les règles disponibles
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lint { path } => {
            println!("Linting {}", path);
            // TODO: engine
        }
        Commands::List => {
            println!("Règles disponibles :");
            // TODO: parser
        }
    }
}
