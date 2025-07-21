use anyhow::{Result, anyhow, Context};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, warn, error};
use std::env;

/// PRODUCTION CONFIG - REAL DATA ONLY
/// All mock/test data sources removed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // REQUIRED REAL USER DATA
    pub name: String,
    pub email: String,
    pub resume_path: String,
    pub cover_letter_path: String,

    // REQUIRED JOB SEARCH CRITERIA
    pub job_keywords: Vec<String>,
    pub industries: Vec<String>,
    pub location: Vec<String>,
    pub preferred_locales: Vec<String>,

    // REQUIRED JOB BOARDS (REAL URLS ONLY)
    pub targets: Vec<String>,

    // AUTOMATION LIMITS
    pub max_concurrent_applications: Option<u32>,
    pub application_rate_limit: Option<u32>,
    pub enable_international_search: bool,

    // BROWSER CONFIG
    pub browser_config: BrowserConfig,

    // ACCOUNT CREATION
    pub account_creation: Option<AccountCreationConfig>,
    
    // SECRETS HANDLING
    pub secrets: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    pub visible: bool,           // ALWAYS TRUE in production
    pub single_window: bool,     // ALWAYS TRUE - no multiple tabs/windows
    pub pause_on_manual: bool,   // Allow user intervention
    pub rate_limit_ms: u64,      // Wait between actions
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreationConfig {
    pub enabled: bool,
    pub use_resume_data: bool,
    pub use_cover_letter_data: bool,
    pub auto_verify_email: bool,
    pub create_professional_profiles: bool,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            visible: true,           // PRODUCTION: Always visible
            single_window: true,     // PRODUCTION: Single window only
            pause_on_manual: true,   // Allow manual intervention
            rate_limit_ms: 3000,    // 3 second wait between actions
            timeout_seconds: 30,
        }
    }
}

impl Config {
    /// Load configuration from REAL sources only
    /// NO MOCK DATA - Environment variables and real files only
    pub fn load(config_path: &str) -> Result<Self> {
        info!("🔧 Loading PRODUCTION configuration from real sources only");

        // Load base config from TOML file
        let mut config: Config = if std::path::Path::new(config_path).exists() {
            let content = std::fs::read_to_string(config_path)
                .context("Failed to read config file")?;
            toml::from_str(&content)
                .context("Failed to parse config TOML")?
        } else {
            warn!("Config file not found, using environment variables only");
            Self::from_environment()?

        };

        // Override with environment variables (highest priority)
        config.load_from_environment()?;

        // Validate all required fields
        config.validate_production_requirements()?;

        // Load real secrets
        config.load_real_secrets()?;

        // Validate file paths exist
        config.validate_file_paths()?;

        info!("✅ PRODUCTION configuration loaded and validated");
        Ok(config)
    }

    /// Create config from environment variables only
    fn from_environment() -> Result<Self> {
        Ok(Config {
            name: env::var("AGENT_NAME")
                .or_else(|_| env::var("USER_NAME"))
                .unwrap_or_else(|_| "Production Agent".to_string()),
            email: env::var("AGENT_EMAIL")
                .context("AGENT_EMAIL environment variable required")?,
            resume_path: env::var("RESUME_PATH")
                .context("RESUME_PATH environment variable required")?,
            cover_letter_path: env::var("COVER_LETTER_PATH")
                .context("COVER_LETTER_PATH environment variable required")?,
            job_keywords: env::var("JOB_KEYWORDS")
                .unwrap_or_else(|_| "software engineer,developer,programmer".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            industries: env::var("INDUSTRIES")
                .unwrap_or_else(|_| "Technology,Software".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            location: env::var("LOCATION")
                .unwrap_or_else(|_| "Remote,United States".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            preferred_locales: vec!["en-US".to_string()],
            targets: Self::get_real_job_boards(),
            max_concurrent_applications: env::var("MAX_APPLICATIONS")
                .ok()
                .and_then(|s| s.parse().ok()),
            application_rate_limit: env::var("RATE_LIMIT")
                .ok()
                .and_then(|s| s.parse().ok()),
            enable_international_search: env::var("INTERNATIONAL_SEARCH")
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(false),
            browser_config: BrowserConfig::default(),
            account_creation: None,
            secrets: std::collections::HashMap::new(),
        })
    }
    
    /// Load real environment variables
    fn load_from_environment(&mut self) -> Result<()> {
        // Override config with environment variables if present
        if let Ok(name) = env::var("AGENT_NAME") {
            self.name = name;
        }
        if let Ok(email) = env::var("AGENT_EMAIL") {
            self.email = email;
        }
        if let Ok(resume) = env::var("RESUME_PATH") {
            self.resume_path = resume;
        }
        if let Ok(cover_letter) = env::var("COVER_LETTER_PATH") {
            self.cover_letter_path = cover_letter;
        }

        // Load .env file if present
        if let Ok(env_content) = std::fs::read_to_string(".env") {
            for line in env_content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    env::set_var(key.trim(), value.trim());
                }
            }
        }

        Ok(())
    }

    /// Get REAL job board URLs only - no mock data
    fn get_real_job_boards() -> Vec<String> {
        vec![
            "https://www.linkedin.com/jobs".to_string(),
            "https://www.indeed.com".to_string(),
            "https://www.glassdoor.com".to_string(),
            "https://www.monster.com".to_string(),
            "https://www.ziprecruiter.com".to_string(),
            "https://jobs.nike.com".to_string(),
            "https://intel.com/careers".to_string(),
            "https://careers.adidas-group.com".to_string(),
        ]
    }

    /// Validate all required production fields
    fn validate_production_requirements(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(anyhow!("PRODUCTION ERROR: Agent name is required"));
        }
        
        if self.email.is_empty() || !self.email.contains('@') {
            return Err(anyhow!("PRODUCTION ERROR: Valid email address is required"));
        }

        if self.resume_path.is_empty() {
            return Err(anyhow!("PRODUCTION ERROR: Resume path is required"));
        }

        if self.cover_letter_path.is_empty() {
            return Err(anyhow!("PRODUCTION ERROR: Cover letter path is required"));
        }
        
        if self.job_keywords.is_empty() {
            return Err(anyhow!("PRODUCTION ERROR: Job keywords are required"));
        }
        
        if self.targets.is_empty() {
            return Err(anyhow!("PRODUCTION ERROR: Job board targets are required"));
        }

        // Validate job boards are real URLs
        for target in &self.targets {
            if !target.starts_with("http") {
                return Err(anyhow!("PRODUCTION ERROR: Invalid job board URL: {}", target));
            }
        }

        info!("✅ All production requirements validated");
        Ok(())
    }
    
    /// Load real secrets from environment and files
    fn load_real_secrets(&mut self) -> Result<()> {
        // Load from environment variables
        for (key, value) in env::vars() {
            if key.starts_with("LINKEDIN_") ||
               key.starts_with("INDEED_") ||
               key.starts_with("GLASSDOOR_") ||
               key.starts_with("MONSTER_") ||
               key.starts_with("ZIPRECRUITER_") ||
               key.contains("EMAIL") ||
               key.contains("PASSWORD") {
                self.secrets.insert(key, value);
            }
        }

        info!("✅ Loaded {} real secrets from environment", self.secrets.len());
        Ok(())
    }
    
    /// Validate that file paths exist and are accessible
    fn validate_file_paths(&self) -> Result<()> {
        let resume_path = PathBuf::from(&self.resume_path);
        if !resume_path.exists() {
            return Err(anyhow!("PRODUCTION ERROR: Resume file not found: {}", self.resume_path));
        }

        let cover_letter_path = PathBuf::from(&self.cover_letter_path);
        if !cover_letter_path.exists() {
            return Err(anyhow!("PRODUCTION ERROR: Cover letter file not found: {}", self.cover_letter_path));
        }
        
        info!("✅ All file paths validated and accessible");
        Ok(())
    }

    /// Get sanitized configuration summary for logging
    pub fn get_summary(&self) -> String {
        format!(
            "PRODUCTION CONFIG: User={}, Email={}, Keywords={:?}, Boards={}, Files=OK",
            self.name,
            self.email,
            self.job_keywords,
            self.targets.len()
        )
    }
}
