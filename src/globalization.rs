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
            "https://shine.com".to_string(),
            "https://foundit.in".to_string(),
        ]);
        international_job_boards.insert("JP".to_string(), vec![
            "https://rikunabi.com".to_string(),
            "https://mynavi.jp".to_string(),
            "https://en-japan.com".to_string(),
            "https://daijob.com".to_string(),
            "https://gaijinpot.com".to_string(),
        ]);
        international_job_boards.insert("KR".to_string(), vec![
            "https://saramin.co.kr".to_string(),
            "https://jobkorea.co.kr".to_string(),
            "https://incruit.com".to_string(),
        ]);
        international_job_boards.insert("CN".to_string(), vec![
            "https://zhaopin.com".to_string(),
            "https://51job.com".to_string(),
            "https://liepin.com".to_string(),
        ]);
        international_job_boards.insert("DE".to_string(), vec![
            "https://stepstone.de".to_string(),
            "https://xing.com".to_string(),
            "https://indeed.de".to_string(),
            "https://monster.de".to_string(),
        ]);
        international_job_boards.insert("FR".to_string(), vec![
            "https://monster.fr".to_string(),
            "https://indeed.fr".to_string(),
            "https://apec.fr".to_string(),
            "https://pole-emploi.fr".to_string(),
        ]);
        international_job_boards.insert("ES".to_string(), vec![
            "https://indeed.es".to_string(),
            "https://infojobs.net".to_string(),
            "https://monster.es".to_string(),
        ]);
        international_job_boards.insert("BR".to_string(), vec![
            "https://vagas.com.br".to_string(),
            "https://catho.com.br".to_string(),
            "https://indeed.com.br".to_string(),
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
        regional_search_terms.insert("JP".to_string(), vec![
            "正社員".to_string(), // Full-time employee
            "契約社員".to_string(), // Contract employee
            "English Teacher".to_string(),
            "Native Speaker".to_string(),
            "International".to_string(),
        ]);
        regional_search_terms.insert("IN".to_string(), vec![
            "full-time".to_string(),
            "permanent".to_string(),
            "benefits".to_string(),
            "provident fund".to_string(),
            "ESI".to_string(),
        ]);
        regional_search_terms.insert("DE".to_string(), vec![
            "Vollzeit".to_string(),
            "Festanstellung".to_string(),
            "Sozialversicherung".to_string(),
            "Urlaub".to_string(),
        ]);
        regional_search_terms.insert("FR".to_string(), vec![
            "temps plein".to_string(),
            "CDI".to_string(),
            "sécurité sociale".to_string(),
            "congés payés".to_string(),
        ]);
        regional_search_terms.insert("ES".to_string(), vec![
            "tiempo completo".to_string(),
            "contrato indefinido".to_string(),
            "seguridad social".to_string(),
            "vacaciones".to_string(),
        ]);
        regional_search_terms.insert("CA".to_string(), vec![
            "full-time".to_string(),
            "permanent".to_string(),
            "benefits".to_string(),
            "RRSP".to_string(),
        ]);
        regional_search_terms.insert("AU".to_string(), vec![
            "full-time".to_string(),
            "permanent".to_string(),
            "superannuation".to_string(),
            "annual leave".to_string(),
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
                "ko-KR".to_string(),
                "zh-CN".to_string(),
                "pt-BR".to_string(),
                "ru-RU".to_string(),
                "ar-SA".to_string(),
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

/// Simple translation service for job-related terms
pub fn translate_job_terms(text: &str, from_lang: &str, to_lang: &str) -> Result<String> {
    // Simple translation dictionary for common job terms
    let translations: HashMap<(&str, &str), &str> = [
        // English to Japanese
        (("en", "ja"), "エンジニア"), // Engineer
        (("en", "ja"), "マネージャー"), // Manager
        (("en", "ja"), "開発者"), // Developer
        (("en", "ja"), "営業"), // Sales
        (("en", "ja"), "マーケティング"), // Marketing
        
        // English to Hindi
        (("en", "hi"), "इंजीनियर"), // Engineer
        (("en", "hi"), "प्रबंधक"), // Manager
        (("en", "hi"), "डेवलपर"), // Developer
        (("en", "hi"), "बिक्री"), // Sales
        (("en", "hi"), "मार्केटिंग"), // Marketing
        
        // Add more translations as needed
    ].iter().cloned().collect();
    
    // Simple keyword-based translation
    let mut result = text.to_string();
    
    // In production, this would use a proper translation service
    // For now, we'll just return the original text with a note
    if from_lang != to_lang {
        result = format!("[{}] {}", to_lang.to_uppercase(), text);
    }
    
    Ok(result)
}

/// Get job categories suitable for international positions
pub fn get_international_job_categories() -> Vec<String> {
    vec![
        "English Teacher".to_string(),
        "ESL Instructor".to_string(),
        "International Business".to_string(),
        "Cross-cultural Communication".to_string(),
        "Cultural Liaison".to_string(),
        "International Sales".to_string(),
        "Global Operations".to_string(),
        "Export/Import Manager".to_string(),
        "International Marketing".to_string(),
        "Cultural Ambassador".to_string(),
        "Language Trainer".to_string(),
        "International Relations".to_string(),
        "Global Project Manager".to_string(),
        "International Consultant".to_string(),
        "Cultural Adaptation Specialist".to_string(),
    ]
}

/// Get job categories suitable for North Americans abroad
pub fn get_north_american_abroad_categories() -> Vec<String> {
    vec![
        "Native English Speaker".to_string(),
        "American/Canadian Business".to_string(),
        "Western Business Practices".to_string(),
        "North American Culture".to_string(),
        "English Language Specialist".to_string(),
        "International Assignment".to_string(),
        "Expat Position".to_string(),
        "Cultural Exchange".to_string(),
        "Western Marketing".to_string(),
        "North American Standards".to_string(),
        "International School".to_string(),
        "American Company Representative".to_string(),
        "Cross-cultural Training".to_string(),
        "Western Technology Transfer".to_string(),
        "North American Business Development".to_string(),
    ]
}

/// Adapt resume content for different cultural contexts
pub fn adapt_resume_for_culture(content: &str, target_culture: &str) -> Result<String> {
    let mut adapted_content = content.to_string();
    
    match target_culture {
        "JP" => {
            // Japanese business culture adaptations
            adapted_content = adapted_content.replace("I achieved", "Achieved through team collaboration");
            adapted_content = adapted_content.replace("I led", "Contributed to leadership of");
            adapted_content = format!("{}{}",
                "[Cultural Note: Adapted for Japanese business culture - Emphasis on team collaboration and continuous improvement]\n\n",
                adapted_content
            );
        },
        "IN" => {
            // Indian business culture adaptations
            adapted_content = format!("{}{}",
                "[Cultural Note: Adapted for Indian business culture - Emphasis on technical skills and process improvement]\n\n",
                adapted_content
            );
        },
        "DE" => {
            // German business culture adaptations
            adapted_content = format!("{}{}",
                "[Cultural Note: Adapted for German business culture - Emphasis on precision, quality, and technical expertise]\n\n",
                adapted_content
            );
        },
        "FR" => {
            // French business culture adaptations
            adapted_content = format!("{}{}",
                "[Cultural Note: Adapted for French business culture - Emphasis on intellectual approach and methodical processes]\n\n",
                adapted_content
            );
        },
        _ => {
            // Default: no adaptation needed
        }
    }
    
    Ok(adapted_content)
}

/// Calculate cultural adaptation score for job matching
pub fn calculate_cultural_fit_score(job_location: &str, job_description: &str, user_background: &str) -> f64 {
    let mut score: f64 = 0.5; // Base score
    
    // Boost score for international/cross-cultural positions
    if job_description.to_lowercase().contains("international") ||
       job_description.to_lowercase().contains("cross-cultural") ||
       job_description.to_lowercase().contains("global") {
        score += 0.2;
    }
    
    // Boost score for English teaching positions
    if job_description.to_lowercase().contains("english teacher") ||
       job_description.to_lowercase().contains("esl") ||
       job_description.to_lowercase().contains("native speaker") {
        score += 0.3;
    }
    
    // Boost score for North American background in international contexts
    if user_background.contains("North American") && !job_location.contains("US") && !job_location.contains("CA") {
        score += 0.2;
    }
    
    // Boost score for manufacturing/technical roles (user's background)
    if job_description.to_lowercase().contains("manufacturing") ||
       job_description.to_lowercase().contains("engineering") ||
       job_description.to_lowercase().contains("process") ||
       job_description.to_lowercase().contains("automation") {
        score += 0.15;
    }
    
    // Cap the score at 1.0
    score.min(1.0)
}