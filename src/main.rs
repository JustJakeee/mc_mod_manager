mod api;
mod config;
mod mods;

use anyhow::Result;
use mods::*;
use config::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = match get_config() {
        Some(config) => {
            println!("Config Found!");
            config
        }
        None => {
            panic!("Couldn't find config!!!");
        }
    };
    download_mods(config).await?;
    Ok(())
}
