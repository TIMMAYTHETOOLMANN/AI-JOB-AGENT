use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::job_listing::JobListing;
use crate::globalization::calculate_cultural_fit_score;
use anyhow::Result;
use tracing::{info, debug};

/// Intelligent job scoring system for automated application decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobScorer {
    /// Weights for different scoring factors
    pub weights: ScoringWeights,
    /// Historical performance data
    pub historical_performance: HashMap<String, f64>,
    /// User preferences and background
    pub user_profile: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub keyword_match: f64,
    pub skill_match: f64,
    pub industry_match: f64,
    pub location_preference: f64,
    pub salary_range: f64,
    pub company_preference: f64,
    pub cultural_fit: f64,
    pub experience_level: f64,
    pub job_freshness: f64,
    pub growth_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub background: String,
    pub preferred_roles: Vec<String>,
    pub preferred_industries: Vec<String>,
    pub preferred_locations: Vec<String>,
    pub salary_range: (u32, u32),
    pub experience_level: String,
    pub cultural_adaptability: f64,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            keyword_match: 0.25,
            skill_match: 0.20,
            industry_match: 0.15,
            location_preference: 0.10,
            salary_range: 0.10,
            company_preference: 0.05,
            cultural_fit: 0.05,
            experience_level: 0.05,
            job_freshness: 0.03,
            growth_potential: 0.02,
        }
    }
}

impl JobScorer {
    pub fn new(config: &Config) -> Self {
        let user_profile = UserProfile {
            background: "North American Manufacturing Engineer".to_string(),
            preferred_roles: config.job_keywords.clone(),
            preferred_industries: config.industries.clone(),
            preferred_locations: config.location.clone(),
            salary_range: (
                config.min_salary.unwrap_or(50000),
                config.max_salary.unwrap_or(200000)
            ),
            experience_level: "Mid-Senior".to_string(),
            cultural_adaptability: 0.8, // High adaptability
        };

        Self {
            weights: ScoringWeights::default(),
            historical_performance: HashMap::new(),
            user_profile,
        }
    }

    /// Calculate comprehensive job score (0.0 to 1.0)
    pub fn calculate_job_score(&self, job: &JobListing, config: &Config) -> f64 {
        let mut total_score = 0.0;

        // 1. Keyword matching score
        let keyword_score = self.calculate_keyword_match_score(job, config);
        total_score += keyword_score * self.weights.keyword_match;

        // 2. Skill matching score
        let skill_score = self.calculate_skill_match_score(job, config);
        total_score += skill_score * self.weights.skill_match;

        // 3. Industry matching score
        let industry_score = self.calculate_industry_match_score(job, config);
        total_score += industry_score * self.weights.industry_match;

        // 4. Location preference score
        let location_score = self.calculate_location_preference_score(job, config);
        total_score += location_score * self.weights.location_preference;

        // 5. Salary range score
        let salary_score = self.calculate_salary_score(job, config);
        total_score += salary_score * self.weights.salary_range;

        // 6. Company preference score
        let company_score = self.calculate_company_preference_score(job, config);
        total_score += company_score * self.weights.company_preference;

        // 7. Cultural fit score
        let cultural_score = calculate_cultural_fit_score(
            &job.location,
            &job.description,
            &self.user_profile.background
        );
        total_score += cultural_score * self.weights.cultural_fit;

        // 8. Experience level match score
        let experience_score = self.calculate_experience_level_score(job);
        total_score += experience_score * self.weights.experience_level;

        // 9. Job freshness score
        let freshness_score = self.calculate_job_freshness_score(job);
        total_score += freshness_score * self.weights.job_freshness;

        // 10. Growth potential score
        let growth_score = self.calculate_growth_potential_score(job);
        total_score += growth_score * self.weights.growth_potential;

        // Apply enhancement factors
        total_score = self.apply_enhancement_factors(total_score, job, config);

        debug!("Job score for '{}' at {}: {:.3}", job.title, job.company, total_score);
        total_score.min(1.0)
    }

    fn calculate_keyword_match_score(&self, job: &JobListing, config: &Config) -> f64 {
        let enhanced_keywords = config.get_enhanced_job_keywords();
        let job_text = format!("{} {}", job.title, job.description).to_lowercase();
        
        let mut matches = 0;
        let mut total_keywords = enhanced_keywords.len();
        
        for keyword in &enhanced_keywords {
            if job_text.contains(&keyword.to_lowercase()) {
                matches += 1;
            }
        }
        
        // Fuzzy matching for higher scores
        if config.enable_fuzzy_matching {
            for keyword in &enhanced_keywords {
                let similarity = self.calculate_fuzzy_similarity(&job_text, &keyword.to_lowercase());
                if similarity > 0.7 {
                    matches += 1;
                }
            }
            total_keywords *= 2; // Account for fuzzy matches
        }
        
        if total_keywords > 0 {
            matches as f64 / total_keywords as f64
        } else {
            0.0
        }
    }

    fn calculate_skill_match_score(&self, job: &JobListing, config: &Config) -> f64 {
        let job_text = format!("{} {}", job.title, job.description).to_lowercase();
        let mut matches = 0;
        
        for skill in &config.skills {
            if job_text.contains(&skill.to_lowercase()) {
                matches += 1;
            }
            
            // Check skill synonyms
            if let Some(synonyms) = config.skill_synonyms.get(skill) {
                for synonym in synonyms {
                    if job_text.contains(&synonym.to_lowercase()) {
                        matches += 1;
                        break;
                    }
                }
            }
        }
        
        if config.skills.len() > 0 {
            (matches as f64 / config.skills.len() as f64).min(1.0)
        } else {
            0.5
        }
    }

    fn calculate_industry_match_score(&self, job: &JobListing, config: &Config) -> f64 {
        let job_text = format!("{} {} {}", job.title, job.description, job.company).to_lowercase();
        
        for industry in &config.industries {
            if job_text.contains(&industry.to_lowercase()) {
                return 1.0;
            }
        }
        
        // Check for related industries
        let related_industries = vec![
            "manufacturing", "engineering", "automation", "technology", "industrial"
        ];
        
        for industry in &related_industries {
            if job_text.contains(industry) {
                return 0.7;
            }
        }
        
        0.3 // Default score for unmatched industries
    }

    fn calculate_location_preference_score(&self, job: &JobListing, config: &Config) -> f64 {
        let job_location = job.location.to_lowercase();
        
        // High score for remote work
        if job_location.contains("remote") {
            return 0.9;
        }
        
        // Check preferred locations
        for location in &config.location {
            if job_location.contains(&location.to_lowercase()) {
                return 1.0;
            }
        }
        
        // International locations get bonus if international search enabled
        if config.should_enable_international_search() {
            let international_indicators = vec![
                "international", "global", "worldwide", "japan", "india", "germany", 
                "france", "spain", "australia", "canada", "uk", "europe", "asia"
            ];
            
            for indicator in &international_indicators {
                if job_location.contains(indicator) {
                    return 0.8;
                }
            }
        }
        
        0.4 // Default score for non-matching locations
    }

    fn calculate_salary_score(&self, job: &JobListing, config: &Config) -> f64 {
        if let Some(salary_str) = &job.salary {
            if let Some(salary_num) = self.extract_salary_number(salary_str) {
                let min_salary = config.min_salary.unwrap_or(50000);
                let max_salary = config.max_salary.unwrap_or(200000);
                
                if salary_num >= min_salary && salary_num <= max_salary {
                    return 1.0;
                } else if salary_num >= (min_salary * 80 / 100) && salary_num <= (max_salary * 120 / 100) {
                    return 0.7;
                } else if salary_num >= (min_salary * 60 / 100) && salary_num <= (max_salary * 140 / 100) {
                    return 0.5;
                }
            }
        }
        
        0.6 // Default score when salary not specified
    }

    fn calculate_company_preference_score(&self, job: &JobListing, config: &Config) -> f64 {
        let company_name = job.company.to_lowercase();
        let all_companies = config.get_all_target_companies();
        
        for company in &all_companies {
            if company_name.contains(&company.to_lowercase()) {
                return 1.0;
            }
        }
        
        // Check for company size indicators
        let large_company_indicators = vec![
            "corp", "corporation", "inc", "ltd", "group", "systems", "technologies", "solutions"
        ];
        
        for indicator in &large_company_indicators {
            if company_name.contains(indicator) {
                return 0.6;
            }
        }
        
        0.4 // Default score for unknown companies
    }

    fn calculate_experience_level_score(&self, job: &JobListing) -> f64 {
        let job_text = format!("{} {}", job.title, job.description).to_lowercase();
        
        // Check for experience level indicators
        let senior_indicators = vec!["senior", "lead", "principal", "architect", "manager"];
        let mid_indicators = vec!["engineer", "analyst", "specialist", "developer"];
        let entry_indicators = vec!["junior", "entry", "associate", "intern"];
        
        for indicator in &senior_indicators {
            if job_text.contains(indicator) {
                return 0.9; // Good match for mid-senior level
            }
        }
        
        for indicator in &mid_indicators {
            if job_text.contains(indicator) {
                return 1.0; // Perfect match
            }
        }
        
        for indicator in &entry_indicators {
            if job_text.contains(indicator) {
                return 0.3; // Lower match for entry level
            }
        }
        
        0.7 // Default score
    }

    fn calculate_job_freshness_score(&self, job: &JobListing) -> f64 {
        if let Some(date_posted) = job.date_posted {
            let now = chrono::Utc::now().naive_utc();
            let days_ago = (now - date_posted).num_days();
            
            match days_ago {
                0..=1 => 1.0,
                2..=7 => 0.9,
                8..=14 => 0.7,
                15..=30 => 0.5,
                _ => 0.3,
            }
        } else {
            0.6 // Default score when date not available
        }
    }

    fn calculate_growth_potential_score(&self, job: &JobListing) -> f64 {
        let job_text = format!("{} {}", job.title, job.description).to_lowercase();
        
        let growth_indicators = vec![
            "growth", "advancement", "career", "leadership", "development", 
            "training", "mentoring", "progression", "opportunity"
        ];
        
        for indicator in &growth_indicators {
            if job_text.contains(indicator) {
                return 0.8;
            }
        }
        
        0.5 // Default score
    }

    fn apply_enhancement_factors(&self, base_score: f64, job: &JobListing, config: &Config) -> f64 {
        let mut enhanced_score = base_score;
        
        // Boost for international positions if enabled
        if config.should_enable_international_search() {
            let job_text = format!("{} {}", job.title, job.description).to_lowercase();
            if job_text.contains("international") || job_text.contains("global") {
                enhanced_score *= 1.1;
            }
        }
        
        // Boost for English teaching positions
        if config.enable_english_teaching_jobs {
            let job_text = format!("{} {}", job.title, job.description).to_lowercase();
            if job_text.contains("english teacher") || job_text.contains("esl") {
                enhanced_score *= 1.2;
            }
        }
        
        // Boost for manufacturing companies
        if config.manufacturing_companies.iter().any(|c| job.company.to_lowercase().contains(&c.to_lowercase())) {
            enhanced_score *= 1.15;
        }
        
        // Skill expansion factor
        if config.skill_expansion_factor > 1.0 {
            enhanced_score *= config.skill_expansion_factor.min(1.5);
        }
        
        enhanced_score
    }

    fn calculate_fuzzy_similarity(&self, text: &str, keyword: &str) -> f64 {
        // Simple fuzzy matching - in production, use a proper fuzzy matching library
        let text_words: Vec<&str> = text.split_whitespace().collect();
        let keyword_words: Vec<&str> = keyword.split_whitespace().collect();
        
        let mut matches = 0;
        for keyword_word in &keyword_words {
            for text_word in &text_words {
                if self.levenshtein_similarity(text_word, keyword_word) > 0.8 {
                    matches += 1;
                    break;
                }
            }
        }
        
        if keyword_words.len() > 0 {
            matches as f64 / keyword_words.len() as f64
        } else {
            0.0
        }
    }

    fn levenshtein_similarity(&self, s1: &str, s2: &str) -> f64 {
        let len1 = s1.len();
        let len2 = s2.len();
        
        if len1 == 0 || len2 == 0 {
            return 0.0;
        }
        
        let max_len = len1.max(len2);
        let distance = self.levenshtein_distance(s1, s2);
        
        1.0 - (distance as f64 / max_len as f64)
    }

    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();
        
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }
        
        matrix[len1][len2]
    }

    fn extract_salary_number(&self, salary_str: &str) -> Option<u32> {
        // Extract numeric value from salary string
        let numbers: String = salary_str.chars().filter(|c| c.is_ascii_digit()).collect();
        numbers.parse::<u32>().ok()
    }

    /// Update historical performance data
    pub fn update_performance(&mut self, job_url: &str, success: bool) {
        let current_performance = self.historical_performance.entry(job_url.to_string()).or_insert(0.0);
        
        if success {
            *current_performance = (*current_performance * 0.9) + 0.1; // Weighted average
        } else {
            *current_performance = *current_performance * 0.9; // Decay on failure
        }
    }

    /// Check if job meets auto-application criteria
    pub fn should_auto_apply(&self, job: &JobListing, config: &Config) -> bool {
        let score = self.calculate_job_score(job, config);
        let meets_threshold = config.should_auto_apply(score);
        
        info!("Job '{}' at {} scored {:.3} (threshold: {:.3}) - Auto-apply: {}", 
              job.title, job.company, score, config.auto_apply_threshold, meets_threshold);
        
        meets_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_job_scoring() {
        let config = Config::load("AgentConfig.toml").unwrap();
        let scorer = JobScorer::new(&config);
        
        let job = JobListing {
            title: "Process Engineer".to_string(),
            company: "Manufacturing Corp".to_string(),
            location: "Portland, OR".to_string(),
            url: "https://example.com/job/123".to_string(),
            description: "Looking for a process engineer with automation experience".to_string(),
            salary: Some("$80,000 - $120,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        };
        
        let score = scorer.calculate_job_score(&job, &config);
        assert!(score > 0.5);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_auto_apply_decision() {
        let config = Config::load("AgentConfig.toml").unwrap();
        let scorer = JobScorer::new(&config);
        
        let high_score_job = JobListing {
            title: "Senior Process Engineer".to_string(),
            company: "Intel".to_string(),
            location: "Remote".to_string(),
            url: "https://example.com/job/456".to_string(),
            description: "Process optimization and automation engineering role".to_string(),
            salary: Some("$90,000 - $150,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        };
        
        let should_apply = scorer.should_auto_apply(&high_score_job, &config);
        // This will depend on the scoring algorithm and thresholds
        println!("Should auto-apply: {}", should_apply);
    }
}