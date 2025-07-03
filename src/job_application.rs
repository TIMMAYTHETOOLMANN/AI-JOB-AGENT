use std::sync::{Arc, Mutex};
use crate::application_state::ApplicationState;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;
use crate::nike_career::apply_nike_job;
use crate::company_career::apply_company_job;
use lettre::{Message, SmtpTransport, Transport, message::Mailbox, transport::smtp::authentication::Credentials};

/// Represents a document profile (resume or cover letter)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentProfile {
    /// Name of the profile
    pub name: String,
    /// Path to the document file
    pub path: String,
    /// Keywords associated with this profile
    pub keywords: Vec<String>,
}

/// Processes a batch of job URLs and applies to matching jobs
pub async fn process_and_apply_to_jobs(
    urls: &[String],
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str,
    job_keywords: &[String],
    app_state: Option<Arc<Mutex<ApplicationState>>>,
    resume_profiles: Option<&[DocumentProfile]>,
    cover_letter_profiles: Option<&[DocumentProfile]>,
    industries: &[String],
    seniority_levels: &[String],
    browser: Option<&WebDriver>, // NEW: pass browser session for automation
    smtp: Option<(&str, &str, &str)>, // NEW: SMTP server, username, password
) -> Result<usize> {
    let mut applied_count = 0;
    for url in urls {
        println!("Processing job: {}", url);
        let mut job_status = String::new();
        let result = async {
            if let Some(driver) = browser {
                if url.contains("linkedin.com/jobs") {
                    driver.goto(url).await?;
                    // TODO: Add LinkedIn Easy Apply automation here
                    job_status = format!("[AUTOMATION] Would apply to LinkedIn job: {}", url);
                    return Ok(()) as Result<()>;
                }
                if url.contains("nike.com/careers") {
                    match apply_nike_job(driver, url, name, email, resume_path, cover_letter_path).await {
                        Ok(status) => {
                            job_status = format!("Nike: {} - {}", url, status);
                            return Ok(()) as Result<()>;
                        },
                        Err(e) => {
                            job_status = format!("Nike: {} - Error: {}", url, e);
                            return Err(e);
                        }
                    }
                }
                // Generic company career site automation for Intel, adidas, etc.
                let company_targets = ["intel.com/careers", "adidas-group.com/careers", "precisioncastparts.com/careers", "daimler.com/careers"];
                for &company_url in &company_targets {
                    if url.contains(company_url) {
                        let company = company_url.split('.').next().unwrap_or("");
                        match apply_company_job(driver, url, name, email, resume_path, cover_letter_path, company).await {
                            Ok(status) => {
                                job_status = format!("{}: {} - {}", company, url, status);
                                return Ok(()) as Result<()>;
                            },
                            Err(e) => {
                                job_status = format!("{}: {} - Error: {}", company, url, e);
                                return Err(e);
                            }
                        }
                    }
                }
            }
            // Fallback: Print or mock apply for other platforms
            job_status = format!('[MOCK] Would apply to job: {}', url);
            Ok(())
        }.await;
        match result {
            Ok(_) => {
                applied_count += 1;
                // Send email notification after a successful application
                if let Some((smtp_server, smtp_username, smtp_password)) = smtp {
                    if let Err(e) = send_email_notification(
                        email,
                        "Job Application Submitted",
                        &job_status,
                        smtp_server,
                        smtp_username,
                        smtp_password,
                    ).await {
                        println!("[ERROR] Failed to send email: {}", e);
                    }
                }
            },
            Err(e) => println!("[ERROR] {}", e),
        }
        if let Some(state) = &app_state {
            let mut state = state.lock().unwrap();
            // In real implementation would add to state.applications
        }
        println!("Job status: {}", job_status);
    }
    Ok(applied_count)
}

/// Send an email notification using lettre
async fn send_email_notification(
    to: &str,
    subject: &str,
    body: &str,
    smtp_server: &str,
    smtp_username: &str,
    smtp_password: &str,
) -> anyhow::Result<()> {
    let email = Message::builder()
        .from("Autonomous Job Agent <autobot@localhost>".parse().unwrap())
        .to(Mailbox::new(None, to.parse().unwrap()))
        .subject(subject)
        .body(body.to_string())?;
    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(creds)
        .build();
    mailer.send(&email)?;
    Ok(())
}
