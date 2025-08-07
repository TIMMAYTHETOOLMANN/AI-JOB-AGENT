use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use tracing::{info, error};

use crate::applier::ApplicationResult;
use crate::scanner::JobListing;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub event: String,
    pub job_id: Option<String>,
    pub job_url: Option<String>,
    pub job_title: Option<String>,
    pub company: Option<String>,
    pub success: Option<bool>,
    pub error_message: Option<String>,
    pub screenshot_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailureEntry {
    pub job_id: String,
    pub job_url: String,
    pub job_title: String,
    pub company: String,
    pub error_message: String,
    pub timestamp: String,
    pub retry_count: u32,
    pub job_board: String,
}

pub struct Logger {
    log_dir: String,
    today: String,
}

impl Logger {
    pub fn new() -> Result<Self> {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let log_dir = format!("./logs/{}", today);
        
        // Create directories
        std::fs::create_dir_all(&log_dir)?;
        std::fs::create_dir_all("./queue")?;
        
        info!("📋 Logger initialized: {}", log_dir);
        
        Ok(Self {
            log_dir,
            today,
        })
    }

    /// Log system startup
    pub fn log_startup(&self) -> Result<()> {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event: "system_startup".to_string(),
            job_id: None,
            job_url: None,
            job_title: None,
            company: None,
            success: None,
            error_message: None,
            screenshot_path: None,
        };

        self.write_log_entry(entry)?;
        info!("🚀 System startup logged");
        Ok(())
    }

    /// Log job search cycle
    pub fn log_job_search(&self, jobs_found: usize, jobs_filtered: usize) -> Result<()> {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event: format!("job_search_cycle"),
            job_id: None,
            job_url: None,
            job_title: None,
            company: None,
            success: Some(jobs_found > 0),
            error_message: Some(format!("Found: {}, Filtered: {}", jobs_found, jobs_filtered)),
            screenshot_path: None,
        };

        self.write_log_entry(entry)?;
        info!("🔍 Job search cycle logged: {} found, {} filtered", jobs_found, jobs_filtered);
        Ok(())
    }

    /// Log application results
    pub fn log_applications(&self, results: &[ApplicationResult]) -> Result<()> {
        for result in results {
            self.log_single_application(result)?;
        }

        // Log summary
        let success_count = results.iter().filter(|r| r.success).count();
        let total_count = results.len();
        
        let summary_entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event: "application_summary".to_string(),
            job_id: None,
            job_url: None,
            job_title: None,
            company: None,
            success: Some(success_count > 0),
            error_message: Some(format!("Success: {}/{}", success_count, total_count)),
            screenshot_path: None,
        };

        self.write_log_entry(summary_entry)?;
        info!("📊 Application summary logged: {}/{} successful", success_count, total_count);
        Ok(())
    }

    /// Log single application result
    pub fn log_single_application(&self, result: &ApplicationResult) -> Result<()> {
        let entry = LogEntry {
            timestamp: result.timestamp.clone(),
            event: if result.success { "application_success" } else { "application_failure" }.to_string(),
            job_id: Some(result.job_id.clone()),
            job_url: Some(result.job_url.clone()),
            job_title: None,
            company: None,
            success: Some(result.success),
            error_message: result.error_message.clone(),
            screenshot_path: result.screenshot_path.clone(),
        };

        self.write_log_entry(entry)?;

        // If failed, add to failure queue for potential retry
        if !result.success {
            self.log_failure(result)?;
        }

        Ok(())
    }

    /// Log error events
    pub fn log_error(&self, event: &str, error_message: &str) -> Result<()> {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event: event.to_string(),
            job_id: None,
            job_url: None,
            job_title: None,
            company: None,
            success: Some(false),
            error_message: Some(error_message.to_string()),
            screenshot_path: None,
        };

        self.write_log_entry(entry)?;
        error!("❌ Error logged: {} - {}", event, error_message);
        Ok(())
    }

    /// Log CAPTCHA detection
    pub fn log_captcha(&self, job_id: &str, job_url: &str) -> Result<()> {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event: "captcha_detected".to_string(),
            job_id: Some(job_id.to_string()),
            job_url: Some(job_url.to_string()),
            job_title: None,
            company: None,
            success: Some(false),
            error_message: Some("CAPTCHA detected - manual intervention required".to_string()),
            screenshot_path: None,
        };

        self.write_log_entry(entry)?;
        info!("🤖 CAPTCHA detection logged for job: {}", job_id);
        Ok(())
    }

    /// Log network errors with retry logic
    pub fn log_network_error(&self, job_id: &str, job_url: &str, error: &str, retry_count: u32) -> Result<()> {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event: "network_error".to_string(),
            job_id: Some(job_id.to_string()),
            job_url: Some(job_url.to_string()),
            job_title: None,
            company: None,
            success: Some(false),
            error_message: Some(format!("Retry {}: {}", retry_count, error)),
            screenshot_path: None,
        };

        self.write_log_entry(entry)?;
        info!("🌐 Network error logged for job: {} (retry {})", job_id, retry_count);
        Ok(())
    }

    /// Log system shutdown
    pub fn log_shutdown(&self) -> Result<()> {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event: "system_shutdown".to_string(),
            job_id: None,
            job_url: None,
            job_title: None,
            company: None,
            success: None,
            error_message: None,
            screenshot_path: None,
        };

        self.write_log_entry(entry)?;
        info!("🔴 System shutdown logged");
        Ok(())
    }

    /// Write log entry to daily JSONL file
    fn write_log_entry(&self, entry: LogEntry) -> Result<()> {
        let log_file = format!("{}/{}.jsonl", self.log_dir, self.today);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        let json_line = serde_json::to_string(&entry)?;
        writeln!(file, "{}", json_line)?;
        
        Ok(())
    }

    /// Add failed application to retry queue
    fn log_failure(&self, result: &ApplicationResult) -> Result<()> {
        let failure_entry = FailureEntry {
            job_id: result.job_id.clone(),
            job_url: result.job_url.clone(),
            job_title: "Unknown".to_string(), // Would be filled from job data in production
            company: "Unknown".to_string(),
            error_message: result.error_message.clone().unwrap_or_else(|| "Unknown error".to_string()),
            timestamp: result.timestamp.clone(),
            retry_count: 0,
            job_board: "unknown".to_string(), // Would be extracted from URL in production
        };

        let failures_file = "./queue/failures.jsonl";
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(failures_file)?;

        let json_line = serde_json::to_string(&failure_entry)?;
        writeln!(file, "{}", json_line)?;
        
        info!("📝 Failure logged to retry queue: {}", result.job_id);
        Ok(())
    }

    /// Get daily statistics
    pub fn get_daily_stats(&self) -> Result<DailyStats> {
        let log_file = format!("{}/{}.jsonl", self.log_dir, self.today);
        
        let mut stats = DailyStats::default();
        
        if let Ok(content) = std::fs::read_to_string(log_file) {
            for line in content.lines() {
                if let Ok(entry) = serde_json::from_str::<LogEntry>(line) {
                    match entry.event.as_str() {
                        "job_search_cycle" => stats.search_cycles += 1,
                        "application_success" => stats.successful_applications += 1,
                        "application_failure" => stats.failed_applications += 1,
                        "captcha_detected" => stats.captcha_encounters += 1,
                        "network_error" => stats.network_errors += 1,
                        _ => {}
                    }
                }
            }
        }
        
        Ok(stats)
    }

    /// Print daily summary to console
    pub fn print_daily_summary(&self) -> Result<()> {
        let stats = self.get_daily_stats()?;
        
        println!("\n📊 ===== DAILY SUMMARY ({}) =====", self.today);
        println!("🔍 Search Cycles: {}", stats.search_cycles);
        println!("✅ Successful Applications: {}", stats.successful_applications);
        println!("❌ Failed Applications: {}", stats.failed_applications);
        println!("🤖 CAPTCHA Encounters: {}", stats.captcha_encounters);
        println!("🌐 Network Errors: {}", stats.network_errors);
        
        let success_rate = if stats.total_applications() > 0 {
            (stats.successful_applications as f32 / stats.total_applications() as f32) * 100.0
        } else {
            0.0
        };
        
        println!("📈 Success Rate: {:.1}%", success_rate);
        println!("═══════════════════════════════════════\n");
        
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DailyStats {
    pub search_cycles: u32,
    pub successful_applications: u32,
    pub failed_applications: u32,
    pub captcha_encounters: u32,
    pub network_errors: u32,
}

impl DailyStats {
    pub fn total_applications(&self) -> u32 {
        self.successful_applications + self.failed_applications
    }
}