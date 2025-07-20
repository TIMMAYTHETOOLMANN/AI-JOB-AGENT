use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("🎮 AI Job Agent - Production GUI");
    println!("🚀 Starting desktop application...");

    // Load configuration
    let config = autonomous_job_agent::config::Config::load("AgentConfig.toml")?;

    // Run the GUI
    autonomous_job_agent::main_gui::ProductionGui::run()?;

    Ok(())
}
