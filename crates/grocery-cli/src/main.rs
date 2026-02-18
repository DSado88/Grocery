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

    /// Chat with Cart Blanche (Claude + household context)
    Chat {
        /// Initial message (omit for interactive REPL)
        #[arg()]
        initial_message: Option<String>,

        /// Model override (e.g., claude-sonnet-4-5-20250929)
        #[arg(long)]
        model: Option<String>,

        /// Single turn — print response and exit (no REPL)
        #[arg(long)]
        once: bool,
    },
}

#[tokio::main]
async fn main() {
    // Init tracing (try_init to avoid panic if already set)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .try_init()
        .ok();

    let cli = Cli::parse();

    // Isolate Butler WAL/session data under data_dir/.state/
    let state_dir = cli.data_dir.join(".state");
    std::env::set_var("BUTLER_HOME", &state_dir);

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
                .map_err(|e| e.to_string())
        }
        Commands::Score { recipe } => {
            commands::score::run(&cli.data_dir, &recipe).map_err(|e| e.to_string())
        }
        Commands::Status => commands::status::run(&cli.data_dir).map_err(|e| e.to_string()),
        Commands::Chat {
            initial_message,
            model,
            once,
        } => {
            commands::chat::run(&cli.data_dir, initial_message.as_deref(), model, once)
                .await
                .map_err(|e| commands::chat::format_user_facing_error(&e))
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
