use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use tracing::{info, debug};

/// Localization and globalization support for international job markets
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfig {
    pub default_locale: String,
    pub supported_locales: Vec<String>,
    pub currency_preferences: HashMap<String, String>,
    pub time_zone_mappings: HashMap<String, String>,
    pub international_job_boards: HashMap<String, Vec<String>>,
    pub regional_search_terms: HashMap<String, Vec<String>>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        let mut currency_preferences = HashMap::new();
        currency_preferences.insert("US".to_string(), "USD".to_string());
        currency_preferences.insert("EU".to_string(), "EUR".to_string());
        currency_preferences.insert("UK".to_string(), "GBP".to_string());
        currency_preferences.insert("CA".to_string(), "CAD".to_string());
        currency_preferences.insert("AU".to_string(), "AUD".to_string());
        currency_preferences.insert("JP".to_string(), "JPY".to_string());
        currency_preferences.insert("IN".to_string(), "INR".to_string());

        let mut time_zone_mappings = HashMap::new();
        time_zone_mappings.insert("US".to_string(), "America/New_York".to_string());
        time_zone_mappings.insert("EU".to_string(), "Europe/London".to_string());
        time_zone_mappings.insert("UK".to_string(), "Europe/London".to_string());
        time_zone_mappings.insert("CA".to_string(), "America/Toronto".to_string());
        time_zone_mappings.insert("AU".to_string(), "Australia/Sydney".to_string());
        time_zone_mappings.insert("JP".to_string(), "Asia/Tokyo".to_string());
        time_zone_mappings.insert("IN".to_string(), "Asia/Kolkata".to_string());

        let mut international_job_boards = HashMap::new();
        international_job_boards.insert("US".to_string(), vec![
            "https://indeed.com".to_string(),
            "https://linkedin.com/jobs".to_string(),
            "https://glassdoor.com".to_string(),
            "https://monster.com".to_string(),
            "https://ziprecruiter.com".to_string(),
            "https://careerbuilder.com".to_string(),
        ]);
        international_job_boards.insert("EU".to_string(), vec![
            "https://jobs.eu".to_string(),
            "https://eurojobs.com".to_string(),
            "https://thelocal.com/jobs".to_string(),
            "https://xing.com".to_string(),
            "https://stepstone.com".to_string(),
        ]);
        international_job_boards.insert("UK".to_string(), vec![
            "https://reed.co.uk".to_string(),
            "https://totaljobs.com".to_string(),
            "https://cv-library.co.uk".to_string(),
            "https://jobsite.co.uk".to_string(),
        ]);
        international_job_boards.insert("CA".to_string(), vec![
            "https://workopolis.com".to_string(),
            "https://jobbank.gc.ca".to_string(),
            "https://monster.ca".to_string(),
        ]);
        international_job_boards.insert("AU".to_string(), vec![
            "https://seek.com.au".to_string(),
            "https://careerone.com.au".to_string(),
            "https://jobs.com.au".to_string(),
        ]);
        international_job_boards.insert("IN".to_string(), vec![
            "https://naukri.com".to_string(),
            "https://monsterindia.com".to_string(),
            "https://timesjobs.com".to_string(),
        ]);

        let mut regional_search_terms = HashMap::new();
        regional_search_terms.insert("US".to_string(), vec![
            "full-time".to_string(),
            "benefits".to_string(),
            "401k".to_string(),
            "healthcare".to_string(),
        ]);
        regional_search_terms.insert("EU".to_string(), vec![
            "full-time".to_string(),
            "permanent".to_string(),
            "benefits".to_string(),
            "pension".to_string(),
        ]);
        regional_search_terms.insert("UK".to_string(), vec![
            "permanent".to_string(),
            "full-time".to_string(),
            "pension".to_string(),
            "benefits".to_string(),
        ]);

        GlobalConfig {
            default_locale: "en-US".to_string(),
            supported_locales: vec![
                "en-US".to_string(),
                "en-GB".to_string(),
                "en-CA".to_string(),
                "en-AU".to_string(),
                "de-DE".to_string(),
                "fr-FR".to_string(),
                "es-ES".to_string(),
                "ja-JP".to_string(),
                "hi-IN".to_string(),
            ],
            currency_preferences,
            time_zone_mappings,
            international_job_boards,
            regional_search_terms,
        }
    }
}

/// Detect user's locale based on location preferences
pub fn detect_locale(locations: &[String]) -> String {
    for location in locations {
        let location_lower = location.to_lowercase();
        if location_lower.contains("united states") || location_lower.contains("usa") || location_lower.contains("us") {
            return "en-US".to_string();
        } else if location_lower.contains("united kingdom") || location_lower.contains("uk") || location_lower.contains("england") {
            return "en-GB".to_string();
        } else if location_lower.contains("canada") || location_lower.contains("ca") {
            return "en-CA".to_string();
        } else if location_lower.contains("australia") || location_lower.contains("au") {
            return "en-AU".to_string();
        } else if location_lower.contains("germany") || location_lower.contains("deutschland") {
            return "de-DE".to_string();
        } else if location_lower.contains("france") || location_lower.contains("française") {
            return "fr-FR".to_string();
        } else if location_lower.contains("spain") || location_lower.contains("españa") {
            return "es-ES".to_string();
        } else if location_lower.contains("japan") || location_lower.contains("日本") {
            return "ja-JP".to_string();
        } else if location_lower.contains("india") || location_lower.contains("भारत") {
            return "hi-IN".to_string();
        }
    }
    "en-US".to_string() // Default fallback
}

/// Convert salary range based on regional currency
pub fn localize_salary(salary: u32, from_currency: &str, to_currency: &str) -> Result<u32> {
    // Simple currency conversion (in production, would use live exchange rates)
    let conversion_rates: HashMap<(&str, &str), f64> = [
        (("USD", "EUR"), 0.85),
        (("USD", "GBP"), 0.75),
        (("USD", "CAD"), 1.25),
        (("USD", "AUD"), 1.35),
        (("USD", "JPY"), 110.0),
        (("USD", "INR"), 75.0),
        (("EUR", "USD"), 1.18),
        (("GBP", "USD"), 1.33),
        (("CAD", "USD"), 0.80),
        (("AUD", "USD"), 0.74),
        (("JPY", "USD"), 0.009),
        (("INR", "USD"), 0.013),
    ].iter().cloned().collect();

    if from_currency == to_currency {
        return Ok(salary);
    }

    if let Some(&rate) = conversion_rates.get(&(from_currency, to_currency)) {
        Ok((salary as f64 * rate) as u32)
    } else {
        // Fallback: no conversion
        debug!("No conversion rate found for {} to {}", from_currency, to_currency);
        Ok(salary)
    }
}

/// Get region-specific job boards
pub fn get_regional_job_boards(region: &str, global_config: &GlobalConfig) -> Vec<String> {
    global_config.international_job_boards
        .get(region)
        .cloned()
        .unwrap_or_else(|| {
            info!("No specific job boards found for region {}, using US defaults", region);
            global_config.international_job_boards
                .get("US")
                .cloned()
                .unwrap_or_default()
        })
}

/// Get region-specific search terms to enhance job searches
pub fn get_regional_search_terms(region: &str, global_config: &GlobalConfig) -> Vec<String> {
    global_config.regional_search_terms
        .get(region)
        .cloned()
        .unwrap_or_default()
}

/// Extract region code from locale (e.g., "en-US" -> "US")
pub fn extract_region_from_locale(locale: &str) -> String {
    if let Some(region) = locale.split('-').nth(1) {
        region.to_string()
    } else {
        "US".to_string() // Default fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_detection() {
        assert_eq!(detect_locale(&["United States".to_string()]), "en-US");
        assert_eq!(detect_locale(&["UK".to_string()]), "en-GB");
        assert_eq!(detect_locale(&["Remote".to_string()]), "en-US");
    }

    #[test]
    fn test_salary_localization() {
        assert_eq!(localize_salary(100000, "USD", "USD").unwrap(), 100000);
        assert_eq!(localize_salary(100000, "USD", "EUR").unwrap(), 85000);
    }

    #[test]
    fn test_region_extraction() {
        assert_eq!(extract_region_from_locale("en-US"), "US");
        assert_eq!(extract_region_from_locale("de-DE"), "DE");
        assert_eq!(extract_region_from_locale("invalid"), "US");
    }
}