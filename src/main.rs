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
mod account_creator;
mod visible_browser;

#[cfg(feature = "mock-browser")]
mod mock_browser;

#[cfg(feature = "headless-chrome")]
mod headless_browser;

// Use visible browser by default when visible-chrome feature is enabled
#[cfg(feature = "visible-chrome")]
use visible_browser::VisibleBrowser as MainBrowser;

// Fallback to headless browser when only headless-chrome is available
#[cfg(all(feature = "headless-chrome", not(feature = "visible-chrome")))]
use headless_browser::HeadlessBrowser as MainBrowser;

// Fallback to mock browser only when no real browser features are available
#[cfg(all(feature = "mock-browser", not(feature = "headless-chrome"), not(feature = "visible-chrome")))]
use mock_browser::HeadlessBrowser as MainBrowser;

use crate::config::Config;
use crate::application_state::{ApplicationState, SharedApplicationState};
use crate::job_search_engine::search_jobs;
use crate::job_application::process_and_apply_to_jobs;
use crate::secrets::load_secrets;
use crate::globalization::{GlobalConfig, detect_locale, get_regional_job_boards};
use crate::adaptive_browser::{AdaptiveBrowserEngine, AdaptiveBrowserConfig};
use crate::account_creator::AccountCreator;
use std::sync::mpsc::{self, Sender};

fn main() {
    // Check for CLI mode flag (default is now browser mode, not GUI mode)
    let args: Vec<String> = std::env::args().collect();
    let gui_mode = args.iter().any(|arg| arg == "--gui" || arg == "--desktop");
    
    if gui_mode {
        println!("❌ GUI Mode Disabled - Browser Automation Agent Only");
        println!("🌐 This system now operates exclusively through browser automation");
        println!("🔍 Every action is visible in the browser window");
        println!("🛠️ Use interactive commands for manual intervention");
        println!("💡 Run without --gui flag for browser automation mode");
        return;
    }
    
    println!("=== 🤖 AI Job Agent - Browser Automation Mode ===");
    println!("🌐 Visible Browser Monitoring Active");
    println!("🔍 Real-time job search and application automation");
    println!("🛠️ Interactive control and manual intervention available");
    
    // Run browser automation agent
    agent_main_cli();
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
    browser: Option<&MainBrowser>,
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
            // Use adaptive browser automation with visible browser
            adaptive_engine.adaptive_apply_visible(
                browser,
                url,
                name,
                email,
                resume_path,
                cover_letter_path,
            ).await
        } else {
            // Fallback to basic processing
            Ok(format!("[SIMULATION] Successfully processed job: {}", url))
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
        // Initialize logging for browser mode
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_writer(std::io::stdout)
            .init();

        println!("=== 🤖 Autonomous Job Agent - Browser Automation Mode ===");
        println!("🌐 Initializing visible browser with real-time monitoring...");
        
        // Print current working directory
        let current_dir = std::env::current_dir().unwrap_or_default();
        println!("📁 Working directory: {:?}", current_dir);

        // Check if AgentConfig.toml exists
        let config_path = "AgentConfig.toml";
        let absolute_path = current_dir.join(config_path);
        println!("⚙️ Configuration file: {:?}", absolute_path);
        println!("✅ Configuration exists: {}", absolute_path.exists());

        if absolute_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&absolute_path) {
                println!("📊 Configuration size: {} bytes", metadata.len());
            }
        }

        // Load configuration
        let config = match Config::load(config_path) {
            Ok(config) => {
                info!("Configuration loaded successfully");
                println!("✅ Configuration loaded successfully");
                
                // Perform comprehensive validation and context gathering
                match config.validate_and_gather_context() {
                    Ok(report) => {
                        println!("📋 Configuration Validation Report:");
                        report.print_summary();
                        config
                    },
                    Err(e) => {
                        error!("Configuration validation failed: {}", e);
                        eprintln!("❌ Configuration validation failed: {}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                eprintln!("❌ Failed to load configuration: {}", e);
                return;
            }
        };

        // Load secrets from AgentSecrets.template
        let secrets = load_secrets("AgentSecrets.template");

        // Inject secrets into config if present
        let name = secrets.get("AGENT_NAME").cloned().unwrap_or(config.name.clone());
        let email = secrets.get("AGENT_EMAIL").cloned().unwrap_or(config.email.clone());
        let resume_path = secrets.get("RESUME_PATH").cloned().unwrap_or(config.resume_path.clone());
        let cover_letter_path = secrets.get("COVER_LETTER_PATH").cloned().unwrap_or(config.cover_letter_path.clone());

        println!("👤 Agent Name: {}", name);
        println!("📧 Agent Email: {}", email);
        println!("📄 Resume Path: {}", resume_path);
        println!("💌 Cover Letter Path: {}", cover_letter_path);

        // Create application state
        let app_state = Arc::new(Mutex::new(ApplicationState::new("state/job_state.json")));
        info!("Application state initialized");
        println!("✅ Application state initialized");

        // Initialize global configuration for internationalization
        let global_config = GlobalConfig::default();
        let detected_locale = config.get_effective_locale();
        let effective_currency = config.get_effective_currency();
        info!("Detected locale: {}, Currency: {}", detected_locale, effective_currency);
        println!("🌍 Locale: {}, Currency: {}", detected_locale, effective_currency);

        // Initialize adaptive browser engine
        let adaptive_config = AdaptiveBrowserConfig::default();
        let mut adaptive_browser_engine = AdaptiveBrowserEngine::new(adaptive_config);
        info!("Adaptive browser engine initialized");
        println!("✅ Adaptive browser engine initialized");

        // Initialize visible browser with real-time monitoring
        let browser = match MainBrowser::new() {
            Ok(browser) => {
                info!("🚀 Visible browser initialized successfully");
                println!("🚀 Visible browser initialized successfully");
                println!("🔍 Real-time monitoring active - you can see every browser action");
                println!("🛠️ Type 'pause', 'resume', 'step', or 'manual' for control");
                Some(browser)
            },
            Err(e) => {
                error!("Failed to initialize visible browser: {}", e);
                println!("❌ Failed to initialize visible browser: {}", e);
                println!("🔄 Continuing with simulation mode...");
                None
            }
        };

        // Initialize account creator if enabled
        let account_creator = if config.account_creation.as_ref().map(|ac| ac.enabled).unwrap_or(false) {
            match AccountCreator::new(&config, secrets.clone()) {
                Ok(creator) => {
                    info!("Account creator initialized");
                    println!("✅ Account creator initialized");
                    Some(creator)
                },
                Err(e) => {
                    error!("Failed to initialize account creator: {}", e);
                    println!("⚠️ Failed to initialize account creator: {}", e);
                    None
                }
            }
        } else {
            info!("Account creation disabled in configuration");
            println!("ℹ️ Account creation disabled in configuration");
            None
        };

        // Create accounts if account creator is available and browser is working
        if let (Some(creator), Some(ref browser)) = (&account_creator, &browser) {
            info!("Creating accounts on major job sites...");
            println!("\n=== 🏗️ Creating Accounts on Major Job Sites ===");
            
            match creator.create_all_accounts_visible(browser).await {
                Ok(results) => {
                    for result in results {
                        info!("Account creation result: {}", result);
                        println!("✅ {}", result);
                    }
                },
                Err(e) => {
                    error!("Failed to create accounts: {}", e);
                    println!("⚠️ Failed to create accounts: {}", e);
                }
            }
            
            println!("=== Account creation phase completed ===\n");
        }

        println!("\n=== 🔍 Starting Automated Job Search Cycles ===");
        println!("🛠️ Interactive Control Available:");
        println!("   • Type 'pause' to pause the agent");
        println!("   • Type 'resume' to resume automation");
        println!("   • Type 'step' to execute one action at a time");
        println!("   • Type 'manual' to take manual control");
        println!("   • Type 'status' to see current status");
        println!("   • Type 'stop' to shut down gracefully\n");
        
        let mut cycle_count = 0;

        // Continuous execution loop with browser automation
        loop {
            cycle_count += 1;
            println!("\n--- 🔄 Job Search Cycle #{} ---", cycle_count);
            info!("Starting job search cycle #{}", cycle_count);

            // Enhanced search with global job boards
            let all_job_boards = config.get_all_job_boards();
            info!("Searching across {} job boards (including international)", all_job_boards.len());
            println!("🌐 Searching across {} job boards (including international)", all_job_boards.len());

            // Search for jobs
            match search_jobs(&config).await {
                Ok(job_listings) => {
                    if !job_listings.is_empty() {
                        debug!("Found {} job listings", job_listings.len());
                        println!("✅ Found {} job listings", job_listings.len());
                        
                        // Extract URLs from job listings
                        let urls: Vec<String> = job_listings.iter().map(|job| job.url.clone()).collect();

                        // Apply to jobs with enhanced capabilities and visible browser
                        let smtp_server = secrets.get("SMTP_SERVER").cloned().unwrap_or_else(|| "smtp.gmail.com".to_string());
                        let smtp_username = secrets.get("SMTP_USERNAME").cloned().unwrap_or_default();
                        let smtp_password = secrets.get("SMTP_PASSWORD").cloned().unwrap_or_default();

                        // Enhanced application process with visible browser automation
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
                                info!("Applied to {} jobs with visible browser automation", applied_count);
                                println!("✅ Applied to {} jobs successfully", applied_count);

                                // Log performance metrics
                                let performance_report = adaptive_browser_engine.get_performance_report();
                                info!("Adaptive Browser Performance:\n{}", performance_report);
                                println!("📊 Performance Report:\n{}", performance_report);
                            },
                            Err(e) => {
                                error!("Failed to process and apply to jobs: {}", e);
                                println!("❌ Failed to process jobs: {}", e);
                            }
                        }
                    } else {
                        info!("No matching jobs found");
                        println!("⚠️ No matching jobs found in this cycle");
                    }
                },
                Err(e) => {
                    error!("Job search failed: {}", e);
                    println!("❌ Job search failed: {}", e);
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

            println!("⏰ Waiting {} seconds before next search cycle...", wait_duration);
            info!("Waiting {} seconds before next search cycle...", wait_duration);
            
            // Wait with ability to interrupt
            for _ in 0..wait_duration {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                
                // Check if browser received stop command
                if let Some(ref browser) = browser {
                    if let Err(_) = browser.check_for_commands() {
                        println!("🛑 Stop command received, shutting down gracefully");
                        return;
                    }
                }
            }
        }
    });
}
