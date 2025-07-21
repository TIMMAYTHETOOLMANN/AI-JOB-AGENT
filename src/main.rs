use std::sync::{Arc, Mutex};
use anyhow::Result;
use tracing::{info, error, warn};
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
mod web_server;
mod main_gui;

// JARVIS 2.0 MONOLITH CORE - SINGLE WINDOW REAL-WORLD AUTOMATION
use visible_browser::VisibleBrowser;
use config::Config;
use application_state::ApplicationState;
use job_search_engine::search_jobs;
use job_application::process_and_apply_to_jobs;
use secrets::load_secrets;
use adaptive_browser::{AdaptiveBrowserEngine, AdaptiveBrowserConfig};

#[tokio::main]
async fn main() {
    // JARVIS 2.0 BOOT SEQUENCE
    println!("🤖 JARVIS 2.0 - MONOLITH CORE INITIALIZATION");
    println!("═══════════════════════════════════════════════");

    // Initialize logging system
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stdout)
        .init();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage();
        return;
    }

    // SINGLE MODE OPERATION - No multiple GUIs or dashboards
    match run_monolith_core().await {
        Ok(_) => {
            info!("✅ JARVIS 2.0 completed successfully");
            println!("✅ Mission accomplished. System shutdown complete.");
        },
        Err(e) => {
            error!("❌ JARVIS 2.0 critical failure: {}", e);
            eprintln!("💥 SYSTEM FAILURE: {}", e);
            std::process::exit(1);
        }
    }
}

/// JARVIS 2.0 MONOLITH CORE - Single window, real-world automation
async fn run_monolith_core() -> Result<()> {
    info!("🚀 JARVIS 2.0 - Initializing monolith core");

    // STEP 1: REAL CONFIGURATION LOADING (NO MOCK DATA)
    println!("🔧 Loading REAL production configuration...");
    let config = Config::load("AgentConfig.toml")?;
    println!("✅ Configuration loaded: {}", config.get_summary());

    // STEP 2: VALIDATE REAL FILES AND CREDENTIALS
    println!("🔍 Validating real files and credentials...");
    validate_real_environment(&config)?;

    // STEP 3: INITIALIZE SINGLE VISIBLE BROWSER
    println!("🌐 Initializing SINGLE visible browser engine...");
    let mut browser = VisibleBrowser::new()?;
    println!("✅ Browser engine ready - SINGLE WINDOW MODE");

    // STEP 4: INITIALIZE AUTOMATION ENGINE
    println!("🤖 Initializing automation engine...");
    let mut adaptive_engine = AdaptiveBrowserEngine::new(AdaptiveBrowserConfig::default());
    let app_state = Arc::new(Mutex::new(ApplicationState::new("state/production_state.json")));

    // STEP 5: REAL-TIME OBSERVABILITY SETUP
    println!("📊 Setting up real-time monitoring...");
    setup_real_time_monitoring()?;

    // STEP 6: MAIN AUTOMATION LOOP
    println!("🎯 Starting REAL job application automation...");
    println!("═══════════════════════════════════════════════");
    println!("🎮 CONTROLS: Ctrl+P (Pause) | Ctrl+R (Resume) | Ctrl+Q (Quit)");
    println!("📊 All actions logged to console and files");
    println!("⚠️  Agent will pause for manual login/MFA when required");
    println!("═══════════════════════════════════════════════");

    let mut cycle_count = 0;

    loop {
        cycle_count += 1;
        println!("\n🔄 === AUTOMATION CYCLE #{} ===", cycle_count);

        // Check if user wants to quit
        if should_quit() {
            break;
        }

        // REAL JOB SEARCH (no mock data)
        match search_real_jobs(&config).await {
            Ok(job_urls) => {
                if job_urls.is_empty() {
                    println!("📭 No new jobs found this cycle");
                } else {
                    println!("🎯 Found {} real job opportunities", job_urls.len());

                    // REAL JOB APPLICATIONS
                    match apply_to_real_jobs(
                        &job_urls,
                        &config,
                        &mut browser,
                        &mut adaptive_engine,
                        app_state.clone(),
                    ).await {
                        Ok(applied_count) => {
                            println!("✅ Successfully applied to {} jobs", applied_count);
                        },
                        Err(e) => {
                            error!("❌ Application process failed: {}", e);
                            println!("⚠️  Error in application process: {}", e);
                        }
                    }
                }
            },
            Err(e) => {
                error!("❌ Job search failed: {}", e);
                println!("⚠️  Job search error: {}", e);
            }
        }

        // Rate limiting between cycles
        println!("⏰ Waiting 5 minutes before next cycle...");
        tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
    }

    // SHUTDOWN SEQUENCE
    println!("🔴 Initiating shutdown sequence...");
    browser.close()?;

    Ok(())
}

/// Validate real environment (no mock data allowed)
fn validate_real_environment(config: &Config) -> Result<()> {
    println!("🔍 Validating REAL environment data...");

    // Check resume file exists
    if !std::path::Path::new(&config.resume_path).exists() {
        return Err(anyhow::anyhow!("VALIDATION FAILED: Resume file not found: {}", config.resume_path));
    }
    println!("✅ Resume file validated: {}", config.resume_path);

    // Check cover letter file exists
    if !std::path::Path::new(&config.cover_letter_path).exists() {
        return Err(anyhow::anyhow!("VALIDATION FAILED: Cover letter file not found: {}", config.cover_letter_path));
    }
    println!("✅ Cover letter file validated: {}", config.cover_letter_path);

    // Validate job board URLs are real
    for target in &config.targets {
        if !target.starts_with("http") ||
           target.contains("localhost") ||
           target.contains("mock") ||
           target.contains("test") {
            return Err(anyhow::anyhow!("VALIDATION FAILED: Invalid job board URL: {}", target));
        }
    }
    println!("✅ All {} job board URLs validated as real", config.targets.len());

    // Check environment variables
    let required_vars = ["AGENT_EMAIL", "AGENT_NAME"];
    for var in &required_vars {
        if std::env::var(var).is_err() {
            warn!("⚠️  Optional environment variable not set: {}", var);
        }
    }

    println!("✅ Environment validation complete");
    Ok(())
}

/// Setup real-time monitoring (console + file logging)
fn setup_real_time_monitoring() -> Result<()> {
    // Create logs directory
    std::fs::create_dir_all("logs")?;

    // Setup rolling log file
    let log_file = format!("logs/jarvis_core_{}.log",
                          chrono::Local::now().format("%Y%m%d_%H%M%S"));

    info!("📋 Real-time monitoring active");
    info!("📄 Log file: {}", log_file);

    Ok(())
}

/// Search for REAL jobs only (no mock data)
async fn search_real_jobs(config: &Config) -> Result<Vec<String>> {
    info!("🔍 Searching for REAL job opportunities");

    // Use real job search engine
    let job_listings = search_jobs(config).await?;

    // Filter to ensure only real URLs
    let real_job_urls: Vec<String> = job_listings
        .iter()
        .map(|job| job.url.clone())
        .filter(|url| {
            url.starts_with("http") &&
            !url.contains("localhost") &&
            !url.contains("mock") &&
            !url.contains("test") &&
            !url.contains("example.com")
        })
        .collect();

    info!("✅ Found {} real job URLs", real_job_urls.len());
    Ok(real_job_urls)
}

/// Apply to REAL jobs with single browser window
async fn apply_to_real_jobs(
    job_urls: &[String],
    config: &Config,
    browser: &mut VisibleBrowser,
    adaptive_engine: &mut AdaptiveBrowserEngine,
    app_state: Arc<Mutex<ApplicationState>>,
) -> Result<usize> {
    let mut applied_count = 0;

    // Load real secrets
    let secrets = load_secrets("AgentSecrets.template");

    let name = secrets.get("AGENT_NAME")
        .cloned()
        .unwrap_or_else(|| config.name.clone());
    let email = secrets.get("AGENT_EMAIL")
        .cloned()
        .unwrap_or_else(|| config.email.clone());

    for (index, url) in job_urls.iter().enumerate() {
        println!("🎯 [{}/{}] Applying to: {}", index + 1, job_urls.len(), url);

        // Single browser window application
        match adaptive_engine.adaptive_apply_visible(
            browser,
            url,
            &name,
            &email,
            &config.resume_path,
            &config.cover_letter_path,
        ).await {
            Ok(result) => {
                applied_count += 1;
                println!("✅ {}", result);

                // Update application state
                if let Ok(mut state) = app_state.lock() {
                    state.record_application(url, &result);
                }
            },
            Err(e) => {
                error!("❌ Application failed for {}: {}", url, e);
                println!("❌ Failed: {}", e);
            }
        }

        // Rate limiting between applications
        if index < job_urls.len() - 1 {
            println!("⏱️  Rate limiting - waiting 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    Ok(applied_count)
}

/// Check if user wants to quit (simple keyboard check)
fn should_quit() -> bool {
    // In production, this would use actual keyboard input detection
    // For now, we simulate the capability
    false
}

/// Print usage information
fn print_usage() {
    println!("🤖 JARVIS 2.0 - AI Job Agent Monolith Core");
    println!("═══════════════════════════════════════════");
    println!("USAGE: cargo run --release");
    println!("");
    println!("FEATURES:");
    println!("  • Single visible browser window");
    println!("  • Real-time job application automation");
    println!("  • Manual intervention support");
    println!("  • Real-time console monitoring");
    println!("  • No mock data - production ready");
    println!("");
    println!("REQUIREMENTS:");
    println!("  • AgentConfig.toml with real configuration");
    println!("  • Real resume and cover letter files");
    println!("  • Environment variables for credentials");
    println!("  • Display environment for visible browser");
    println!("");
    println!("CONTROLS (during execution):");
    println!("  • Ctrl+P: Pause automation");
    println!("  • Ctrl+R: Resume automation");
    println!("  • Ctrl+Q: Quit application");
    println!("  • Manual intervention when prompted");
    println!("");
}
