// Simple GUI demonstration for location configuration
use autonomous_job_agent::location_config_gui::DEFAULT_LOCATIONS;

#[cfg(feature = "egui_gui")]
use autonomous_job_agent::location_gui_app::run_gui_threaded;

fn main() {
    println!("Location Configuration GUI Demo");
    println!("Default locations: {:?}", DEFAULT_LOCATIONS);
    
    #[cfg(feature = "egui_gui")]
    {
        println!("GUI feature is enabled - attempting to show location configuration GUI...");
        println!("Note: This will show a graphical interface for location selection.");
        
        // Test the GUI with some existing locations
        let existing_locations = vec!["Beaverton, OR".to_string(), "Portland, OR".to_string()];
        
        match run_gui_threaded(existing_locations) {
            Ok(result) => {
                println!("GUI Result:");
                println!("- Cancelled: {}", result.cancelled);
                println!("- Selected locations: {:?}", result.locations);
                println!("- Hawaii included: {}", result.locations.iter().any(|loc| loc.to_lowercase().contains("hawaii")));
            },
            Err(e) => {
                println!("GUI error: {}", e);
            }
        }
    }
    
    #[cfg(not(feature = "egui_gui"))]
    {
        println!("GUI feature is disabled - running in command-line mode.");
        println!("To enable GUI, run with: cargo run --features egui_gui --bin gui_demo");
    }
}