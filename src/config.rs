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

    /// Comprehensive configuration validation with context gathering
    pub fn validate_and_gather_context(&self) -> Result<ValidationReport, Error> {
        let mut report = ValidationReport::new();
        
        // Basic field validation
        self.validate_basic_fields(&mut report);
        
        // Document path validation
        self.validate_document_paths(&mut report);
        
        // Email validation
        self.validate_email(&mut report);
        
        // Salary range validation
        self.validate_salary_range(&mut report);
        
        // Location and regional settings validation
        self.validate_location_settings(&mut report);
        
        // Job search criteria validation
        self.validate_job_criteria(&mut report);
        
        // Advanced settings validation
        self.validate_advanced_settings(&mut report);
        
        // Profile consistency validation
        self.validate_profile_consistency(&mut report);
        
        if report.has_errors() {
            Err(anyhow::anyhow!("Configuration validation failed: {}", report.get_error_summary()))
        } else {
            Ok(report)
        }
    }
    
    fn validate_basic_fields(&self, report: &mut ValidationReport) {
        if self.name.trim().is_empty() {
            report.add_error("name", "Name cannot be empty");
        }
        
        if self.email.trim().is_empty() {
            report.add_error("email", "Email cannot be empty");
        }
        
        if self.location.is_empty() {
            report.add_error("location", "At least one location must be specified");
        }
        
        if self.job_keywords.is_empty() {
            report.add_warning("job_keywords", "No job keywords specified - this may limit search effectiveness");
        }
        
        if self.industries.is_empty() {
            report.add_warning("industries", "No industries specified - this may limit search targeting");
        }
        
        report.add_info("basic_fields", "Basic field validation completed");
    }
    
    fn validate_document_paths(&self, report: &mut ValidationReport) {
        // Validate main resume path
        if self.resume_path.trim().is_empty() {
            report.add_error("resume_path", "Resume path cannot be empty");
        } else if !std::path::Path::new(&self.resume_path).exists() {
            report.add_warning("resume_path", &format!("Resume file not found: {}", self.resume_path));
        } else {
            report.add_info("resume_path", "Resume file found and accessible");
        }
        
        // Validate main cover letter path
        if self.cover_letter_path.trim().is_empty() {
            report.add_error("cover_letter_path", "Cover letter path cannot be empty");
        } else if !std::path::Path::new(&self.cover_letter_path).exists() {
            report.add_warning("cover_letter_path", &format!("Cover letter file not found: {}", self.cover_letter_path));
        } else {
            report.add_info("cover_letter_path", "Cover letter file found and accessible");
        }
        
        // Validate resume profiles if present
        if let Some(ref profiles) = self.resume_profiles {
            for (i, profile) in profiles.iter().enumerate() {
                if !std::path::Path::new(&profile.path).exists() {
                    report.add_warning(&format!("resume_profiles[{}]", i), 
                                     &format!("Resume profile file not found: {}", profile.path));
                }
            }
            report.add_info("resume_profiles", &format!("Found {} resume profiles", profiles.len()));
        }
        
        // Validate cover letter profiles if present
        if let Some(ref profiles) = self.cover_letter_profiles {
            for (i, profile) in profiles.iter().enumerate() {
                if !std::path::Path::new(&profile.path).exists() {
                    report.add_warning(&format!("cover_letter_profiles[{}]", i), 
                                     &format!("Cover letter profile file not found: {}", profile.path));
                }
            }
            report.add_info("cover_letter_profiles", &format!("Found {} cover letter profiles", profiles.len()));
        }
    }
    
    fn validate_email(&self, report: &mut ValidationReport) {
        let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&self.email) {
            report.add_error("email", "Invalid email format");
        } else {
            report.add_info("email", "Email format is valid");
        }
    }
    
    fn validate_salary_range(&self, report: &mut ValidationReport) {
        match (self.min_salary, self.max_salary) {
            (Some(min), Some(max)) => {
                if min >= max {
                    report.add_error("salary_range", "Minimum salary must be less than maximum salary");
                } else if min < 1000 {
                    report.add_warning("min_salary", "Minimum salary seems unusually low");
                } else if max > 1000000 {
                    report.add_warning("max_salary", "Maximum salary seems unusually high");
                } else {
                    report.add_info("salary_range", &format!("Salary range: ${} - ${}", min, max));
                }
            },
            (Some(min), None) => {
                report.add_info("salary_range", &format!("Minimum salary: ${}", min));
            },
            (None, Some(max)) => {
                report.add_info("salary_range", &format!("Maximum salary: ${}", max));
            },
            (None, None) => {
                report.add_warning("salary_range", "No salary preferences specified");
            }
        }
    }
    
    fn validate_location_settings(&self, report: &mut ValidationReport) {
        let mut has_remote = false;
        let mut has_specific_location = false;
        
        for location in &self.location {
            let loc_lower = location.to_lowercase();
            if loc_lower.contains("remote") || loc_lower.contains("anywhere") {
                has_remote = true;
            } else {
                has_specific_location = true;
            }
        }
        
        if has_remote && has_specific_location {
            report.add_info("location_strategy", "Hybrid strategy: specific locations + remote work");
        } else if has_remote {
            report.add_info("location_strategy", "Remote-first strategy");
        } else {
            report.add_info("location_strategy", "Location-specific strategy");
        }
        
        // Validate relocation setting
        if let Some(relocation) = self.open_to_relocation {
            if relocation && !has_remote {
                report.add_info("relocation", "Open to relocation - expands search opportunities");
            }
        }
        
        // Validate international settings
        if self.enable_international_search {
            report.add_info("international_search", "International search enabled");
            
            if self.preferred_locales.is_empty() {
                report.add_warning("preferred_locales", "International search enabled but no preferred locales specified");
            }
        }
    }
    
    fn validate_job_criteria(&self, report: &mut ValidationReport) {
        // Validate keyword diversity
        if self.job_keywords.len() < 3 {
            report.add_warning("job_keywords", "Consider adding more job keywords for better search coverage");
        } else if self.job_keywords.len() > 20 {
            report.add_warning("job_keywords", "Very large number of keywords may reduce search precision");
        } else {
            report.add_info("job_keywords", &format!("Good keyword coverage: {} keywords", self.job_keywords.len()));
        }
        
        // Validate skills
        if self.skills.len() < 3 {
            report.add_warning("skills", "Consider adding more skills for better job matching");
        } else {
            report.add_info("skills", &format!("Skills listed: {}", self.skills.len()));
        }
        
        // Validate seniority levels
        if self.seniority_levels.is_empty() {
            report.add_warning("seniority_levels", "No seniority levels specified - may miss relevant opportunities");
        } else {
            report.add_info("seniority_levels", &format!("Target seniority levels: {}", self.seniority_levels.join(", ")));
        }
        
        // Validate job boards
        if self.targets.is_empty() {
            report.add_error("targets", "No job boards specified");
        } else {
            report.add_info("targets", &format!("Configured job boards: {}", self.targets.len()));
        }
    }
    
    fn validate_advanced_settings(&self, report: &mut ValidationReport) {
        // Validate rate limiting
        if let Some(rate_limit) = self.application_rate_limit {
            if rate_limit > 50 {
                report.add_warning("application_rate_limit", "High application rate may trigger anti-spam measures");
            } else if rate_limit < 5 {
                report.add_warning("application_rate_limit", "Very low application rate may reduce effectiveness");
            } else {
                report.add_info("application_rate_limit", &format!("Application rate limit: {} per hour", rate_limit));
            }
        }
        
        // Validate concurrency
        if let Some(concurrent) = self.max_concurrent_applications {
            if concurrent > 10 {
                report.add_warning("max_concurrent_applications", "High concurrency may overwhelm job sites");
            } else {
                report.add_info("max_concurrent_applications", &format!("Max concurrent applications: {}", concurrent));
            }
        }
        
        // Validate retry attempts
        if let Some(retries) = self.browser_retry_attempts {
            if retries > 5 {
                report.add_warning("browser_retry_attempts", "High retry count may cause delays");
            } else {
                report.add_info("browser_retry_attempts", &format!("Browser retry attempts: {}", retries));
            }
        }
    }
    
    fn validate_profile_consistency(&self, report: &mut ValidationReport) {
        // Check if resume profiles match job criteria
        if let Some(ref profiles) = self.resume_profiles {
            for profile in profiles {
                let matching_industries = profile.industries.iter()
                    .filter(|industry| self.industries.contains(industry))
                    .count();
                
                if matching_industries == 0 {
                    report.add_warning("profile_consistency", 
                        &format!("Resume profile '{}' has no matching industries", profile.name));
                }
            }
        }
        
        // Check if cover letter profiles match job criteria
        if let Some(ref profiles) = self.cover_letter_profiles {
            for profile in profiles {
                let matching_keywords = profile.keywords.iter()
                    .filter(|keyword| self.job_keywords.contains(keyword))
                    .count();
                
                if matching_keywords == 0 {
                    report.add_warning("profile_consistency", 
                        &format!("Cover letter profile '{}' has no matching keywords", profile.name));
                }
            }
        }
        
        report.add_info("profile_consistency", "Profile consistency validation completed");
    }
}

/// Comprehensive validation report for configuration analysis
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub info: Vec<ValidationIssue>,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub field: String,
    pub message: String,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, field: &str, message: &str) {
        self.errors.push(ValidationIssue {
            field: field.to_string(),
            message: message.to_string(),
        });
    }
    
    pub fn add_warning(&mut self, field: &str, message: &str) {
        self.warnings.push(ValidationIssue {
            field: field.to_string(),
            message: message.to_string(),
        });
    }
    
    pub fn add_info(&mut self, field: &str, message: &str) {
        self.info.push(ValidationIssue {
            field: field.to_string(),
            message: message.to_string(),
        });
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn get_error_summary(&self) -> String {
        self.errors.iter()
            .map(|issue| format!("{}: {}", issue.field, issue.message))
            .collect::<Vec<_>>()
            .join("; ")
    }
    
    pub fn print_summary(&self) {
        if !self.errors.is_empty() {
            println!("\n❌ ERRORS ({})", self.errors.len());
            for error in &self.errors {
                println!("  • {}: {}", error.field, error.message);
            }
        }
        
        if !self.warnings.is_empty() {
            println!("\n⚠️  WARNINGS ({})", self.warnings.len());
            for warning in &self.warnings {
                println!("  • {}: {}", warning.field, warning.message);
            }
        }
        
        if !self.info.is_empty() {
            println!("\n✅ INFORMATION ({})", self.info.len());
            for info in &self.info {
                println!("  • {}: {}", info.field, info.message);
            }
        }
        
        println!();
        if self.has_errors() {
            println!("🔴 Configuration validation FAILED - please fix errors before proceeding");
        } else if !self.warnings.is_empty() {
            println!("🟡 Configuration validation PASSED with warnings");
        } else {
            println!("🟢 Configuration validation PASSED - all systems ready");
        }
    }
}
