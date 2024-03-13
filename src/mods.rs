use crate::api::*;
use crate::config::*;
use std::path::Path;
use std::fs;
use anyhow::Result;

pub async fn download_mods(config: Config) -> Result<()> {
    let mods_path = Path::new(&config.mods_path);
    match clear_directory(mods_path) {
        Ok(()) => println!("Directory cleared successfully"),
        Err(err) => {
            eprintln!("Error clearing directory: {err}");
            // Additional error handling or logging
        }
    }

    for mod_slug in config.mod_slugs {
        let project = get_project(&mod_slug).await?;
        println!("-- {} --", project.title);
        println!("{}", project.description);
        let versions = get_versions(&mod_slug).await?;
        let latest_version = versions
            .iter()
            .find(|x| x.game_versions.contains(&config.version));

        match latest_version {
            Some(version) => {
                println!(
                    "Latest Matching Version - {}: {:?}",
                    version.name, version.game_versions
                );
                download_mod(version, &config.mods_path).await;
            }
            None => {
                println!("No Matching Version !!!!");
            }
        }
        println!();
    }
    Ok(())
}

fn clear_directory(dir_path: &Path) -> Result<()> {
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                clear_directory(&path)?;
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}