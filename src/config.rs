use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    fs::{read_to_string, File},
    io::Write,
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version: String,
    pub mod_slugs: Vec<String>,
    pub mods_path: String,
}

pub fn get_config() -> Option<Config> {
    let config_str = match read_to_string("config.json") {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Couldn't read config file: {}", err);
            return None;
        }
    };

    match from_str(&config_str) {
        Ok(config) => Some(config),
        Err(err) => {
            eprintln!("Couldn't parse config file: {}", err);
            None
        }
    }
}

pub fn save_config(config: &mut Config) -> Result<()> {
    let json_data = serde_json::to_string_pretty(&config)?;

    let mut file = File::create("config.json")?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
