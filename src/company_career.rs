use thirtyfour::prelude::*;
use anyhow::{Result, anyhow};
use std::time::Duration;

/// Attempts to log in and apply to a job on a generic company career site.
/// Handles self-healing for dead links, login errors, and already-applied jobs.
pub async fn apply_company_job(
    driver: &WebDriver,
    job_url: &str,
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str,
    company: &str,
) -> Result<String> {
    // Go to the job posting
    if let Err(e) = driver.goto(job_url).await {
        return Err(anyhow!("Dead link or navigation error: {}", e));
    }
    // Try to detect login page and perform login if needed (selectors may differ by company)
    if driver.find(By::Name("email")).await.is_ok() {
        driver.find(By::Name("email")).await?.send_keys(email).await?;
        driver.find(By::Name("password")).await?.send_keys("YOUR_COMPANY_PASSWORD").await?;
        driver.find(By::Css("button[type='submit']")).await?.click().await?;
        driver.set_implicit_wait_timeout(Duration::from_secs(10)).await?;
    }
    // Check for already-applied or closed job
    if let Ok(elem) = driver.find(By::XPath("//*[contains(text(),'applied') or contains(text(),'no longer accepting applications') or contains(text(),'closed')]")).await {
        let text = elem.text().await.unwrap_or_default();
        return Ok(format!("Skipped (already applied/closed): {}", text));
    }
    // Try to find and click the apply button
    if let Ok(apply_btn) = driver.find(By::XPath("//button[contains(text(),'Apply') or contains(text(),'Start Application')]")).await {
        apply_btn.click().await?;
        // Fill out application form (simplified, selectors may differ)
        if let Ok(name_field) = driver.find(By::Name("name")).await {
            name_field.send_keys(name).await?;
        }
        if let Ok(email_field) = driver.find(By::Name("email")).await {
            email_field.send_keys(email).await?;
        }
        // Upload resume
        if let Ok(resume_upload) = driver.find(By::Name("resume")).await {
            resume_upload.send_keys(resume_path).await?;
        }
        // Upload cover letter (optional)
        if let Ok(cover_upload) = driver.find(By::Name("coverLetter")).await {
            cover_upload.send_keys(cover_letter_path).await?;
        }
        // Submit application
        if let Ok(submit_btn) = driver.find(By::XPath("//button[contains(text(),'Submit') or contains(text(),'Finish')]")).await {
            submit_btn.click().await?;
            return Ok(format!("Applied successfully to {}", company));
        }
        return Err(anyhow!("Could not find submit button after filling form"));
    }
    Err(anyhow!("Could not find apply button on {} job page", company))
}

