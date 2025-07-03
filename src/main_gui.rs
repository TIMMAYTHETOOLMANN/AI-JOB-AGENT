use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use eframe::egui;
use tokio::runtime::Runtime;

use crate::config::Config;
use crate::job_search_engine::search_jobs;
use crate::job_application::process_and_apply_to_jobs;
use crate::application_state::ApplicationState;

#[derive(Default, Clone)]
pub struct JobDetail {
    pub title: String,
    pub company: String,
    pub location: String,
    pub url: String,
    pub status: String,
}

#[derive(Default, Clone)]
pub struct AgentStats {
    pub jobs_found: usize,
    pub jobs_applied: usize,
    pub last_status: String,
    pub logs: Vec<String>,
    pub job_details: Vec<JobDetail>,
}

pub struct AgentApp {
    stats: Arc<Mutex<AgentStats>>,
}

impl eframe::App for AgentApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let stats = self.stats.lock().unwrap().clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Autonomous Job Agent GUI");
            ui.label(format!("Jobs found: {}", stats.jobs_found));
            ui.label(format!("Jobs applied: {}", stats.jobs_applied));
            ui.label(format!("Status: {}", stats.last_status));
            ui.separator();
            ui.label("Job Details:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for job in stats.job_details.iter().rev().take(50) {
                    ui.group(|ui| {
                        ui.label(format!("{} | {} | {}", job.title, job.company, job.location));
                        ui.label(format!("Status: {}", job.status));
                        ui.hyperlink(&job.url);
                    });
                }
            });
            ui.separator();
            ui.label("Recent Activity:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in stats.logs.iter().rev().take(20) {
                    ui.label(log);
                }
            });
        });
        ctx.request_repaint_after(Duration::from_millis(1000));
    }
}

fn main() {
    let stats = Arc::new(Mutex::new(AgentStats::default()));
    let stats_bg = stats.clone();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        let config = Config::load("AgentConfig.toml").expect("Failed to load config");
        let app_state = Arc::new(Mutex::new(ApplicationState::new("state/job_state.json")));
        loop {
            {
                let mut s = stats_bg.lock().unwrap();
                s.last_status = "Searching for jobs...".to_string();
                s.logs.push("Starting job search cycle".to_string());
                s.job_details.clear();
            }
            let jobs = rt.block_on(search_jobs(&config)).unwrap_or_default();
            {
                let mut s = stats_bg.lock().unwrap();
                s.jobs_found = jobs.len();
                s.last_status = format!("Found {} jobs", jobs.len());
                s.logs.push(format!("Found {} jobs", jobs.len()));
                for job in &jobs {
                    s.job_details.push(JobDetail {
                        title: job.title.clone(),
                        company: job.company.clone(),
                        location: job.location.clone(),
                        url: job.url.clone(),
                        status: "Queued".to_string(),
                    });
                }
            }
            let mut job_statuses = vec![];
            let applied = rt.block_on(async {
                let mut applied_count = 0;
                for (i, job) in jobs.iter().enumerate() {
                    // Simulate per-job application and status update
                    let status = match process_and_apply_to_jobs(
                        &[job.url.clone()],
                        &config.name,
                        &config.email,
                        &config.resume_path,
                        &config.cover_letter_path,
                        &config.job_keywords,
                        Some(app_state.clone()),
                        config.resume_profiles.as_deref(),
                        config.cover_letter_profiles.as_deref(),
                        &config.industries,
                        &config.seniority_levels,
                        None, // TODO: pass browser session if needed
                    ).await {
                        Ok(_) => {
                            applied_count += 1;
                            "Applied".to_string()
                        },
                        Err(e) => format!("Error: {}", e),
                    };
                    job_statuses.push((i, status));
                }
                applied_count
            });
            {
                let mut s = stats_bg.lock().unwrap();
                s.jobs_applied = applied;
                s.last_status = format!("Applied to {} jobs", applied);
                s.logs.push(format!("Applied to {} jobs", applied));
                for (i, status) in job_statuses {
                    if let Some(job) = s.job_details.get_mut(i) {
                        job.status = status;
                    }
                }
            }
            thread::sleep(Duration::from_secs(600));
        }
    });
    let app = AgentApp { stats };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
