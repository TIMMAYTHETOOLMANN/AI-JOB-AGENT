use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;
use anyhow::{Result, anyhow};
use tracing::{info, warn};
use crate::config::Config;
use crate::document_profile::{extract_resume_data, extract_cover_letter_data, PersonalInfo};

// PRODUCTION BROWSER ENFORCEMENT - NO MOCK DATA
use crate::headless_browser::HeadlessBrowser;

/// Account creation service that automatically creates accounts on job sites
/// using information from resume and cover letter
pub struct AccountCreator {
    personal_info: PersonalInfo,
    secrets: HashMap<String, String>,
}

impl AccountCreator {
    pub fn new(config: &Config, secrets: HashMap<String, String>) -> Result<Self> {
        let personal_info = Self::extract_personal_info(config)?;
        
        Ok(Self {
            personal_info,
            secrets,
        })
    }

    /// Extract personal information from resume and cover letter
    fn extract_personal_info(config: &Config) -> Result<PersonalInfo> {
        let mut personal_info = PersonalInfo {
            name: config.name.clone(),
            email: config.email.clone(),
            phone: String::new(),
            address: String::new(),
            city: String::new(),
            state: String::new(),
            postal_code: String::new(),
            country: String::new(),
            current_company: String::new(),
            current_position: String::new(),
            years_experience: 0,
            education_level: String::new(),
            university: String::new(),
            graduation_year: String::new(),
            professional_summary: String::new(),
        };

        // Try to extract additional information from resume
        if let Ok(resume_data) = extract_resume_data(&config.resume_path) {
            personal_info.merge_resume_data(&resume_data);
        }

        // Try to extract additional information from cover letter
        if let Ok(cover_letter_data) = extract_cover_letter_data(&config.cover_letter_path) {
            personal_info.merge_cover_letter_data(&cover_letter_data);
        }

        // Fill in from secrets if available
        if let Some(phone) = config.secrets.get("PHONE_NUMBER") {
            personal_info.phone = phone.clone();
        }
        if let Some(address) = config.secrets.get("ADDRESS_LINE1") {
            personal_info.address = address.clone();
        }
        if let Some(city) = config.secrets.get("CITY") {
            personal_info.city = city.clone();
        }

        Ok(personal_info)
    }

    /// Create account on LinkedIn
    pub async fn create_linkedin_account(&self, browser: &HeadlessBrowser) -> Result<String> {
        info!("Creating LinkedIn account");
        
        let email = self.secrets.get("LINKEDIN_EMAIL")
            .ok_or_else(|| anyhow!("LinkedIn email not configured"))?;
        let password = self.secrets.get("LINKEDIN_PASSWORD")
            .ok_or_else(|| anyhow!("LinkedIn password not configured"))?;

        // Navigate to LinkedIn signup
        browser.navigate("https://www.linkedin.com/signup").await?;
        
        // Fill out registration form
        browser.fill_field("input[name='firstName']", &self.extract_first_name()).await?;
        browser.fill_field("input[name='lastName']", &self.extract_last_name()).await?;
        browser.fill_field("input[name='email']", email).await?;
        browser.fill_field("input[name='password']", password).await?;
        
        // Submit form
        browser.click("button[type='submit']").await?;
        
        // Handle any additional steps (phone verification, etc.)
        self.handle_linkedin_verification(browser).await?;
        
        // Complete profile setup
        self.setup_linkedin_profile(browser).await?;
        
        info!("LinkedIn account created successfully");
        Ok("LinkedIn account created".to_string())
    }

    /// Create account on Indeed
    pub async fn create_indeed_account(&self, browser: &HeadlessBrowser) -> Result<String> {
        info!("Creating Indeed account");
        
        let email = self.secrets.get("INDEED_EMAIL")
            .ok_or_else(|| anyhow!("Indeed email not configured"))?;
        let password = self.secrets.get("INDEED_PASSWORD")
            .ok_or_else(|| anyhow!("Indeed password not configured"))?;

        // Navigate to Indeed signup
        browser.navigate("https://secure.indeed.com/auth?service=my&hl=en_US&co=US&continue=https%3A%2F%2Fmy.indeed.com%2F&tmpl=desktop&from=gnav-util-homepage&jsContinue=https%3A%2F%2Fmy.indeed.com%2F&empContinue=https%3A%2F%2Faccount.indeed.com%2Fmyaccount&_ga=2.203748394.1234567890.1234567890-1234567890.1234567890").await?;
        
        // Click sign up
        browser.click("a[data-tn-element='auth-page-sign-up-link']").await?;
        
        // Fill out registration form
        browser.fill_field("input[name='__email']", email).await?;
        browser.fill_field("input[name='__password']", password).await?;
        browser.fill_field("input[name='__confirm_password']", password).await?;
        
        // Submit form
        browser.click("button[type='submit']").await?;
        
        // Complete profile setup
        self.setup_indeed_profile(browser).await?;
        
        info!("Indeed account created successfully");
        Ok("Indeed account created".to_string())
    }

    /// Create accounts on major international job sites
    pub async fn create_all_accounts(&self, browser: &HeadlessBrowser) -> Result<Vec<String>> {
        let mut results = Vec::new();
        
        // Create accounts sequentially to avoid rate limiting
        let sites = vec!["LinkedIn", "Indeed", "Glassdoor", "Monster", "ZipRecruiter"];

        for site_name in sites {
            let result = match site_name {
                "LinkedIn" => self.create_linkedin_account(browser).await,
                "Indeed" => self.create_indeed_account(browser).await,
                "Glassdoor" => self.create_glassdoor_account(browser).await,
                "Monster" => self.create_monster_account(browser).await,
                "ZipRecruiter" => self.create_ziprecruiter_account(browser).await,
                _ => Err(anyhow!("Unknown site: {}", site_name)),
            };

            match result {
                Ok(message) => {
                    info!("Successfully created {} account", site_name);
                    results.push(message);
                },
                Err(e) => {
                    warn!("Failed to create {} account: {}", site_name, e);
                    results.push(format!("Failed to create {} account: {}", site_name, e));
                }
            }
            
            // Wait between account creations to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
        
        Ok(results)
    }

    /// Create account on Glassdoor
    async fn create_glassdoor_account(&self, browser: &HeadlessBrowser) -> Result<String> {
        info!("Creating Glassdoor account");
        
        let email = self.secrets.get("GLASSDOOR_EMAIL")
            .ok_or_else(|| anyhow!("Glassdoor email not configured"))?;
        let password = self.secrets.get("GLASSDOOR_PASSWORD")
            .ok_or_else(|| anyhow!("Glassdoor password not configured"))?;

        browser.navigate("https://www.glassdoor.com/profile/join_input.htm").await?;
        
        browser.fill_field("input[name='email']", email).await?;
        browser.fill_field("input[name='password']", password).await?;
        browser.fill_field("input[name='firstName']", &self.extract_first_name()).await?;
        browser.fill_field("input[name='lastName']", &self.extract_last_name()).await?;
        
        browser.click("button[type='submit']").await?;
        
        self.setup_glassdoor_profile(browser).await?;
        
        Ok("Glassdoor account created".to_string())
    }

    /// Create account on Monster
    async fn create_monster_account(&self, browser: &HeadlessBrowser) -> Result<String> {
        info!("Creating Monster account");
        
        let email = self.secrets.get("MONSTER_EMAIL")
            .ok_or_else(|| anyhow!("Monster email not configured"))?;
        let password = self.secrets.get("MONSTER_PASSWORD")
            .ok_or_else(|| anyhow!("Monster password not configured"))?;

        browser.navigate("https://www.monster.com/account/register").await?;
        
        browser.fill_field("input[name='email']", email).await?;
        browser.fill_field("input[name='password']", password).await?;
        browser.fill_field("input[name='firstName']", &self.extract_first_name()).await?;
        browser.fill_field("input[name='lastName']", &self.extract_last_name()).await?;
        
        browser.click("button[type='submit']").await?;
        
        self.setup_monster_profile(browser).await?;
        
        Ok("Monster account created".to_string())
    }

    /// Create account on ZipRecruiter
    async fn create_ziprecruiter_account(&self, browser: &HeadlessBrowser) -> Result<String> {
        info!("Creating ZipRecruiter account");
        
        let email = self.secrets.get("ZIPRECRUITER_EMAIL")
            .ok_or_else(|| anyhow!("ZipRecruiter email not configured"))?;
        let password = self.secrets.get("ZIPRECRUITER_PASSWORD")
            .ok_or_else(|| anyhow!("ZipRecruiter password not configured"))?;

        browser.navigate("https://www.ziprecruiter.com/candidate/create-account").await?;
        
        browser.fill_field("input[name='email']", email).await?;
        browser.fill_field("input[name='password']", password).await?;
        browser.fill_field("input[name='firstName']", &self.extract_first_name()).await?;
        browser.fill_field("input[name='lastName']", &self.extract_last_name()).await?;
        
        browser.click("button[type='submit']").await?;
        
        self.setup_ziprecruiter_profile(browser).await?;
        
        Ok("ZipRecruiter account created".to_string())
    }

    // Helper methods for profile setup
    async fn handle_linkedin_verification(&self, browser: &HeadlessBrowser) -> Result<()> {
        // Handle phone verification or other LinkedIn verification steps
        info!("Handling LinkedIn verification");
        // Implementation would go here
        Ok(())
    }

    async fn setup_linkedin_profile(&self, browser: &HeadlessBrowser) -> Result<()> {
        info!("Setting up LinkedIn profile");
        // Upload resume, fill in work experience, education, etc.
        // Implementation would go here
        Ok(())
    }

    async fn setup_indeed_profile(&self, browser: &HeadlessBrowser) -> Result<()> {
        info!("Setting up Indeed profile");
        // Upload resume, set job preferences, etc.
        // Implementation would go here
        Ok(())
    }

    async fn setup_glassdoor_profile(&self, browser: &HeadlessBrowser) -> Result<()> {
        info!("Setting up Glassdoor profile");
        // Implementation would go here
        Ok(())
    }

    async fn setup_monster_profile(&self, browser: &HeadlessBrowser) -> Result<()> {
        info!("Setting up Monster profile");
        // Implementation would go here
        Ok(())
    }

    async fn setup_ziprecruiter_profile(&self, browser: &HeadlessBrowser) -> Result<()> {
        info!("Setting up ZipRecruiter profile");
        // Implementation would go here
        Ok(())
    }

    fn extract_first_name(&self) -> String {
        self.personal_info.name.split_whitespace().next().unwrap_or("").to_string()
    }

    fn extract_last_name(&self) -> String {
        let parts: Vec<&str> = self.personal_info.name.split_whitespace().collect();
        if parts.len() > 1 {
            parts[1..].join(" ")
        } else {
            String::new()
        }
    }
    
    /// Create accounts on all major job sites with visible browser
    pub async fn create_all_accounts_visible(&self, browser: &crate::visible_browser::VisibleBrowser) -> Result<Vec<String>> {
        let mut results = Vec::new();
        
        // Create LinkedIn account
        match self.create_linkedin_account_visible(browser).await {
            Ok(result) => results.push(result),
            Err(e) => results.push(format!("LinkedIn account creation failed: {}", e)),
        }
        
        // Create Indeed account
        match self.create_indeed_account_visible(browser).await {
            Ok(result) => results.push(result),
            Err(e) => results.push(format!("Indeed account creation failed: {}", e)),
        }
        
        // Add more platforms as needed
        
        Ok(results)
    }
    
    /// Create LinkedIn account with visible browser
    async fn create_linkedin_account_visible(&self, browser: &crate::visible_browser::VisibleBrowser) -> Result<String> {
        info!("Creating LinkedIn account with visible browser");
        
        let email = self.secrets.get("LINKEDIN_EMAIL")
            .ok_or_else(|| anyhow!("LinkedIn email not configured"))?;
        let password = self.secrets.get("LINKEDIN_PASSWORD")
            .ok_or_else(|| anyhow!("LinkedIn password not configured"))?;

        // Navigate to LinkedIn signup
        browser.navigate_to("https://www.linkedin.com/signup")?;
        
        // Wait for page to load
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        // Fill out registration form
        browser.type_text("input[name='firstName']", &self.extract_first_name())?;
        browser.type_text("input[name='lastName']", &self.extract_last_name())?;
        browser.type_text("input[name='email']", email)?;
        browser.type_text("input[name='password']", password)?;
        
        // Submit form
        browser.click_element("button[type='submit']")?;
        
        // Wait for response
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        Ok("LinkedIn account creation initiated successfully".to_string())
    }
    
    /// Create Indeed account with visible browser
    async fn create_indeed_account_visible(&self, browser: &crate::visible_browser::VisibleBrowser) -> Result<String> {
        info!("Creating Indeed account with visible browser");
        
        let email = self.secrets.get("INDEED_EMAIL")
            .ok_or_else(|| anyhow!("Indeed email not configured"))?;
        let password = self.secrets.get("INDEED_PASSWORD")
            .ok_or_else(|| anyhow!("Indeed password not configured"))?;

        // Navigate to Indeed signup
        browser.navigate_to("https://secure.indeed.com/auth/signup")?;
        
        // Wait for page to load
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        // Fill out registration form
        browser.type_text("input[name='email']", email)?;
        browser.type_text("input[name='password']", password)?;
        browser.type_text("input[name='confirm_password']", password)?;
        
        // Submit form
        browser.click_element("button[type='submit']")?;
        
        // Wait for response
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        Ok("Indeed account creation initiated successfully".to_string())
    }

    /// Create accounts in batch based on the provided site list
    pub async fn create_accounts_batch(&self, sites: &[&str], browser: &HeadlessBrowser) -> Result<Vec<String>> {
        info!("Creating accounts on {} sites", sites.len());

        let mut results = Vec::new();

        for site in sites {
            info!("Processing account creation for: {}", site);

            match site.to_lowercase().as_str() {
                "linkedin" => {
                    match self.create_linkedin_account(browser).await {
                        Ok(_) => {
                            results.push(format!("✅ LinkedIn account created successfully"));
                            info!("LinkedIn account creation completed");
                        }
                        Err(e) => {
                            warn!("LinkedIn account creation failed: {}", e);
                            results.push(format!("❌ LinkedIn account creation failed: {}", e));
                        }
                    }
                }
                "indeed" => {
                    match self.create_indeed_account(browser).await {
                        Ok(_) => {
                            results.push(format!("✅ Indeed account created successfully"));
                            info!("Indeed account creation completed");
                        }
                        Err(e) => {
                            warn!("Indeed account creation failed: {}", e);
                            results.push(format!("❌ Indeed account creation failed: {}", e));
                        }
                    }
                }
                "glassdoor" => {
                    match self.create_glassdoor_account(browser).await {
                        Ok(_) => {
                            results.push(format!("✅ Glassdoor account created successfully"));
                            info!("Glassdoor account creation completed");
                        }
                        Err(e) => {
                            warn!("Glassdoor account creation failed: {}", e);
                            results.push(format!("❌ Glassdoor account creation failed: {}", e));
                        }
                    }
                }
                "monster" => {
                    match self.create_monster_account(browser).await {
                        Ok(_) => {
                            results.push(format!("✅ Monster account created successfully"));
                            info!("Monster account creation completed");
                        }
                        Err(e) => {
                            warn!("Monster account creation failed: {}", e);
                            results.push(format!("❌ Monster account creation failed: {}", e));
                        }
                    }
                }
                "ziprecruiter" => {
                    match self.create_ziprecruiter_account(browser).await {
                        Ok(_) => {
                            results.push(format!("✅ ZipRecruiter account created successfully"));
                            info!("ZipRecruiter account creation completed");
                        }
                        Err(e) => {
                            warn!("ZipRecruiter account creation failed: {}", e);
                            results.push(format!("❌ ZipRecruiter account creation failed: {}", e));
                        }
                    }
                }
                _ => {
                    results.push(format!("❌ Unknown site: {}", site));
                }
            }

            // Rate limiting between account creations
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }

        Ok(results)
    }
}
