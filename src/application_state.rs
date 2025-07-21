use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{debug, info};

/// PRODUCTION APPLICATION STATE - REAL DATA TRACKING ONLY
/// No mock data, real job applications and browser sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationState {
    // REAL APPLICATION TRACKING
    pub applications_today: u32,
    pub total_applications: u32,
    pub successful_applications: u32,
    pub failed_applications: u32,

    // REAL BROWSER SESSION STATE
    pub browser_session_active: bool,
    pub current_url: String,
    pub last_action_timestamp: Option<DateTime<Local>>,

    // REAL JOB TRACKING
    pub applied_jobs: HashMap<String, ApplicationRecord>,
    pub failed_jobs: HashMap<String, String>, // URL -> Error reason

    // REAL PERFORMANCE METRICS
    pub success_rate: f64,
    pub average_application_time_seconds: f64,
    pub uptime_seconds: u64,
    pub session_start_time: DateTime<Local>,

    // REAL AUTOMATION STATE
    pub automation_running: bool,
    pub manual_intervention_active: bool,
    pub pause_requested: bool,

    // FILE PERSISTENCE
    state_file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationRecord {
    pub url: String,
    pub company: String,
    pub position: String,
    pub applied_at: DateTime<Local>,
    pub status: ApplicationStatus,
    pub response_message: String,
    pub application_method: String, // "Easy Apply", "Company Site", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Applied,
    Failed,
    Pending,
    AlreadyApplied,
    RequiresManualIntervention,
}

/// Type alias for shared application state
pub type SharedApplicationState = Arc<Mutex<ApplicationState>>;

impl ApplicationState {
    /// Create new PRODUCTION application state with real data tracking
    pub fn new(state_file_path: &str) -> Self {
        info!("🔧 Initializing PRODUCTION application state");

        let mut state = Self {
            applications_today: 0,
            total_applications: 0,
            successful_applications: 0,
            failed_applications: 0,
            browser_session_active: false,
            current_url: String::new(),
            last_action_timestamp: None,
            applied_jobs: HashMap::new(),
            failed_jobs: HashMap::new(),
            success_rate: 0.0,
            average_application_time_seconds: 0.0,
            uptime_seconds: 0,
            session_start_time: Local::now(),
            automation_running: false,
            manual_intervention_active: false,
            pause_requested: false,
            state_file_path: state_file_path.to_string(),
        };

        // Try to load existing state from file
        if let Err(e) = state.load_from_file() {
            debug!("No existing state file found, starting fresh: {}", e);
        }

        // Reset session-specific data
        state.session_start_time = Local::now();
        state.uptime_seconds = 0;
        state.automation_running = false;
        state.manual_intervention_active = false;
        state.pause_requested = false;

        info!("✅ Application state initialized");
        state
    }

    /// Record a REAL job application attempt
    pub fn record_application(&mut self, url: &str, result: &str) {
        let now = Local::now();

        // Parse result to determine status
        let (status, company, position) = self.parse_application_result(result);

        let record = ApplicationRecord {
            url: url.to_string(),
            company: company.unwrap_or_else(|| self.extract_company_from_url(url)),
            position: position.unwrap_or_else(|| "Unknown Position".to_string()),
            applied_at: now,
            status: status.clone(),
            response_message: result.to_string(),
            application_method: self.determine_application_method(url),
        };

        self.applied_jobs.insert(url.to_string(), record);

        // Update counters
        match status {
            ApplicationStatus::Applied => {
                self.successful_applications += 1;
                self.applications_today += 1;
            }
            ApplicationStatus::Failed => {
                self.failed_applications += 1;
                self.failed_jobs.insert(url.to_string(), result.to_string());
            }
            ApplicationStatus::AlreadyApplied => {
                // Don't count as new application
            }
            _ => {
                self.applications_today += 1;
            }
        }

        self.total_applications = self.successful_applications + self.failed_applications;
        self.last_action_timestamp = Some(now);

        // Recalculate success rate
        if self.total_applications > 0 {
            self.success_rate = self.successful_applications as f64 / self.total_applications as f64;
        }

        // Auto-save state
        if let Err(e) = self.save_to_file() {
            tracing::error!("Failed to save application state: {}", e);
        }

        info!("📝 Recorded application: {} -> {}", url, result);
    }

    /// Update browser session state
    pub fn update_browser_state(&mut self, url: &str, active: bool) {
        self.current_url = url.to_string();
        self.browser_session_active = active;
        self.last_action_timestamp = Some(Local::now());

        debug!("🌐 Browser state updated: {} (active: {})", url, active);
    }

    /// Set automation running state
    pub fn set_automation_running(&mut self, running: bool) {
        self.automation_running = running;
        if running {
            info!("🚀 Automation started");
        } else {
            info!("🛑 Automation stopped");
        }
    }

    /// Set manual intervention state
    pub fn set_manual_intervention(&mut self, active: bool, reason: Option<&str>) {
        self.manual_intervention_active = active;
        if active {
            info!(
                "⏸️ Manual intervention activated: {}",
                reason.unwrap_or("User request")
            );
        } else {
            info!("▶️ Manual intervention completed");
        }
    }

    /// Update uptime counter
    pub fn update_uptime(&mut self) {
        self.uptime_seconds = Local::now()
            .signed_duration_since(self.session_start_time)
            .num_seconds() as u64;
    }

    /// Get real-time statistics summary
    pub fn get_statistics_summary(&self) -> String {
        format!(
            "REAL-TIME STATS: Applied: {} | Success Rate: {:.1}% | Uptime: {}m | Status: {}",
            self.applications_today,
            self.success_rate * 100.0,
            self.uptime_seconds / 60,
            if self.automation_running { "RUNNING" } else { "STOPPED" }
        )
    }

    /// Get today's application URLs for deduplication
    pub fn get_todays_applications(&self) -> Vec<String> {
        let today = Local::now().date_naive();

        self.applied_jobs
            .values()
            .filter(|record| record.applied_at.date_naive() == today)
            .map(|record| record.url.clone())
            .collect()
    }

    /// Check if URL was already applied to today
    pub fn is_already_applied_today(&self, url: &str) -> bool {
        let today = Local::now().date_naive();

        if let Some(record) = self.applied_jobs.get(url) {
            record.applied_at.date_naive() == today
        } else {
            false
        }
    }

    /// Parse application result to extract status and details
    fn parse_application_result(
        &self,
        result: &str,
    ) -> (ApplicationStatus, Option<String>, Option<String>) {
        let result_lower = result.to_lowercase();

        let status = if result_lower.contains("successfully") || result_lower.contains("applied") {
            ApplicationStatus::Applied
        } else if result_lower.contains("already applied") || result_lower.contains("duplicate") {
            ApplicationStatus::AlreadyApplied
        } else if result_lower.contains("manual") || result_lower.contains("intervention") {
            ApplicationStatus::RequiresManualIntervention
        } else if result_lower.contains("failed") || result_lower.contains("error") {
            ApplicationStatus::Failed
        } else {
            ApplicationStatus::Pending
        };

        // Try to extract company and position from result
        let company = None; // Would parse from result in production
        let position = None; // Would parse from result in production

        (status, company, position)
    }

    /// Extract company name from URL
    fn extract_company_from_url(&self, url: &str) -> String {
        if url.contains("linkedin.com") {
            "LinkedIn".to_string()
        } else if url.contains("indeed.com") {
            "Indeed".to_string()
        } else if url.contains("glassdoor.com") {
            "Glassdoor".to_string()
        } else if url.contains("nike.com") {
            "Nike".to_string()
        } else if url.contains("intel.com") {
            "Intel".to_string()
        } else if url.contains("adidas") {
            "Adidas".to_string()
        } else {
            "Unknown Company".to_string()
        }
    }

    /// Determine application method from URL
    fn determine_application_method(&self, url: &str) -> String {
        if url.contains("linkedin.com") {
            "LinkedIn Easy Apply".to_string()
        } else if url.contains("indeed.com") {
            "Indeed Quick Apply".to_string()
        } else {
            "Company Website".to_string()
        }
    }

    /// Save state to file
    fn save_to_file(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize application state")?;

        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(&self.state_file_path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {:?}", parent))?;
            }
        }

        // Write to file
        let mut file = File::create(&self.state_file_path)
            .with_context(|| {
                format!(
                    "Failed to create application state file: {}",
                    self.state_file_path
                )
            })?;

        file.write_all(json.as_bytes())
            .with_context(|| {
                format!(
                    "Failed to write to application state file: {}",
                    self.state_file_path
                )
            })?;

        debug!("💾 Application state saved to {}", self.state_file_path);
        Ok(())
    }

    /// Load state from file
    fn load_from_file(&mut self) -> Result<()> {
        let content = std::fs::read_to_string(&self.state_file_path)
            .context("Failed to read state file")?;

        let loaded_state: ApplicationState = serde_json::from_str(&content)
            .context("Failed to parse state file")?;

        // Preserve session-specific data but load historical data
        self.total_applications = loaded_state.total_applications;
        self.successful_applications = loaded_state.successful_applications;
        self.failed_applications = loaded_state.failed_applications;
        self.applied_jobs = loaded_state.applied_jobs;
        self.failed_jobs = loaded_state.failed_jobs;
        self.success_rate = loaded_state.success_rate;

        info!("📂 Application state loaded from {}", self.state_file_path);
        Ok(())
    }

    /// Get performance metrics for monitoring
    pub fn get_performance_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();

        metrics.insert("applications_today".to_string(), self.applications_today as f64);
        metrics.insert("total_applications".to_string(), self.total_applications as f64);
        metrics.insert("success_rate".to_string(), self.success_rate);
        metrics.insert("uptime_hours".to_string(), self.uptime_seconds as f64 / 3600.0);
        metrics.insert("average_app_time".to_string(), self.average_application_time_seconds);

        metrics
    }
}
