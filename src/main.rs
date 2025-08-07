use std::time::Duration;
use anyhow::Result;
use tracing::{info, error, warn};
use tracing_subscriber::EnvFilter;

mod config;
mod scanner;
mod applier;
mod logger;

use config::Config;
use scanner::{JobScanner, filter_jobs};
use applier::JobApplier;
use logger::Logger;

#[tokio::main]
async fn main() {
    // Initialize logging system
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stdout)
        .init();

    println!("🤖 AI JOB AGENT - AUTONOMOUS MODE");
    println!("═══════════════════════════════════════");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage();
        return;
    }

    // Run autonomous job agent
    match run_autonomous_agent().await {
        Ok(_) => {
            info!("✅ AI Job Agent completed successfully");
            println!("✅ Mission accomplished. System shutdown complete.");
        },
        Err(e) => {
            error!("❌ AI Job Agent critical failure: {}", e);
            eprintln!("💥 SYSTEM FAILURE: {}", e);
            std::process::exit(1);
        }
    }
}

/// Run autonomous AI job application agent
async fn run_autonomous_agent() -> Result<()> {
    info!("🚀 Starting autonomous AI job application agent");

    // STEP 1: Validate environment and configuration
    println!("🔍 Validating environment configuration...");
    validate_environment()?;
    println!("✅ Environment validation complete");

    // STEP 2: Load configuration
    println!("📋 Loading agent configuration...");
    let config = Config::load("AgentConfig.toml")?;
    info!("✅ Configuration loaded: {} job boards, {} keywords", 
          config.job_boards.len(), config.keywords.len());

    // STEP 3: Initialize logger
    println!("📝 Initializing logging system...");
    let logger = Logger::new()?;
    logger.log_startup()?;
    println!("✅ Logging system ready");

    // STEP 4: Initialize job scanner
    println!("🔍 Initializing job scanner...");
    let scanner = JobScanner::new(config.clone());
    println!("✅ Job scanner ready for {} boards", config.job_boards.len());

    // STEP 5: Initialize job applier
    println!("📝 Initializing job applier...");
    let mut applier = JobApplier::new(config.clone())?;
    println!("✅ Job applier ready");

    // STEP 6: Start autonomous main loop
    println!("🤖 Starting autonomous job application loop...");
    println!("═══════════════════════════════════════");
    println!("🎯 Polling interval: {} seconds", config.polling_interval);
    println!("🔍 Target keywords: {:?}", config.keywords);
    println!("📍 Target locations: {:?}", config.locations);
    println!("💰 Minimum salary: ${}", config.min_salary.unwrap_or(0));
    println!("🌍 Remote only: {}", config.remote_only);
    println!("═══════════════════════════════════════");

    let mut cycle_count = 0;

    loop {
        cycle_count += 1;
        info!("🔄 Starting automation cycle #{}", cycle_count);
        println!("\n🔄 === AUTOMATION CYCLE #{} ===", cycle_count);

        // Check for early exit conditions
        if should_exit() {
            break;
        }

        // MAIN AUTOMATION LOGIC
        match run_job_cycle(&scanner, &mut applier, &logger, &config).await {
            Ok(_) => {
                info!("✅ Cycle #{} completed successfully", cycle_count);
            }
            Err(e) => {
                error!("❌ Cycle #{} failed: {}", cycle_count, e);
                logger.log_error("cycle_failure", &e.to_string())?;
                
                // Continue to next cycle on failure
                warn!("⚠️  Continuing to next cycle after error");
            }
        }

        // Rate limiting between cycles
        info!("⏰ Waiting {} seconds before next cycle...", config.polling_interval);
        println!("⏰ Waiting {} seconds before next cycle...", config.polling_interval);
        tokio::time::sleep(Duration::from_secs(config.polling_interval as u64)).await;
    }

    // SHUTDOWN SEQUENCE
    println!("🔴 Initiating shutdown sequence...");
    logger.log_shutdown()?;
    logger.print_daily_summary()?;

    Ok(())
}

/// Run a single job search and application cycle
async fn run_job_cycle(
    scanner: &JobScanner, 
    applier: &mut JobApplier, 
    logger: &Logger, 
    config: &Config
) -> Result<()> {
    // STEP 1: Fetch jobs from all job boards
    let jobs = scanner.fetch_jobs().await?;
    
    if jobs.is_empty() {
        info!("📭 No jobs found this cycle");
        println!("📭 No jobs found this cycle");
        return Ok(());
    }

    // STEP 2: Filter jobs based on criteria
    let filtered_jobs = filter_jobs(jobs.clone(), config);
    logger.log_job_search(jobs.len(), filtered_jobs.len())?;

    if filtered_jobs.is_empty() {
        info!("🚫 No jobs matched filter criteria");
        println!("🚫 No jobs matched filter criteria");
        return Ok(());
    }

    info!("🎯 Found {} matching jobs, proceeding with applications", filtered_jobs.len());
    println!("🎯 Found {} matching jobs, proceeding with applications", filtered_jobs.len());

    // STEP 3: Apply to filtered jobs
    let results = applier.apply_to_jobs(filtered_jobs).await?;

    // STEP 4: Log all results
    logger.log_applications(&results)?;

    // STEP 5: Print cycle summary
    let success_count = results.iter().filter(|r| r.success).count();
    let total_count = results.len();
    
    println!("📊 Cycle Summary:");
    println!("   🔍 Jobs found: {}", jobs.len());
    println!("   ✅ Jobs matched: {}", total_count);
    println!("   📝 Applications sent: {}", success_count);
    println!("   ❌ Applications failed: {}", total_count - success_count);

    Ok(())
}

/// Validate environment configuration and required files
fn validate_environment() -> Result<()> {
    info!("🔍 Validating environment configuration");

    // Check for required environment variables
    let required_vars = [
        "LINKEDIN_EMAIL",
        "LINKEDIN_PASSWORD", 
        "DEFAULT_NAME",
        "DEFAULT_EMAIL"
    ];

    let mut missing_vars = Vec::new();
    
    for var in &required_vars {
        if std::env::var(var).is_err() {
            missing_vars.push(*var);
        }
    }

    if !missing_vars.is_empty() {
        let missing_str = missing_vars.join(", ");
        return Err(anyhow::anyhow!("Missing environment variables: {}", missing_str));
    }

    // Check if resume file exists
    let resume_path = std::env::var("RESUME_PATH")
        .unwrap_or_else(|_| "./assets/resume.pdf".to_string());
    
    if !std::path::Path::new(&resume_path).exists() {
        return Err(anyhow::anyhow!("Resume file not found: {}", resume_path));
    }

    info!("✅ Environment validation passed");
    Ok(())
}

/// Check if the system should exit (for graceful shutdown)
fn should_exit() -> bool {
    // In production, this could check for signal files, user input, etc.
    // For now, just return false to run continuously
    false
}

/// Print usage information
fn print_usage() {
    println!("🤖 AI JOB AGENT - Autonomous Job Application System");
    println!("═══════════════════════════════════════════════════");
    println!("USAGE: cargo run --release");
    println!("   OR: docker-compose up");
    println!("");
    println!("FEATURES:");
    println!("  • Fully autonomous job search and application");
    println!("  • Multi-job-board support (LinkedIn, Indeed, Monster, WeWorkRemotely)");
    println!("  • Intelligent job filtering based on criteria");
    println!("  • Automatic form filling and resume upload");
    println!("  • Comprehensive logging with screenshots");
    println!("  • Retry logic and failure handling");
    println!("  • Configurable via .env and AgentConfig.toml");
    println!("");
    println!("REQUIREMENTS:");
    println!("  • .env file with credentials (copy from .env.template)");
    println!("  • AgentConfig.toml with job search criteria");
    println!("  • Resume PDF file in ./assets/ directory");
    println!("  • Cover letter template in ./templates/ directory");
    println!("");
    println!("OPERATION:");
    println!("  • Scans job boards every {} seconds", 1800);
    println!("  • Applies to matching jobs automatically");
    println!("  • Pauses only for CAPTCHA or missing config");
    println!("  • Logs all activity to ./logs/ directory");
    println!("  • Saves screenshots of applications");
    println!("");
    println!("Use Ctrl+C to stop the agent.");
    println!("");
}
