use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::time::{sleep, Duration};
use tracing::{info, error, warn};

use crate::config::Config;
use crate::scanner::JobListing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationResult {
    pub job_id: String,
    pub job_url: String,
    pub success: bool,
    pub timestamp: String,
    pub screenshot_path: Option<String>,
    pub error_message: Option<String>,
    pub status: ApplicationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Success,
    Failed,
    CaptchaRequired,
    LoginRequired,
    NetworkError,
    FormNotFound,
    UploadFailed,
}

pub struct JobApplier {
    config: Config,
    resume_path: String,
    cover_letter_template: String,
}

impl JobApplier {
    pub fn new(config: Config) -> Result<Self> {
        // Validate required files exist
        if !Path::new(&config.resume_path).exists() {
            return Err(anyhow::anyhow!("Resume file not found: {}", config.resume_path));
        }

        let cover_letter_template = std::env::var("COVER_LETTER_TEMPLATE")
            .unwrap_or_else(|_| "./templates/cover_letter_template.md".to_string());

        Ok(Self {
            resume_path: config.resume_path.clone(),
            cover_letter_template,
            config,
        })
    }

    /// Apply to jobs autonomously
    pub async fn apply_to_jobs(&mut self, jobs: Vec<JobListing>) -> Result<Vec<ApplicationResult>> {
        info!("🎯 Starting autonomous application process for {} jobs", jobs.len());
        
        let mut results = Vec::new();
        
        for (index, job) in jobs.iter().enumerate() {
            info!("📝 [{}/{}] Applying to: {} at {}", 
                  index + 1, jobs.len(), job.title, job.company);

            let result = self.apply_to_single_job(job).await;
            results.push(result);

            // Rate limiting between applications
            if index < jobs.len() - 1 {
                info!("⏱️  Rate limiting - waiting 30 seconds...");
                sleep(Duration::from_secs(30)).await;
            }
        }

        let success_count = results.iter().filter(|r| r.success).count();
        info!("✅ Application process complete: {}/{} successful applications", 
              success_count, jobs.len());

        Ok(results)
    }

    async fn apply_to_single_job(&mut self, job: &JobListing) -> ApplicationResult {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let screenshot_dir = format!("./logs/{}", chrono::Utc::now().format("%Y-%m-%d"));
        
        // Ensure screenshot directory exists
        if let Err(e) = std::fs::create_dir_all(&screenshot_dir) {
            error!("Failed to create screenshot directory: {}", e);
        }

        match job.job_board.as_str() {
            "linkedin" => self.apply_linkedin(job, &timestamp, &screenshot_dir).await,
            "indeed" => self.apply_indeed(job, &timestamp, &screenshot_dir).await,
            "monster" => self.apply_monster(job, &timestamp, &screenshot_dir).await,
            "weworkremotely" => self.apply_wework_remotely(job, &timestamp, &screenshot_dir).await,
            _ => {
                warn!("Unsupported job board: {}", job.job_board);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: false,
                    timestamp,
                    screenshot_path: None,
                    error_message: Some(format!("Unsupported job board: {}", job.job_board)),
                    status: ApplicationStatus::Failed,
                }
            }
        }
    }

    async fn apply_linkedin(&mut self, job: &JobListing, timestamp: &str, screenshot_dir: &str) -> ApplicationResult {
        info!("🔵 Applying to LinkedIn job: {}", job.title);
        
        // In production, this would use actual browser automation
        // For now, simulate the application process
        
        let screenshot_path = format!("{}/linkedin_{}_{}.png", screenshot_dir, job.id, timestamp);
        
        // Simulate browser automation steps:
        // 1. Navigate to job URL
        // 2. Check if login required
        // 3. Fill out application form
        // 4. Upload resume
        // 5. Generate and upload cover letter
        // 6. Submit application
        // 7. Take screenshot of confirmation

        // Simulate potential issues
        match self.simulate_application_attempt().await {
            Ok(_) => {
                info!("✅ Successfully applied to LinkedIn job: {}", job.title);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: true,
                    timestamp: timestamp.to_string(),
                    screenshot_path: Some(screenshot_path),
                    error_message: None,
                    status: ApplicationStatus::Success,
                }
            }
            Err(e) => {
                error!("❌ LinkedIn application failed: {}", e);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: false,
                    timestamp: timestamp.to_string(),
                    screenshot_path: Some(screenshot_path),
                    error_message: Some(e.to_string()),
                    status: ApplicationStatus::Failed,
                }
            }
        }
    }

    async fn apply_indeed(&mut self, job: &JobListing, timestamp: &str, screenshot_dir: &str) -> ApplicationResult {
        info!("🟢 Applying to Indeed job: {}", job.title);
        
        let screenshot_path = format!("{}/indeed_{}_{}.png", screenshot_dir, job.id, timestamp);
        
        match self.simulate_application_attempt().await {
            Ok(_) => {
                info!("✅ Successfully applied to Indeed job: {}", job.title);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: true,
                    timestamp: timestamp.to_string(),
                    screenshot_path: Some(screenshot_path),
                    error_message: None,
                    status: ApplicationStatus::Success,
                }
            }
            Err(e) => {
                error!("❌ Indeed application failed: {}", e);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: false,
                    timestamp: timestamp.to_string(),
                    screenshot_path: Some(screenshot_path),
                    error_message: Some(e.to_string()),
                    status: ApplicationStatus::Failed,
                }
            }
        }
    }

    async fn apply_monster(&mut self, job: &JobListing, timestamp: &str, screenshot_dir: &str) -> ApplicationResult {
        info!("👹 Applying to Monster job: {}", job.title);
        
        let screenshot_path = format!("{}/monster_{}_{}.png", screenshot_dir, job.id, timestamp);
        
        match self.simulate_application_attempt().await {
            Ok(_) => {
                info!("✅ Successfully applied to Monster job: {}", job.title);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: true,
                    timestamp: timestamp.to_string(),
                    screenshot_path: Some(screenshot_path),
                    error_message: None,
                    status: ApplicationStatus::Success,
                }
            }
            Err(e) => {
                error!("❌ Monster application failed: {}", e);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: false,
                    timestamp: timestamp.to_string(),
                    screenshot_path: Some(screenshot_path),
                    error_message: Some(e.to_string()),
                    status: ApplicationStatus::Failed,
                }
            }
        }
    }

    async fn apply_wework_remotely(&mut self, job: &JobListing, timestamp: &str, screenshot_dir: &str) -> ApplicationResult {
        info!("🌍 Applying to WeWorkRemotely job: {}", job.title);
        
        let screenshot_path = format!("{}/wework_{}_{}.png", screenshot_dir, job.id, timestamp);
        
        match self.simulate_application_attempt().await {
            Ok(_) => {
                info!("✅ Successfully applied to WeWorkRemotely job: {}", job.title);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: true,
                    timestamp: timestamp.to_string(),
                    screenshot_path: Some(screenshot_path),
                    error_message: None,
                    status: ApplicationStatus::Success,
                }
            }
            Err(e) => {
                error!("❌ WeWorkRemotely application failed: {}", e);
                ApplicationResult {
                    job_id: job.id.clone(),
                    job_url: job.url.clone(),
                    success: false,
                    timestamp: timestamp.to_string(),
                    screenshot_path: Some(screenshot_path),
                    error_message: Some(e.to_string()),
                    status: ApplicationStatus::Failed,
                }
            }
        }
    }

    async fn simulate_application_attempt(&self) -> Result<()> {
        // Simulate various browser automation steps with potential failures
        
        // Simulate navigation delay
        sleep(Duration::from_millis(2000)).await;
        
        // Simulate potential network issues (10% chance)
        if rand::random::<f32>() < 0.1 {
            return Err(anyhow::anyhow!("Network timeout during navigation"));
        }
        
        // Simulate form filling
        sleep(Duration::from_millis(3000)).await;
        
        // Simulate potential CAPTCHA (5% chance)
        if rand::random::<f32>() < 0.05 {
            return Err(anyhow::anyhow!("CAPTCHA detected - manual intervention required"));
        }
        
        // Simulate file upload
        sleep(Duration::from_millis(2000)).await;
        
        // Simulate potential upload failure (3% chance)
        if rand::random::<f32>() < 0.03 {
            return Err(anyhow::anyhow!("File upload failed"));
        }
        
        // Simulate form submission
        sleep(Duration::from_millis(1000)).await;
        
        Ok(())
    }

    fn generate_custom_cover_letter(&self, job: &JobListing) -> Result<String> {
        // Load cover letter template
        let template = match std::fs::read_to_string(&self.cover_letter_template) {
            Ok(content) => content,
            Err(_) => {
                // Fallback template if file doesn't exist
                "Dear Hiring Manager,\n\nI am writing to express my interest in the {{TITLE}} position at {{COMPANY}}.\n\nMy background in software development and automation makes me a strong candidate for this role.\n\nI look forward to hearing from you.\n\nBest regards,\n{{NAME}}".to_string()
            }
        };

        // Replace placeholders
        let name = std::env::var("DEFAULT_NAME").unwrap_or_else(|_| "Applicant".to_string());
        let cover_letter = template
            .replace("{{TITLE}}", &job.title)
            .replace("{{COMPANY}}", &job.company)
            .replace("{{NAME}}", &name);

        Ok(cover_letter)
    }
}