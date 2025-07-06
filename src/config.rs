use serde::Deserialize;
use std::fs;
use std::collections::HashMap;
use anyhow::{Result, Error};
use toml;
use crate::document_profile::DocumentProfile;

/// Enhanced application configuration with global and adaptive capabilities
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // User profile
    pub name: String,
    pub email: String,
    pub location: Vec<String>,
    pub open_to_relocation: Option<bool>,
    
    // Job search criteria
    pub job_keywords: Vec<String>,
    pub skills: Vec<String>,
    pub industries: Vec<String>,
    pub seniority_levels: Vec<String>,
    pub targets: Vec<String>,
    
    // Document paths
    pub resume_path: String,
    pub cover_letter_path: String,
    pub resume_profiles: Option<Vec<DocumentProfile>>,
    pub cover_letter_profiles: Option<Vec<DocumentProfile>>,
    pub applied_jobs_path: Option<String>,
    
    // Salary preferences
    pub min_salary: Option<u32>,
    pub max_salary: Option<u32>,
    
    // Search engine configuration
    #[serde(default)]
    pub search_engines: Vec<String>,
    #[serde(default)]
    pub company_targets: Vec<String>,
    #[serde(default)]
    pub search_locations: Vec<String>,
    
    // NEW: Global configuration
    #[serde(default)]
    pub preferred_locales: Vec<String>,
    #[serde(default)]
    pub preferred_currency: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub enable_international_search: bool,
    #[serde(default)]
    pub regional_job_boards: HashMap<String, Vec<String>>,
    
    // NEW: Adaptive automation settings
    #[serde(default)]
    pub enable_adaptive_automation: bool,
    #[serde(default)]
    pub browser_retry_attempts: Option<u32>,
    #[serde(default)]
    pub performance_monitoring: bool,
    #[serde(default)]
    pub dynamic_profile_selection: bool,
    
    // NEW: Optimization settings
    #[serde(default)]
    pub max_concurrent_applications: Option<u32>,
    #[serde(default)]
    pub application_rate_limit: Option<u32>, // Applications per hour
    #[serde(default)]
    pub intelligent_scheduling: bool,
    #[serde(default)]
    pub a_b_testing_enabled: bool,
}


impl Config {
    pub fn load(path: &str) -> Result<Self, Error> {
        println!("Loading config from: {}", path);
        let content = fs::read_to_string(path)?;
        println!("File content length: {} bytes", content.len());
        println!("First 100 characters: {}", &content[..std::cmp::min(100, content.len())]);

        // Try to parse the TOML
        match toml::from_str::<Self>(&content) {
            Ok(config) => {
                println!("Successfully parsed config");
                Ok(config)
            },
            Err(e) => {
                println!("Error parsing TOML: {}", e);
                Err(anyhow::anyhow!("TOML parse error: {}", e))
            }
        }
    }

    /// Get the effective locale based on user preferences and detected locale
    pub fn get_effective_locale(&self) -> String {
        if !self.preferred_locales.is_empty() {
            return self.preferred_locales[0].clone();
        }
        
        // Auto-detect based on location
        crate::globalization::detect_locale(&self.location)
    }

    /// Get the effective currency for salary calculations
    pub fn get_effective_currency(&self) -> String {
        if let Some(ref currency) = self.preferred_currency {
            return currency.clone();
        }
        
        // Auto-detect based on locale
        let locale = self.get_effective_locale();
        let region = crate::globalization::extract_region_from_locale(&locale);
        
        let global_config = crate::globalization::GlobalConfig::default();
        global_config.currency_preferences
            .get(&region)
            .cloned()
            .unwrap_or_else(|| "USD".to_string())
    }

    /// Check if international search should be enabled
    pub fn should_enable_international_search(&self) -> bool {
        self.enable_international_search || 
        self.location.iter().any(|loc| 
            loc.to_lowercase().contains("remote") || 
            loc.to_lowercase().contains("international") ||
            loc.to_lowercase().contains("global")
        )
    }

    /// Get all applicable job boards including regional ones
    pub fn get_all_job_boards(&self) -> Vec<String> {
        let mut job_boards = self.targets.clone();
        
        if self.should_enable_international_search() {
            let locale = self.get_effective_locale();
            let region = crate::globalization::extract_region_from_locale(&locale);
            let global_config = crate::globalization::GlobalConfig::default();
            
            let regional_boards = crate::globalization::get_regional_job_boards(&region, &global_config);
            job_boards.extend(regional_boards);
        }
        
        // Add custom regional job boards from config
        for (_, boards) in &self.regional_job_boards {
            job_boards.extend(boards.clone());
        }
        
        // Remove duplicates
        job_boards.sort();
        job_boards.dedup();
        job_boards
    }
}
