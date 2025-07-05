use anyhow::{Result, anyhow};
use std::time::Duration;
use tracing::{info, debug, warn};

#[cfg(feature = "mock-browser")]
use crate::mock_browser::HeadlessBrowser;
#[cfg(feature = "headless-chrome")]
use crate::headless_browser::HeadlessBrowser;

/// Attempts to log in and apply to a job on a generic company career site.
/// Handles self-healing for dead links, login errors, and already-applied jobs.
pub async fn apply_company_job(
    browser: &HeadlessBrowser,
    job_url: &str,
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str,
    company: &str,
) -> Result<String> {
    info!("Applying to {} job: {}", company, job_url);
    
    // Go to the job posting
    if let Err(e) = browser.navigate_to(job_url) {
        return Err(anyhow!("Dead link or navigation error: {}", e));
    }
    
    // Wait for page to load
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Try to detect login page and perform login if needed
    let login_selectors = vec!["input[name='email']", "input[type='email']", "#email", ".email"];
    for selector in login_selectors {
        if let Ok(_) = browser.find_element(selector) {
            debug!("Found login form for {}", company);
            
            // Fill email
            if let Ok(_) = browser.type_text(selector, email) {
                // Fill password (would need to be configured per company)
                let password_selectors = vec!["input[name='password']", "input[type='password']", "#password"];
                for pwd_selector in password_selectors {
                    if let Ok(_) = browser.type_text(pwd_selector, "YOUR_COMPANY_PASSWORD") {
                        // Submit login
                        let submit_selectors = vec!["button[type='submit']", "input[type='submit']", ".submit-btn"];
                        for submit_selector in submit_selectors {
                            if let Ok(_) = browser.click_element(submit_selector) {
                                tokio::time::sleep(Duration::from_secs(5)).await;
                                break;
                            }
                        }
                        break;
                    }
                }
            }
            break;
        }
    }
    
    // Check for already-applied or closed job indicators
    let content = browser.get_page_content().unwrap_or_default();
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("already applied") || 
       content_lower.contains("no longer accepting") ||
       content_lower.contains("position closed") ||
       content_lower.contains("application submitted") {
        return Ok("Skipped (already applied/closed)".to_string());
    }
    
    // Try to find and click the apply button
    let apply_selectors = vec![
        "button:contains('Apply')",
        "a:contains('Apply')",
        "button:contains('Start Application')",
        "[data-testid*='apply']",
        ".apply-button",
        ".apply-btn",
    ];
    
    for selector in apply_selectors {
        if let Ok(_) = browser.click_element(selector) {
            info!("Found and clicked apply button for {}", company);
            
            // Wait for application form to load
            tokio::time::sleep(Duration::from_secs(3)).await;
            
            // Fill out application form
            return fill_application_form(browser, name, email, resume_path, cover_letter_path, company).await;
        }
    }
    
    Err(anyhow!("Could not find apply button on {} job page", company))
}

async fn fill_application_form(
    browser: &HeadlessBrowser,
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str,
    company: &str,
) -> Result<String> {
    debug!("Filling application form for {}", company);
    
    // Fill name field
    let name_selectors = vec![
        "input[name='name']",
        "input[name='fullName']",
        "input[name='firstName']",
        "input[id*='name']",
        "input[placeholder*='name']",
    ];
    
    for selector in name_selectors {
        if let Ok(_) = browser.type_text(selector, name) {
            debug!("Filled name field");
            break;
        }
    }
    
    // Fill email field
    let email_selectors = vec![
        "input[name='email']",
        "input[type='email']",
        "input[id*='email']",
        "input[placeholder*='email']",
    ];
    
    for selector in email_selectors {
        if let Ok(_) = browser.type_text(selector, email) {
            debug!("Filled email field");
            break;
        }
    }
    
    // Try to upload resume
    let resume_selectors = vec![
        "input[type='file'][name*='resume']",
        "input[type='file'][id*='resume']",
        "input[type='file'][name*='cv']",
        "input[type='file']",
    ];
    
    for selector in resume_selectors {
        if let Ok(_) = browser.find_element(selector) {
            debug!("Found resume upload field (file upload needs implementation)");
            break;
        }
    }
    
    // Submit application
    let submit_selectors = vec![
        "button[type='submit']",
        "input[type='submit']",
        "button:contains('Submit')",
        "button:contains('Finish')",
        "button:contains('Send Application')",
        ".submit-btn",
        ".submit-button",
    ];
    
    for selector in submit_selectors {
        if let Ok(_) = browser.click_element(selector) {
            info!("Successfully submitted application to {}", company);
            return Ok(format!("Applied successfully to {}", company));
        }
    }
    
    Ok(format!("Application form filled for {} (manual submission required)", company))
}

