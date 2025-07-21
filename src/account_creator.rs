use std::collections::HashMap;
use anyhow::{Result, anyhow};
use tracing::{info, warn};
use crate::config::Config;
use crate::document_profile::{extract_resume_data, extract_cover_letter_data, PersonalInfo};

// PRODUCTION BROWSER ENFORCEMENT - VISIBLE BROWSER ONLY
use crate::visible_browser::VisibleBrowser;

/// Account creation service that automatically creates accounts on job sites
/// using information from resume and cover letter - PRODUCTION VISIBLE BROWSER ONLY
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

    /// Create accounts on all major job sites with visible browser
    pub async fn create_all_accounts_visible(&self, browser: &mut VisibleBrowser) -> Result<Vec<String>> {
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
    async fn create_linkedin_account_visible(&self, browser: &mut VisibleBrowser) -> Result<String> {
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
    async fn create_indeed_account_visible(&self, browser: &mut VisibleBrowser) -> Result<String> {
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
}
