use serde::Deserialize;
use std::fs;
use anyhow::{Result, Error};
use toml;
use crate::document_profile::DocumentProfile;

/// Main application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub name: String,
    pub email: String,
    pub location: Vec<String>,
    pub open_to_relocation: Option<bool>,
    pub job_keywords: Vec<String>,
    pub skills: Vec<String>,
    pub industries: Vec<String>,
    pub seniority_levels: Vec<String>,
    pub targets: Vec<String>,
    pub resume_path: String,
    pub cover_letter_path: String,
    pub resume_profiles: Option<Vec<DocumentProfile>>,
    pub cover_letter_profiles: Option<Vec<DocumentProfile>>,
    pub applied_jobs_path: Option<String>,
    pub min_salary: Option<u32>,
    pub max_salary: Option<u32>,
    #[serde(default)]
    pub search_engines: Vec<String>,
    #[serde(default)]
    pub company_targets: Vec<String>,
    #[serde(default)]
    pub search_locations: Vec<String>,
}


impl Config {
    pub fn load(path: &str) -> Result<Self, Error> {
        println!("Loading config from: {}", path);
        let content = fs::read_to_string(path)?;
        println!("File content length: {} bytes", content.len());
        println!("First 100 characters: {}", &content[..std::cmp::min(100, content.len())]);

        // Try to parse the TOML
        match toml::from_str::<Self>(&content) {
            Ok(config) => {
                println!("Successfully parsed config");
                Ok(config)
            },
            Err(e) => {
                println!("Error parsing TOML: {}", e);
                Err(anyhow::anyhow!("TOML parse error: {}", e))
            }
        }
    }
}
