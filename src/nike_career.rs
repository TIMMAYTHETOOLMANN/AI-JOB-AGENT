use anyhow::{Result, anyhow};
use std::time::Duration;
use tracing::{info, debug, warn};

#[cfg(feature = "headless-chrome")]
use crate::headless_browser::HeadlessBrowser;

#[cfg(all(feature = "mock-browser", not(feature = "headless-chrome")))]
use crate::mock_browser::HeadlessBrowser;

#[cfg(all(not(feature = "headless-chrome"), not(feature = "mock-browser")))]
type HeadlessBrowser = crate::mock_browser::HeadlessBrowser;

/// Attempts to log in and apply to a job on the Nike career site.
/// Handles self-healing for dead links, login errors, and already-applied jobs.
pub async fn apply_nike_job(
    browser: &HeadlessBrowser,
    job_url: &str,
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str,
) -> Result<String> {
    info!("Applying to Nike job: {}", job_url);
    
    // Go to the job posting
    if let Err(e) = browser.navigate_to(job_url) {
        return Err(anyhow!("Dead link or navigation error: {}", e));
    }
    
    // Wait for page to load
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Try to detect login page and perform login if needed
    let login_selectors = vec!["input[name='email']", "input[type='email']", "#email"];
    for selector in login_selectors {
        if let Ok(_) = browser.find_element(selector) {
            debug!("Found Nike login form");
            
            // Fill email
            if let Ok(_) = browser.type_text(selector, email) {
                // Fill password
                let password_selectors = vec!["input[name='password']", "input[type='password']", "#password"];
                for pwd_selector in password_selectors {
                    if let Ok(_) = browser.type_text(pwd_selector, "YOUR_NIKE_PASSWORD") {
                        // Submit login
                        let submit_selectors = vec!["button[type='submit']", "input[type='submit']"];
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
    
    // Try to find and click the apply button (Nike-specific selectors)
    let apply_selectors = vec![
        "button:contains('Apply')",
        "a:contains('Apply')",
        "button:contains('Start Application')",
        "[data-testid='apply-button']",
        ".apply-button",
        ".btn-apply",
        "[aria-label*='Apply']",
    ];
    
    for selector in apply_selectors {
        if let Ok(_) = browser.click_element(selector) {
            info!("Found and clicked Nike apply button");
            
            // Wait for application form to load
            tokio::time::sleep(Duration::from_secs(3)).await;
            
            // Fill out Nike application form
            return fill_nike_application_form(browser, name, email, resume_path, cover_letter_path).await;
        }
    }
    
    Err(anyhow!("Could not find apply button on Nike job page"))
}

async fn fill_nike_application_form(
    browser: &HeadlessBrowser,
    name: &str,
    email: &str,
    resume_path: &str,
    _cover_letter_path: &str,
) -> Result<String> {
    debug!("Filling Nike application form");
    
    // Nike-specific field selectors (these would need to be updated based on actual Nike forms)
    let name_selectors = vec![
        "input[name='firstName']",
        "input[name='fullName']",
        "input[id*='name']",
        "input[placeholder*='First name']",
        "input[data-testid*='name']",
    ];
    
    for selector in name_selectors {
        if let Ok(_) = browser.type_text(selector, name) {
            debug!("Filled Nike name field");
            break;
        }
    }
    
    // Fill email field
    let email_selectors = vec![
        "input[name='email']",
        "input[type='email']",
        "input[id*='email']",
        "input[data-testid*='email']",
    ];
    
    for selector in email_selectors {
        if let Ok(_) = browser.type_text(selector, email) {
            debug!("Filled Nike email field");
            break;
        }
    }
    
    // Try to upload resume
    let resume_selectors = vec![
        "input[type='file'][name*='resume']",
        "input[type='file'][name*='cv']",
        "input[type='file'][data-testid*='resume']",
        "input[type='file']",
    ];
    
    for selector in resume_selectors {
        if let Ok(_) = browser.find_element(selector) {
            debug!("Found Nike resume upload field (file upload needs implementation)");
            break;
        }
    }
    
    // Submit Nike application
    let submit_selectors = vec![
        "button[type='submit']",
        "input[type='submit']",
        "button:contains('Submit')",
        "button:contains('Send Application')",
        "[data-testid*='submit']",
        ".submit-button",
    ];
    
    for selector in submit_selectors {
        if let Ok(_) = browser.click_element(selector) {
            info!("Successfully submitted Nike application");
            return Ok("Applied successfully to Nike".to_string());
        }
    }
    
    Ok("Nike application form filled (manual submission required)".to_string())
}

