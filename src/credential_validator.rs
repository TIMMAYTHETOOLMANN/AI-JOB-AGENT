use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use serde::{Serialize, Deserialize};

/// Instant credential validation and reporting system
/// Immediately reports missing credentials and environment variables
#[derive(Debug, Clone)]
pub struct CredentialValidator {
    required_credentials: HashMap<String, CredentialRequirement>,
    optional_credentials: HashMap<String, CredentialRequirement>,
    validation_results: ValidationResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialRequirement {
    pub name: String,
    pub description: String,
    pub env_var_names: Vec<String>,
    pub file_paths: Vec<String>,
    pub validation_type: ValidationType,
    pub critical: bool,
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    Email,
    Password,
    FilePath,
    Url,
    ApiKey,
    Text,
    Boolean,
}

#[derive(Debug, Clone, Default)]
pub struct ValidationResults {
    pub missing_critical: Vec<String>,
    pub missing_optional: Vec<String>,
    pub invalid_credentials: Vec<String>,
    pub valid_credentials: Vec<String>,
    pub file_validation_errors: Vec<String>,
    pub total_score: f64,
}

impl CredentialValidator {
    /// Initialize credential validator with comprehensive requirements
    pub fn new() -> Self {
        info!("🔐 Initializing credential validator");
        
        let mut validator = Self {
            required_credentials: HashMap::new(),
            optional_credentials: HashMap::new(),
            validation_results: ValidationResults::default(),
        };
        
        validator.setup_credential_requirements();
        validator
    }

    /// Perform instant validation of all credentials and environment variables
    pub async fn validate_instantly(&mut self) -> Result<ValidationReport> {
        info!("⚡ INSTANT CREDENTIAL VALIDATION STARTING");
        println!("\n🔍 INSTANT CREDENTIAL CHECK");
        println!("═══════════════════════════════════════");
        
        // Reset validation results
        self.validation_results = ValidationResults::default();
        
        // Collect credentials to validate
        let required_creds: Vec<(String, CredentialRequirement)> = self.required_credentials
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        let optional_creds: Vec<(String, CredentialRequirement)> = self.optional_credentials
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        // Validate all required credentials
        for (key, requirement) in required_creds {
            let result = self.validate_credential(&requirement).await?;
            self.process_validation_result(&key, &requirement, result);
        }
        
        // Validate all optional credentials
        for (key, requirement) in optional_creds {
            let result = self.validate_credential(&requirement).await?;
            self.process_validation_result(&key, &requirement, result);
        }
        
        // Generate comprehensive report
        let report = self.generate_validation_report();
        
        // Immediate reporting to CLI
        self.report_results_immediately(&report)?;
        
        // Check if system can proceed
        if !self.validation_results.missing_critical.is_empty() {
            error!("❌ CRITICAL CREDENTIALS MISSING - SYSTEM CANNOT PROCEED");
            return Err(anyhow!("Critical credentials missing: {:?}", self.validation_results.missing_critical));
        }
        
        Ok(report)
    }

    /// Validate a single credential requirement
    async fn validate_credential(&self, requirement: &CredentialRequirement) -> Result<CredentialValidationResult> {
        let mut result = CredentialValidationResult {
            found: false,
            valid: false,
            source: String::new(),
            value_preview: String::new(),
            error_message: None,
        };
        
        // Check environment variables first
        for env_var in &requirement.env_var_names {
            if let Ok(value) = env::var(env_var) {
                result.found = true;
                result.source = format!("Environment variable: {}", env_var);
                result.value_preview = self.create_safe_preview(&value);
                
                // Validate the value
                result.valid = self.validate_credential_value(&value, &requirement.validation_type);
                if !result.valid {
                    result.error_message = Some(format!("Invalid format for {}", requirement.validation_type.description()));
                }
                
                debug!("✅ Found credential {} in env var {}", requirement.name, env_var);
                break;
            }
        }
        
        // If not found in env vars, check file paths
        if !result.found {
            for file_path in &requirement.file_paths {
                if Path::new(file_path).exists() {
                    result.found = true;
                    result.source = format!("File: {}", file_path);
                    result.value_preview = format!("File exists: {}", file_path);
                    result.valid = true; // Files are considered valid if they exist
                    
                    debug!("✅ Found credential {} in file {}", requirement.name, file_path);
                    break;
                }
            }
        }
        
        if !result.found {
            debug!("❌ Credential {} not found", requirement.name);
        }
        
        Ok(result)
    }

    /// Process validation result and update internal state
    fn process_validation_result(&mut self, key: &str, requirement: &CredentialRequirement, result: CredentialValidationResult) {
        if !result.found {
            if requirement.critical {
                self.validation_results.missing_critical.push(key.to_string());
            } else {
                self.validation_results.missing_optional.push(key.to_string());
            }
        } else if !result.valid {
            self.validation_results.invalid_credentials.push(format!("{}: {}", key, result.error_message.unwrap_or_default()));
        } else {
            self.validation_results.valid_credentials.push(format!("{} ({})", key, result.source));
        }
    }

    /// Generate comprehensive validation report
    fn generate_validation_report(&mut self) -> ValidationReport {
        let total_credentials = self.required_credentials.len() + self.optional_credentials.len();
        let valid_count = self.validation_results.valid_credentials.len();
        
        self.validation_results.total_score = if total_credentials > 0 {
            (valid_count as f64 / total_credentials as f64) * 100.0
        } else {
            0.0
        };
        
        ValidationReport {
            validation_passed: self.validation_results.missing_critical.is_empty(),
            critical_missing: self.validation_results.missing_critical.clone(),
            optional_missing: self.validation_results.missing_optional.clone(),
            invalid_credentials: self.validation_results.invalid_credentials.clone(),
            valid_credentials: self.validation_results.valid_credentials.clone(),
            completion_percentage: self.validation_results.total_score,
            recommendations: self.generate_recommendations(),
            immediate_actions: self.generate_immediate_actions(),
        }
    }

    /// Report validation results immediately to CLI
    fn report_results_immediately(&self, report: &ValidationReport) -> Result<()> {
        println!("📊 VALIDATION RESULTS:");
        println!("  Completion: {:.1}%", report.completion_percentage);
        println!("  Status: {}", if report.validation_passed { "✅ PASS" } else { "❌ FAIL" });
        
        if !report.critical_missing.is_empty() {
            println!("\n🚨 CRITICAL MISSING CREDENTIALS:");
            for credential in &report.critical_missing {
                println!("  ❌ {}", credential);
                if let Some(requirement) = self.required_credentials.get(credential) {
                    println!("     Description: {}", requirement.description);
                    println!("     Environment variables: {:?}", requirement.env_var_names);
                    if let Some(example) = &requirement.example {
                        println!("     Example: {}", example);
                    }
                }
            }
        }
        
        if !report.optional_missing.is_empty() {
            println!("\n⚠️  OPTIONAL MISSING CREDENTIALS:");
            for credential in &report.optional_missing {
                println!("  ⚠️  {}", credential);
            }
        }
        
        if !report.invalid_credentials.is_empty() {
            println!("\n❌ INVALID CREDENTIALS:");
            for credential in &report.invalid_credentials {
                println!("  ❌ {}", credential);
            }
        }
        
        if !report.valid_credentials.is_empty() {
            println!("\n✅ VALID CREDENTIALS:");
            for credential in &report.valid_credentials {
                println!("  ✅ {}", credential);
            }
        }
        
        if !report.immediate_actions.is_empty() {
            println!("\n🔧 IMMEDIATE ACTIONS REQUIRED:");
            for (i, action) in report.immediate_actions.iter().enumerate() {
                println!("  {}. {}", i + 1, action);
            }
        }
        
        println!("═══════════════════════════════════════");
        
        // If critical credentials are missing, provide detailed guidance
        if !report.validation_passed {
            self.provide_setup_guidance(report)?;
        }
        
        Ok(())
    }

    /// Provide detailed setup guidance for missing credentials
    fn provide_setup_guidance(&self, report: &ValidationReport) -> Result<()> {
        println!("\n📚 SETUP GUIDANCE:");
        println!("═══════════════════════════════════════");
        
        // Create template .env file content
        println!("1. Create a .env file with the following template:");
        println!("   ───────────────────────────────────────────");
        
        for credential in &report.critical_missing {
            if let Some(requirement) = self.required_credentials.get(credential) {
                for env_var in &requirement.env_var_names {
                    println!("   {}=your_{}_{}", env_var, credential.to_lowercase(), "here");
                }
            }
        }
        
        println!("   ───────────────────────────────────────────");
        
        // Provide specific instructions
        println!("\n2. Specific instructions:");
        for credential in &report.critical_missing {
            if let Some(requirement) = self.required_credentials.get(credential) {
                println!("   • {}: {}", requirement.name, requirement.description);
                if let Some(example) = &requirement.example {
                    println!("     Example: {}", example);
                }
            }
        }
        
        println!("\n3. Alternative: Set environment variables directly:");
        for credential in &report.critical_missing {
            if let Some(requirement) = self.required_credentials.get(credential) {
                for env_var in &requirement.env_var_names {
                    println!("   export {}=\"your_value_here\"", env_var);
                }
            }
        }
        
        println!("\n⚠️  SYSTEM WILL NOT START WITHOUT CRITICAL CREDENTIALS");
        println!("   Complete the setup above and restart the application.");
        
        Ok(())
    }

    /// Setup all credential requirements
    fn setup_credential_requirements(&mut self) {
        // Job board credentials
        self.add_required_credential("linkedin_credentials", CredentialRequirement {
            name: "LinkedIn Credentials".to_string(),
            description: "LinkedIn email and password for job applications".to_string(),
            env_var_names: vec!["LINKEDIN_EMAIL".to_string(), "LINKEDIN_PASSWORD".to_string()],
            file_paths: vec![],
            validation_type: ValidationType::Email,
            critical: true,
            example: Some("your.email@example.com".to_string()),
        });
        
        self.add_required_credential("agent_name", CredentialRequirement {
            name: "Agent Name".to_string(),
            description: "Your full name for job applications".to_string(),
            env_var_names: vec!["AGENT_NAME".to_string(), "USER_NAME".to_string()],
            file_paths: vec![],
            validation_type: ValidationType::Text,
            critical: true,
            example: Some("John Doe".to_string()),
        });
        
        self.add_required_credential("agent_email", CredentialRequirement {
            name: "Agent Email".to_string(),
            description: "Your email address for job applications".to_string(),
            env_var_names: vec!["AGENT_EMAIL".to_string(), "USER_EMAIL".to_string()],
            file_paths: vec![],
            validation_type: ValidationType::Email,
            critical: true,
            example: Some("john.doe@example.com".to_string()),
        });
        
        self.add_required_credential("resume_path", CredentialRequirement {
            name: "Resume File".to_string(),
            description: "Path to your resume PDF file".to_string(),
            env_var_names: vec!["RESUME_PATH".to_string()],
            file_paths: vec!["resume.pdf".to_string(), "documents/resume.pdf".to_string()],
            validation_type: ValidationType::FilePath,
            critical: true,
            example: Some("/path/to/resume.pdf".to_string()),
        });
        
        self.add_required_credential("cover_letter_path", CredentialRequirement {
            name: "Cover Letter File".to_string(),
            description: "Path to your cover letter PDF file".to_string(),
            env_var_names: vec!["COVER_LETTER_PATH".to_string()],
            file_paths: vec!["cover_letter.pdf".to_string(), "documents/cover_letter.pdf".to_string()],
            validation_type: ValidationType::FilePath,
            critical: true,
            example: Some("/path/to/cover_letter.pdf".to_string()),
        });
        
        // Optional job board credentials
        self.add_optional_credential("indeed_credentials", CredentialRequirement {
            name: "Indeed Credentials".to_string(),
            description: "Indeed email and password (optional but recommended)".to_string(),
            env_var_names: vec!["INDEED_EMAIL".to_string(), "INDEED_PASSWORD".to_string()],
            file_paths: vec![],
            validation_type: ValidationType::Email,
            critical: false,
            example: Some("your.email@example.com".to_string()),
        });
        
        self.add_optional_credential("glassdoor_credentials", CredentialRequirement {
            name: "Glassdoor Credentials".to_string(),
            description: "Glassdoor email and password (optional)".to_string(),
            env_var_names: vec!["GLASSDOOR_EMAIL".to_string(), "GLASSDOOR_PASSWORD".to_string()],
            file_paths: vec![],
            validation_type: ValidationType::Email,
            critical: false,
            example: Some("your.email@example.com".to_string()),
        });
    }

    fn add_required_credential(&mut self, key: &str, requirement: CredentialRequirement) {
        self.required_credentials.insert(key.to_string(), requirement);
    }

    fn add_optional_credential(&mut self, key: &str, requirement: CredentialRequirement) {
        self.optional_credentials.insert(key.to_string(), requirement);
    }

    /// Validate credential value based on type
    fn validate_credential_value(&self, value: &str, validation_type: &ValidationType) -> bool {
        match validation_type {
            ValidationType::Email => {
                value.contains('@') && value.contains('.') && value.len() > 5
            }
            ValidationType::Password => {
                value.len() >= 6 // Minimum password length
            }
            ValidationType::FilePath => {
                Path::new(value).exists()
            }
            ValidationType::Url => {
                value.starts_with("http://") || value.starts_with("https://")
            }
            ValidationType::ApiKey => {
                value.len() >= 10 && !value.contains(' ')
            }
            ValidationType::Text => {
                !value.trim().is_empty()
            }
            ValidationType::Boolean => {
                matches!(value.to_lowercase().as_str(), "true" | "false" | "1" | "0" | "yes" | "no")
            }
        }
    }

    /// Create a safe preview of credential value
    fn create_safe_preview(&self, value: &str) -> String {
        if value.len() <= 4 {
            "****".to_string()
        } else if value.contains('@') {
            // Email - show first few chars and domain
            if let Some(at_pos) = value.find('@') {
                let (local, domain) = value.split_at(at_pos);
                format!("{}***{}", &local[..std::cmp::min(2, local.len())], domain)
            } else {
                format!("{}***", &value[..2])
            }
        } else {
            // Other values - show first 2 and last 2 chars
            format!("{}***{}", &value[..2], &value[value.len()-2..])
        }
    }

    /// Generate recommendations for improving credential setup
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !self.validation_results.missing_critical.is_empty() {
            recommendations.push("Set up all critical credentials before proceeding".to_string());
        }
        
        if !self.validation_results.missing_optional.is_empty() {
            recommendations.push("Consider setting up optional credentials for better job board coverage".to_string());
        }
        
        if !self.validation_results.invalid_credentials.is_empty() {
            recommendations.push("Fix invalid credential formats".to_string());
        }
        
        if self.validation_results.total_score < 80.0 {
            recommendations.push("Aim for at least 80% credential completion for optimal performance".to_string());
        }
        
        recommendations.push("Store credentials securely in environment variables or .env file".to_string());
        recommendations.push("Never commit credentials to version control".to_string());
        
        recommendations
    }

    /// Generate immediate actions required
    fn generate_immediate_actions(&self) -> Vec<String> {
        let mut actions = Vec::new();
        
        for credential in &self.validation_results.missing_critical {
            if let Some(requirement) = self.required_credentials.get(credential) {
                actions.push(format!("Set {}: {}", requirement.name, requirement.description));
            }
        }
        
        for invalid in &self.validation_results.invalid_credentials {
            actions.push(format!("Fix invalid credential: {}", invalid));
        }
        
        if actions.is_empty() && !self.validation_results.missing_optional.is_empty() {
            actions.push("Consider setting up optional credentials for better coverage".to_string());
        }
        
        actions
    }

    /// Get validation summary
    pub fn get_validation_summary(&self) -> String {
        format!(
            "Credentials: {}/{} valid ({:.1}%)",
            self.validation_results.valid_credentials.len(),
            self.required_credentials.len() + self.optional_credentials.len(),
            self.validation_results.total_score
        )
    }
}

impl ValidationType {
    fn description(&self) -> &'static str {
        match self {
            ValidationType::Email => "email address",
            ValidationType::Password => "password",
            ValidationType::FilePath => "file path",
            ValidationType::Url => "URL",
            ValidationType::ApiKey => "API key",
            ValidationType::Text => "text",
            ValidationType::Boolean => "boolean",
        }
    }
}

// Supporting data structures

#[derive(Debug, Clone)]
pub struct CredentialValidationResult {
    pub found: bool,
    pub valid: bool,
    pub source: String,
    pub value_preview: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub validation_passed: bool,
    pub critical_missing: Vec<String>,
    pub optional_missing: Vec<String>,
    pub invalid_credentials: Vec<String>,
    pub valid_credentials: Vec<String>,
    pub completion_percentage: f64,
    pub recommendations: Vec<String>,
    pub immediate_actions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_credential_validation() {
        env::set_var("TEST_EMAIL", "test@example.com");
        
        let mut validator = CredentialValidator::new();
        validator.add_required_credential("test_email", CredentialRequirement {
            name: "Test Email".to_string(),
            description: "Test email address".to_string(),
            env_var_names: vec!["TEST_EMAIL".to_string()],
            file_paths: vec![],
            validation_type: ValidationType::Email,
            critical: true,
            example: None,
        });
        
        let report = validator.validate_instantly().await.unwrap();
        assert!(report.validation_passed);
        
        env::remove_var("TEST_EMAIL");
    }

    #[test]
    fn test_email_validation() {
        let validator = CredentialValidator::new();
        
        assert!(validator.validate_credential_value("test@example.com", &ValidationType::Email));
        assert!(!validator.validate_credential_value("invalid-email", &ValidationType::Email));
    }

    #[test]
    fn test_safe_preview() {
        let validator = CredentialValidator::new();
        
        let email_preview = validator.create_safe_preview("john.doe@example.com");
        assert!(email_preview.contains("jo***@example.com"));
        
        let password_preview = validator.create_safe_preview("password123");
        assert!(email_preview.contains("***"));
    }
}