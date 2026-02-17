use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "grocery", about = "Cart Blanche grocery automation CLI")]
struct Cli {
    /// Path to data directory containing YAML/JSON data files
    #[arg(long, default_value = ".")]
    data_dir: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a shopping list from recipes + household staples
    Plan {
        /// Recipe names to include (fuzzy matched)
        #[arg(required = true)]
        recipes: Vec<String>,

        /// Output format: text, json, or compact
        #[arg(long, default_value = "text")]
        format: String,

        /// Exclude household staples from the list
        #[arg(long)]
        no_staples: bool,
    },

    /// Score a recipe against household purchasing patterns
    Score {
        /// Recipe name (fuzzy matched)
        recipe: String,
    },

    /// Show household model and recipe collection stats
    Status,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Plan {
            recipes,
            format,
            no_staples,
        } => {
            let fmt = match format.parse::<commands::plan::OutputFormat>() {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: {e}");
                    process::exit(1);
                }
            };
            commands::plan::run(&cli.data_dir, &recipes, &fmt, !no_staples)
        }
        Commands::Score { recipe } => {
            commands::score::run(&cli.data_dir, &recipe)
        }
        Commands::Status => {
            commands::status::run(&cli.data_dir)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
