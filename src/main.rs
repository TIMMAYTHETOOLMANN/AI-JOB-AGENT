use log::{info, warn};
use serde::Deserialize;
use anyhow::Result;
use config::Config;
use std::time::Duration;

mod job_search_engine;
mod job_application;

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

async fn search_jobs(config: &AgentConfig) -> Result<(usize, usize)> {
    info!("Searching jobs for: {:?}", config.job_keywords);

    let mut total_jobs_found = 0;
    let mut total_jobs_applied = 0;

    // Standard job board search (existing functionality)
    info!("Searching standard job boards: {:?}", config.targets);
    // Insert existing job search logic here

    // New: Search engine reconnaissance for company-specific jobs
    if config.company_targets.is_empty() {
        info!("No company targets specified for search engine reconnaissance. Skipping this step.");
    } else {
        info!("Starting search engine reconnaissance for companies: {:?}", config.company_targets);

        for company in &config.company_targets {
            info!("Searching for jobs at company: {}", company);
            match job_search_engine::search_jobs_via_engine(
                company,
                &config.job_keywords,
                &config.search_locations,
                &config.search_engines
            ).await {
                Ok(urls) => {
                    if urls.is_empty() {
                        info!("No job URLs found for company {}", company);
                    } else {
                        let jobs_found = urls.len();
                        total_jobs_found += jobs_found;
                        info!("Found {} potential job URLs for company {}", jobs_found, company);

                        // Process the found URLs and apply to jobs
                        info!("Processing job URLs for company {} and applying to matching jobs", company);
                        match job_search_engine::process_search_engine_results(
                            &urls,
                            &config.company_targets,
                            &config.job_keywords,
                            &config.name,
                            &config.email,
                            &config.resume_path,
                            &config.cover_letter_path
                        ).await {
                            Ok(applied_count) => {
                                total_jobs_applied += applied_count;
                                info!("Successfully processed search results for {}. Applied to {} jobs", company, applied_count);
                            },
                            Err(e) => warn!("Error processing search results for {}: {}", company, e),
                        }
                    }
                },
                Err(e) => warn!("Error during search engine reconnaissance for {}: {}", company, e),
            }

            // Add a small delay between companies to avoid overwhelming resources
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    info!("Job search and application cycle completed. Found {} jobs, applied to {} jobs", total_jobs_found, total_jobs_applied);
    Ok((total_jobs_found, total_jobs_applied))
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

    // Run the job search process continuously
    info!("Starting continuous job search process...");

    let mut iteration = 1;
    let mut total_jobs_found = 0;
    let mut total_jobs_applied = 0;

    loop {
        info!("Starting job search and application iteration {}", iteration);

        match search_jobs(&config).await {
            Ok((jobs_found, jobs_applied)) => {
                total_jobs_found += jobs_found;
                total_jobs_applied += jobs_applied;
                info!("Job search iteration {} completed successfully. Found {} jobs, applied to {} jobs in this iteration.", 
                      iteration, jobs_found, jobs_applied);
                info!("Total stats: Found {} jobs, applied to {} jobs across {} iterations", 
                      total_jobs_found, total_jobs_applied, iteration);
            },
            Err(e) => {
                warn!("Error during job search iteration {}: {}", iteration, e);
                // Continue running despite errors
            }
        }

        // Wait before starting the next iteration (5 minutes)
        let wait_time = Duration::from_secs(300);
        info!("Waiting for {} seconds before starting next job search and application iteration...", wait_time.as_secs());
        tokio::time::sleep(wait_time).await;

        iteration += 1;
    }
}
