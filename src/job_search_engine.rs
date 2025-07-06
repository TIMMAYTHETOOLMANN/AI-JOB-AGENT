use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::job_listing::JobListing;
use anyhow::{Result, Context};
use tracing::{info, error, debug, warn};
use std::time::Duration;

/// Configuration for job search
#[derive(Debug, Serialize, Deserialize)]
pub struct JobSearchConfig {
    /// Search engines to use
    pub search_engines: Vec<String>,
    /// Target companies
    pub company_targets: Vec<String>,
    /// Search locations
    pub search_locations: Vec<String>,
    /// Interval between searches (in seconds)
    pub interval: u64,
}

/// Search for jobs based on configuration with enhanced global support
pub async fn search_jobs(config: &Config) -> Result<Vec<JobListing>> {
    let mut job_listings = Vec::new();
    info!("Starting enhanced global job search simulation");

    // Generate search queries based on criteria including international terms
    let search_queries = generate_enhanced_search_queries(config);
    debug!("Generated {} enhanced search queries", search_queries.len());

    // Include regional job boards and search terms
    let locale = config.get_effective_locale();
    let region = crate::globalization::extract_region_from_locale(&locale);
    let global_config = crate::globalization::GlobalConfig::default();
    let regional_terms = crate::globalization::get_regional_search_terms(&region, &global_config);
    
    info!("Using regional search terms for {}: {:?}", region, regional_terms);

    // In production mode, simulate finding relevant jobs with enhanced criteria
    for (i, query) in search_queries.iter().enumerate().take(5) {
        info!("Processing enhanced query {}: {}", i + 1, query);
        
        // Generate realistic job listings based on configuration
        let mut mock_jobs = generate_mock_jobs_for_query(query, config);
        
        // Apply regional enhancements to job listings
        for job in &mut mock_jobs {
            // Add regional salary adjustments
            if let Some(ref salary_str) = job.salary {
                if let Some(current_salary) = extract_salary_from_string(salary_str) {
                    let currency = config.get_effective_currency();
                    if currency != "USD" {
                        if let Ok(localized_salary) = crate::globalization::localize_salary(current_salary, "USD", &currency) {
                            job.salary = Some(format!("{}{}-{}{}", 
                                get_currency_symbol(&currency),
                                localized_salary * 80 / 100, // Min range
                                get_currency_symbol(&currency),
                                localized_salary * 120 / 100  // Max range
                            ));
                        }
                    }
                }
            }
        }
        
        job_listings.extend(mock_jobs);
        
        // Simulate processing delay
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }

    // Remove duplicates
    job_listings.sort_by(|a, b| a.url.cmp(&b.url));
    job_listings.dedup_by(|a, b| a.url == b.url);

    info!("Total unique jobs found: {}", job_listings.len());
    Ok(job_listings)
}

fn generate_mock_jobs_for_query(query: &str, config: &Config) -> Vec<JobListing> {
    let mut jobs = Vec::new();
    
    // Extract keywords from query for realistic job generation
    let query_lower = query.to_lowercase();
    
    // Generate 2-3 jobs per query that match the criteria
    for i in 1..=3 {
        let title = if query_lower.contains("process") {
            format!("Process Engineer {}", i)
        } else if query_lower.contains("automation") {
            format!("Automation Specialist {}", i)
        } else if query_lower.contains("technical") {
            format!("Technical Support Engineer {}", i)
        } else if query_lower.contains("operations") {
            format!("Operations Manager {}", i)
        } else {
            format!("Software Engineer {}", i)
        };
        
        let company = if query_lower.contains("nike") {
            "Nike"
        } else if query_lower.contains("intel") {
            "Intel"
        } else if query_lower.contains("adidas") {
            "adidas"
        } else {
            match i {
                1 => "TechCorp",
                2 => "InnovateCo",
                _ => "GlobalSystems",
            }
        };
        
        let location = if config.search_locations.is_empty() {
            "Remote".to_string()
        } else {
            config.search_locations.get(i % config.search_locations.len())
                .unwrap_or(&"Remote".to_string())
                .clone()
        };
        
        let url = format!("https://careers.{}.com/job/{}", 
                         company.to_lowercase().replace(' ', ""), 
                         i + (query.len() % 1000));
        
        jobs.push(JobListing {
            title,
            company: company.to_string(),
            location,
            url,
            description: format!("Exciting opportunity in {} for a motivated professional", query),
            salary: Some(format!("${}-${}", 
                              config.min_salary.unwrap_or(60000), 
                              config.max_salary.unwrap_or(120000))),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
    }
    
    jobs
}

fn generate_search_queries(config: &Config) -> Vec<String> {
    let mut queries = Vec::new();

    // Combine keywords, industries, and locations into search queries
    for keyword in &config.job_keywords {
        for location in &config.search_locations {
            let query = format!("\"{}\" jobs {}", keyword, location);
            queries.push(query);
        }
    }

    // Add company-specific queries
    for company in &config.company_targets {
        for location in &config.search_locations {
            let query = format!("{} careers {}", company, location);
            queries.push(query);
        }
    }

    // Add industry-specific queries
    for industry in &config.industries {
        for location in &config.search_locations {
            let query = format!("{} jobs {}", industry, location);
            queries.push(query);
        }
    }

    queries
}

/// Enhanced search query generation with international support
fn generate_enhanced_search_queries(config: &Config) -> Vec<String> {
    let mut queries = Vec::new();

    // Get regional search terms
    let locale = config.get_effective_locale();
    let region = crate::globalization::extract_region_from_locale(&locale);
    let global_config = crate::globalization::GlobalConfig::default();
    let regional_terms = crate::globalization::get_regional_search_terms(&region, &global_config);

    // Combine keywords, industries, and locations into search queries
    for keyword in &config.job_keywords {
        for location in &config.search_locations {
            let query = format!("\"{}\" jobs {}", keyword, location);
            queries.push(query);
            
            // Add regional variations
            for regional_term in &regional_terms {
                let regional_query = format!("\"{}\" {} jobs {}", keyword, regional_term, location);
                queries.push(regional_query);
            }
        }
    }

    // Add company-specific queries with international sites
    for company in &config.company_targets {
        for location in &config.search_locations {
            let query = format!("{} careers {}", company, location);
            queries.push(query);
            
            // Add international variations
            let intl_query = format!("{} international careers {}", company, location);
            queries.push(intl_query);
        }
    }

    // Add industry-specific queries with regional terms
    for industry in &config.industries {
        for location in &config.search_locations {
            let query = format!("{} jobs {}", industry, location);
            queries.push(query);
            
            // Add regional enhancements
            for regional_term in &regional_terms {
                let enhanced_query = format!("{} {} {} jobs", industry, regional_term, location);
                queries.push(enhanced_query);
            }
        }
    }

    // Add international job board specific queries
    if config.should_enable_international_search() {
        let international_job_boards = config.get_all_job_boards();
        for board in &international_job_boards {
            if board.contains("international") || !board.contains(".com") {
                for keyword in &config.job_keywords {
                    let intl_query = format!("\"{}\" remote international", keyword);
                    queries.push(intl_query);
                }
            }
        }
    }

    queries
}

// Helper functions for salary and currency handling
fn extract_salary_from_string(salary_str: &str) -> Option<u32> {
    // Simple regex-like extraction for salary numbers
    let numbers: Vec<u32> = salary_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .map(|n| vec![n])
        .unwrap_or_default();
    
    numbers.first().copied()
}

fn get_currency_symbol(currency: &str) -> &str {
    match currency {
        "USD" => "$",
        "EUR" => "€",
        "GBP" => "£",
        "CAD" => "C$",
        "AUD" => "A$",
        "JPY" => "¥",
        "INR" => "₹",
        _ => "$", // Default to USD symbol
    }
}
