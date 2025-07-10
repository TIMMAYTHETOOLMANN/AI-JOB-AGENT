use std::env;
use std::fs;
use std::path::Path;
use autonomous_job_agent::config::Config;
use autonomous_job_agent::location_config_gui::{needs_location_config, run_location_config_gui, LocationConfigGui};

fn main() {
    println!("Testing location configuration GUI...");
    
    // Test 1: Configuration with Hawaii should be complete
    let config_with_hawaii = Config {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        location: vec!["Beaverton, OR".to_string(), "Hawaii".to_string()],
        search_locations: vec!["Beaverton, OR".to_string(), "Hawaii".to_string()],
        // ... other fields would be filled with defaults
        open_to_relocation: Some(true),
        job_keywords: vec![],
        skills: vec![],
        industries: vec![],
        seniority_levels: vec![],
        targets: vec![],
        resume_path: "".to_string(),
        cover_letter_path: "".to_string(),
        resume_profiles: None,
        cover_letter_profiles: None,
        applied_jobs_path: None,
        min_salary: None,
        max_salary: None,
        search_engines: vec![],
        company_targets: vec![],
        preferred_locales: vec![],
        preferred_currency: None,
        timezone: None,
        enable_international_search: false,
        regional_job_boards: std::collections::HashMap::new(),
        enable_adaptive_automation: false,
        browser_retry_attempts: None,
        performance_monitoring: false,
        dynamic_profile_selection: false,
        max_concurrent_applications: None,
        application_rate_limit: None,
        intelligent_scheduling: false,
        a_b_testing_enabled: false,
    };
    
    println!("Config with Hawaii needs setup: {}", needs_location_config(&config_with_hawaii));
    
    // Test 2: Configuration without Hawaii should need setup
    let config_without_hawaii = Config {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        location: vec!["Beaverton, OR".to_string(), "Portland, OR".to_string()],
        search_locations: vec!["Beaverton, OR".to_string(), "Portland, OR".to_string()],
        // ... other fields would be filled with defaults
        open_to_relocation: Some(true),
        job_keywords: vec![],
        skills: vec![],
        industries: vec![],
        seniority_levels: vec![],
        targets: vec![],
        resume_path: "".to_string(),
        cover_letter_path: "".to_string(),
        resume_profiles: None,
        cover_letter_profiles: None,
        applied_jobs_path: None,
        min_salary: None,
        max_salary: None,
        search_engines: vec![],
        company_targets: vec![],
        preferred_locales: vec![],
        preferred_currency: None,
        timezone: None,
        enable_international_search: false,
        regional_job_boards: std::collections::HashMap::new(),
        enable_adaptive_automation: false,
        browser_retry_attempts: None,
        performance_monitoring: false,
        dynamic_profile_selection: false,
        max_concurrent_applications: None,
        application_rate_limit: None,
        intelligent_scheduling: false,
        a_b_testing_enabled: false,
    };
    
    println!("Config without Hawaii needs setup: {}", needs_location_config(&config_without_hawaii));
    
    // Test 3: Test the GUI with existing locations
    println!("\nTesting GUI with existing locations...");
    match run_location_config_gui(vec!["Beaverton, OR".to_string(), "Portland, OR".to_string()]) {
        Ok(locations) => {
            println!("GUI returned locations: {:?}", locations);
            println!("Hawaii included: {}", locations.iter().any(|loc| loc.to_lowercase().contains("hawaii")));
        },
        Err(e) => {
            println!("GUI error: {}", e);
        }
    }
    
    // Test 4: Test the config save functionality 
    println!("\nTesting config save functionality...");
    let gui = LocationConfigGui::new(vec!["Beaverton, OR".to_string(), "Hawaii".to_string()]);
    println!("Selected locations: {:?}", gui.get_selected_locations());
    
    println!("\nAll tests completed!");
}