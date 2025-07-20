use anyhow::{Result, Context};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{info, debug, warn, error};
use serde::{Deserialize, Serialize};

#[cfg(feature = "headless-chrome")]
use crate::headless_browser::HeadlessBrowser;

#[cfg(all(feature = "mock-browser", not(feature = "headless-chrome")))]
use crate::mock_browser::HeadlessBrowser;

#[cfg(all(not(feature = "headless-chrome"), not(feature = "mock-browser")))]
type HeadlessBrowser = crate::mock_browser::HeadlessBrowser;

/// Adaptive browser strategies for different job sites and scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveBrowserConfig {
    pub site_strategies: HashMap<String, SiteStrategy>,
    pub default_strategy: SiteStrategy,
    pub retry_configs: RetryConfig,
    pub performance_thresholds: PerformanceThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteStrategy {
    pub selectors: SelectorConfig,
    pub timing: TimingConfig,
    pub error_handling: ErrorHandlingConfig,
    pub authentication: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectorConfig {
    pub apply_button: Vec<String>,
    pub email_field: Vec<String>,
    pub name_field: Vec<String>,
    pub resume_upload: Vec<String>,
    pub cover_letter_upload: Vec<String>,
    pub submit_button: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingConfig {
    pub page_load_timeout: u64,
    pub element_wait_timeout: u64,
    pub typing_delay: u64,
    pub click_delay: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlingConfig {
    pub max_retries: u32,
    pub retry_delay: u64,
    pub fallback_selectors: bool,
    pub screenshot_on_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub login_required: bool,
    pub login_url: Option<String>,
    pub username_selector: Option<String>,
    pub password_selector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: u64,
    pub exponential_backoff: bool,
    pub max_delay: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_page_load_time: u64,
    pub max_application_time: u64,
    pub success_rate_threshold: f64,
}

impl Default for AdaptiveBrowserConfig {
    fn default() -> Self {
        let mut site_strategies = HashMap::new();
        
        // LinkedIn strategy
        site_strategies.insert("linkedin.com".to_string(), SiteStrategy {
            selectors: SelectorConfig {
                apply_button: vec![
                    "button[aria-label*='Easy Apply']".to_string(),
                    ".jobs-apply-button".to_string(),
                    "[data-test='apply-button']".to_string(),
                ],
                email_field: vec![
                    "input[type='email']".to_string(),
                    "#email".to_string(),
                    "[data-test='email-input']".to_string(),
                ],
                name_field: vec![
                    "input[name='name']".to_string(),
                    "#name".to_string(),
                    "[data-test='name-input']".to_string(),
                ],
                resume_upload: vec![
                    "input[type='file'][accept*='pdf']".to_string(),
                    "[data-test='resume-upload']".to_string(),
                ],
                cover_letter_upload: vec![
                    "input[type='file'][name*='cover']".to_string(),
                    "[data-test='cover-letter-upload']".to_string(),
                ],
                submit_button: vec![
                    "button[type='submit']".to_string(),
                    "[data-test='submit-application']".to_string(),
                ],
            },
            timing: TimingConfig {
                page_load_timeout: 30000,
                element_wait_timeout: 10000,
                typing_delay: 100,
                click_delay: 500,
            },
            error_handling: ErrorHandlingConfig {
                max_retries: 3,
                retry_delay: 2000,
                fallback_selectors: true,
                screenshot_on_error: true,
            },
            authentication: AuthConfig {
                login_required: true,
                login_url: Some("https://www.linkedin.com/login".to_string()),
                username_selector: Some("#username".to_string()),
                password_selector: Some("#password".to_string()),
            },
        });

        // Indeed strategy
        site_strategies.insert("indeed.com".to_string(), SiteStrategy {
            selectors: SelectorConfig {
                apply_button: vec![
                    "[data-jk] .indeedApplyButton".to_string(),
                    ".jobsearch-SerpJobCard .indeedApplyButton".to_string(),
                    "button:contains('Apply')".to_string(),
                ],
                email_field: vec![
                    "input[type='email']".to_string(),
                    "#applicant.email".to_string(),
                ],
                name_field: vec![
                    "input[name='applicant.name']".to_string(),
                    "#applicant\\.name".to_string(),
                ],
                resume_upload: vec![
                    "input[type='file'][name='resume']".to_string(),
                    "#resume".to_string(),
                ],
                cover_letter_upload: vec![
                    "input[type='file'][name='coverLetter']".to_string(),
                ],
                submit_button: vec![
                    "input[type='submit'][value*='Apply']".to_string(),
                    "button[type='submit']".to_string(),
                ],
            },
            timing: TimingConfig {
                page_load_timeout: 25000,
                element_wait_timeout: 8000,
                typing_delay: 80,
                click_delay: 300,
            },
            error_handling: ErrorHandlingConfig {
                max_retries: 2,
                retry_delay: 1500,
                fallback_selectors: true,
                screenshot_on_error: false,
            },
            authentication: AuthConfig {
                login_required: false,
                login_url: None,
                username_selector: None,
                password_selector: None,
            },
        });

        // Default strategy for unknown sites
        let default_strategy = SiteStrategy {
            selectors: SelectorConfig {
                apply_button: vec![
                    "button:contains('Apply')".to_string(),
                    "input[type='submit'][value*='Apply']".to_string(),
                    ".apply-button".to_string(),
                    "#apply".to_string(),
                ],
                email_field: vec![
                    "input[type='email']".to_string(),
                    "input[name*='email']".to_string(),
                    "#email".to_string(),
                ],
                name_field: vec![
                    "input[name*='name']".to_string(),
                    "input[name*='full']".to_string(),
                    "#name".to_string(),
                ],
                resume_upload: vec![
                    "input[type='file'][accept*='pdf']".to_string(),
                    "input[type='file'][name*='resume']".to_string(),
                    "input[type='file'][name*='cv']".to_string(),
                ],
                cover_letter_upload: vec![
                    "input[type='file'][name*='cover']".to_string(),
                    "input[type='file'][name*='letter']".to_string(),
                ],
                submit_button: vec![
                    "button[type='submit']".to_string(),
                    "input[type='submit']".to_string(),
                ],
            },
            timing: TimingConfig {
                page_load_timeout: 20000,
                element_wait_timeout: 5000,
                typing_delay: 50,
                click_delay: 200,
            },
            error_handling: ErrorHandlingConfig {
                max_retries: 2,
                retry_delay: 1000,
                fallback_selectors: true,
                screenshot_on_error: false,
            },
            authentication: AuthConfig {
                login_required: false,
                login_url: None,
                username_selector: None,
                password_selector: None,
            },
        };

        AdaptiveBrowserConfig {
            site_strategies,
            default_strategy,
            retry_configs: RetryConfig {
                max_attempts: 3,
                base_delay: 1000,
                exponential_backoff: true,
                max_delay: 10000,
            },
            performance_thresholds: PerformanceThresholds {
                max_page_load_time: 30000,
                max_application_time: 120000,
                success_rate_threshold: 0.8,
            },
        }
    }
}

/// Adaptive browser automation engine
pub struct AdaptiveBrowserEngine {
    config: AdaptiveBrowserConfig,
    performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    pub total_attempts: u32,
    pub successful_attempts: u32,
    pub failed_attempts: u32,
    pub average_page_load_time: f64,
    pub average_application_time: f64,
    pub site_specific_metrics: HashMap<String, SiteMetrics>,
}

#[derive(Debug, Default)]
pub struct SiteMetrics {
    pub attempts: u32,
    pub successes: u32,
    pub failures: u32,
    pub avg_load_time: f64,
    pub last_success: Option<SystemTime>,
    pub last_failure: Option<SystemTime>,
}

impl AdaptiveBrowserEngine {
    pub fn new(config: AdaptiveBrowserConfig) -> Self {
        Self {
            config,
            performance_metrics: PerformanceMetrics::default(),
        }
    }

    /// Get the appropriate strategy for a given URL
    pub fn get_strategy_for_url(&self, url: &str) -> &SiteStrategy {
        for (domain, strategy) in &self.config.site_strategies {
            if url.contains(domain) {
                debug!("Using specialized strategy for domain: {}", domain);
                return strategy;
            }
        }
        debug!("Using default strategy for URL: {}", url);
        &self.config.default_strategy
    }

    /// Adaptively apply to a job with retries and fallbacks (for visible browser)
    pub async fn adaptive_apply_visible(
        &mut self,
        browser: &crate::visible_browser::VisibleBrowser,
        url: &str,
        name: &str,
        email: &str,
        resume_path: &str,
        cover_letter_path: &str,
    ) -> Result<String> {
        let start_time = SystemTime::now();
        let strategy = self.get_strategy_for_url(url).clone();
        let domain = self.extract_domain(url);

        self.update_attempt_metrics(&domain);

        let mut last_error = None;
        
        for attempt in 1..=strategy.error_handling.max_retries {
            match self.try_apply_with_strategy_visible(
                browser, 
                url, 
                name, 
                email, 
                resume_path, 
                cover_letter_path, 
                &strategy
            ).await {
                Ok(result) => {
                    let duration = start_time.elapsed().unwrap_or(Duration::ZERO);
                    self.update_success_metrics(&domain, duration);
                    info!("Successfully applied to {} on attempt {}", url, attempt);
                    return Ok(result);
                },
                Err(e) => {
                    last_error = Some(e);
                    warn!("Attempt {} failed for {}: {}", attempt, url, last_error.as_ref().unwrap());
                    
                    if attempt < strategy.error_handling.max_retries {
                        let delay = self.calculate_retry_delay(attempt, &strategy.error_handling);
                        info!("Retrying in {}ms...", delay);
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                    }
                }
            }
        }

        let duration = start_time.elapsed().unwrap_or(Duration::ZERO);
        self.update_failure_metrics(&domain, duration);
        
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All retry attempts failed for {}", url)))
    }
    
    /// Try to apply using a specific strategy (for visible browser)
    async fn try_apply_with_strategy_visible(
        &self,
        browser: &crate::visible_browser::VisibleBrowser,
        url: &str,
        name: &str,
        email: &str,
        _resume_path: &str,
        _cover_letter_path: &str,
        strategy: &SiteStrategy,
    ) -> Result<String> {
        // Navigate to the job page
        browser.navigate_to(url)
            .context("Failed to navigate to job page")?;

        // Wait for page to load
        tokio::time::sleep(Duration::from_millis(strategy.timing.page_load_timeout)).await;

        // Try to find and click apply button with multiple selectors
        let mut apply_clicked = false;
        for selector in &strategy.selectors.apply_button {
            if let Ok(_) = browser.click_element(selector) {
                apply_clicked = true;
                debug!("Successfully clicked apply button with selector: {}", selector);
                break;
            }
        }

        if !apply_clicked {
            return Err(anyhow::anyhow!("Could not find apply button with any selector"));
        }

        // Wait for application form to load
        tokio::time::sleep(Duration::from_millis(strategy.timing.element_wait_timeout)).await;

        // Fill out name field
        let mut name_filled = false;
        for selector in &strategy.selectors.name_field {
            if let Ok(_) = browser.type_text(selector, name) {
                name_filled = true;
                debug!("Successfully filled name with selector: {}", selector);
                break;
            }
        }

        // Fill out email field
        let mut email_filled = false;
        for selector in &strategy.selectors.email_field {
            if let Ok(_) = browser.type_text(selector, email) {
                email_filled = true;
                debug!("Successfully filled email with selector: {}", selector);
                break;
            }
        }

        // Try to submit the application
        let mut submitted = false;
        for selector in &strategy.selectors.submit_button {
            if let Ok(_) = browser.click_element(selector) {
                submitted = true;
                debug!("Successfully submitted with selector: {}", selector);
                break;
            }
        }

        if !submitted {
            return Err(anyhow::anyhow!("Could not submit application"));
        }

        Ok(format!("Application submitted successfully to {} (Name: {}, Email: {})", 
                  url, name_filled, email_filled))
    }
    pub async fn adaptive_apply(
        &mut self,
        browser: &HeadlessBrowser,
        url: &str,
        name: &str,
        email: &str,
        resume_path: &str,
        cover_letter_path: &str,
    ) -> Result<String> {
        let start_time = SystemTime::now();
        let strategy = self.get_strategy_for_url(url).clone();
        let domain = self.extract_domain(url);

        self.update_attempt_metrics(&domain);

        let mut last_error = None;
        
        for attempt in 1..=strategy.error_handling.max_retries {
            match self.try_apply_with_strategy(
                browser, 
                url, 
                name, 
                email, 
                resume_path, 
                cover_letter_path, 
                &strategy
            ).await {
                Ok(result) => {
                    let duration = start_time.elapsed().unwrap_or(Duration::ZERO);
                    self.update_success_metrics(&domain, duration);
                    info!("Successfully applied to {} on attempt {}", url, attempt);
                    return Ok(result);
                },
                Err(e) => {
                    last_error = Some(e);
                    warn!("Attempt {} failed for {}: {}", attempt, url, last_error.as_ref().unwrap());
                    
                    if attempt < strategy.error_handling.max_retries {
                        let delay = self.calculate_retry_delay(attempt, &strategy.error_handling);
                        info!("Retrying in {}ms...", delay);
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                    }
                }
            }
        }

        let duration = start_time.elapsed().unwrap_or(Duration::ZERO);
        self.update_failure_metrics(&domain, duration);
        
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All retry attempts failed for {}", url)))
    }

    /// Try to apply using a specific strategy
    async fn try_apply_with_strategy(
        &self,
        browser: &HeadlessBrowser,
        url: &str,
        name: &str,
        email: &str,
        _resume_path: &str,
        _cover_letter_path: &str,
        strategy: &SiteStrategy,
    ) -> Result<String> {
        // Navigate to the job page
        browser.navigate_to(url)
            .context("Failed to navigate to job page")?;

        // Wait for page to load
        tokio::time::sleep(Duration::from_millis(strategy.timing.page_load_timeout)).await;

        // Try to find and click apply button with multiple selectors
        let mut apply_clicked = false;
        for selector in &strategy.selectors.apply_button {
            if let Ok(_) = browser.click_element(selector) {
                apply_clicked = true;
                debug!("Successfully clicked apply button with selector: {}", selector);
                break;
            }
        }

        if !apply_clicked {
            return Err(anyhow::anyhow!("Could not find apply button with any selector"));
        }

        // Wait for application form to load
        tokio::time::sleep(Duration::from_millis(strategy.timing.element_wait_timeout)).await;

        // Fill out name field
        let mut name_filled = false;
        for selector in &strategy.selectors.name_field {
            if let Ok(_) = browser.type_text(selector, name) {
                name_filled = true;
                debug!("Successfully filled name with selector: {}", selector);
                break;
            }
        }

        // Fill out email field
        let mut email_filled = false;
        for selector in &strategy.selectors.email_field {
            if let Ok(_) = browser.type_text(selector, email) {
                email_filled = true;
                debug!("Successfully filled email with selector: {}", selector);
                break;
            }
        }

        // Try to submit the application
        let mut submitted = false;
        for selector in &strategy.selectors.submit_button {
            if let Ok(_) = browser.click_element(selector) {
                submitted = true;
                debug!("Successfully submitted with selector: {}", selector);
                break;
            }
        }

        if !submitted {
            return Err(anyhow::anyhow!("Could not submit application"));
        }

        Ok(format!("Application submitted successfully to {} (Name: {}, Email: {})", 
                  url, name_filled, email_filled))
    }

    fn calculate_retry_delay(&self, attempt: u32, error_config: &ErrorHandlingConfig) -> u64 {
        if self.config.retry_configs.exponential_backoff {
            let delay = self.config.retry_configs.base_delay * (2_u64.pow(attempt - 1));
            std::cmp::min(delay, self.config.retry_configs.max_delay)
        } else {
            error_config.retry_delay
        }
    }

    fn extract_domain(&self, url: &str) -> String {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(domain) = parsed_url.host_str() {
                return domain.to_string();
            }
        }
        "unknown".to_string()
    }

    fn update_attempt_metrics(&mut self, domain: &str) {
        self.performance_metrics.total_attempts += 1;
        
        let site_metrics = self.performance_metrics.site_specific_metrics
            .entry(domain.to_string())
            .or_insert_with(SiteMetrics::default);
        site_metrics.attempts += 1;
    }

    fn update_success_metrics(&mut self, domain: &str, duration: Duration) {
        self.performance_metrics.successful_attempts += 1;
        
        let site_metrics = self.performance_metrics.site_specific_metrics
            .entry(domain.to_string())
            .or_insert_with(SiteMetrics::default);
        site_metrics.successes += 1;
        site_metrics.last_success = Some(SystemTime::now());
        
        // Update timing metrics
        let duration_ms = duration.as_millis() as f64;
        self.performance_metrics.average_application_time = 
            (self.performance_metrics.average_application_time * (self.performance_metrics.successful_attempts - 1) as f64 + duration_ms) 
            / self.performance_metrics.successful_attempts as f64;
    }

    fn update_failure_metrics(&mut self, domain: &str, _duration: Duration) {
        self.performance_metrics.failed_attempts += 1;
        
        let site_metrics = self.performance_metrics.site_specific_metrics
            .entry(domain.to_string())
            .or_insert_with(SiteMetrics::default);
        site_metrics.failures += 1;
        site_metrics.last_failure = Some(SystemTime::now());
    }

    /// Get current success rate
    pub fn get_success_rate(&self) -> f64 {
        if self.performance_metrics.total_attempts == 0 {
            return 0.0;
        }
        self.performance_metrics.successful_attempts as f64 / self.performance_metrics.total_attempts as f64
    }

    /// Get performance report
    pub fn get_performance_report(&self) -> String {
        format!(
            "Performance Report:\n\
             Total Attempts: {}\n\
             Successful: {}\n\
             Failed: {}\n\
             Success Rate: {:.2}%\n\
             Average Application Time: {:.2}ms\n\
             Sites Tracked: {}",
            self.performance_metrics.total_attempts,
            self.performance_metrics.successful_attempts,
            self.performance_metrics.failed_attempts,
            self.get_success_rate() * 100.0,
            self.performance_metrics.average_application_time,
            self.performance_metrics.site_specific_metrics.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_selection() {
        let engine = AdaptiveBrowserEngine::new(AdaptiveBrowserConfig::default());
        
        let linkedin_strategy = engine.get_strategy_for_url("https://linkedin.com/jobs/123");
        assert!(linkedin_strategy.selectors.apply_button.contains(&"button[aria-label*='Easy Apply']".to_string()));
        
        let indeed_strategy = engine.get_strategy_for_url("https://indeed.com/job/456");
        assert!(indeed_strategy.selectors.apply_button.contains(&"[data-jk] .indeedApplyButton".to_string()));
        
        let unknown_strategy = engine.get_strategy_for_url("https://unknown-site.com/job/789");
        assert!(unknown_strategy.selectors.apply_button.contains(&"button:contains('Apply')".to_string()));
    }

    #[test]
    fn test_retry_delay_calculation() {
        let engine = AdaptiveBrowserEngine::new(AdaptiveBrowserConfig::default());
        let error_config = ErrorHandlingConfig {
            max_retries: 3,
            retry_delay: 1000,
            fallback_selectors: true,
            screenshot_on_error: false,
        };
        
        assert_eq!(engine.calculate_retry_delay(1, &error_config), 1000);
        assert_eq!(engine.calculate_retry_delay(2, &error_config), 2000);
        assert_eq!(engine.calculate_retry_delay(3, &error_config), 4000);
    }

    #[test]
    fn test_domain_extraction() {
        let engine = AdaptiveBrowserEngine::new(AdaptiveBrowserConfig::default());
        
        assert_eq!(engine.extract_domain("https://linkedin.com/jobs/123"), "linkedin.com");
        assert_eq!(engine.extract_domain("https://indeed.com/job/456"), "indeed.com");
        assert_eq!(engine.extract_domain("invalid-url"), "unknown");
    }
}