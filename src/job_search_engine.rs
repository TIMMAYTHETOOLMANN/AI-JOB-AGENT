use reqwest;
use scraper::{Html, Selector};
use log::{info, warn};
use anyhow::Result;
use std::time::Duration;

// No need to import the job application module as we use it directly

/// Searches for jobs via search engines based on company, keywords, and locations
pub async fn search_jobs_via_engine(
    company: &str, 
    keywords: &[String], 
    locations: &[String],
    search_engines: &[String]
) -> Result<Vec<String>> {
    // Check if company name is empty or invalid
    if company.trim().is_empty() {
        warn!("Empty company name provided. Skipping search.");
        return Ok(Vec::new());
    }

    info!("Starting search engine reconnaissance for company: {}", company);

    // Check if search_engines or locations are empty
    if search_engines.is_empty() {
        warn!("No search engines specified for company: {}. Skipping search.", company);
        return Ok(Vec::new());
    }

    if locations.is_empty() {
        warn!("No locations specified for company: {}. Skipping search.", company);
        return Ok(Vec::new());
    }

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .timeout(Duration::from_secs(30))
        .build()?;

    let mut found_urls = Vec::new();

    for location in locations {
        for search_engine_base in search_engines {
            // Create search query - more permissive in production mode
            let query = if cfg!(debug_assertions) {
                // Development mode - more restrictive query
                format!("{} jobs {} site:{}.com", company, location, company.to_lowercase())
            } else {
                // Production mode - more permissive query
                format!("{} jobs {}", company, location)
            };
            let url = format!("{}{}", search_engine_base, urlencoding::encode(&query));

            info!("Searching: {} (Mode: {})", url, if cfg!(debug_assertions) { "Development" } else { "Production" });

            // Add delay to avoid rate limiting
            tokio::time::sleep(Duration::from_secs(2)).await;

            match client.get(&url).send().await {
                Ok(response) => {
                    match response.text().await {
                        Ok(html) => {
                            let document = Html::parse_document(&html);
                            let selector = match Selector::parse("a") {
                                Ok(selector) => selector,
                                Err(e) => {
                                    warn!("Failed to parse selector: {}", e);
                                    continue;
                                }
                            };

                            // Log the number of links found in the HTML
                            let all_links: Vec<_> = document.select(&selector).collect();
                            info!("Found {} total links in the HTML response", all_links.len());

                            if all_links.is_empty() {
                                warn!("No links found in HTML response. This might indicate a captcha or empty response.");
                                // Log a sample of the HTML to help diagnose issues
                                let html_sample = if html.len() > 200 { 
                                    format!("{}...", &html[0..200]) 
                                } else { 
                                    html.clone() 
                                };
                                info!("HTML sample: {}", html_sample);
                            }

                            for element in document.select(&selector) {
                                if let Some(href) = element.value().attr("href") {
                                    // Log all hrefs for debugging in development mode
                                    if cfg!(debug_assertions) {
                                        info!("Found href: {}", href);
                                    }

                                    // Extract job URLs - less restrictive in production mode
                                    let href_contains_company = href.to_lowercase().contains(&company.to_lowercase());
                                    let href_contains_job_terms = href.contains("careers") || href.contains("jobs") || 
                                                                href.contains("employment") || href.contains("work") ||
                                                                href.contains("position") || href.contains("opening") ||
                                                                href.contains("apply") || href.contains("hire");

                                    // In production mode, log all URLs for debugging
                                    if !cfg!(debug_assertions) && href.contains("http") {
                                        info!("Examining URL: {}", href);
                                    }

                                    let url_criteria = if cfg!(debug_assertions) {
                                        // Development mode - more restrictive
                                        (href.contains("/url?q=") || href.contains("http")) && 
                                        (href_contains_company || href_contains_job_terms)
                                    } else {
                                        // Production mode - extremely permissive
                                        // Accept any URL that contains http
                                        href.contains("http")
                                    };

                                    if url_criteria {
                                        let job_url = if href.contains("/url?q=") {
                                            // Google search result format
                                            href.split("/url?q=").nth(1)
                                                .and_then(|u| u.split('&').next())
                                                .unwrap_or(href)
                                                .to_string()
                                        } else {
                                            // Direct URL
                                            href.to_string()
                                        };

                                        // Filter URLs to ensure they're relevant - less restrictive in production
                                        if is_relevant_job_url(&job_url, company, keywords) {
                                            info!("Found job URL: {}", job_url);
                                            found_urls.push(job_url);
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => warn!("Failed to parse HTML from {}: {}", url, e),
                    }
                },
                Err(e) => warn!("Failed to fetch {}: {}", url, e),
            }
        }
    }

    info!("Search engine reconnaissance complete. Found {} potential job URLs", found_urls.len());
    Ok(found_urls)
}

/// Determines if a URL is relevant based on company name and keywords
fn is_relevant_job_url(url: &str, company: &str, keywords: &[String]) -> bool {
    let url_lower = url.to_lowercase();

    // Check if URL contains company name or common job site patterns
    let company_relevant = url_lower.contains(&company.to_lowercase()) || 
                          url_lower.contains("careers") || 
                          url_lower.contains("jobs") ||
                          url_lower.contains("employment") ||
                          url_lower.contains("work") ||
                          url_lower.contains("position") ||
                          url_lower.contains("opening");

    // Check if URL contains any of the keywords
    let keyword_relevant = keywords.iter()
        .any(|keyword| url_lower.contains(&keyword.to_lowercase()));

    if cfg!(debug_assertions) {
        // Development mode - more restrictive
        company_relevant && (keywords.is_empty() || keyword_relevant)
    } else {
        // Production mode - extremely permissive
        // In production, accept any URL to ensure we don't miss potential job postings
        info!("Production mode: Accepting URL: {}", url);
        true
    }
}

/// Processes job URLs found via search engines and applies to matching jobs
pub async fn process_search_engine_results(
    urls: &[String],
    _company_targets: &[String],
    keywords: &[String],
    name: &str,
    email: &str,
    resume_path: &str,
    cover_letter_path: &str
) -> Result<usize> {
    info!("Processing {} job URLs found via search engines", urls.len());

    // Use the job application module to process and apply to jobs
    let applied_count = crate::job_application::process_and_apply_to_jobs(
        urls,
        name,
        email,
        resume_path,
        cover_letter_path,
        keywords
    ).await?;

    info!("Completed processing search engine results. Applied to {} jobs", applied_count);
    Ok(applied_count)
}
