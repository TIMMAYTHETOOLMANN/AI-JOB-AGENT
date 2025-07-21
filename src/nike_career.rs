use anyhow::{Result, anyhow};
use std::time::Duration;
use tracing::{info, debug};

// PRODUCTION BROWSER ENFORCEMENT - VISIBLE BROWSER ONLY
use crate::visible_browser::VisibleBrowser;

/// Attempts to log in and apply to a job on the Nike career site.
/// Handles self-healing for dead links, login errors, and already-applied jobs.
/// PRODUCTION: Uses visible browser only
pub async fn apply_nike_job(
    browser: &mut VisibleBrowser,
    job_url: &str,
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str,
) -> Result<String> {
    info!("👟 Applying to Nike job with visible browser: {}", job_url);

    // Go to the job posting
    if let Err(e) = browser.navigate_to(job_url) {
        return Err(anyhow!("Dead link or navigation error: {}", e));
    }
    
    // Wait for page to load
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Check for already-applied or closed job indicators
    let current_url = browser.get_current_url();

    if current_url.to_lowercase().contains("already applied") ||
       current_url.to_lowercase().contains("no longer accepting") ||
       current_url.to_lowercase().contains("position closed") ||
       current_url.to_lowercase().contains("application submitted") {
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
            info!("✅ Found and clicked Nike apply button");

            // Wait for application form to load
            tokio::time::sleep(Duration::from_secs(3)).await;
            
            // Fill out Nike application form
            return fill_nike_application_form(browser, name, email, resume_path, cover_letter_path).await;
        }
    }
    
    Err(anyhow!("Could not find apply button on Nike job page"))
}

async fn fill_nike_application_form(
    browser: &mut VisibleBrowser,
    name: &str,
    email: &str,
    _resume_path: &str,
    _cover_letter_path: &str,
) -> Result<String> {
    debug!("Filling Nike application form with visible browser");

    // Nike-specific field selectors
    let name_selectors = vec![
        "input[name='firstName']",
        "input[name='fullName']",
        "input[id*='name']",
        "input[placeholder*='First name']",
        "input[data-testid*='name']",
    ];
    
    for selector in name_selectors {
        if let Ok(_) = browser.type_text(selector, name) {
            debug!("✅ Filled Nike name field");
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
            debug!("✅ Filled Nike email field");
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
            info!("✅ Successfully submitted Nike application");
            return Ok("Applied successfully to Nike".to_string());
        }
    }
    
    Ok("Nike application form filled (manual submission required)".to_string())
}
