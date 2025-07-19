use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::application_state::ApplicationState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub is_running: bool,
    pub jobs_found: usize,
    pub applications_submitted: usize,
    pub success_rate: f64,
    pub uptime_seconds: u64,
    pub last_activity: String,
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            jobs_found: 0,
            applications_submitted: 0,
            success_rate: 0.0,
            uptime_seconds: 0,
            last_activity: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

pub struct WebServer {
    status: Arc<Mutex<AgentStatus>>,
    app_state: Option<Arc<Mutex<ApplicationState>>>,
    start_time: std::time::SystemTime,
}

impl WebServer {
    pub fn new(app_state: Option<Arc<Mutex<ApplicationState>>>) -> Self {
        Self {
            status: Arc::new(Mutex::new(AgentStatus::default())),
            app_state,
            start_time: std::time::SystemTime::now(),
        }
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        println!("Web server started on port {}", port);

        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let status = self.status.clone();
                    let app_state = self.app_state.clone();
                    let start_time = self.start_time;
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_request(stream, status, app_state, start_time).await {
                            eprintln!("Error handling request: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }

    async fn handle_request(
        mut stream: TcpStream,
        status: Arc<Mutex<AgentStatus>>,
        app_state: Option<Arc<Mutex<ApplicationState>>>,
        start_time: std::time::SystemTime,
    ) -> Result<()> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        let (method, path) = Self::parse_request(&request);
        
        let response = match (method.as_str(), path.as_str()) {
            ("GET", "/health") => Self::handle_health(status, start_time).await,
            ("POST", "/start") => Self::handle_start(status).await,
            ("POST", "/stop") => Self::handle_stop(status).await,
            ("POST", "/deploy") => Self::handle_deploy().await,
            ("POST", "/rollback") => Self::handle_rollback().await,
            ("GET", "/status") => Self::handle_status(status, app_state, start_time).await,
            _ => Self::handle_404().await,
        };

        stream.write_all(response.as_bytes()).await?;
        stream.flush().await?;

        Ok(())
    }

    fn parse_request(request: &str) -> (String, String) {
        let lines: Vec<&str> = request.lines().collect();
        if let Some(first_line) = lines.first() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 2 {
                return (parts[0].to_string(), parts[1].to_string());
            }
        }
        ("GET".to_string(), "/".to_string())
    }

    async fn handle_health(status: Arc<Mutex<AgentStatus>>, start_time: std::time::SystemTime) -> String {
        let uptime = start_time.elapsed().unwrap_or(std::time::Duration::ZERO).as_secs();
        
        if let Ok(mut s) = status.lock() {
            s.uptime_seconds = uptime;
            s.last_activity = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }

        let response_body = json!({
            "status": "healthy",
            "uptime_seconds": uptime,
            "timestamp": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        });

        Self::create_response(200, "OK", &response_body.to_string())
    }

    async fn handle_start(status: Arc<Mutex<AgentStatus>>) -> String {
        if let Ok(mut s) = status.lock() {
            s.is_running = true;
            s.last_activity = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }

        let response_body = json!({
            "message": "Agent started successfully",
            "timestamp": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        });

        Self::create_response(200, "OK", &response_body.to_string())
    }

    async fn handle_stop(status: Arc<Mutex<AgentStatus>>) -> String {
        if let Ok(mut s) = status.lock() {
            s.is_running = false;
            s.last_activity = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }

        let response_body = json!({
            "message": "Agent stopped successfully",
            "timestamp": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        });

        Self::create_response(200, "OK", &response_body.to_string())
    }

    async fn handle_deploy() -> String {
        let response_body = json!({
            "message": "Deployment initiated",
            "timestamp": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        });

        Self::create_response(200, "OK", &response_body.to_string())
    }

    async fn handle_rollback() -> String {
        let response_body = json!({
            "message": "Rollback initiated",
            "timestamp": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        });

        Self::create_response(200, "OK", &response_body.to_string())
    }

    async fn handle_status(
        status: Arc<Mutex<AgentStatus>>,
        app_state: Option<Arc<Mutex<ApplicationState>>>,
        start_time: std::time::SystemTime,
    ) -> String {
        let uptime = start_time.elapsed().unwrap_or(std::time::Duration::ZERO).as_secs();
        
        let mut current_status = if let Ok(s) = status.lock() {
            s.clone()
        } else {
            AgentStatus::default()
        };

        current_status.uptime_seconds = uptime;

        // Update status from application state if available
        if let Some(app_state) = app_state {
            if let Ok(state) = app_state.lock() {
                // In a real implementation, you would extract metrics from the application state
                // For now, we'll use mock data
                current_status.jobs_found = 42; // Mock data
                current_status.applications_submitted = 18; // Mock data
                current_status.success_rate = 75.5; // Mock data
            }
        }

        Self::create_response(200, "OK", &serde_json::to_string(&current_status).unwrap_or_default())
    }

    async fn handle_404() -> String {
        let response_body = json!({
            "error": "Not Found",
            "message": "The requested endpoint was not found"
        });

        Self::create_response(404, "Not Found", &response_body.to_string())
    }

    fn create_response(status_code: u16, status_text: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Access-Control-Allow-Origin: *\r\n\
             Access-Control-Allow-Methods: GET, POST, PUT, DELETE\r\n\
             Access-Control-Allow-Headers: Content-Type\r\n\
             \r\n\
             {}",
            status_code,
            status_text,
            body.len(),
            body
        )
    }

    pub fn update_status<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut AgentStatus),
    {
        if let Ok(mut status) = self.status.lock() {
            update_fn(&mut status);
            status.last_activity = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }
}