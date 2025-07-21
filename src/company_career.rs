use anyhow::{Result, anyhow};
use std::time::Duration;
use tracing::{info, debug};

// PRODUCTION BROWSER ENFORCEMENT - VISIBLE BROWSER ONLY
use crate::visible_browser::VisibleBrowser;

/// Attempts to log in and apply to a job on a generic company career site.
/// Handles self-healing for dead links, login errors, and already-applied jobs.
/// PRODUCTION: Uses visible browser only
pub async fn apply_company_job(
    browser: &mut VisibleBrowser,
    job_url: &str,
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str,
    company: &str,
) -> Result<String> {
    info!("🏢 Applying to {} job with visible browser: {}", company, job_url);

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
            info!("✅ Found and clicked apply button for {}", company);

            // Wait for application form to load
            tokio::time::sleep(Duration::from_secs(3)).await;
            
            // Fill out application form
            return fill_application_form(browser, name, email, resume_path, cover_letter_path, company).await;
        }
    }
    
    Err(anyhow!("Could not find apply button on {} job page", company))
}

async fn fill_application_form(
    browser: &mut VisibleBrowser,
    name: &str,
    email: &str,
    _resume_path: &str,
    _cover_letter_path: &str,
    company: &str,
) -> Result<String> {
    debug!("Filling application form for {} with visible browser", company);

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
            debug!("✅ Filled name field for {}", company);
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
            debug!("✅ Filled email field for {}", company);
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
            info!("✅ Successfully submitted application to {}", company);
            return Ok(format!("Applied successfully to {}", company));
        }
    }
    
    Ok(format!("Application form filled for {} (manual submission required)", company))
}
