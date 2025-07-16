use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::job_listing::JobListing;
use crate::intelligent_scoring::JobScorer;
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

/// Search for jobs based on configuration with enhanced global support and intelligent scoring
pub async fn search_jobs(config: &Config) -> Result<Vec<JobListing>> {
    let mut job_listings = Vec::new();
    info!("Starting enhanced global job search with intelligent scoring");

    // Initialize the intelligent job scorer
    let job_scorer = JobScorer::new(config);

    // Generate search queries based on criteria including international terms
    let search_queries = generate_enhanced_search_queries(config);
    debug!("Generated {} enhanced search queries", search_queries.len());

    // Include regional job boards and search terms
    let locale = config.get_effective_locale();
    let region = crate::globalization::extract_region_from_locale(&locale);
    let global_config = crate::globalization::GlobalConfig::default();
    let regional_terms = crate::globalization::get_regional_search_terms(&region, &global_config);
    
    info!("Using regional search terms for {}: {:?}", region, regional_terms);

    // Add international job categories if enabled
    if config.enable_english_teaching_jobs || config.enable_cross_cultural_jobs || config.north_american_abroad_jobs {
        let international_categories = crate::globalization::get_international_job_categories();
        let na_abroad_categories = crate::globalization::get_north_american_abroad_categories();
        
        info!("Including {} international job categories", international_categories.len());
        info!("Including {} North American abroad categories", na_abroad_categories.len());
    }

    // In production mode, simulate finding relevant jobs with enhanced criteria
    for (i, query) in search_queries.iter().enumerate().take(8) { // Increased from 5 to 8 for wider coverage
        info!("Processing enhanced query {}: {}", i + 1, query);
        
        // Generate realistic job listings based on configuration
        let mut mock_jobs = generate_enhanced_mock_jobs_for_query(query, config);
        
        // Apply intelligent scoring to each job
        let mut scored_jobs = Vec::new();
        for mut job in mock_jobs {
            // Apply regional enhancements to job listings
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
            
            // Calculate intelligent score for each job
            let job_score = job_scorer.calculate_job_score(&job, config);
            
            // Only include jobs above a minimum threshold
            if job_score > 0.3 {
                scored_jobs.push((job, job_score));
            }
        }
        
        // Sort by score (highest first) and take top jobs
        scored_jobs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Log top scoring jobs
        for (job, score) in scored_jobs.iter().take(3) {
            info!("High scoring job: '{}' at {} (score: {:.3})", job.title, job.company, score);
        }
        
        // Add scored jobs to the final list
        job_listings.extend(scored_jobs.into_iter().map(|(job, _)| job));
        
        // Simulate processing delay
        tokio::time::sleep(Duration::from_millis(800)).await;
    }

    // Remove duplicates
    job_listings.sort_by(|a, b| a.url.cmp(&b.url));
    job_listings.dedup_by(|a, b| a.url == b.url);

    // Final scoring and sorting
    let mut final_scored_jobs = Vec::new();
    for job in job_listings {
        let score = job_scorer.calculate_job_score(&job, config);
        final_scored_jobs.push((job, score));
    }
    
    final_scored_jobs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let final_jobs: Vec<JobListing> = final_scored_jobs.into_iter().map(|(job, _)| job).collect();

    info!("Total unique jobs found: {} (intelligently scored and sorted)", final_jobs.len());
    Ok(final_jobs)
}

fn generate_enhanced_mock_jobs_for_query(query: &str, config: &Config) -> Vec<JobListing> {
    let mut jobs = Vec::new();
    let query_lower = query.to_lowercase();
    
    // Enhanced job generation based on new capabilities
    
    // 1. Manufacturing and Engineering Jobs
    if query_lower.contains("process") || query_lower.contains("manufacturing") || query_lower.contains("engineering") {
        jobs.push(JobListing {
            title: "Senior Process Engineer".to_string(),
            company: "Beckwood Press".to_string(),
            location: "Fenton, MO".to_string(),
            url: "https://beckwoodpress.com/careers/process-engineer".to_string(),
            description: "Lead process optimization initiatives for metal forming equipment. Experience with hydraulic press systems and automation preferred.".to_string(),
            salary: Some("$85,000 - $120,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
        
        jobs.push(JobListing {
            title: "Manufacturing Engineer".to_string(),
            company: "Keyful Plastic".to_string(),
            location: "Remote".to_string(),
            url: "https://keyfulplastic.com/careers/manufacturing-engineer".to_string(),
            description: "Design and optimize thermal forming processes for plastic manufacturing. CAD experience required.".to_string(),
            salary: Some("$75,000 - $105,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
    }
    
    // 2. Automation and Robotics
    if query_lower.contains("automation") || query_lower.contains("robot") || query_lower.contains("control") {
        jobs.push(JobListing {
            title: "Automation Engineer".to_string(),
            company: "Acid Alloy Robot Technologies".to_string(),
            location: "Portland, OR".to_string(),
            url: "https://acidalloy.com/careers/automation".to_string(),
            description: "Develop robotic automation solutions for industrial applications. PLC programming and safety systems experience required.".to_string(),
            salary: Some("$90,000 - $130,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
        
        jobs.push(JobListing {
            title: "Safety Systems Engineer".to_string(),
            company: "Kence Light Curtain Systems".to_string(),
            location: "Beaverton, OR".to_string(),
            url: "https://kence.com/careers/safety-engineer".to_string(),
            description: "Design and implement industrial safety systems. Experience with light curtains and safety PLCs preferred.".to_string(),
            salary: Some("$82,000 - $115,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
    }
    
    // 3. International/English Teaching Jobs
    if config.enable_english_teaching_jobs {
        jobs.push(JobListing {
            title: "English Conversation Instructor".to_string(),
            company: "International English Academy".to_string(),
            location: "Tokyo, Japan".to_string(),
            url: "https://english-academy.jp/careers/instructor".to_string(),
            description: "Teach English conversation to Japanese business professionals. Native English speaker required.".to_string(),
            salary: Some("¥3,500,000 - ¥4,200,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
        
        jobs.push(JobListing {
            title: "Corporate English Trainer".to_string(),
            company: "Global Language Solutions".to_string(),
            location: "Mumbai, India".to_string(),
            url: "https://globallang.in/careers/trainer".to_string(),
            description: "Provide English language training to corporate clients. Business English and cross-cultural communication experience preferred.".to_string(),
            salary: Some("₹800,000 - ₹1,200,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
    }
    
    // 4. Cross-Cultural and International Business
    if config.enable_cross_cultural_jobs {
        jobs.push(JobListing {
            title: "International Business Development Manager".to_string(),
            company: "Global Manufacturing Solutions".to_string(),
            location: "Remote - Asia Pacific".to_string(),
            url: "https://globalmfg.com/careers/business-dev".to_string(),
            description: "Develop business relationships across Asia-Pacific region. Manufacturing industry experience and cultural adaptability required.".to_string(),
            salary: Some("$95,000 - $140,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
        
        jobs.push(JobListing {
            title: "Cultural Liaison - Manufacturing".to_string(),
            company: "International Manufacturing Corp".to_string(),
            location: "Seoul, South Korea".to_string(),
            url: "https://intlmfg.com/careers/cultural-liaison".to_string(),
            description: "Bridge cultural gaps in manufacturing operations. Korean language skills preferred but not required.".to_string(),
            salary: Some("$88,000 - $125,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
    }
    
    // 5. North American Abroad Opportunities
    if config.north_american_abroad_jobs {
        jobs.push(JobListing {
            title: "North American Market Representative".to_string(),
            company: "European Manufacturing Alliance".to_string(),
            location: "Berlin, Germany".to_string(),
            url: "https://euromfg.de/careers/na-rep".to_string(),
            description: "Represent North American business interests in European manufacturing markets. Native English speaker with business experience required.".to_string(),
            salary: Some("€65,000 - €85,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
        
        jobs.push(JobListing {
            title: "Western Technology Consultant".to_string(),
            company: "Asia Tech Solutions".to_string(),
            location: "Singapore".to_string(),
            url: "https://asiatech.sg/careers/consultant".to_string(),
            description: "Provide expertise on Western manufacturing technologies and processes. Strong technical background and cultural adaptability required.".to_string(),
            salary: Some("S$90,000 - S$120,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
    }
    
    // 6. Standard manufacturing and engineering roles
    jobs.extend(generate_standard_jobs_for_query(query, config));
    
    jobs
}

fn generate_standard_jobs_for_query(query: &str, config: &Config) -> Vec<JobListing> {
    let mut jobs = Vec::new();
    let query_lower = query.to_lowercase();
    
    // Generate 2-3 standard jobs per query that match the criteria
    for i in 1..=2 {
        let title = if query_lower.contains("process") {
            format!("Process Engineer {}", i)
        } else if query_lower.contains("automation") {
            format!("Automation Specialist {}", i)
        } else if query_lower.contains("technical") {
            format!("Technical Support Engineer {}", i)
        } else if query_lower.contains("operations") {
            format!("Operations Manager {}", i)
        } else if query_lower.contains("quality") {
            format!("Quality Analyst {}", i)
        } else {
            format!("Manufacturing Engineer {}", i)
        };
        
        let company = if query_lower.contains("nike") {
            "Nike"
        } else if query_lower.contains("intel") {
            "Intel"
        } else if query_lower.contains("adidas") {
            "adidas"
        } else {
            match i {
                1 => "Advanced Manufacturing Corp",
                2 => "Industrial Solutions Inc",
                _ => "Global Engineering Systems",
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
                         company.to_lowercase().replace(' ', "").replace('.', ""), 
                         i + (query.len() % 1000));
        
        jobs.push(JobListing {
            title,
            company: company.to_string(),
            location,
            url,
            description: format!("Exciting opportunity in {} for a motivated professional with manufacturing and engineering experience", query),
            salary: Some(format!("${}-${}", 
                              config.min_salary.unwrap_or(60000), 
                              config.max_salary.unwrap_or(120000))),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        });
    }
    
    jobs
}

fn generate_mock_jobs_for_query(query: &str, config: &Config) -> Vec<JobListing> {
    // Legacy function - redirect to enhanced version
    generate_enhanced_mock_jobs_for_query(query, config)
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

/// Enhanced search query generation with international support and specialized categories
fn generate_enhanced_search_queries(config: &Config) -> Vec<String> {
    let mut queries = Vec::new();

    // Get enhanced keywords including synonyms and international categories
    let enhanced_keywords = config.get_enhanced_job_keywords();
    
    // Get regional search terms
    let locale = config.get_effective_locale();
    let region = crate::globalization::extract_region_from_locale(&locale);
    let global_config = crate::globalization::GlobalConfig::default();
    let regional_terms = crate::globalization::get_regional_search_terms(&region, &global_config);

    // 1. Enhanced keyword + location combinations
    for keyword in &enhanced_keywords {
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

    // 2. Company-specific queries including new manufacturing companies
    let all_companies = config.get_all_target_companies();
    for company in &all_companies {
        for location in &config.search_locations {
            let query = format!("{} careers {}", company, location);
            queries.push(query);
            
            // Add international variations
            let intl_query = format!("{} international careers {}", company, location);
            queries.push(intl_query);
        }
    }

    // 3. Manufacturing-specific queries
    for company in &config.manufacturing_companies {
        queries.push(format!("{} manufacturing engineer jobs", company));
        queries.push(format!("{} process engineer careers", company));
        queries.push(format!("{} automation specialist positions", company));
    }

    // 4. Industry-specific queries with regional enhancements
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

    // 5. International and cross-cultural job queries
    if config.should_enable_international_search() {
        let international_job_boards = config.get_all_job_boards();
        for board in &international_job_boards {
            if board.contains("international") || !board.contains(".com") {
                for keyword in &enhanced_keywords {
                    let intl_query = format!("\"{}\" remote international", keyword);
                    queries.push(intl_query);
                }
            }
        }
    }

    // 6. English teaching and ESL queries
    if config.enable_english_teaching_jobs {
        let teaching_locations = vec!["Japan", "South Korea", "China", "Thailand", "Vietnam", "India"];
        for location in &teaching_locations {
            queries.push(format!("English Teacher jobs {}", location));
            queries.push(format!("ESL Instructor {}", location));
            queries.push(format!("Native English Speaker {}", location));
            queries.push(format!("English Conversation Teacher {}", location));
        }
    }

    // 7. Cross-cultural and international business queries  
    if config.enable_cross_cultural_jobs {
        queries.push("International Business Development Engineer".to_string());
        queries.push("Cross-cultural Communication Specialist".to_string());
        queries.push("Global Operations Manager".to_string());
        queries.push("Cultural Liaison Manufacturing".to_string());
        queries.push("International Project Manager".to_string());
        queries.push("Global Supply Chain Engineer".to_string());
    }

    // 8. North American abroad opportunities
    if config.north_american_abroad_jobs {
        let abroad_locations = vec!["Europe", "Asia", "Germany", "Japan", "Singapore", "Australia"];
        for location in &abroad_locations {
            queries.push(format!("North American Manufacturing Engineer {}", location));
            queries.push(format!("American Business Representative {}", location));
            queries.push(format!("Western Technology Consultant {}", location));
            queries.push(format!("Native English Speaker Engineering {}", location));
        }
    }

    // 9. Specialized manufacturing equipment queries
    let specialized_equipment = vec![
        "hydraulic press", "metal forming", "plastic injection", "thermal forming",
        "robotics automation", "safety systems", "light curtain", "PLC programming"
    ];
    
    for equipment in &specialized_equipment {
        queries.push(format!("{} engineer jobs", equipment));
        queries.push(format!("{} specialist positions", equipment));
    }

    // 10. Skill expansion queries (slightly outside normal skillset)
    if config.skill_expansion_factor > 1.0 {
        let expanded_skills = vec![
            "Quality Assurance Engineer", "Project Coordinator", "Technical Writer",
            "Business Process Analyst", "Operations Coordinator", "Supply Chain Analyst",
            "Manufacturing Coordinator", "Process Improvement Specialist"
        ];
        
        for skill in &expanded_skills {
            queries.push(format!("{} jobs remote", skill));
            queries.push(format!("{} positions international", skill));
        }
    }

    // Remove duplicates and limit to reasonable number
    queries.sort();
    queries.dedup();
    
    // Prioritize queries - take top 50 most relevant
    queries.truncate(50);
    
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
