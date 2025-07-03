use log::{info, debug};
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{Rng, SeedableRng};

/// Represents a resume or cover letter profile
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
            },
            DocumentProfile {
                path: "resume2.pdf".to_string(),
                name: "Marketing Resume".to_string(),
                industries: vec!["Marketing".to_string()],
                keywords: vec!["Digital".to_string()],
                seniority: vec!["Mid".to_string()],
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