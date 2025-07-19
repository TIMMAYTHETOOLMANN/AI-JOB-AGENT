use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use eframe::egui;

use crate::config;

// === JARVIS 2.0 AUTONOMY INJECTION ===
// Global runtime monitoring, recursive troubleshooting, self-heal, full-stack autonomy

#[derive(Clone, Debug)]
pub struct RuntimeEvent {
    pub timestamp: String,
    pub message: String,
    pub level: String, // info, warn, error
}

struct AgentStatus {
    jobs_found: usize,
    applications_sent: usize,
    errors: usize,
    logs: Vec<String>,
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self {
            jobs_found: 0,
            applications_sent: 0,
            errors: 0,
            logs: vec!["Agent started.".to_string()],
        }
    }
}

pub struct AgentMonitorApp {
    status: Arc<Mutex<AgentStatus>>,
    events: Arc<Mutex<VecDeque<RuntimeEvent>>>,
    rx: Receiver<RuntimeEvent>,
}

impl AgentMonitorApp {
    pub fn new(rx: Receiver<RuntimeEvent>) -> Self {
        Self {
            status: Arc::new(Mutex::new(AgentStatus::default())),
            events: Arc::new(Mutex::new(VecDeque::with_capacity(200))),
            rx,
        }
    }

    fn poll_events(&mut self) {
        while let Ok(event) = self.rx.try_recv() {
            let mut events = self.events.lock().unwrap();
            if events.len() == 200 { events.pop_front(); }
            events.push_back(event.clone());
            let mut status = self.status.lock().unwrap();
            // Log every event for debugging
            status.logs.push(format!("[DEBUG] Received event: {:?}", event));
            // Update jobs_found for job search events
            if event.level == "info" && (event.message.contains("Found ") && event.message.contains("job listing")) {
                if let Some(count) = event.message.split_whitespace().nth(1) {
                    if let Ok(num) = count.parse::<usize>() {
                        status.jobs_found += num;
                    } else {
                        status.jobs_found += 1;
                    }
                } else {
                    status.jobs_found += 1;
                }
            }
            // Update applications_sent for application events
            if event.message.contains("Applied to") {
                if let Some(count) = event.message.split_whitespace().nth(2) {
                    if let Ok(num) = count.parse::<usize>() {
                        status.applications_sent += num;
                    } else {
                        status.applications_sent += 1;
                    }
                } else {
                    status.applications_sent += 1;
                }
            } else if event.message.contains("Successfully applied") {
                status.applications_sent += 1;
            }
            // Update errors for warn/error events
            match event.level.as_str() {
                "warn" | "error" => status.errors += 1,
                _ => {}
            }
            status.logs.push(format!("[{}] {}: {}", event.timestamp, event.level, event.message));
        }
    }
}

impl eframe::App for AgentMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_events();
        let status = self.status.lock().unwrap();
        let events = self.events.lock().unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🤖 AI Job Agent - Production Browser Monitor");
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("📊 Status:");
                ui.colored_label(egui::Color32::GREEN, format!("Jobs Found: {}", status.jobs_found));
                ui.colored_label(egui::Color32::BLUE, format!("Applications: {}", status.applications_sent));
                ui.colored_label(egui::Color32::RED, format!("Errors: {}", status.errors));
            });
            
            ui.separator();
            ui.label("🌐 Real Browser Activity Monitor:");
            ui.separator();
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                for event in events.iter().rev() {
                    let (color, icon) = match event.level.as_str() {
                        "error" => (egui::Color32::RED, "❌"),
                        "warn" => (egui::Color32::YELLOW, "⚠️"),
                        "info" => {
                            if event.message.contains("Production browser") {
                                (egui::Color32::GREEN, "🚀")
                            } else if event.message.contains("Navigating to") || event.message.contains("Found") {
                                (egui::Color32::LIGHT_BLUE, "🌐")
                            } else if event.message.contains("Applied") || event.message.contains("clicked") {
                                (egui::Color32::LIGHT_GREEN, "✅")
                            } else {
                                (egui::Color32::WHITE, "ℹ️")
                            }
                        },
                        _ => (egui::Color32::WHITE, "ℹ️"),
                    };
                    
                    ui.horizontal(|ui| {
                        ui.label(icon);
                        ui.colored_label(color, format!("[{}]", event.timestamp));
                        ui.colored_label(color, &event.message);
                    });
                }
            });
        });
        
        // Auto-refresh the UI to show real-time updates
        ctx.request_repaint_after(std::time::Duration::from_millis(1000));
    }
}

pub fn run_gui(rx: Receiver<RuntimeEvent>) {
    let app = AgentMonitorApp::new(rx);
    let native_options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "🤖 AI Job Agent - Production Browser Monitor",
        native_options,
        Box::new(|_cc| Box::new(app)),
    ) {
        eprintln!("Failed to run GUI: {}", e);
        std::process::exit(1);
    }
}
