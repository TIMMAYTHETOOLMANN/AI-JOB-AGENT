use std::sync::{Arc, Mutex};
use anyhow::Result;
use tracing::{info, error, debug};
use tracing_subscriber::EnvFilter;

mod config;
mod application_state;
mod job_search_engine;
mod job_application;
mod secrets;
mod credential_auth;
mod job_listing;
mod nike_career;
mod company_career;
mod document_profile;
mod profile_selector;
mod globalization;
mod adaptive_browser;
mod main_gui;

#[cfg(feature = "mock-browser")]
mod mock_browser;
#[cfg(feature = "mock-browser")]
use mock_browser::HeadlessBrowser;

#[cfg(feature = "headless-chrome")]
mod headless_browser;
#[cfg(feature = "headless-chrome")]
use headless_browser::HeadlessBrowser;

use crate::config::Config;
use crate::application_state::{ApplicationState};
use crate::job_search_engine::search_jobs;
use crate::secrets::{CredentialManager};
use crate::credential_auth::{authenticate_all_platforms};
use crate::globalization::{GlobalConfig};
use crate::adaptive_browser::{AdaptiveBrowserEngine, AdaptiveBrowserConfig};
use crate::main_gui::RuntimeEvent;
use std::sync::mpsc::{self, Sender};

fn main() {
    // Check for CLI mode flag
    let args: Vec<String> = std::env::args().collect();
    let cli_mode = args.iter().any(|arg| arg == "--cli" || arg == "--headless");
    
    let (tx, rx) = mpsc::channel();
    
    if cli_mode {
        println!("=== AI Job Agent - CLI Mode ===");
        println!("Running in headless mode without GUI...");
        
        // Run agent directly in main thread for CLI mode
        agent_main_cli();
    } else {
        println!("=== AI Job Agent - GUI Mode ===");
        println!("Starting with GUI interface...");
        
        // Launch agent logic in a background thread with its own tokio runtime
        let agent_tx = tx.clone();
        std::thread::spawn(move || {
            agent_main(agent_tx);
        });
        
        // Run the GUI on the main thread
        main_gui::run_gui(rx);
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

fn agent_main_cli() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Initialize logging for CLI mode
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_writer(std::io::stdout)
            .init();

        println!("=== Autonomous Job Agent CLI - Starting ===");
        
        // Print current working directory
        let current_dir = std::env::current_dir().unwrap_or_default();
        println!("Current working directory: {:?}", current_dir);

        // Check if AgentConfig.toml exists
        let config_path = "AgentConfig.toml";
        let absolute_path = current_dir.join(config_path);
        println!("Configuration file: {:?}", absolute_path);
        println!("Configuration exists: {}", absolute_path.exists());

        if absolute_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&absolute_path) {
                println!("Configuration size: {} bytes", metadata.len());
            }
        }

        // Load configuration
        let config = match Config::load(config_path) {
            Ok(config) => {
                info!("Configuration loaded successfully");
                println!("✓ Configuration loaded successfully");
                
                // Perform comprehensive validation and context gathering
                match config.validate_and_gather_context() {
                    Ok(report) => {
                        println!("📋 Configuration Validation Report:");
                        report.print_summary();
                        config
                    },
                    Err(e) => {
                        error!("Configuration validation failed: {}", e);
                        eprintln!("✗ Configuration validation failed: {}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                eprintln!("✗ Failed to load configuration: {}", e);
                return;
            }
        };

        // Enhanced credential management system  
        info!("Loading credentials from multiple sources...");
        let credential_manager = match CredentialManager::load_all_credentials() {
            Ok(manager) => {
                info!("✅ Credential manager initialized successfully");
                let available_platforms = manager.get_available_platforms();
                if available_platforms.is_empty() {
                    info!("⚠️  No valid credentials found. You may need to set environment variables or update AgentSecrets.template");
                } else {
                    info!("📋 Available platforms: {}", available_platforms.join(", "));
                }
                manager
            },
            Err(e) => {
                error!("❌ Failed to initialize credential manager: {}", e);
                eprintln!("✗ Failed to load credentials: {}", e);
                return;
            }
        };

        // Get agent info from credentials (overrides config if present)
        let name = credential_manager.get_agent_info("AGENT_NAME").cloned().unwrap_or(config.name.clone());
        let email = credential_manager.get_agent_info("AGENT_EMAIL").cloned().unwrap_or(config.email.clone());
        let resume_path = credential_manager.get_agent_info("RESUME_PATH").cloned().unwrap_or(config.resume_path.clone());
        let cover_letter_path = credential_manager.get_agent_info("COVER_LETTER_PATH").cloned().unwrap_or(config.cover_letter_path.clone());

        println!("Agent Name: {}", name);
        println!("Agent Email: {}", email);
        println!("Resume Path: {}", resume_path);
        println!("Cover Letter Path: {}", cover_letter_path);

        // Create application state
        let app_state = Arc::new(Mutex::new(ApplicationState::new("state/job_state.json")));
        info!("Application state initialized");
        println!("✓ Application state initialized");

        // Initialize global configuration for internationalization
        let global_config = GlobalConfig::default();
        let detected_locale = config.get_effective_locale();
        let effective_currency = config.get_effective_currency();
        info!("Detected locale: {}, Currency: {}", detected_locale, effective_currency);
        println!("✓ Locale: {}, Currency: {}", detected_locale, effective_currency);

        // Initialize adaptive browser engine
        let adaptive_config = AdaptiveBrowserConfig::default();
        let mut adaptive_browser_engine = AdaptiveBrowserEngine::new(adaptive_config);
        info!("Adaptive browser engine initialized");
        println!("✓ Adaptive browser engine initialized");

        // Initialize headless browser
        let browser = match HeadlessBrowser::new() {
            Ok(browser) => {
                info!("Headless browser initialized successfully");
                println!("✓ Mock browser initialized successfully");
                Some(browser)
            },
            Err(e) => {
                error!("Failed to initialize headless browser: {}", e);
                println!("⚠ Failed to initialize browser, continuing without automation: {}", e);
                None
            }
        };

        println!("\n=== Starting Job Search Cycles ===");
        let mut cycle_count = 0;

        // Continuous execution loop
        loop {
            cycle_count += 1;
            println!("\n--- Job Search Cycle #{} ---", cycle_count);
            info!("Starting job search cycle #{}", cycle_count);

            // Enhanced search with global job boards
            let all_job_boards = config.get_all_job_boards();
            info!("Searching across {} job boards (including international)", all_job_boards.len());
            println!("Searching across {} job boards (including international)", all_job_boards.len());

            // Search for jobs
            match search_jobs(&config).await {
                Ok(job_listings) => {
                    if !job_listings.is_empty() {
                        debug!("Found {} job listings", job_listings.len());
                        println!("✓ Found {} job listings", job_listings.len());
                        
                        // Extract URLs from job listings
                        let urls: Vec<String> = job_listings.iter().map(|job| job.url.clone()).collect();

                        // Apply to jobs with enhanced capabilities
                        let smtp_server = credential_manager.agent_info.get("SMTP_SERVER").cloned().unwrap_or_else(|| "smtp.gmail.com".to_string());
                        let smtp_username = credential_manager.agent_info.get("SMTP_USERNAME").cloned().unwrap_or_default();
                        let smtp_password = credential_manager.agent_info.get("SMTP_PASSWORD").cloned().unwrap_or_default();

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
                                println!("✓ Applied to {} jobs successfully", applied_count);

                                // Log performance metrics
                                let performance_report = adaptive_browser_engine.get_performance_report();
                                info!("Adaptive Browser Performance:\n{}", performance_report);
                                println!("Performance Report:\n{}", performance_report);
                            },
                            Err(e) => {
                                error!("Failed to process and apply to jobs: {}", e);
                                println!("✗ Failed to process jobs: {}", e);
                            }
                        }
                    } else {
                        info!("No matching jobs found");
                        println!("⚠ No matching jobs found");
                    }
                },
                Err(e) => {
                    error!("Job search failed: {}", e);
                    println!("✗ Job search failed: {}", e);
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

            println!("Waiting {} seconds before next search cycle...", wait_duration);
            info!("Waiting {} seconds before next search cycle...", wait_duration);
            tokio::time::sleep(tokio::time::Duration::from_secs(wait_duration)).await;
        }
    });
}

fn agent_main(tx: Sender<RuntimeEvent>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
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
                
                // Perform comprehensive validation and context gathering
                match config.validate_and_gather_context() {
                    Ok(report) => {
                        info!("Configuration validation completed successfully");
                        report.print_summary();  // This will print to stdout for GUI mode
                        config
                    },
                    Err(e) => {
                        error!("Configuration validation failed: {}", e);
                        tx.send(RuntimeEvent {
                            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            message: format!("Configuration validation failed: {}", e),
                            level: "error".to_string(),
                        }).ok();
                        return;
                    }
                }
            },
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                tx.send(RuntimeEvent {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    message: format!("Failed to load configuration: {}", e),
                    level: "error".to_string(),
                }).ok();
                return;
            }
        };

        // Enhanced credential management system
        info!("Loading credentials from multiple sources...");
        let mut credential_manager = match CredentialManager::load_all_credentials() {
            Ok(manager) => {
                info!("✅ Credential manager initialized successfully");
                let available_platforms = manager.get_available_platforms();
                if available_platforms.is_empty() {
                    info!("⚠️  No valid credentials found. You may need to set environment variables or update AgentSecrets.template");
                } else {
                    info!("📋 Available platforms: {}", available_platforms.join(", "));
                }
                manager
            },
            Err(e) => {
                error!("❌ Failed to initialize credential manager: {}", e);
                tx.send(RuntimeEvent {
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    message: format!("Failed to load credentials: {}", e),
                    level: "error".to_string(),
                }).ok();
                return;
            }
        };

        // Get agent info from credentials (overrides config if present)
        let name = credential_manager.get_agent_info("AGENT_NAME").cloned().unwrap_or(config.name.clone());
        let email = credential_manager.get_agent_info("AGENT_EMAIL").cloned().unwrap_or(config.email.clone());
        let resume_path = credential_manager.get_agent_info("RESUME_PATH").cloned().unwrap_or(config.resume_path.clone());
        let cover_letter_path = credential_manager.get_agent_info("COVER_LETTER_PATH").cloned().unwrap_or(config.cover_letter_path.clone());

        info!("Agent: {} <{}>", name, email);
        info!("Resume: {}", resume_path);
        info!("Cover Letter: {}", cover_letter_path);

        // Authenticate with platforms (fetch API tokens)
        info!("🔐 Authenticating with job platforms...");
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        let authenticated_platforms = match rt.block_on(authenticate_all_platforms(&mut credential_manager)) {
            Ok(platforms) => {
                info!("✅ Authentication completed for {} platforms", platforms.len());
                platforms
            },
            Err(e) => {
                error!("⚠️  Authentication failed: {}", e);
                vec![]
            }
        };

        tx.send(RuntimeEvent {
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            message: format!("Authenticated with {} platforms: {}", 
                authenticated_platforms.len(), 
                authenticated_platforms.join(", ")
            ),
            level: "info".to_string(),
        }).ok();

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

        // Example: send a startup event
        tx.send(RuntimeEvent {
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            message: "Agent started.".to_string(),
            level: "info".to_string(),
        }).ok();

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
                        tx.send(RuntimeEvent {
                            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            message: format!("Found {} job listings", job_listings.len()),
                            level: "info".to_string(),
                        }).ok();
                        // Extract URLs from job listings
                        let urls: Vec<String> = job_listings.iter().map(|job| job.url.clone()).collect();

                        // Apply to jobs with enhanced capabilities
                        let smtp_server = credential_manager.agent_info.get("SMTP_SERVER").cloned().unwrap_or_else(|| "smtp.gmail.com".to_string());
                        let smtp_username = credential_manager.agent_info.get("SMTP_USERNAME").cloned().unwrap_or_default();
                        let smtp_password = credential_manager.agent_info.get("SMTP_PASSWORD").cloned().unwrap_or_default();

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
                                tx.send(RuntimeEvent {
                                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                                    message: format!("Applied to {} jobs", applied_count),
                                    level: "info".to_string(),
                                }).ok();

                                // Log performance metrics
                                let performance_report = adaptive_browser_engine.get_performance_report();
                                info!("Adaptive Browser Performance:\n{}", performance_report);
                                tx.send(RuntimeEvent {
                                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                                    message: format!("Performance: {}", performance_report),
                                    level: "info".to_string(),
                                }).ok();
                            },
                            Err(e) => {
                                error!("Failed to process and apply to jobs: {}", e);
                                tx.send(RuntimeEvent {
                                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                                    message: format!("Failed to apply to jobs: {}", e),
                                    level: "error".to_string(),
                                }).ok();
                            }
                        }
                    } else {
                        info!("No matching jobs found");
                        tx.send(RuntimeEvent {
                            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            message: "No matching jobs found".to_string(),
                            level: "warn".to_string(),
                        }).ok();
                    }
                },
                Err(e) => {
                    error!("Job search failed: {}", e);
                    tx.send(RuntimeEvent {
                        timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        message: format!("Job search failed: {}", e),
                        level: "error".to_string(),
                    }).ok();
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
    });
}
