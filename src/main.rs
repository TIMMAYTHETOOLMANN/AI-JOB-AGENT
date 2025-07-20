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
mod headless_browser;
mod web_server;
mod main_gui;

// PRODUCTION MODE: Full GUI + Backend + Browser automation
use visible_browser::VisibleBrowser;
use web_server::WebServer;
use main_gui::ProductionGui;

use crate::config::Config;
use crate::application_state::{ApplicationState, SharedApplicationState};
use crate::job_search_engine::search_jobs;
use crate::job_application::process_and_apply_to_jobs;
use crate::secrets::load_secrets;
use crate::globalization::{GlobalConfig, detect_locale, get_regional_job_boards};
use crate::adaptive_browser::{AdaptiveBrowserEngine, AdaptiveBrowserConfig};
use crate::account_creator::AccountCreator;
use std::sync::mpsc::{self, Sender};

#[tokio::main]
async fn main() {
    // Parse command line arguments for mode selection
    let args: Vec<String> = std::env::args().collect();
    let gui_mode = args.iter().any(|arg| arg == "--gui" || arg == "--desktop");
    let cli_mode = args.iter().any(|arg| arg == "--cli" || arg == "--headless");
    let web_mode = args.iter().any(|arg| arg == "--web" || arg == "--server");

    // Initialize logging system
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stdout)
        .init();

    println!("=== 🤖 AI Job Agent - Production Deployment System ===");

    // Determine operation mode
    if gui_mode {
        println!("🎮 PRODUCTION GUI MODE - Full Desktop Application");
        production_gui_mode().await;
    } else if web_mode {
        println!("🌐 PRODUCTION WEB MODE - Browser-based Control Center");
        production_web_mode().await;
    } else if cli_mode {
        println!("⚡ PRODUCTION CLI MODE - Headless Automation");
        production_cli_mode().await;
    } else {
        println!("🚀 PRODUCTION HYBRID MODE - GUI + Web + CLI");
        production_hybrid_mode().await;
    }
}

/// Full production GUI with desktop application
async fn production_gui_mode() {
    info!("🎮 Starting Production GUI Application");

    match ProductionGui::run() {
        Ok(_) => {
            info!("✅ Production GUI completed successfully");
        },
        Err(e) => {
            error!("❌ Production GUI failed: {}", e);
            eprintln!("Failed to start Production GUI: {}", e);
        }
    }
}

/// Web-based control center with REST API
async fn production_web_mode() {
    info!("🌐 Starting Production Web Server");

    let config = match Config::load("AgentConfig.toml") {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return;
        }
    };

    let app_state = Arc::new(Mutex::new(ApplicationState::new("state/web_state.json")));
    let web_server = WebServer::new(8080, app_state.clone());

    println!("🌐 Production Web Server Configuration:");
    println!("   📊 Dashboard: http://localhost:8080/dashboard");
    println!("   🎮 Control Panel: http://localhost:8080/control");
    println!("   📋 API Docs: http://localhost:8080/api/docs");
    println!("   🔧 WebSocket: ws://localhost:8080/ws");

    if let Err(e) = web_server.start().await {
        error!("Web server failed: {}", e);
        eprintln!("Failed to start web server: {}", e);
    }
}

/// Headless CLI automation
async fn production_cli_mode() {
    info!("⚡ Starting Production CLI Mode");

    let config = match Config::load("AgentConfig.toml") {
        Ok(config) => {
            println!("✅ Configuration loaded successfully");
            config
        },
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            eprintln!("❌ Failed to load configuration: {}", e);
            return;
        }
    };

    // Load secrets and initialize components
    let secrets = load_secrets("AgentSecrets.template");
    let app_state = Arc::new(Mutex::new(ApplicationState::new("state/cli_state.json")));
    let mut adaptive_engine = AdaptiveBrowserEngine::new(AdaptiveBrowserConfig::default());

    // Initialize browser for CLI mode
    let browser = match VisibleBrowser::new() {
        Ok(browser) => {
            println!("✅ Browser initialized for CLI automation");
            Some(browser)
        },
        Err(e) => {
            error!("Browser initialization failed: {}", e);
            println!("⚠️ Running in simulation mode");
            None
        }
    };

    // Run automation cycles
    let mut cycle_count = 0;
    loop {
        cycle_count += 1;
        println!("\n--- 🔄 CLI Automation Cycle #{} ---", cycle_count);

        match search_jobs(&config).await {
            Ok(job_listings) => {
                if !job_listings.is_empty() {
                    println!("✅ Found {} job listings", job_listings.len());

                    let urls: Vec<String> = job_listings.iter().map(|job| job.url.clone()).collect();

                    // Process applications
                    match process_cli_applications(
                        &urls,
                        &config,
                        &secrets,
                        app_state.clone(),
                        &mut adaptive_engine,
                        browser.as_ref(),
                    ).await {
                        Ok(applied_count) => {
                            println!("✅ Applied to {} jobs successfully", applied_count);
                        },
                        Err(e) => {
                            error!("Application processing failed: {}", e);
                        }
                    }
                }
            },
            Err(e) => {
                error!("Job search failed: {}", e);
            }
        }

        // Wait before next cycle
        println!("⏰ Waiting 10 minutes before next cycle...");
        tokio::time::sleep(tokio::time::Duration::from_secs(600)).await;
    }
}

/// Hybrid mode: GUI + Web + CLI combined
async fn production_hybrid_mode() {
    info!("🚀 Starting Production Hybrid Mode");
    println!("🚀 HYBRID DEPLOYMENT: All systems active");

    let config = match Config::load("AgentConfig.toml") {
        Ok(config) => config,
        Err(e) => {
            error!("Configuration load failed: {}", e);
            return;
        }
    };

    let app_state = Arc::new(Mutex::new(ApplicationState::new("state/hybrid_state.json")));

    // Start web server in background
    let web_server = WebServer::new(8080, app_state.clone());
    let web_handle = tokio::spawn(async move {
        if let Err(e) = web_server.start().await {
            error!("Web server error: {}", e);
        }
    });

    // Start CLI automation in background
    let config_clone = config.clone();
    let app_state_clone = app_state.clone();
    let cli_handle = tokio::spawn(async move {
        run_background_automation(config_clone, app_state_clone).await;
    });

    println!("🌐 Web Interface: http://localhost:8080");
    println!("🎮 Starting GUI Application...");

    // Start GUI in main thread
    if let Err(e) = ProductionGui::run() {
        error!("GUI failed: {}", e);
    }

    // Cleanup background tasks
    web_handle.abort();
    cli_handle.abort();
}

async fn process_cli_applications(
    urls: &[String],
    config: &Config,
    secrets: &std::collections::HashMap<String, String>,
    app_state: Arc<Mutex<ApplicationState>>,
    adaptive_engine: &mut AdaptiveBrowserEngine,
    browser: Option<&VisibleBrowser>,
) -> Result<usize> {
    let mut applied_count = 0;

    let name = secrets.get("AGENT_NAME").cloned().unwrap_or(config.name.clone());
    let email = secrets.get("AGENT_EMAIL").cloned().unwrap_or(config.email.clone());
    let resume_path = secrets.get("RESUME_PATH").cloned().unwrap_or(config.resume_path.clone());
    let cover_letter_path = secrets.get("COVER_LETTER_PATH").cloned().unwrap_or(config.cover_letter_path.clone());

    for (index, url) in urls.iter().enumerate() {
        println!("🎯 [{}/{}] Processing: {}", index + 1, urls.len(), url);

        let result = if let Some(browser) = browser {
            adaptive_engine.adaptive_apply_visible(
                browser,
                url,
                &name,
                &email,
                &resume_path,
                &cover_letter_path,
            ).await
        } else {
            Ok(format!("Simulated application to {}", url))
        };

        match result {
            Ok(status) => {
                applied_count += 1;
                println!("✅ {}", status);
            },
            Err(e) => {
                println!("❌ Failed: {}", e);
            }
        }

        // Rate limiting
        if index < urls.len() - 1 {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    }

    Ok(applied_count)
}

async fn run_background_automation(
    config: Config,
    app_state: Arc<Mutex<ApplicationState>>,
) {
    info!("🤖 Background automation started");

    let secrets = load_secrets("AgentSecrets.template");
    let mut adaptive_engine = AdaptiveBrowserEngine::new(AdaptiveBrowserConfig::default());

    let mut cycle_count = 0;
    loop {
        cycle_count += 1;
        info!("Background automation cycle #{}", cycle_count);

        match search_jobs(&config).await {
            Ok(job_listings) => {
                if !job_listings.is_empty() {
                    let urls: Vec<String> = job_listings.iter().map(|job| job.url.clone()).collect();

                    if let Err(e) = process_cli_applications(
                        &urls,
                        &config,
                        &secrets,
                        app_state.clone(),
                        &mut adaptive_engine,
                        None, // Background mode uses simulation
                    ).await {
                        error!("Background processing failed: {}", e);
                    }
                }
            },
            Err(e) => {
                error!("Background job search failed: {}", e);
            }
        }

        // Background automation runs every 30 minutes
        tokio::time::sleep(tokio::time::Duration::from_secs(1800)).await;
    }
}
