use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub type VersionsResponse = Vec<Version>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub hits: Vec<Project>,
    pub limit: i8,
    pub total_hits: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub version_number: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub id: String,
    pub project_id: String,
    pub files: Vec<ModFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModFile {
    pub url: String,
    pub filename: String,
    pub primary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub slug: String,
    pub description: String,
}

async fn download_file(url: &String, file_path: &String) -> Result<()> {
    let client = Client::new();
    let response = client.get(url).send().await?.bytes().await?;

    let mut file = File::create(file_path)?;
    file.write_all(&response)?;

    Ok(())
}

pub async fn download_mod(version: &Version, path: &String) {
    let primary_file = version
        .files
        .iter()
        .find(|x| x.primary == true)
        .or(version.files.first());

    match primary_file {
        Some(file) => {
            let file_path = format!("{}{}", path, file.filename);
            println!("Downloading to: {}", file_path);
            match download_file(&file.url, &file_path).await {
                Ok(_) => println!("Downloaded!"),
                Err(e) => println!("Error downloading file: {}", e),
            };
        }
        None => {
            println!("NO MOD FILE FOUND !!!!")
        }
    }
}

pub async fn get_versions(mod_slug: &String) -> Result<VersionsResponse> {
    let client = Client::new();
    let url = format!("https://api.modrinth.com/v2/project/{mod_slug}/version");
    Ok(client
        .get(url)
        .send()
        .await?
        .json::<VersionsResponse>()
        .await?)
}

pub async fn get_project(mod_slug: &String) -> Result<Project> {
    let client = Client::new();
    let url = format!("https://api.modrinth.com/v2/project/{mod_slug}");
    Ok(client
        .get(url)
        .send()
        .await?
        .json::<Project>()
        .await?)
}

pub async fn search_projects(query: &String, version: &String, limit: &i8) -> Result<SearchResponse> {
    let client = Client::new();
    let url = format!("https://api.modrinth.com/v2/search?query=\"{query}\"&facets=[[\"versions:{version}\"], [\"project_type:mod\"]]&limit={limit}");
    Ok(client
        .get(url)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?)
}
