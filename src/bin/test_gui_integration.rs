use autonomous_job_agent::config::Config;
use autonomous_job_agent::location_config_gui::{needs_location_config, run_location_config_gui, LocationConfigGui};

fn main() {
    println!("Testing location configuration GUI functionality...");
    
    // Test with a config that doesn't have Hawaii
    let config_without_hawaii = Config {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        location: vec!["Beaverton, OR".to_string(), "Portland, OR".to_string()],
        search_locations: vec!["Beaverton, OR".to_string(), "Portland, OR".to_string()],
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
    
    if needs_location_config(&config_without_hawaii) {
        println!("Running location configuration GUI...");
        match run_location_config_gui(config_without_hawaii.location.clone()) {
            Ok(locations) => {
                println!("Configuration completed!");
                println!("Selected locations: {:?}", locations);
                println!("Hawaii included: {}", locations.iter().any(|loc| loc.to_lowercase().contains("hawaii")));
                
                // Test saving to a config file
                let gui = LocationConfigGui::new(locations);
                println!("Would save the following locations to config:");
                println!("- Selected locations: {:?}", gui.get_selected_locations());
                
                // Test saving to the test config file
                match gui.save_to_config("AgentConfig_test.toml") {
                    Ok(()) => println!("Successfully saved configuration to AgentConfig_test.toml"),
                    Err(e) => println!("Failed to save configuration: {}", e),
                }
            },
            Err(e) => {
                println!("Configuration failed: {}", e);
            }
        }
    } else {
        println!("Location configuration is already complete!");
    }
    
    println!("\nTest completed!");
}