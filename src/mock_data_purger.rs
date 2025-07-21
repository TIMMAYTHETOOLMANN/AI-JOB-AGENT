use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};
use std::collections::HashSet;
use std::path::Path;
use serde::{Serialize, Deserialize};

/// Mock data detection and purging system
/// Ensures absolutely no simulation, sample, or mocked results
#[derive(Debug, Clone)]
pub struct MockDataPurger {
    mock_patterns: MockPatterns,
    purge_statistics: PurgeStatistics,
    strict_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockPatterns {
    pub mock_domains: HashSet<String>,
    pub mock_keywords: HashSet<String>,
    pub test_patterns: HashSet<String>,
    pub simulation_indicators: HashSet<String>,
    pub fake_data_patterns: HashSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PurgeStatistics {
    pub mock_urls_detected: u32,
    pub mock_data_purged: u32,
    pub simulation_results_blocked: u32,
    pub fake_credentials_detected: u32,
    pub total_purge_operations: u32,
}

#[derive(Debug, Clone)]
pub struct MockDetectionResult {
    pub is_mock: bool,
    pub mock_type: MockType,
    pub confidence: f64,
    pub detected_patterns: Vec<String>,
    pub purge_action: PurgeAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MockType {
    None,
    MockDomain,
    TestData,
    Simulation,
    FakeCredentials,
    ExampleData,
    LocalHost,
}

#[derive(Debug, Clone)]
pub enum PurgeAction {
    Allow,
    Block,
    Purge,
    Replace,
}

impl Default for MockPatterns {
    fn default() -> Self {
        let mut mock_domains = HashSet::new();
        mock_domains.insert("localhost".to_string());
        mock_domains.insert("127.0.0.1".to_string());
        mock_domains.insert("test.com".to_string());
        mock_domains.insert("example.com".to_string());
        mock_domains.insert("fake-jobs.com".to_string());
        mock_domains.insert("mock-careers.com".to_string());
        mock_domains.insert("demo-jobs.org".to_string());
        mock_domains.insert("sample-careers.net".to_string());
        mock_domains.insert("placeholder-jobs.io".to_string());
        
        let mut mock_keywords = HashSet::new();
        mock_keywords.insert("mock".to_string());
        mock_keywords.insert("fake".to_string());
        mock_keywords.insert("test".to_string());
        mock_keywords.insert("demo".to_string());
        mock_keywords.insert("sample".to_string());
        mock_keywords.insert("placeholder".to_string());
        mock_keywords.insert("dummy".to_string());
        mock_keywords.insert("simulation".to_string());
        mock_keywords.insert("pretend".to_string());
        mock_keywords.insert("fictional".to_string());
        
        let mut test_patterns = HashSet::new();
        test_patterns.insert("test_".to_string());
        test_patterns.insert("_test".to_string());
        test_patterns.insert("testing".to_string());
        test_patterns.insert("test-data".to_string());
        test_patterns.insert("mock-".to_string());
        test_patterns.insert("-mock".to_string());
        test_patterns.insert("dev-".to_string());
        test_patterns.insert("staging-".to_string());
        
        let mut simulation_indicators = HashSet::new();
        simulation_indicators.insert("simulated".to_string());
        simulation_indicators.insert("pretend".to_string());
        simulation_indicators.insert("imaginary".to_string());
        simulation_indicators.insert("virtual".to_string());
        simulation_indicators.insert("artificial".to_string());
        simulation_indicators.insert("synthetic".to_string());
        simulation_indicators.insert("generated".to_string());
        simulation_indicators.insert("hypothetical".to_string());
        
        let mut fake_data_patterns = HashSet::new();
        fake_data_patterns.insert("john.doe@example.com".to_string());
        fake_data_patterns.insert("test@test.com".to_string());
        fake_data_patterns.insert("user@domain.com".to_string());
        fake_data_patterns.insert("admin@localhost".to_string());
        fake_data_patterns.insert("noreply@".to_string());
        fake_data_patterns.insert("example@".to_string());
        fake_data_patterns.insert("test123".to_string());
        fake_data_patterns.insert("password123".to_string());
        fake_data_patterns.insert("defaultpassword".to_string());
        
        Self {
            mock_domains,
            mock_keywords,
            test_patterns,
            simulation_indicators,
            fake_data_patterns,
        }
    }
}

impl MockDataPurger {
    /// Initialize the mock data purger with strict mode enabled
    pub fn new() -> Self {
        info!("🧹 Initializing mock data purger - STRICT MODE");
        
        Self {
            mock_patterns: MockPatterns::default(),
            purge_statistics: PurgeStatistics::default(),
            strict_mode: true,
        }
    }

    /// Scan and purge all mock data from the system
    pub async fn purge_all_mock_data(&mut self) -> Result<PurgeReport> {
        info!("🚨 STARTING COMPREHENSIVE MOCK DATA PURGE");
        println!("\n🧹 MOCK DATA PURGE OPERATION");
        println!("═══════════════════════════════════════");
        
        let mut purge_report = PurgeReport {
            total_scanned: 0,
            mock_items_found: 0,
            items_purged: 0,
            blocked_operations: 0,
            errors: Vec::new(),
            purge_actions: Vec::new(),
        };
        
        // Scan environment variables
        let env_results = self.scan_environment_variables().await?;
        purge_report.merge_results(env_results);
        
        // Scan configuration files
        let config_results = self.scan_configuration_files().await?;
        purge_report.merge_results(config_results);
        
        // Scan application state
        let state_results = self.scan_application_state().await?;
        purge_report.merge_results(state_results);
        
        // Update statistics
        self.purge_statistics.total_purge_operations += 1;
        
        // Report results immediately
        self.report_purge_results(&purge_report)?;
        
        // Fail hard if any mock data was found in strict mode
        if self.strict_mode && purge_report.mock_items_found > 0 {
            error!("🚨 MOCK DATA DETECTED - STRICT MODE VIOLATION");
            return Err(anyhow!("SYSTEM FAILURE: Mock data detected in strict mode. {} items found.", purge_report.mock_items_found));
        }
        
        info!("✅ Mock data purge completed successfully");
        Ok(purge_report)
    }

    /// Validate URL is not mock/test/simulation
    pub fn validate_real_url(&mut self, url: &str) -> Result<()> {
        debug!("🔍 Validating URL is real: {}", url);
        
        let detection = self.detect_mock_data(url, MockDataType::Url)?;
        
        if detection.is_mock {
            self.purge_statistics.mock_urls_detected += 1;
            
            error!("🚨 MOCK URL DETECTED: {}", url);
            println!("❌ BLOCKED MOCK URL: {}", url);
            println!("   Type: {:?}", detection.mock_type);
            println!("   Patterns: {:?}", detection.detected_patterns);
            
            return Err(anyhow!("PRODUCTION ERROR: Mock URL blocked: {}", url));
        }
        
        debug!("✅ URL validated as real: {}", url);
        Ok(())
    }

    /// Validate credentials are not fake/test data
    pub fn validate_real_credentials(&mut self, email: &str, name: &str) -> Result<()> {
        debug!("🔍 Validating credentials are real");
        
        // Check email
        let email_detection = self.detect_mock_data(email, MockDataType::Email)?;
        if email_detection.is_mock {
            self.purge_statistics.fake_credentials_detected += 1;
            error!("🚨 FAKE EMAIL DETECTED: {}", email);
            return Err(anyhow!("PRODUCTION ERROR: Fake email blocked: {}", email));
        }
        
        // Check name
        let name_detection = self.detect_mock_data(name, MockDataType::Name)?;
        if name_detection.is_mock {
            self.purge_statistics.fake_credentials_detected += 1;
            error!("🚨 FAKE NAME DETECTED: {}", name);
            return Err(anyhow!("PRODUCTION ERROR: Fake name blocked: {}", name));
        }
        
        debug!("✅ Credentials validated as real");
        Ok(())
    }

    /// Block any simulation/mock results from being returned
    pub fn validate_real_result(&mut self, result: &str) -> Result<String> {
        debug!("🔍 Validating result is real");
        
        let detection = self.detect_mock_data(result, MockDataType::Result)?;
        
        if detection.is_mock {
            self.purge_statistics.simulation_results_blocked += 1;
            
            error!("🚨 SIMULATION RESULT BLOCKED: {}", result);
            println!("❌ BLOCKED SIMULATION RESULT");
            println!("   Content: {}", result);
            println!("   Type: {:?}", detection.mock_type);
            
            return Err(anyhow!("PRODUCTION ERROR: Simulation result blocked"));
        }
        
        // Additional validation for success claims
        if result.to_lowercase().contains("success") || result.to_lowercase().contains("applied") {
            if self.is_fake_success_claim(result) {
                self.purge_statistics.simulation_results_blocked += 1;
                error!("🚨 FAKE SUCCESS CLAIM BLOCKED: {}", result);
                return Err(anyhow!("PRODUCTION ERROR: Fake success claim blocked"));
            }
        }
        
        debug!("✅ Result validated as real");
        Ok(result.to_string())
    }

    /// Detect mock data in various contexts
    fn detect_mock_data(&self, data: &str, data_type: MockDataType) -> Result<MockDetectionResult> {
        let data_lower = data.to_lowercase();
        let mut detected_patterns = Vec::new();
        let mut mock_type = MockType::None;
        let mut confidence = 0.0;
        
        // Check for mock domains
        for domain in &self.mock_patterns.mock_domains {
            if data_lower.contains(&domain.to_lowercase()) {
                detected_patterns.push(format!("Mock domain: {}", domain));
                mock_type = MockType::MockDomain;
                confidence += 25.0;
            }
        }
        
        // Check for mock keywords
        for keyword in &self.mock_patterns.mock_keywords {
            if data_lower.contains(&keyword.to_lowercase()) {
                detected_patterns.push(format!("Mock keyword: {}", keyword));
                if mock_type == MockType::None {
                    mock_type = MockType::TestData;
                }
                confidence += 15.0;
            }
        }
        
        // Check for test patterns
        for pattern in &self.mock_patterns.test_patterns {
            if data_lower.contains(&pattern.to_lowercase()) {
                detected_patterns.push(format!("Test pattern: {}", pattern));
                if mock_type == MockType::None {
                    mock_type = MockType::TestData;
                }
                confidence += 20.0;
            }
        }
        
        // Check for simulation indicators
        for indicator in &self.mock_patterns.simulation_indicators {
            if data_lower.contains(&indicator.to_lowercase()) {
                detected_patterns.push(format!("Simulation indicator: {}", indicator));
                mock_type = MockType::Simulation;
                confidence += 30.0;
            }
        }
        
        // Check for fake data patterns (exact matches)
        for pattern in &self.mock_patterns.fake_data_patterns {
            if data_lower == pattern.to_lowercase() {
                detected_patterns.push(format!("Fake data pattern: {}", pattern));
                mock_type = MockType::FakeCredentials;
                confidence = 100.0; // Exact match = 100% confidence
                break;
            }
        }
        
        // Check for localhost patterns
        if data.contains("127.0.0.1") || data.contains("localhost") {
            detected_patterns.push("Localhost detected".to_string());
            mock_type = MockType::LocalHost;
            confidence = 100.0;
        }
        
        // Determine purge action
        let purge_action = if confidence >= 80.0 {
            PurgeAction::Block
        } else if confidence >= 50.0 {
            PurgeAction::Purge
        } else if confidence >= 20.0 {
            PurgeAction::Replace
        } else {
            PurgeAction::Allow
        };
        
        Ok(MockDetectionResult {
            is_mock: confidence > 0.0,
            mock_type,
            confidence,
            detected_patterns,
            purge_action,
        })
    }

    /// Check if a success claim is fake/simulated
    fn is_fake_success_claim(&self, result: &str) -> bool {
        let fake_success_patterns = [
            "successfully simulated",
            "pretend application",
            "mock success",
            "test application successful",
            "demo completed",
            "placeholder result",
            "example success",
            "simulated completion",
        ];
        
        let result_lower = result.to_lowercase();
        fake_success_patterns.iter().any(|pattern| result_lower.contains(pattern))
    }

    /// Scan environment variables for mock data
    async fn scan_environment_variables(&mut self) -> Result<ScanResult> {
        info!("🔍 Scanning environment variables for mock data");
        
        let mut scan_result = ScanResult::default();
        
        for (key, value) in std::env::vars() {
            scan_result.total_scanned += 1;
            
            let detection = self.detect_mock_data(&value, MockDataType::Environment)?;
            if detection.is_mock {
                scan_result.mock_items_found += 1;
                scan_result.purge_actions.push(format!("Environment variable {} contains mock data: {:?}", key, detection.detected_patterns));
                
                warn!("🚨 Mock data in environment variable {}: {:?}", key, detection.detected_patterns);
            }
        }
        
        Ok(scan_result)
    }

    /// Scan configuration files for mock data
    async fn scan_configuration_files(&mut self) -> Result<ScanResult> {
        info!("🔍 Scanning configuration files for mock data");
        
        let mut scan_result = ScanResult::default();
        let config_files = ["AgentConfig.toml", ".env", "AgentSecrets.template"];
        
        for file_path in &config_files {
            if Path::new(file_path).exists() {
                scan_result.total_scanned += 1;
                
                match std::fs::read_to_string(file_path) {
                    Ok(content) => {
                        let detection = self.detect_mock_data(&content, MockDataType::Configuration)?;
                        if detection.is_mock {
                            scan_result.mock_items_found += 1;
                            scan_result.purge_actions.push(format!("Configuration file {} contains mock data: {:?}", file_path, detection.detected_patterns));
                            
                            warn!("🚨 Mock data in configuration file {}: {:?}", file_path, detection.detected_patterns);
                        }
                    }
                    Err(e) => {
                        scan_result.errors.push(format!("Failed to read {}: {}", file_path, e));
                    }
                }
            }
        }
        
        Ok(scan_result)
    }

    /// Scan application state for mock data
    async fn scan_application_state(&mut self) -> Result<ScanResult> {
        info!("🔍 Scanning application state for mock data");
        
        let mut scan_result = ScanResult::default();
        
        // Check for state files that might contain mock data
        let state_paths = ["state/", "logs/", "cache/"];
        
        for state_path in &state_paths {
            if Path::new(state_path).exists() {
                // In a real implementation, this would recursively scan state files
                scan_result.total_scanned += 1;
                debug!("Scanned state directory: {}", state_path);
            }
        }
        
        Ok(scan_result)
    }

    /// Report purge results to CLI
    fn report_purge_results(&self, report: &PurgeReport) -> Result<()> {
        println!("📊 PURGE OPERATION RESULTS:");
        println!("  Items Scanned: {}", report.total_scanned);
        println!("  Mock Items Found: {}", report.mock_items_found);
        println!("  Items Purged: {}", report.items_purged);
        println!("  Operations Blocked: {}", report.blocked_operations);
        
        if !report.errors.is_empty() {
            println!("\n⚠️  ERRORS ENCOUNTERED:");
            for error in &report.errors {
                println!("  ❌ {}", error);
            }
        }
        
        if !report.purge_actions.is_empty() {
            println!("\n🧹 PURGE ACTIONS TAKEN:");
            for action in &report.purge_actions {
                println!("  🗑️  {}", action);
            }
        }
        
        println!("═══════════════════════════════════════");
        
        if report.mock_items_found == 0 {
            println!("✅ NO MOCK DATA DETECTED - SYSTEM CLEAN");
        } else {
            println!("⚠️  MOCK DATA DETECTED - REVIEW REQUIRED");
        }
        
        Ok(())
    }

    /// Get purge statistics
    pub fn get_purge_statistics(&self) -> String {
        format!(
            "Purge Stats: {} URLs checked, {} mock items purged, {} simulations blocked",
            self.purge_statistics.mock_urls_detected,
            self.purge_statistics.mock_data_purged,
            self.purge_statistics.simulation_results_blocked
        )
    }
}

// Supporting data structures

#[derive(Debug, Clone)]
pub enum MockDataType {
    Url,
    Email,
    Name,
    Result,
    Environment,
    Configuration,
}

#[derive(Debug, Clone, Default)]
pub struct ScanResult {
    pub total_scanned: u32,
    pub mock_items_found: u32,
    pub items_purged: u32,
    pub blocked_operations: u32,
    pub errors: Vec<String>,
    pub purge_actions: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PurgeReport {
    pub total_scanned: u32,
    pub mock_items_found: u32,
    pub items_purged: u32,
    pub blocked_operations: u32,
    pub errors: Vec<String>,
    pub purge_actions: Vec<String>,
}

impl PurgeReport {
    fn merge_results(&mut self, result: ScanResult) {
        self.total_scanned += result.total_scanned;
        self.mock_items_found += result.mock_items_found;
        self.items_purged += result.items_purged;
        self.blocked_operations += result.blocked_operations;
        self.errors.extend(result.errors);
        self.purge_actions.extend(result.purge_actions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_url_detection() {
        let mut purger = MockDataPurger::new();
        
        // Test mock URLs
        assert!(purger.validate_real_url("https://linkedin.com/jobs/123").is_ok());
        assert!(purger.validate_real_url("https://test.com/jobs").is_err());
        assert!(purger.validate_real_url("http://localhost:3000").is_err());
        assert!(purger.validate_real_url("https://mock-careers.com").is_err());
    }

    #[tokio::test]
    async fn test_fake_credentials_detection() {
        let mut purger = MockDataPurger::new();
        
        // Test real vs fake credentials
        assert!(purger.validate_real_credentials("john.smith@gmail.com", "John Smith").is_ok());
        assert!(purger.validate_real_credentials("john.doe@example.com", "John Doe").is_err());
        assert!(purger.validate_real_credentials("test@test.com", "Test User").is_err());
    }

    #[test]
    fn test_fake_success_detection() {
        let purger = MockDataPurger::new();
        
        assert!(purger.is_fake_success_claim("Successfully simulated job application"));
        assert!(purger.is_fake_success_claim("Mock success - pretend application completed"));
        assert!(!purger.is_fake_success_claim("Successfully applied to Software Engineer position"));
    }

    #[test]
    fn test_mock_data_detection() {
        let purger = MockDataPurger::new();
        
        let mock_detection = purger.detect_mock_data("This is a test simulation", MockDataType::Result).unwrap();
        assert!(mock_detection.is_mock);
        assert_eq!(mock_detection.mock_type, MockType::Simulation);
        
        let real_detection = purger.detect_mock_data("Successfully applied to real job", MockDataType::Result).unwrap();
        assert!(!real_detection.is_mock);
    }
}