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

#[derive(Clone)]
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
            ui.heading("JARVIS 2.0 - Global Autonomous Job Agent Monitor");
            ui.horizontal(|ui| {
                ui.label(format!("Jobs found: {}", status.jobs_found));
                ui.label(format!("Applications sent: {}", status.applications_sent));
                ui.label(format!("Errors: {}", status.errors));
            });
            ui.separator();
            ui.label("Recent Events:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for event in events.iter().rev() {
                    ui.colored_label(
                        match event.level.as_str() {
                            "error" => egui::Color32::RED,
                            "warn" => egui::Color32::YELLOW,
                            _ => egui::Color32::WHITE,
                        },
                        format!("[{}] {}: {}", event.timestamp, event.level, event.message)
                    );
                }
            });
        });
    }
}

pub fn run_gui(rx: Receiver<RuntimeEvent>) {
    let app = AgentMonitorApp::new(rx);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "JARVIS 2.0 - Global Autonomous Job Agent Monitor",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}
