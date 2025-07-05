use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Type alias for shared application state
pub type SharedApplicationState = Arc<Mutex<ApplicationState>>;

/// Represents a job application entry
#[derive(Debug, Serialize, Deserialize)]
pub struct JobApplication {
    /// URL of the job posting
    pub url: String,
    /// Hash of the URL for quick lookup
    pub url_hash: String,
    /// When the application was submitted (ISO 8601 format)
    pub applied_at: String,
    /// Whether a confirmation email was received
    pub confirmation_received: bool,
    /// The resume used for this application
    pub resume_used: String,
    /// The cover letter used for this application
    pub cover_letter_used: String,
    /// Any additional notes or metadata
    pub metadata: HashMap<String, String>,
}

/// Manages the state of job applications
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationState {
    /// Map of URL hash to job application
    pub applications: HashMap<String, JobApplication>,
    /// Path to the state file
    #[serde(skip)]
    pub state_file_path: String,
}

impl ApplicationState {
    /// Creates a new application state
    pub fn new(state_file_path: &str) -> Self {
        Self {
            applications: HashMap::new(),
            state_file_path: state_file_path.to_string(),
        }
    }

    /// Loads the application state from a file
    pub fn load(state_file_path: &str) -> Result<Self> {
        // Mock implementation - in real code would read and parse JSON file
        Ok(Self {
            applications: HashMap::new(),
            state_file_path: state_file_path.to_string(),
        })
    }

    /// Saves the application state to a file
    pub fn save(&self) -> Result<()> {
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
            .with_context(|| format!("Failed to create application state file: {}", self.state_file_path))?;

        file.write_all(json.as_bytes())
            .with_context(|| format!("Failed to write to application state file: {}", self.state_file_path))?;

        Ok(())
    }

    /// Checks if a job has already been applied to
    pub fn has_applied(&self, url: &str) -> bool {
        let hash = Self::hash_url(url);
        self.applications.contains_key(&hash)
    }

    /// Adds a new job application
    pub fn add_application(
        &mut self,
        url: &str,
        resume_used: &str,
        cover_letter_used: &str,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<()> {
        let url_hash = Self::hash_url(url);

        // Check if already applied
        if self.has_applied(url) {
            return Ok(());
        }

        // Create new application entry
        let application = JobApplication {
            url: url.to_string(),
            url_hash: url_hash.clone(),
            applied_at: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            confirmation_received: false,
            resume_used: resume_used.to_string(),
            cover_letter_used: cover_letter_used.to_string(),
            metadata: metadata.unwrap_or_default(),
        };

        // Add to map
        self.applications.insert(url_hash, application);

        // Save state
        self.save()?;

        Ok(())
    }

    /// Updates the confirmation status for a job application
    pub fn update_confirmation_status(&mut self, url: &str, confirmed: bool) -> Result<()> {
        let hash = Self::hash_url(url);

        if let Some(application) = self.applications.get_mut(&hash) {
            application.confirmation_received = confirmed;
            self.save()?;
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Gets all job applications
    pub fn get_all_applications(&self) -> Vec<&JobApplication> {
        self.applications.values().collect()
    }

    /// Hashes a URL to create a unique identifier
    fn hash_url(url: &str) -> String {
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}
