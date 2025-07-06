use tracing::{info, debug};
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::time::SystemTime;
use rand::{Rng, SeedableRng};

/// Represents a resume or cover letter profile with enhanced matching capabilities
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentProfile {
    /// Path to the document file
    pub path: String,
    /// Name of the profile
    pub name: String,
    /// Industries this profile is suitable for
    pub industries: Vec<String>,
    /// Keywords this profile is suitable for
    pub keywords: Vec<String>,
    /// Seniority levels this profile is suitable for
    pub seniority: Vec<String>,
    /// Performance metrics for this profile
    #[serde(default)]
    pub performance_metrics: ProfilePerformanceMetrics,
}

/// Performance tracking for document profiles
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ProfilePerformanceMetrics {
    pub applications_count: u32,
    pub success_count: u32,
    pub last_used: Option<SystemTime>,
    pub industry_success_rates: HashMap<String, f64>,
    pub keyword_success_rates: HashMap<String, f64>,
    pub average_response_time: Option<f64>, // Days until response
}

/// Advanced profile selection engine with dynamic learning
#[derive(Debug)]
pub struct DynamicProfileSelector {
    profiles: Vec<DocumentProfile>,
    global_success_rates: HashMap<String, f64>,
    industry_weights: HashMap<String, f64>,
    keyword_weights: HashMap<String, f64>,
    seniority_weights: HashMap<String, f64>,
}

impl DynamicProfileSelector {
    pub fn new(profiles: Vec<DocumentProfile>) -> Self {
        Self {
            profiles,
            global_success_rates: HashMap::new(),
            industry_weights: HashMap::new(),
            keyword_weights: HashMap::new(),
            seniority_weights: HashMap::new(),
        }
    }

    /// Dynamic profile selection with machine learning-inspired scoring
    pub fn select_optimal_profile(
        &self,
        job_industries: &[String],
        job_keywords: &[String],
        job_seniority: &[String],
        profile_type: &str,
    ) -> Option<&DocumentProfile> {
        if self.profiles.is_empty() {
            return None;
        }

        let mut scored_profiles: Vec<(f64, &DocumentProfile)> = self.profiles
            .iter()
            .map(|profile| {
                let score = self.calculate_advanced_score(
                    profile, 
                    job_industries, 
                    job_keywords, 
                    job_seniority
                );
                (score, profile)
            })
            .collect();

        // Sort by score (highest first)
        scored_profiles.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Log scoring details
        for (score, profile) in &scored_profiles {
            debug!("{} profile '{}' scored {:.3}", profile_type, profile.name, score);
        }

        scored_profiles.first().map(|(_, profile)| *profile)
    }

    /// Calculate advanced scoring using multiple factors
    fn calculate_advanced_score(
        &self,
        profile: &DocumentProfile,
        job_industries: &[String],
        job_keywords: &[String],
        job_seniority: &[String],
    ) -> f64 {
        let mut score = 0.0;

        // Base matching scores
        let industry_score = self.calculate_industry_match_score(profile, job_industries);
        let keyword_score = self.calculate_keyword_match_score(profile, job_keywords);
        let seniority_score = self.calculate_seniority_match_score(profile, job_seniority);

        // Performance-based adjustments
        let performance_multiplier = self.calculate_performance_multiplier(profile, job_industries, job_keywords);
        
        // Recency bonus (prefer recently successful profiles)
        let recency_bonus = self.calculate_recency_bonus(profile);
        
        // Diversity penalty (avoid overusing the same profile)
        let diversity_penalty = self.calculate_diversity_penalty(profile);

        // Weighted combination
        score += industry_score * 3.0;  // Industry match is most important
        score += keyword_score * 2.5;   // Keywords are very important
        score += seniority_score * 1.5; // Seniority match is moderately important
        score *= performance_multiplier; // Multiply by historical performance
        score += recency_bonus;          // Add recency bonus
        score -= diversity_penalty;     // Subtract diversity penalty

        debug!("Profile '{}' - Industry: {:.2}, Keyword: {:.2}, Seniority: {:.2}, Performance: {:.2}, Recency: {:.2}, Diversity: {:.2}, Final: {:.2}",
               profile.name, industry_score, keyword_score, seniority_score, performance_multiplier, recency_bonus, diversity_penalty, score);

        score.max(0.0) // Ensure non-negative score
    }

    fn calculate_industry_match_score(&self, profile: &DocumentProfile, job_industries: &[String]) -> f64 {
        if job_industries.is_empty() || profile.industries.is_empty() {
            return 0.0;
        }

        let mut _matches = 0;
        let mut weighted_score = 0.0;

        for job_industry in job_industries {
            for profile_industry in &profile.industries {
                if self.industry_similarity(job_industry, profile_industry) > 0.7 {
                    _matches += 1;
                    // Weight by historical success rate for this industry
                    let weight = self.industry_weights.get(job_industry).copied().unwrap_or(1.0);
                    weighted_score += weight;
                    break;
                }
            }
        }

        // Normalize by the number of job industries
        weighted_score / job_industries.len() as f64
    }

    fn calculate_keyword_match_score(&self, profile: &DocumentProfile, job_keywords: &[String]) -> f64 {
        if job_keywords.is_empty() || profile.keywords.is_empty() {
            return 0.0;
        }

        let mut total_score = 0.0;
        let mut total_possible = 0.0;

        for job_keyword in job_keywords {
            let mut best_match = 0.0;
            
            for profile_keyword in &profile.keywords {
                let similarity = self.keyword_similarity(job_keyword, profile_keyword);
                if similarity > best_match {
                    best_match = similarity;
                }
            }
            
            // Weight by keyword importance
            let weight = self.keyword_weights.get(job_keyword).copied().unwrap_or(1.0);
            total_score += best_match * weight;
            total_possible += weight;
        }

        if total_possible > 0.0 {
            total_score / total_possible
        } else {
            0.0
        }
    }

    fn calculate_seniority_match_score(&self, profile: &DocumentProfile, job_seniority: &[String]) -> f64 {
        if job_seniority.is_empty() || profile.seniority.is_empty() {
            return 0.5; // Neutral score when no seniority info
        }

        for job_level in job_seniority {
            for profile_level in &profile.seniority {
                if self.seniority_similarity(job_level, profile_level) > 0.8 {
                    return 1.0;
                }
            }
        }

        0.0
    }

    fn calculate_performance_multiplier(&self, profile: &DocumentProfile, job_industries: &[String], job_keywords: &[String]) -> f64 {
        let base_multiplier = 1.0;
        
        if profile.performance_metrics.applications_count == 0 {
            return base_multiplier; // No data, use neutral multiplier
        }

        let overall_success_rate = profile.performance_metrics.success_count as f64 / 
                                  profile.performance_metrics.applications_count as f64;

        let mut weighted_success_rate = overall_success_rate;

        // Weight by industry-specific success rates
        for industry in job_industries {
            if let Some(&industry_rate) = profile.performance_metrics.industry_success_rates.get(industry) {
                weighted_success_rate = (weighted_success_rate + industry_rate) / 2.0;
            }
        }

        // Weight by keyword-specific success rates
        for keyword in job_keywords {
            if let Some(&keyword_rate) = profile.performance_metrics.keyword_success_rates.get(keyword) {
                weighted_success_rate = (weighted_success_rate + keyword_rate) / 2.0;
            }
        }

        // Convert success rate to multiplier (0.5 to 1.5 range)
        0.5 + weighted_success_rate
    }

    fn calculate_recency_bonus(&self, profile: &DocumentProfile) -> f64 {
        if let Some(last_used) = profile.performance_metrics.last_used {
            if let Ok(duration) = SystemTime::now().duration_since(last_used) {
                let days_since = duration.as_secs() as f64 / (24.0 * 3600.0);
                
                // Bonus decreases over time, maxes at 0.5
                return (0.5 * (-days_since / 30.0).exp()).max(0.0);
            }
        }
        0.0
    }

    fn calculate_diversity_penalty(&self, profile: &DocumentProfile) -> f64 {
        // Penalize profiles that have been used too much recently
        let usage_ratio = if profile.performance_metrics.applications_count > 0 {
            let total_applications: u32 = self.profiles.iter()
                .map(|p| p.performance_metrics.applications_count)
                .sum();
            
            if total_applications > 0 {
                profile.performance_metrics.applications_count as f64 / total_applications as f64
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Penalty increases with usage ratio, maxes at 0.3
        (usage_ratio * 0.3).min(0.3)
    }

    // Similarity calculation functions
    fn industry_similarity(&self, industry1: &str, industry2: &str) -> f64 {
        let i1 = industry1.to_lowercase();
        let i2 = industry2.to_lowercase();
        
        if i1 == i2 {
            return 1.0;
        }
        
        // Check for partial matches and synonyms
        if i1.contains(&i2) || i2.contains(&i1) {
            return 0.8;
        }
        
        // Industry synonym mapping
        let synonyms = vec![
            (vec!["technology", "tech", "software", "it"], 0.9),
            (vec!["manufacturing", "production", "factory"], 0.9),
            (vec!["healthcare", "medical", "hospital"], 0.9),
            (vec!["finance", "financial", "banking"], 0.9),
            (vec!["education", "academic", "university"], 0.9),
        ];
        
        for (group, similarity) in synonyms {
            if group.contains(&i1.as_str()) && group.contains(&i2.as_str()) {
                return similarity;
            }
        }
        
        0.0
    }

    fn keyword_similarity(&self, keyword1: &str, keyword2: &str) -> f64 {
        let k1 = keyword1.to_lowercase();
        let k2 = keyword2.to_lowercase();
        
        if k1 == k2 {
            return 1.0;
        }
        
        if k1.contains(&k2) || k2.contains(&k1) {
            return 0.8;
        }
        
        // Simple Jaccard similarity on words
        let words1: Vec<&str> = k1.split_whitespace().collect();
        let words2: Vec<&str> = k2.split_whitespace().collect();
        
        let intersection: usize = words1.iter()
            .filter(|word| words2.contains(word))
            .count();
        
        let union = words1.len() + words2.len() - intersection;
        
        if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        }
    }

    fn seniority_similarity(&self, level1: &str, level2: &str) -> f64 {
        let l1 = level1.to_lowercase();
        let l2 = level2.to_lowercase();
        
        if l1 == l2 {
            return 1.0;
        }
        
        let seniority_levels = vec![
            vec!["entry", "junior", "associate", "level1", "l1"],
            vec!["mid", "middle", "intermediate", "level2", "l2"],
            vec!["senior", "sr", "experienced", "level3", "l3"],
            vec!["lead", "principal", "staff", "level4", "l4"],
            vec!["manager", "director", "vp", "executive"],
        ];
        
        for level_group in seniority_levels {
            if level_group.contains(&l1.as_str()) && level_group.contains(&l2.as_str()) {
                return 1.0;
            }
        }
        
        0.0
    }

    /// Update profile performance metrics after application
    pub fn update_profile_performance(
        &mut self,
        profile_path: &str,
        was_successful: bool,
        job_industries: &[String],
        job_keywords: &[String],
        response_time_days: Option<f64>,
    ) {
        if let Some(profile) = self.profiles.iter_mut().find(|p| p.path == profile_path) {
            profile.performance_metrics.applications_count += 1;
            
            if was_successful {
                profile.performance_metrics.success_count += 1;
            }
            
            profile.performance_metrics.last_used = Some(SystemTime::now());
            
            // Update industry-specific success rates
            for industry in job_industries {
                let current_rate = profile.performance_metrics.industry_success_rates
                    .get(industry)
                    .copied()
                    .unwrap_or(0.0);
                
                let new_rate = if was_successful {
                    (current_rate + 1.0) / 2.0
                } else {
                    current_rate / 2.0
                };
                
                profile.performance_metrics.industry_success_rates
                    .insert(industry.clone(), new_rate);
            }
            
            // Update keyword-specific success rates
            for keyword in job_keywords {
                let current_rate = profile.performance_metrics.keyword_success_rates
                    .get(keyword)
                    .copied()
                    .unwrap_or(0.0);
                
                let new_rate = if was_successful {
                    (current_rate + 1.0) / 2.0
                } else {
                    current_rate / 2.0
                };
                
                profile.performance_metrics.keyword_success_rates
                    .insert(keyword.clone(), new_rate);
            }
            
            // Update response time
            if let Some(response_days) = response_time_days {
                profile.performance_metrics.average_response_time = Some(
                    if let Some(current_avg) = profile.performance_metrics.average_response_time {
                        (current_avg + response_days) / 2.0
                    } else {
                        response_days
                    }
                );
            }
            
            info!("Updated performance metrics for profile '{}': {} applications, {} successes", 
                  profile.name, 
                  profile.performance_metrics.applications_count,
                  profile.performance_metrics.success_count);
        }
    }
}

/// Selects the most appropriate resume for a job based on job details
pub fn select_resume(
    profiles: &[DocumentProfile],
    job_industries: &[String],
    job_keywords: &[String],
    job_seniority: &[String],
    fallback_path: &str,
) -> String {
    if profiles.is_empty() {
        info!("No resume profiles available, using fallback resume");
        return fallback_path.to_string();
    }

    select_document(profiles, job_industries, job_keywords, job_seniority, fallback_path, "resume")
}

/// Selects the most appropriate cover letter for a job based on job details
pub fn select_cover_letter(
    profiles: &[DocumentProfile],
    job_industries: &[String],
    job_keywords: &[String],
    job_seniority: &[String],
    fallback_path: &str,
) -> String {
    if profiles.is_empty() {
        info!("No cover letter profiles available, using fallback cover letter");
        return fallback_path.to_string();
    }

    select_document(profiles, job_industries, job_keywords, job_seniority, fallback_path, "cover letter")
}

/// Selects the most appropriate document (resume or cover letter) for a job based on job details
fn select_document(
    profiles: &[DocumentProfile],
    job_industries: &[String],
    job_keywords: &[String],
    job_seniority: &[String],
    fallback_path: &str,
    document_type: &str,
) -> String {
    // Score each profile based on how well it matches the job details
    let mut scored_profiles: Vec<(usize, &DocumentProfile)> = profiles
        .iter()
        .map(|profile| {
            let mut score = 0;

            // Score based on industry match
            for industry in job_industries {
                if profile.industries.iter().any(|i| i.to_lowercase() == industry.to_lowercase()) {
                    score += 3;
                    debug!("{} profile '{}' matched industry: {}", document_type, profile.name, industry);
                }
            }

            // Score based on keyword match
            for keyword in job_keywords {
                if profile.keywords.iter().any(|k| keyword.to_lowercase().contains(&k.to_lowercase())) {
                    score += 2;
                    debug!("{} profile '{}' matched keyword: {}", document_type, profile.name, keyword);
                }
            }

            // Score based on seniority match
            for level in job_seniority {
                if profile.seniority.iter().any(|s| s.to_lowercase() == level.to_lowercase()) {
                    score += 1;
                    debug!("{} profile '{}' matched seniority: {}", document_type, profile.name, level);
                }
            }

            (score, profile)
        })
        .collect();

    // Sort profiles by score (highest first)
    scored_profiles.sort_by(|a, b| b.0.cmp(&a.0));

    // Get all profiles with the highest score
    let max_score = scored_profiles.first().map(|(score, _)| *score).unwrap_or(0);
    let best_profiles: Vec<&DocumentProfile> = scored_profiles
        .iter()
        .filter(|(score, _)| *score == max_score && *score > 0)
        .map(|(_, profile)| *profile)
        .collect();

    if best_profiles.is_empty() {
        // No good matches, use fallback
        info!("No matching {} profiles found, using fallback {}", document_type, document_type);
        fallback_path.to_string()
    } else if best_profiles.len() == 1 {
        // Only one best match
        let profile = best_profiles[0];
        info!("Selected {} profile '{}' with score {}", document_type, profile.name, max_score);
        profile.path.clone()
    } else {
        // Multiple best matches, choose randomly
        // Create a hash of the job details to seed the random number generator
        let mut hasher = DefaultHasher::new();
        job_industries.hash(&mut hasher);
        job_keywords.hash(&mut hasher);
        job_seniority.hash(&mut hasher);
        let seed = hasher.finish();

        // Use the hash as a seed for the random number generator
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let index = rng.gen_range(0..best_profiles.len());
        let profile = best_profiles[index];
        info!("Randomly selected {} profile '{}' from {} equally scored profiles (score {})", 
              document_type, profile.name, best_profiles.len(), max_score);
        profile.path.clone()
    }
}

/// Verifies that all document paths in profiles exist
pub fn verify_document_paths(profiles: &[DocumentProfile], document_type: &str) -> Vec<String> {
    let mut missing_paths = Vec::new();
    
    for profile in profiles {
        let path = Path::new(&profile.path);
        if !path.exists() {
            missing_paths.push(profile.path.clone());
            info!("Warning: {} profile '{}' path does not exist: {}", document_type, profile.name, profile.path);
        }
    }
    
    missing_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_document_with_exact_match() {
        let profiles = vec![
            DocumentProfile {
                path: "resume1.pdf".to_string(),
                name: "Engineering Resume".to_string(),
                industries: vec!["Engineering".to_string()],
                keywords: vec!["Software".to_string()],
                seniority: vec!["Senior".to_string()],
                performance_metrics: ProfilePerformanceMetrics::default(),
            },
            DocumentProfile {
                path: "resume2.pdf".to_string(),
                name: "Marketing Resume".to_string(),
                industries: vec!["Marketing".to_string()],
                keywords: vec!["Digital".to_string()],
                seniority: vec!["Mid".to_string()],
                performance_metrics: ProfilePerformanceMetrics::default(),
            },
        ];

        let job_industries = vec!["Engineering".to_string()];
        let job_keywords = vec!["Software Developer".to_string()];
        let job_seniority = vec!["Senior".to_string()];

        let selected = select_document(&profiles, &job_industries, &job_keywords, &job_seniority, "fallback.pdf", "resume");
        assert_eq!(selected, "resume1.pdf");
    }

    #[test]
    fn test_select_document_with_no_match() {
        let profiles = vec![
            DocumentProfile {
                path: "resume1.pdf".to_string(),
                name: "Engineering Resume".to_string(),
                industries: vec!["Engineering".to_string()],
                keywords: vec!["Software".to_string()],
                seniority: vec!["Senior".to_string()],
                performance_metrics: ProfilePerformanceMetrics::default(),
            },
        ];

        let job_industries = vec!["Finance".to_string()];
        let job_keywords = vec!["Accounting".to_string()];
        let job_seniority = vec!["Junior".to_string()];

        let selected = select_document(&profiles, &job_industries, &job_keywords, &job_seniority, "fallback.pdf", "resume");
        assert_eq!(selected, "fallback.pdf");
    }

    #[test]
    fn test_select_document_with_empty_profiles() {
        let profiles: Vec<DocumentProfile> = vec![];
        let job_industries = vec!["Engineering".to_string()];
        let job_keywords = vec!["Software".to_string()];
        let job_seniority = vec!["Senior".to_string()];

        let selected = select_document(&profiles, &job_industries, &job_keywords, &job_seniority, "fallback.pdf", "resume");
        assert_eq!(selected, "fallback.pdf");
    }
}