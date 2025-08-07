use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, error, warn};
use tokio::time::{sleep, Duration};

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobListing {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub url: String,
    pub description: String,
    pub salary: Option<String>,
    pub job_board: String,
    pub posted_date: String,
}

pub struct JobScanner {
    config: Config,
}

impl JobScanner {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Fetch jobs from all configured job boards
    pub async fn fetch_jobs(&self) -> Result<Vec<JobListing>> {
        info!("🔍 Starting job scan across {} job boards", self.config.job_boards.len());
        
        let mut all_jobs = Vec::new();
        
        for board in &self.config.job_boards {
            info!("📋 Scanning {} for opportunities", board);
            
            match board.as_str() {
                "linkedin" => {
                    match self.scan_linkedin().await {
                        Ok(mut jobs) => {
                            info!("✅ LinkedIn: Found {} jobs", jobs.len());
                            all_jobs.append(&mut jobs);
                        }
                        Err(e) => {
                            error!("❌ LinkedIn scan failed: {}", e);
                            warn!("⚠️  Continuing with other job boards");
                        }
                    }
                }
                "indeed" => {
                    match self.scan_indeed().await {
                        Ok(mut jobs) => {
                            info!("✅ Indeed: Found {} jobs", jobs.len());
                            all_jobs.append(&mut jobs);
                        }
                        Err(e) => {
                            error!("❌ Indeed scan failed: {}", e);
                            warn!("⚠️  Continuing with other job boards");
                        }
                    }
                }
                "monster" => {
                    match self.scan_monster().await {
                        Ok(mut jobs) => {
                            info!("✅ Monster: Found {} jobs", jobs.len());
                            all_jobs.append(&mut jobs);
                        }
                        Err(e) => {
                            error!("❌ Monster scan failed: {}", e);
                            warn!("⚠️  Continuing with other job boards");
                        }
                    }
                }
                "weworkremotely" => {
                    match self.scan_wework_remotely().await {
                        Ok(mut jobs) => {
                            info!("✅ WeWorkRemotely: Found {} jobs", jobs.len());
                            all_jobs.append(&mut jobs);
                        }
                        Err(e) => {
                            error!("❌ WeWorkRemotely scan failed: {}", e);
                            warn!("⚠️  Continuing with other job boards");
                        }
                    }
                }
                board_name => {
                    warn!("⚠️  Unknown job board: {}", board_name);
                }
            }

            // Rate limiting between job board requests
            sleep(Duration::from_millis(2000)).await;
        }

        info!("✅ Job scan complete: {} total opportunities found", all_jobs.len());
        self.log_scan_summary(&all_jobs);
        
        Ok(all_jobs)
    }

    async fn scan_linkedin(&self) -> Result<Vec<JobListing>> {
        let keywords = self.config.keywords.join(" OR ");
        let locations = self.config.locations.join(",");
        
        // LinkedIn API would be used here in production
        // For now, simulate job discovery
        let jobs = vec![
            JobListing {
                id: format!("linkedin_{}", uuid::Uuid::new_v4()),
                title: "Senior Software Engineer".to_string(),
                company: "TechCorp Inc".to_string(),
                location: "Remote".to_string(),
                url: "https://www.linkedin.com/jobs/view/123456789".to_string(),
                description: "Looking for experienced software engineer with AI/ML background".to_string(),
                salary: Some("$120k-150k".to_string()),
                job_board: "linkedin".to_string(),
                posted_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            }
        ];

        Ok(jobs)
    }

    async fn scan_indeed(&self) -> Result<Vec<JobListing>> {
        let keywords = self.config.keywords.join(",");
        
        // Indeed API/scraping would be used here
        let jobs = vec![
            JobListing {
                id: format!("indeed_{}", uuid::Uuid::new_v4()),
                title: "Machine Learning Engineer".to_string(),
                company: "DataTech Solutions".to_string(),
                location: "San Francisco, CA".to_string(),
                url: "https://www.indeed.com/viewjob?jk=abcd1234".to_string(),
                description: "Remote ML engineer position focusing on automation".to_string(),
                salary: Some("$130k-160k".to_string()),
                job_board: "indeed".to_string(),
                posted_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            }
        ];

        Ok(jobs)
    }

    async fn scan_monster(&self) -> Result<Vec<JobListing>> {
        // Monster API/scraping implementation
        let jobs = vec![
            JobListing {
                id: format!("monster_{}", uuid::Uuid::new_v4()),
                title: "DevOps Engineer".to_string(),
                company: "CloudFirst Inc".to_string(),
                location: "Remote".to_string(),
                url: "https://www.monster.com/jobs/123456".to_string(),
                description: "DevOps engineer with automation experience".to_string(),
                salary: Some("$110k-140k".to_string()),
                job_board: "monster".to_string(),
                posted_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            }
        ];

        Ok(jobs)
    }

    async fn scan_wework_remotely(&self) -> Result<Vec<JobListing>> {
        // WeWorkRemotely API/scraping implementation
        let jobs = vec![
            JobListing {
                id: format!("wework_{}", uuid::Uuid::new_v4()),
                title: "Full Stack Developer".to_string(),
                company: "RemoteFirst LLC".to_string(),
                location: "Worldwide".to_string(),
                url: "https://weworkremotely.com/remote-jobs/123456".to_string(),
                description: "Full stack developer for global remote team".to_string(),
                salary: Some("$100k-130k".to_string()),
                job_board: "weworkremotely".to_string(),
                posted_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            }
        ];

        Ok(jobs)
    }

    fn log_scan_summary(&self, jobs: &[JobListing]) {
        let mut board_counts: HashMap<String, usize> = HashMap::new();
        
        for job in jobs {
            *board_counts.entry(job.job_board.clone()).or_insert(0) += 1;
        }

        info!("📊 Scan Summary:");
        for (board, count) in board_counts {
            info!("   📋 {}: {} jobs", board, count);
        }
    }
}

pub fn filter_jobs(jobs: Vec<JobListing>, config: &Config) -> Vec<JobListing> {
    info!("🔍 Filtering {} jobs based on criteria", jobs.len());
    
    let filtered: Vec<JobListing> = jobs.into_iter()
        .filter(|job| {
            // Check keywords
            let title_lower = job.title.to_lowercase();
            let desc_lower = job.description.to_lowercase();
            
            let has_keyword = config.keywords.iter().any(|keyword| {
                let keyword_lower = keyword.to_lowercase();
                title_lower.contains(&keyword_lower) || desc_lower.contains(&keyword_lower)
            });

            if !has_keyword {
                return false;
            }

            // Check remote preference
            if config.remote_only {
                let location_lower = job.location.to_lowercase();
                if !location_lower.contains("remote") && !location_lower.contains("worldwide") {
                    return false;
                }
            }

            // Check salary requirements if available
            if let Some(ref salary) = job.salary {
                if let Some(min_salary) = config.min_salary {
                    // Simple salary parsing - in production would be more sophisticated
                    if let Some(extracted_salary) = extract_salary_number(salary) {
                        if extracted_salary < min_salary {
                            return false;
                        }
                    }
                }
            }

            true
        })
        .collect();

    info!("✅ Filtered to {} matching jobs", filtered.len());
    filtered
}

fn extract_salary_number(salary_str: &str) -> Option<u32> {
    // Simple salary extraction - finds first number in salary string
    let re = regex::Regex::new(r"\$?(\d+),?(\d+)?k?").ok()?;
    let caps = re.captures(salary_str)?;
    
    if let Some(num_str) = caps.get(1) {
        if let Ok(num) = num_str.as_str().parse::<u32>() {
            // Handle 'k' suffix
            if salary_str.contains('k') || salary_str.contains('K') {
                return Some(num * 1000);
            }
            return Some(num);
        }
    }
    
    None
}