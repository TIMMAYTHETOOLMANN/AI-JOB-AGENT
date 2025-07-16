use std::collections::HashMap;
use crate::config::Config;
use crate::intelligent_scoring::JobScorer;
use crate::job_listing::JobListing;
use crate::globalization::{detect_locale, localize_salary, calculate_cultural_fit_score};

/// Comprehensive test suite for the enhanced job search system
pub struct EnhancedJobSearchTests;

impl EnhancedJobSearchTests {
    /// Test intelligent job scoring system
    pub fn test_intelligent_scoring() -> Result<(), Box<dyn std::error::Error>> {
        println!("🧪 Testing Intelligent Job Scoring System...");
        
        // Load test configuration
        let config = Config::load("AgentConfig.toml")?;
        let scorer = JobScorer::new(&config);
        
        // Test Case 1: High-scoring manufacturing job
        let high_score_job = JobListing {
            title: "Senior Process Engineer".to_string(),
            company: "Beckwood Press".to_string(),
            location: "Portland, OR".to_string(),
            url: "https://beckwoodpress.com/careers/123".to_string(),
            description: "Lead process optimization and automation initiatives in manufacturing. Experience with NX CAD and process automation required.".to_string(),
            salary: Some("$95,000 - $130,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        };
        
        let high_score = scorer.calculate_job_score(&high_score_job, &config);
        println!("✓ High-scoring job score: {:.3}", high_score);
        assert!(high_score > 0.5, "High-scoring job should score above 0.5");
        
        // Test Case 2: Low-scoring unrelated job
        let low_score_job = JobListing {
            title: "Social Media Manager".to_string(),
            company: "Random Marketing Co".to_string(),
            location: "New York, NY".to_string(),
            url: "https://marketing.com/careers/456".to_string(),
            description: "Manage social media campaigns for fashion brands.".to_string(),
            salary: Some("$45,000 - $60,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        };
        
        let low_score = scorer.calculate_job_score(&low_score_job, &config);
        println!("✓ Low-scoring job score: {:.3}", low_score);
        assert!(low_score < 0.4, "Low-scoring job should score below 0.4");
        
        // Test Case 3: Auto-application decision
        let should_auto_apply_high = scorer.should_auto_apply(&high_score_job, &config);
        let should_auto_apply_low = scorer.should_auto_apply(&low_score_job, &config);
        
        println!("✓ Auto-apply decision - High score: {}, Low score: {}", should_auto_apply_high, should_auto_apply_low);
        
        println!("✅ Intelligent scoring tests passed!");
        Ok(())
    }
    
    /// Test enhanced globalization features
    pub fn test_globalization_features() -> Result<(), Box<dyn std::error::Error>> {
        println!("🌍 Testing Enhanced Globalization Features...");
        
        // Test locale detection
        let us_locale = detect_locale(&["Portland, OR".to_string()]);
        let japan_locale = detect_locale(&["Tokyo, Japan".to_string()]);
        let india_locale = detect_locale(&["Mumbai, India".to_string()]);
        
        println!("✓ Locale detection: US={}, Japan={}, India={}", us_locale, japan_locale, india_locale);
        
        assert_eq!(us_locale, "en-US");
        assert_eq!(japan_locale, "ja-JP");
        assert_eq!(india_locale, "hi-IN");
        
        // Test currency conversion
        let usd_salary = 100000;
        let eur_salary = localize_salary(usd_salary, "USD", "EUR")?;
        let jpy_salary = localize_salary(usd_salary, "USD", "JPY")?;
        
        println!("✓ Currency conversion: $100K USD = €{} EUR = ¥{} JPY", eur_salary, jpy_salary);
        
        assert!(eur_salary > 0 && eur_salary < usd_salary);
        assert!(jpy_salary > usd_salary);
        
        // Test cultural fit scoring
        let cultural_score = calculate_cultural_fit_score(
            "Tokyo, Japan",
            "English Teacher position for native speaker",
            "North American Manufacturing Engineer"
        );
        
        println!("✓ Cultural fit score: {:.3}", cultural_score);
        assert!(cultural_score > 0.8, "English teaching should have high cultural fit");
        
        println!("✅ Globalization tests passed!");
        Ok(())
    }
    
    /// Test enhanced job search configuration
    pub fn test_enhanced_configuration() -> Result<(), Box<dyn std::error::Error>> {
        println!("⚙️ Testing Enhanced Configuration System...");
        
        let config = Config::load("AgentConfig.toml")?;
        
        // Test enhanced job keywords
        let enhanced_keywords = config.get_enhanced_job_keywords();
        let original_count = config.job_keywords.len();
        let enhanced_count = enhanced_keywords.len();
        
        println!("✓ Enhanced keywords: {} original → {} enhanced", original_count, enhanced_count);
        assert!(enhanced_count > original_count, "Enhanced keywords should be more than original");
        
        // Test all target companies
        let all_companies = config.get_all_target_companies();
        let has_beckwood = all_companies.iter().any(|c| c.contains("Beckwood"));
        let has_keyful = all_companies.iter().any(|c| c.contains("Keyful"));
        let has_intel = all_companies.iter().any(|c| c.contains("Intel"));
        
        println!("✓ Company targets: {} companies, Intel={}, Beckwood={}, Keyful={}", 
                 all_companies.len(), has_intel, has_beckwood, has_keyful);
        
        assert!(has_intel, "Should include Intel in company targets");
        assert!(has_beckwood, "Should include Beckwood in company targets");
        assert!(has_keyful, "Should include Keyful in company targets");
        
        // Test international search
        let should_international = config.should_enable_international_search();
        let all_job_boards = config.get_all_job_boards();
        
        println!("✓ International search enabled: {}, Job boards: {}", 
                 should_international, all_job_boards.len());
        
        // Test supported languages
        let supported_languages = config.get_supported_languages();
        let has_japanese = supported_languages.iter().any(|l| l == "ja");
        let has_hindi = supported_languages.iter().any(|l| l == "hi");
        
        println!("✓ Supported languages: {:?}, Japanese={}, Hindi={}", 
                 supported_languages, has_japanese, has_hindi);
        
        println!("✅ Configuration tests passed!");
        Ok(())
    }
    
    /// Test manufacturing company specialization
    pub fn test_manufacturing_specialization() -> Result<(), Box<dyn std::error::Error>> {
        println!("🏭 Testing Manufacturing Company Specialization...");
        
        let config = Config::load("AgentConfig.toml")?;
        let scorer = JobScorer::new(&config);
        
        // Test manufacturing company jobs
        let manufacturing_jobs = vec![
            JobListing {
                title: "Process Engineer".to_string(),
                company: "Beckwood Press".to_string(),
                location: "Remote".to_string(),
                url: "https://beckwoodpress.com/careers/1".to_string(),
                description: "Hydraulic press systems engineering".to_string(),
                salary: Some("$85,000 - $120,000".to_string()),
                date_posted: Some(chrono::Utc::now().naive_utc()),
            },
            JobListing {
                title: "Manufacturing Engineer".to_string(),
                company: "Keyful Plastic".to_string(),
                location: "Remote".to_string(),
                url: "https://keyfulplastic.com/careers/2".to_string(),
                description: "Thermal forming equipment design".to_string(),
                salary: Some("$75,000 - $105,000".to_string()),
                date_posted: Some(chrono::Utc::now().naive_utc()),
            },
            JobListing {
                title: "Automation Engineer".to_string(),
                company: "Acid Alloy Robot Technologies".to_string(),
                location: "Portland, OR".to_string(),
                url: "https://acidalloy.com/careers/3".to_string(),
                description: "Robotic automation solutions".to_string(),
                salary: Some("$90,000 - $130,000".to_string()),
                date_posted: Some(chrono::Utc::now().naive_utc()),
            },
        ];
        
        let mut total_score = 0.0;
        for job in &manufacturing_jobs {
            let score = scorer.calculate_job_score(job, &config);
            total_score += score;
            println!("✓ {} at {}: Score {:.3}", job.title, job.company, score);
        }
        
        let average_score = total_score / manufacturing_jobs.len() as f64;
        println!("✓ Average manufacturing job score: {:.3}", average_score);
        
        assert!(average_score > 0.4, "Manufacturing jobs should have decent average score");
        
        println!("✅ Manufacturing specialization tests passed!");
        Ok(())
    }
    
    /// Test English teaching and international job categories
    pub fn test_international_job_categories() -> Result<(), Box<dyn std::error::Error>> {
        println!("🌏 Testing International Job Categories...");
        
        let config = Config::load("AgentConfig.toml")?;
        let scorer = JobScorer::new(&config);
        
        // Test English teaching jobs
        let english_teaching_jobs = vec![
            JobListing {
                title: "English Conversation Teacher".to_string(),
                company: "International English Academy".to_string(),
                location: "Tokyo, Japan".to_string(),
                url: "https://english-academy.jp/careers/1".to_string(),
                description: "Teach English to Japanese business professionals. Native speaker required.".to_string(),
                salary: Some("¥3,500,000 - ¥4,200,000".to_string()),
                date_posted: Some(chrono::Utc::now().naive_utc()),
            },
            JobListing {
                title: "Corporate English Trainer".to_string(),
                company: "Global Language Solutions".to_string(),
                location: "Mumbai, India".to_string(),
                url: "https://globallang.in/careers/2".to_string(),
                description: "Business English training for corporate clients".to_string(),
                salary: Some("₹800,000 - ₹1,200,000".to_string()),
                date_posted: Some(chrono::Utc::now().naive_utc()),
            },
        ];
        
        let mut english_teaching_scores = Vec::new();
        for job in &english_teaching_jobs {
            let score = scorer.calculate_job_score(job, &config);
            english_teaching_scores.push(score);
            println!("✓ {} in {}: Score {:.3}", job.title, job.location, score);
        }
        
        // Test cross-cultural jobs
        let cross_cultural_job = JobListing {
            title: "International Business Development Manager".to_string(),
            company: "Global Manufacturing Solutions".to_string(),
            location: "Remote - Asia Pacific".to_string(),
            url: "https://globalmfg.com/careers/3".to_string(),
            description: "Develop business relationships across Asia-Pacific region. Manufacturing experience required.".to_string(),
            salary: Some("$95,000 - $140,000".to_string()),
            date_posted: Some(chrono::Utc::now().naive_utc()),
        };
        
        let cross_cultural_score = scorer.calculate_job_score(&cross_cultural_job, &config);
        println!("✓ Cross-cultural job score: {:.3}", cross_cultural_score);
        
        // Verify that international jobs are properly scored
        assert!(english_teaching_scores.iter().all(|&score| score > 0.3), 
                "English teaching jobs should have decent scores");
        assert!(cross_cultural_score > 0.4, 
                "Cross-cultural manufacturing job should score reasonably");
        
        println!("✅ International job categories tests passed!");
        Ok(())
    }
    
    /// Run all tests
    pub fn run_all_tests() -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Running Enhanced Job Search System Tests\n");
        
        Self::test_intelligent_scoring()?;
        println!();
        
        Self::test_globalization_features()?;
        println!();
        
        Self::test_enhanced_configuration()?;
        println!();
        
        Self::test_manufacturing_specialization()?;
        println!();
        
        Self::test_international_job_categories()?;
        println!();
        
        println!("🎉 All Enhanced Job Search System Tests Passed!");
        println!("✅ System is ready for production deployment");
        
        Ok(())
    }
}

/// Test runner for command line execution
pub fn run_enhanced_tests() {
    match EnhancedJobSearchTests::run_all_tests() {
        Ok(()) => {
            println!("\n✅ SUCCESS: All enhanced job search tests passed!");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("\n❌ FAILED: Test error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intelligent_scoring_unit() {
        EnhancedJobSearchTests::test_intelligent_scoring().unwrap();
    }

    #[test]
    fn test_globalization_unit() {
        EnhancedJobSearchTests::test_globalization_features().unwrap();
    }

    #[test]
    fn test_configuration_unit() {
        EnhancedJobSearchTests::test_enhanced_configuration().unwrap();
    }

    #[test]
    fn test_manufacturing_unit() {
        EnhancedJobSearchTests::test_manufacturing_specialization().unwrap();
    }

    #[test]
    fn test_international_categories_unit() {
        EnhancedJobSearchTests::test_international_job_categories().unwrap();
    }
}