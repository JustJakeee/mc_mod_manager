use std::fs::read_to_string;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version: String,
    pub mod_slugs: Vec<String>,
    pub mods_path: String,
}

pub fn get_config() -> Result<Config> {
    let config_str = read_to_string("config.json")?;
    Ok(from_str(&config_str)?)
}