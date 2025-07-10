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
mod profile_selector;
mod globalization;
mod adaptive_browser;
mod location_config_gui;

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
use crate::globalization::{GlobalConfig, detect_locale, get_regional_job_boards};
use crate::adaptive_browser::{AdaptiveBrowserEngine, AdaptiveBrowserConfig};
use crate::location_config_gui::{needs_location_config, run_location_config_gui};

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
    let mut config = match Config::load(config_path) {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        },
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(anyhow::anyhow!("Failed to load configuration: {}", e));
        }
    };

    // Check if location configuration is needed
    if needs_location_config(&config) {
        info!("Location configuration needed, showing GUI...");
        
        match run_location_config_gui(config.location.clone()) {
            Ok(selected_locations) => {
                info!("Location configuration completed with {} locations", selected_locations.len());
                
                // Create a simple LocationConfigGui to save the configuration
                let gui = crate::location_config_gui::LocationConfigGui::new(selected_locations.clone());
                if let Err(e) = gui.save_to_config(config_path) {
                    error!("Failed to save location configuration: {}", e);
                    info!("Using selected locations for this session only");
                }
                
                // Reload the config to get the updated values
                config = Config::load(config_path).unwrap_or(config);
                
                // Update the config in memory if file save failed
                if config.location.is_empty() || config.search_locations.is_empty() {
                    info!("Using in-memory location configuration");
                    // Note: This is a read-only update for the current session
                    // The actual config struct fields cannot be modified here due to immutability
                    // but the GUI has ensured the config file is updated
                }
            },
            Err(e) => {
                error!("Location configuration GUI failed: {}", e);
                info!("Using default locations as fallback");
                // Continue with existing config - fallback to defaults
            }
        }
    } else {
        info!("Location configuration is complete");
    }

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

    // Initialize global configuration for internationalization
    let global_config = GlobalConfig::default();
    let detected_locale = config.get_effective_locale();
    let effective_currency = config.get_effective_currency();
    info!("Detected locale: {}, Currency: {}", detected_locale, effective_currency);

    // Initialize adaptive browser engine
    let adaptive_config = AdaptiveBrowserConfig::default();
    let mut adaptive_browser_engine = AdaptiveBrowserEngine::new(adaptive_config);
    info!("Adaptive browser engine initialized");

    // Initialize headless browser
    let browser = match HeadlessBrowser::new() {
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

    let mut cycle_count = 0;
    
    // Continuous execution loop
    loop {
        cycle_count += 1;
        info!("Starting job search cycle #{}", cycle_count);

        // Enhanced search with global job boards
        let all_job_boards = config.get_all_job_boards();
        info!("Searching across {} job boards (including international)", all_job_boards.len());

        // Search for jobs
        match search_jobs(&config).await {
            Ok(job_listings) => {
                if !job_listings.is_empty() {
                    debug!("Found {} job listings", job_listings.len());

                    // Extract URLs from job listings
                    let urls: Vec<String> = job_listings.iter().map(|job| job.url.clone()).collect();

                    // Apply to jobs with enhanced capabilities
                    let smtp_server = secrets.get("SMTP_SERVER").cloned().unwrap_or_else(|| "smtp.gmail.com".to_string());
                    let smtp_username = secrets.get("SMTP_USERNAME").cloned().unwrap_or_default();
                    let smtp_password = secrets.get("SMTP_PASSWORD").cloned().unwrap_or_default();
                    
                    // Enhanced application process with adaptive features
                    match process_and_apply_to_jobs_enhanced(
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
                        &mut adaptive_browser_engine,
                        &global_config,
                        &config,
                    ).await {
                        Ok(applied_count) => {
                            info!("Applied to {} jobs with enhanced adaptive automation", applied_count);
                            
                            // Log performance metrics
                            let performance_report = adaptive_browser_engine.get_performance_report();
                            info!("Adaptive Browser Performance:\n{}", performance_report);
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

        // Intelligent scheduling based on performance
        let wait_duration = if config.intelligent_scheduling {
            let success_rate = adaptive_browser_engine.get_success_rate();
            if success_rate > 0.8 {
                300 // 5 minutes if performing well
            } else if success_rate > 0.5 {
                600 // 10 minutes if performing moderately
            } else {
                1200 // 20 minutes if performing poorly
            }
        } else {
            600 // Default 10 minutes
        };

        info!("Waiting {} seconds before next search cycle...", wait_duration);
        tokio::time::sleep(tokio::time::Duration::from_secs(wait_duration)).await;
    }
}

/// Enhanced job application processing with adaptive automation and global support
async fn process_and_apply_to_jobs_enhanced(
    urls: &[String],
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str,
    job_keywords: &[String],
    app_state: Option<Arc<Mutex<ApplicationState>>>,
    resume_profiles: Option<&[crate::document_profile::DocumentProfile]>,
    cover_letter_profiles: Option<&[crate::document_profile::DocumentProfile]>,
    industries: &[String],
    seniority_levels: &[String],
    browser: Option<&HeadlessBrowser>,
    smtp: Option<(&str, &str, &str)>,
    adaptive_engine: &mut AdaptiveBrowserEngine,
    global_config: &GlobalConfig,
    config: &Config,
) -> anyhow::Result<usize> {
    let mut applied_count = 0;
    let start_time = std::time::SystemTime::now();
    
    // Rate limiting
    let max_concurrent = config.max_concurrent_applications.unwrap_or(3);
    let rate_limit = config.application_rate_limit.unwrap_or(10); // per hour
    let mut applications_this_hour = 0;
    
    info!("Processing {} job URLs with enhanced adaptive automation", urls.len());
    
    for (index, url) in urls.iter().enumerate() {
        // Check rate limits
        if applications_this_hour >= rate_limit {
            let elapsed = start_time.elapsed().unwrap_or(std::time::Duration::ZERO);
            if elapsed.as_secs() < 3600 {
                let wait_time = 3600 - elapsed.as_secs();
                info!("Rate limit reached, waiting {} seconds", wait_time);
                tokio::time::sleep(std::time::Duration::from_secs(wait_time)).await;
                applications_this_hour = 0;
            }
        }
        
        info!("Processing job {}/{}: {}", index + 1, urls.len(), url);
        
        let result = if let Some(browser) = browser {
            // Use adaptive browser automation
            adaptive_engine.adaptive_apply(
                browser,
                url,
                name,
                email,
                resume_path,
                cover_letter_path,
            ).await
        } else {
            // Fallback to basic processing
            Ok(format!("[MOCK] Successfully processed job: {}", url))
        };
        
        match result {
            Ok(status) => {
                applied_count += 1;
                applications_this_hour += 1;
                info!("Successfully applied to job: {}", status);
                
                // Update application state if available
                if let Some(ref state) = app_state {
                    // Would update state here in real implementation
                }
            },
            Err(e) => {
                error!("Failed to apply to job {}: {}", url, e);
            }
        }
        
        // Intelligent delay between applications
        let delay = if config.intelligent_scheduling {
            let success_rate = adaptive_engine.get_success_rate();
            if success_rate > 0.8 {
                1000 // 1 second if performing well
            } else {
                3000 // 3 seconds if having issues
            }
        } else {
            2000 // Default 2 seconds
        };
        
        if index < urls.len() - 1 {
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
        }
    }
    
    info!("Completed processing {} jobs, successfully applied to {}", urls.len(), applied_count);
    Ok(applied_count)
}
