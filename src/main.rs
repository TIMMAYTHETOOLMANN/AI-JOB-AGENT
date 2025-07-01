use log::{info, warn};
use serde::Deserialize;
use anyhow::Result;
use config::Config;

mod job_search_engine;

#[derive(Debug, Deserialize)]
struct AgentConfig {
    name: String,
    email: String,
    location: Vec<String>,
    open_to_relocation: bool,
    job_keywords: Vec<String>,
    skills: Vec<String>,
    industries: Vec<String>,
    seniority_levels: Vec<String>,
    min_salary: u32,
    max_salary: u32,
    targets: Vec<String>,
    resume_path: String,
    cover_letter_path: String,
    // Search engine reconnaissance configuration
    search_engines: Vec<String>,
    company_targets: Vec<String>,
    search_locations: Vec<String>,
}

async fn search_jobs(config: &AgentConfig) -> Result<()> {
    info!("Searching jobs for: {:?}", config.job_keywords);

    // Standard job board search (existing functionality)
    info!("Searching standard job boards: {:?}", config.targets);
    // Insert existing job search logic here

    // New: Search engine reconnaissance for company-specific jobs
    if config.company_targets.is_empty() {
        info!("No company targets specified for search engine reconnaissance. Skipping this step.");
    } else {
        info!("Starting search engine reconnaissance for companies: {:?}", config.company_targets);

        for company in &config.company_targets {
            match job_search_engine::search_jobs_via_engine(
                company,
                &config.job_keywords,
                &config.search_locations,
                &config.search_engines
            ).await {
                Ok(urls) => {
                    info!("Found {} potential job URLs for company {}", urls.len(), company);

                    // Process the found URLs
                    if !urls.is_empty() {
                        match job_search_engine::process_search_engine_results(
                            &urls,
                            &config.company_targets,
                            &config.job_keywords
                        ).await {
                            Ok(_) => info!("Successfully processed search results for {}", company),
                            Err(e) => warn!("Error processing search results for {}: {}", company, e),
                        }
                    }
                },
                Err(e) => warn!("Error during search engine reconnaissance for {}: {}", company, e),
            }
        }
    }

    info!("Job search completed");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger with a more verbose configuration
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Determine if we're running in debug or release mode
    let mode = if cfg!(debug_assertions) {
        "DEVELOPMENT"
    } else {
        "PRODUCTION"
    };

    info!("Starting Autonomous Job Agent in {} mode...", mode);

    if cfg!(debug_assertions) {
        warn!("Running in DEVELOPMENT mode. For full URL detection, run in PRODUCTION mode with 'cargo run --release'");
    } else {
        info!("Running in PRODUCTION mode with optimized URL detection");
    }

    info!("Loading configuration from AgentConfig.toml...");
    let config: AgentConfig = match Config::builder()
        .add_source(config::File::with_name("AgentConfig"))
        .build() {
            Ok(config_builder) => {
                match config_builder.try_deserialize() {
                    Ok(config) => {
                        info!("Configuration loaded successfully");
                        config
                    },
                    Err(e) => {
                        eprintln!("Failed to deserialize configuration: {}", e);
                        return Err(anyhow::anyhow!("Configuration deserialization error: {}", e));
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to build configuration: {}", e);
                return Err(anyhow::anyhow!("Configuration build error: {}", e));
            }
        };

    info!("Loaded config: {:?}", config);

    info!("Starting job search process...");
    match search_jobs(&config).await {
        Ok(_) => info!("Job search completed successfully"),
        Err(e) => eprintln!("Error during job search: {}", e),
    }

    info!("Autonomous Job Agent execution completed");
    Ok(())
}
