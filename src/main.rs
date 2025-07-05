use std::sync::{Arc, Mutex};
use anyhow::Result;
use tracing::{info, error, debug};
use tracing_subscriber::EnvFilter;

mod config;
mod application_state;
mod job_search_engine;
mod job_application;
mod secrets;
mod job_listing;
mod nike_career;
mod company_career;
mod document_profile;

#[cfg(feature = "mock-browser")]
mod mock_browser;
#[cfg(feature = "mock-browser")]
use mock_browser::HeadlessBrowser;

#[cfg(feature = "headless-chrome")]
mod headless_browser;
#[cfg(feature = "headless-chrome")]
use headless_browser::HeadlessBrowser;

use crate::config::Config;
use crate::application_state::{ApplicationState, SharedApplicationState};
use crate::job_search_engine::search_jobs;
use crate::job_application::process_and_apply_to_jobs;
use crate::secrets::load_secrets;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    // Print current working directory
    let current_dir = std::env::current_dir().unwrap_or_default();
    println!("Current working directory: {:?}", current_dir);

    // Check if AgentConfig.toml exists
    let config_path = "AgentConfig.toml";
    let absolute_path = current_dir.join(config_path);
    println!("Absolute path to config: {:?}", absolute_path);
    println!("Config file exists: {}", absolute_path.exists());

    // If the file exists, print its size
    if absolute_path.exists() {
        if let Ok(metadata) = std::fs::metadata(&absolute_path) {
            println!("Config file size: {} bytes", metadata.len());
        }
    }

    // Load configuration
    let config = match Config::load(config_path) {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        },
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(anyhow::anyhow!("Failed to load configuration: {}", e));
        }
    };

    // Load secrets from AgentSecrets.template (or .env/AgentSecrets.toml)
    let secrets = load_secrets("AgentSecrets.template");

    // Inject secrets into config if present (overrides AgentConfig.toml if filled)
    let name = secrets.get("AGENT_NAME").cloned().unwrap_or(config.name.clone());
    let email = secrets.get("AGENT_EMAIL").cloned().unwrap_or(config.email.clone());
    let resume_path = secrets.get("RESUME_PATH").cloned().unwrap_or(config.resume_path.clone());
    let cover_letter_path = secrets.get("COVER_LETTER_PATH").cloned().unwrap_or(config.cover_letter_path.clone());

    // Platform credentials (example: LinkedIn, Indeed, etc.)
    let linkedin_email = secrets.get("LINKEDIN_EMAIL").cloned().unwrap_or_default();
    let linkedin_password = secrets.get("LINKEDIN_PASSWORD").cloned().unwrap_or_default();
    let indeed_email = secrets.get("INDEED_EMAIL").cloned().unwrap_or_default();
    let indeed_password = secrets.get("INDEED_PASSWORD").cloned().unwrap_or_default();
    let glassdoor_email = secrets.get("GLASSDOOR_EMAIL").cloned().unwrap_or_default();
    let glassdoor_password = secrets.get("GLASSDOOR_PASSWORD").cloned().unwrap_or_default();
    let monster_email = secrets.get("MONSTER_EMAIL").cloned().unwrap_or_default();
    let monster_password = secrets.get("MONSTER_PASSWORD").cloned().unwrap_or_default();
    let ziprecruiter_email = secrets.get("ZIPRECRUITER_EMAIL").cloned().unwrap_or_default();
    let ziprecruiter_password = secrets.get("ZIPRECRUITER_PASSWORD").cloned().unwrap_or_default();
    let careerbuilder_email = secrets.get("CAREERBUILDER_EMAIL").cloned().unwrap_or_default();
    let careerbuilder_password = secrets.get("CAREERBUILDER_PASSWORD").cloned().unwrap_or_default();
    // ...pass these to platform modules as needed...

    // Create application state
    let app_state = Arc::new(Mutex::new(ApplicationState::new("state/job_state.json")));
    info!("Application state initialized");

    // Initialize headless browser
    let mut browser = match HeadlessBrowser::new() {
        Ok(browser) => {
            info!("Headless browser initialized successfully");
            Some(browser)
        },
        Err(e) => {
            error!("Failed to initialize headless browser: {}", e);
            info!("Continuing without browser automation");
            None
        }
    };

    // Continuous execution loop
    loop {
        info!("Starting job search cycle");

        // Search for jobs
        match search_jobs(&config).await {
            Ok(job_listings) => {
                if !job_listings.is_empty() {
                    debug!("Found {} job listings", job_listings.len());

                    // Extract URLs from job listings
                    let urls: Vec<String> = job_listings.iter().map(|job| job.url.clone()).collect();

                    // Apply to jobs
                    let smtp_server = secrets.get("SMTP_SERVER").cloned().unwrap_or_else(|| "smtp.gmail.com".to_string());
                    let smtp_username = secrets.get("SMTP_USERNAME").cloned().unwrap_or_default();
                    let smtp_password = secrets.get("SMTP_PASSWORD").cloned().unwrap_or_default();
                    match process_and_apply_to_jobs(
                        &urls,
                        &name,
                        &email,
                        &resume_path,
                        &cover_letter_path,
                        &config.job_keywords,
                        Some(app_state.clone()),
                        config.resume_profiles.as_deref(),
                        config.cover_letter_profiles.as_deref(),
                        &config.industries,
                        &config.seniority_levels,
                        browser.as_ref(),
                        Some((&smtp_server, &smtp_username, &smtp_password)),
                    ).await {
                        Ok(applied_count) => {
                            info!("Applied to {} jobs", applied_count);
                        },
                        Err(e) => {
                            error!("Failed to process and apply to jobs: {}", e);
                        }
                    }
                } else {
                    info!("No matching jobs found");
                }
            },
            Err(e) => {
                error!("Job search failed: {}", e);
            }
        }

        // Wait before next iteration
        info!("Waiting for next search interval...");
        tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;
    }

    Ok(())
}
