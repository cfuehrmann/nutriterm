pub mod catalog;
pub mod commands;
pub mod error;
pub mod utils;


use clap::{Parser, Subcommand};
use error::AppResult;

#[derive(Parser)]
#[command(name = "nutriterm")]
#[command(about = "Calculate nutritional information for ingredients and recipes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize current directory as a recipe catalog")]
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
            let catalog_dir = catalog::find_dir()?;
            commands::recipe::run(&catalog_dir, name)?;
        }

        Commands::KitchenRef => {
            let catalog_dir = catalog::find_dir()?;
            commands::kitchen_ref::run(&catalog_dir)?;
        }
    }
    Ok(())
}
