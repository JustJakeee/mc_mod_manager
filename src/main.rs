mod api;
mod config;
mod mods;

use std::io::{self, Write};

use anyhow::Result;
use api::*;
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
    Search { query: String, limit: i8 },
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = get_config().unwrap();
    let cli = Cli::parse();
    match cli.command {
        Commands::Download => download_mods(config).await?,
        Commands::Add { mod_slug } => add_mod_to_config(&mut config, &mod_slug),
        Commands::Search { query, limit } => {
            println!("Getting projects matching {query}");
            let search = search_projects(&query, &config.version, &limit).await?;
            println!("{:#?}", search);
            let hits = search.hits;
            let mut desired_mod: &Project;
            loop {
                println!();
                io::stdout().flush().unwrap();
                println!("-- Search results for \"{query}\" -- ({})", hits.len());
                for (index, item) in hits.iter().enumerate() {
                    println!("({index}): {}", item.title)
                }
                print!("Enter your choice: ");
                io::stdout().flush().unwrap();
                let mut choice = String::new();

                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");

                let choice: usize = match choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if choice < hits.len() {
                    desired_mod = &hits[choice];
                    println!("You chose: {}", &desired_mod.title);
                    break;
                } else {
                    println!("Invalid choice");
                    continue;
                }
            }
            println!();
            println!("-- {} -- ", &desired_mod.title);
            println!("{}", &desired_mod.description);
            println!();
            println!("Add this mod to config?");
            print!("(y/n): ");
            io::stdout().flush().unwrap();
            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: bool = match choice
                .chars()
                .next()
                .unwrap()
                .to_lowercase()
                .next()
                .unwrap()
            {
                'y' => true,
                _ => false,
            };

            if choice {
                add_mod_to_config(&mut config, &desired_mod.slug);
            }
        }
    }
    //download_mods(config).await?;
    Ok(())
}
