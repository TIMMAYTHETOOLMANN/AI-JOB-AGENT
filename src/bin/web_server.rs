use std::sync::{Arc, Mutex};
use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("🌐 AI Job Agent - Production Web Server");
    println!("🚀 Starting web-based control center...");

    // Load configuration
    let config = autonomous_job_agent::config::Config::load("AgentConfig.toml")?;
    let app_state = Arc::new(Mutex::new(
        autonomous_job_agent::application_state::ApplicationState::new("state/web_state.json")
    ));

    // Create and start web server
    let web_server = autonomous_job_agent::web_server::WebServer::new(8080, app_state);

    println!("🌐 Production Control Center URLs:");
    println!("   📊 Dashboard: http://localhost:8080/dashboard");
    println!("   🎮 Control Panel: http://localhost:8080/control");
    println!("   📋 Logs: http://localhost:8080/logs");
    println!("   🔧 API: http://localhost:8080/api");
    println!("   📡 WebSocket: ws://localhost:8080/ws");

    web_server.start().await?;

    Ok(())
}
