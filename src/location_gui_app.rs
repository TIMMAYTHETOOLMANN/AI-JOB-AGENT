use eframe::egui;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use anyhow::Result;

use crate::location_config_gui::{LocationConfigGui, DEFAULT_LOCATIONS};

/// Result of the GUI configuration
#[derive(Debug, Clone)]
pub struct GuiConfigResult {
    pub locations: Vec<String>,
    pub cancelled: bool,
}

/// GUI Application for location configuration
pub struct LocationConfigApp {
    gui: LocationConfigGui,
    result_sender: Option<mpsc::Sender<GuiConfigResult>>,
}

impl LocationConfigApp {
    pub fn new(existing_locations: Vec<String>, sender: mpsc::Sender<GuiConfigResult>) -> Self {
        Self {
            gui: LocationConfigGui::new(existing_locations),
            result_sender: Some(sender),
        }
    }
}

impl eframe::App for LocationConfigApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Job Search Location Configuration");
            ui.separator();
            
            ui.label("Select your preferred job search locations:");
            ui.add_space(10.0);
            
            // Show error message if any
            if let Some(ref error) = self.gui.error_message {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                ui.add_space(5.0);
            }
            
            // Default locations checkboxes
            ui.group(|ui| {
                ui.label("Recommended Locations:");
                for i in 0..DEFAULT_LOCATIONS.len() {
                    if i < self.gui.selected_locations.len() {
                        let (loc, selected) = &mut self.gui.selected_locations[i];
                        ui.checkbox(selected, loc.as_str());
                    }
                }
            });
            
            ui.add_space(10.0);
            
            // Custom locations
            ui.group(|ui| {
                ui.label("Custom Locations:");
                
                // Show existing custom locations
                let mut to_remove = Vec::new();
                for (i, (loc, selected)) in self.gui.selected_locations.iter_mut().enumerate() {
                    if i >= DEFAULT_LOCATIONS.len() {
                        ui.horizontal(|ui| {
                            ui.checkbox(selected, loc.as_str());
                            if ui.small_button("Remove").clicked() {
                                to_remove.push(i);
                            }
                        });
                    }
                }
                
                // Remove marked locations
                for &i in to_remove.iter().rev() {
                    self.gui.selected_locations.remove(i);
                }
                
                // Add new custom location
                ui.horizontal(|ui| {
                    ui.label("Add custom location:");
                    ui.text_edit_singleline(&mut self.gui.custom_location);
                    if ui.button("Add").clicked() && !self.gui.custom_location.trim().is_empty() {
                        let new_location = self.gui.custom_location.trim().to_string();
                        if !self.gui.selected_locations.iter().any(|(loc, _)| loc == &new_location) {
                            self.gui.selected_locations.push((new_location, true));
                        }
                        self.gui.custom_location.clear();
                    }
                });
            });
            
            ui.add_space(20.0);
            
            // Status and action buttons
            ui.horizontal(|ui| {
                let selected_count = self.gui.selected_locations.iter().filter(|(_, selected)| *selected).count();
                ui.label(format!("Selected: {} locations", selected_count));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Save Configuration").clicked() {
                        if self.gui.is_config_complete() {
                            if let Some(sender) = self.result_sender.take() {
                                let result = GuiConfigResult {
                                    locations: self.gui.get_selected_locations(),
                                    cancelled: false,
                                };
                                let _ = sender.send(result);
                            }
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        } else {
                            self.gui.error_message = Some("Please select at least one location.".to_string());
                        }
                    }
                    
                    if ui.button("Cancel").clicked() {
                        if let Some(sender) = self.result_sender.take() {
                            let result = GuiConfigResult {
                                locations: DEFAULT_LOCATIONS.iter().map(|&s| s.to_string()).collect(),
                                cancelled: true,
                            };
                            let _ = sender.send(result);
                        }
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
            
            ui.add_space(10.0);
            ui.label("Note: Hawaii is now included as a recommended location for remote work opportunities.");
        });
    }
}

/// Run the GUI in a separate thread and return the result
pub fn run_gui_threaded(existing_locations: Vec<String>) -> Result<GuiConfigResult> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(egui::vec2(600.0, 500.0))
                .with_resizable(false)
                .with_min_inner_size(egui::vec2(400.0, 300.0)),
            ..Default::default()
        };
        
        let app = LocationConfigApp::new(existing_locations, sender);
        
        let _ = eframe::run_native(
            "Location Configuration",
            options,
            Box::new(|_cc| Box::new(app)),
        );
    });
    
    // Wait for the result with a timeout
    match receiver.recv_timeout(Duration::from_secs(300)) {
        Ok(result) => Ok(result),
        Err(_) => {
            // Timeout or channel closed, return defaults
            Ok(GuiConfigResult {
                locations: DEFAULT_LOCATIONS.iter().map(|&s| s.to_string()).collect(),
                cancelled: true,
            })
        }
    }
}