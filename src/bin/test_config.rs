use std::fs;
use std::path::Path;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    name: Option<String>,
    email: Option<String>,
    location: Option<Vec<String>>,
    open_to_relocation: Option<bool>,
    job_keywords: Option<Vec<String>>,
    skills: Option<Vec<String>>,
    industries: Option<Vec<String>>,
    seniority_levels: Option<Vec<String>>,
    targets: Option<Vec<String>>,
    resume_path: Option<String>,
    cover_letter_path: Option<String>,
    resume_profiles: Option<Vec<DocumentProfile>>,
    cover_letter_profiles: Option<Vec<DocumentProfile>>,
    applied_jobs_path: Option<String>,
    search_engines: Option<Vec<String>>,
    company_targets: Option<Vec<String>>,
    search_locations: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct DocumentProfile {
    name: String,
    path: String,
    keywords: Vec<String>,
    industries: Vec<String>,
    seniority: Vec<String>,
}

fn main() {
    println!("Testing config loading...");

    // Print current working directory
    let current_dir = std::env::current_dir().unwrap_or_default();
    println!("Current working directory: {:?}", current_dir);

    // Check if AgentConfig.toml exists
    let config_path = "AgentConfig.toml";
    let absolute_path = current_dir.join(config_path);
    println!("Absolute path to config: {:?}", absolute_path);
    println!("Config file exists: {}", absolute_path.exists());

    // If the file exists, print its size and content
    if absolute_path.exists() {
        if let Ok(metadata) = std::fs::metadata(&absolute_path) {
            println!("Config file size: {} bytes", metadata.len());
        }

        // Read the file content
        match fs::read_to_string(&absolute_path) {
            Ok(content) => {
                println!("File content length: {} bytes", content.len());
                println!("First 100 characters: {}", &content[..std::cmp::min(100, content.len())]);

                // Try to parse the TOML as a generic Value
                match toml::from_str::<toml::Value>(&content) {
                    Ok(value) => {
                        println!("Successfully parsed as generic TOML value");
                        println!("TOML structure: {:#?}", value);
                    },
                    Err(e) => {
                        println!("Error parsing TOML as generic value: {}", e);
                    }
                }

                // Try to parse the TOML as a Config struct
                println!("\nTrying to parse as Config struct...");
                match toml::from_str::<Config>(&content) {
                    Ok(config) => {
                        println!("Successfully parsed as Config struct");
                        println!("Config name: {}", config.name.as_ref().map_or("None", |s| s));
                        println!("Config email: {}", config.email.as_ref().map_or("None", |s| s));
                        println!("Config search_engines: {}", config.search_engines.as_ref().map_or("None", |_| "Some"));
                        println!("Config company_targets: {}", config.company_targets.as_ref().map_or("None", |_| "Some"));
                        println!("Config search_locations: {}", config.search_locations.as_ref().map_or("None", |_| "Some"));
                    },
                    Err(e) => {
                        println!("Error parsing TOML as Config struct: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Error reading file: {}", e);
            }
        }
    } else {
        println!("Config file does not exist!");
    }
}
