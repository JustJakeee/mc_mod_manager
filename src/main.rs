mod api;
mod config;
mod mods;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::*;
use mods::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download,
    Add { mod_slug: String },
    Remove { mod_slug: String },
    List,
    Search { query: String, limit: i8 },
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = get_config().unwrap();
    let cli = Cli::parse();
    match cli.command {
        Commands::Download => download_mods(config).await?,
        Commands::Add { mod_slug } => add_mod_to_config(&mut config, &mod_slug),
        Commands::Remove { mod_slug } => remove_mod_from_config(&mut config, &mod_slug),
        Commands::List => list_mods_in_config(&config),
        Commands::Search { query, limit } => search_and_add_mods(&mut config, &query, &limit).await?,
    }
    Ok(())
}
