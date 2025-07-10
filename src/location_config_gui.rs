use std::fs;
use std::path::Path;
use std::io::{self, Write};
use anyhow::Result;

/// Default locations recommended for job search
const DEFAULT_LOCATIONS: &[&str] = &[
    "Beaverton, OR",
    "Portland, OR", 
    "Remote",
    "Hawaii",
];

/// Location configuration GUI state
pub struct LocationConfigGui {
    pub selected_locations: Vec<(String, bool)>,
    pub custom_location: String,
    pub show_gui: bool,
    pub config_complete: bool,
    pub error_message: Option<String>,
}

impl Default for LocationConfigGui {
    fn default() -> Self {
        let default_selected = DEFAULT_LOCATIONS.iter()
            .map(|&loc| (loc.to_string(), false))
            .collect();
        
        Self {
            selected_locations: default_selected,
            custom_location: String::new(),
            show_gui: false,
            config_complete: false,
            error_message: None,
        }
    }
}

impl LocationConfigGui {
    /// Create a new LocationConfigGui with existing config loaded
    pub fn new(existing_locations: Vec<String>) -> Self {
        let mut gui = Self::default();
        
        // Mark existing locations as selected
        for existing_loc in existing_locations {
            // Check if it's in defaults
            if let Some(pos) = gui.selected_locations.iter().position(|(loc, _)| loc == &existing_loc) {
                gui.selected_locations[pos].1 = true;
            } else {
                // Add as custom location
                gui.selected_locations.push((existing_loc, true));
            }
        }
        
        gui
    }

    /// Check if configuration is complete (at least one location selected)
    pub fn is_config_complete(&self) -> bool {
        self.selected_locations.iter().any(|(_, selected)| *selected)
    }

    /// Get the list of selected locations
    pub fn get_selected_locations(&self) -> Vec<String> {
        self.selected_locations.iter()
            .filter_map(|(loc, selected)| if *selected { Some(loc.clone()) } else { None })
            .collect()
    }

    /// Save the configuration to AgentConfig.toml
    pub fn save_to_config(&self, config_path: &str) -> Result<()> {
        let config_path = Path::new(config_path);
        
        if !config_path.exists() {
            return Err(anyhow::anyhow!("Config file does not exist: {}", config_path.display()));
        }
        
        // Read existing config
        let content = fs::read_to_string(config_path)?;
        let mut config_value: toml::Value = toml::from_str(&content)?;
        
        // Update the location and search_locations fields
        let selected_locations = self.get_selected_locations();
        
        // Update location field (user's preferred locations)
        if let Some(table) = config_value.as_table_mut() {
            table.insert("location".to_string(), toml::Value::Array(
                selected_locations.iter().map(|loc| toml::Value::String(loc.clone())).collect()
            ));
            
            // Update search_locations field (for search engine configuration)
            table.insert("search_locations".to_string(), toml::Value::Array(
                selected_locations.iter().map(|loc| toml::Value::String(loc.clone())).collect()
            ));
        }
        
        // Write back to file
        let updated_content = toml::to_string_pretty(&config_value)?;
        fs::write(config_path, updated_content)?;
        
        Ok(())
    }
}

/// Check if the configuration needs to be set up via GUI
pub fn needs_location_config(config: &crate::config::Config) -> bool {
    // Check if location configuration is missing or incomplete
    config.location.is_empty() || 
    config.search_locations.is_empty() ||
    !config.location.iter().any(|loc| loc.to_lowercase().contains("hawaii"))
}

/// Run the location configuration (simplified version without GUI for now)
pub fn run_location_config_gui(existing_locations: Vec<String>) -> Result<Vec<String>> {
    println!("\n=== Job Search Location Configuration ===");
    println!("Current locations: {:?}", existing_locations);
    
    let mut gui = LocationConfigGui::new(existing_locations);
    
    // If existing locations include Hawaii, consider it complete
    if gui.selected_locations.iter().any(|(loc, selected)| *selected && loc.to_lowercase().contains("hawaii")) {
        println!("Location configuration is complete (Hawaii is included)");
        return Ok(gui.get_selected_locations());
    }
    
    println!("Hawaii is not in your current location list. Adding it as a recommended location.");
    
    // Auto-select Hawaii if not already selected
    if let Some(pos) = gui.selected_locations.iter().position(|(loc, _)| loc == "Hawaii") {
        gui.selected_locations[pos].1 = true;
    }
    
    // For now, return the updated locations with Hawaii included
    println!("Updated locations: {:?}", gui.get_selected_locations());
    
    Ok(gui.get_selected_locations())
}

#[cfg(feature = "egui_gui")]
mod egui_gui {
    use super::*;
    use eframe::egui;
    
    impl LocationConfigGui {
        /// Show the GUI configuration window
        pub fn show_config_window(&mut self, ctx: &egui::Context) -> bool {
            let mut should_close = false;
            
            egui::Window::new("Job Search Location Configuration")
                .collapsible(false)
                .resizable(false)
                .default_width(500.0)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.heading("Configure Job Search Locations");
                    ui.separator();
                    
                    ui.label("Select your preferred job search locations:");
                    ui.add_space(10.0);
                    
                    // Show error message if any
                    if let Some(ref error) = self.error_message {
                        ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                        ui.add_space(5.0);
                    }
                    
                    // Default locations checkboxes
                    ui.group(|ui| {
                        ui.label("Recommended Locations:");
                        for i in 0..DEFAULT_LOCATIONS.len() {
                            if i < self.selected_locations.len() {
                                let (loc, selected) = &mut self.selected_locations[i];
                                ui.checkbox(selected, loc);
                            }
                        }
                    });
                    
                    ui.add_space(10.0);
                    
                    // Custom locations
                    ui.group(|ui| {
                        ui.label("Custom Locations:");
                        
                        // Show existing custom locations
                        let mut to_remove = Vec::new();
                        for (i, (loc, selected)) in self.selected_locations.iter_mut().enumerate() {
                            if i >= DEFAULT_LOCATIONS.len() {
                                ui.horizontal(|ui| {
                                    ui.checkbox(selected, loc);
                                    if ui.small_button("Remove").clicked() {
                                        to_remove.push(i);
                                    }
                                });
                            }
                        }
                        
                        // Remove marked locations
                        for &i in to_remove.iter().rev() {
                            self.selected_locations.remove(i);
                        }
                        
                        // Add new custom location
                        ui.horizontal(|ui| {
                            ui.label("Add custom location:");
                            ui.text_edit_singleline(&mut self.custom_location);
                            if ui.button("Add").clicked() && !self.custom_location.trim().is_empty() {
                                let new_location = self.custom_location.trim().to_string();
                                if !self.selected_locations.iter().any(|(loc, _)| loc == &new_location) {
                                    self.selected_locations.push((new_location, true));
                                }
                                self.custom_location.clear();
                            }
                        });
                    });
                    
                    ui.add_space(20.0);
                    
                    // Status and action buttons
                    ui.horizontal(|ui| {
                        let selected_count = self.selected_locations.iter().filter(|(_, selected)| *selected).count();
                        ui.label(format!("Selected: {} locations", selected_count));
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Save Configuration").clicked() {
                                if self.is_config_complete() {
                                    should_close = true;
                                    self.config_complete = true;
                                    self.error_message = None;
                                } else {
                                    self.error_message = Some("Please select at least one location.".to_string());
                                }
                            }
                            
                            if ui.button("Cancel").clicked() {
                                should_close = true;
                                self.config_complete = false;
                            }
                        });
                    });
                    
                    ui.add_space(10.0);
                    ui.label("Note: Hawaii is now included as a recommended location for remote work opportunities.");
                });
            
            should_close
        }
    }
}