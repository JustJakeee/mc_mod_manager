use crate::api::*;
use crate::config::*;
use anyhow::Result;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

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

pub fn add_mod_to_config(config: &mut Config, mod_slug: &String) {
    config.mod_slugs.push(mod_slug.clone());
    match save_config(config) {
        Ok(_) => {
            println!("Mod \"{mod_slug}\" successfully added to config!")
        }
        Err(err) => {
            println!("Error saving mod: {err}")
        }
    }
}

pub fn remove_mod_from_config(config: &mut Config, mod_slug: &String) {
    let mut removed = false;
    config.mod_slugs.retain(|slug| {
        if slug != mod_slug {
            true
        } else {
            removed = true;
            false
        }
    });

    match save_config(config) {
        Ok(_) => {
            if removed {
                println!("Mod \"{mod_slug}\" successfully removed from config!")
            } else {
                println!("Mod \"{mod_slug}\" not found in config.")
            }
        }
        Err(err) => {
            println!("Error saving config: {err}")
        }
    }
}

pub fn list_mods_in_config(config: &Config) {
    if config.mod_slugs.is_empty() {
        println!("No mods in the config.");
    } else {
        println!("-- Mods in the config --");
        for (index, mod_slug) in config.mod_slugs.iter().enumerate() {
            println!("{} - {}", index + 1, mod_slug);
        }
    }
}

pub async fn search_and_add_mods(config: &mut Config, query: &String, limit: &i8) -> Result<()> {
    println!("Getting projects matching {query}");
    let search = search_projects(&query, &config.version, &limit).await?;
    println!("{:#?}", search);
    let hits = search.hits;
    let desired_mod;
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
        add_mod_to_config(config, &desired_mod.slug);
    }

    Ok(())
}
