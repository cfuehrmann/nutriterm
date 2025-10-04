pub mod commands;
pub mod data;
pub mod error;
pub mod models;
pub mod schema;
pub mod utils;
pub mod workspace;

use clap::{Parser, Subcommand};
use error::AppResult;
use workspace::find_workspace;

#[derive(Parser)]
#[command(name = "nutriterm")]
#[command(about = "Calculate nutritional information for ingredients and recipes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize current directory as a recipe workspace")]
    Init,
    #[command(about = "Display nutrition for a specific recipe")]
    Recipe {
        #[arg(help = "Recipe name (e.g., chicken-rice-bowl)")]
        name: String,
    },
    #[command(about = "Generate kitchen reference with all recipes in HTML format")]
    KitchenRef,
}

fn main() {
    if let Err(e) = run_app() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_app() -> AppResult<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            let current_dir = std::env::current_dir()?;
            commands::init::run(&current_dir)?;
        }
        Commands::Recipe { name } => {
            let workspace = find_workspace()?;
            commands::recipe::run(&workspace, name)?;
        }

        Commands::KitchenRef => {
            let workspace = find_workspace()?;
            commands::kitchen_ref::run(&workspace)?;
        }
    }
    Ok(())
}
