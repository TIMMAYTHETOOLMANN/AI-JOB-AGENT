use autonomous_job_agent::secrets::CredentialManager;
use autonomous_job_agent::credential_auth::authenticate_all_platforms;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== AI Job Agent - Credential Manager Test ===\n");
    
    // Initialize credential manager
    println!("🔍 Loading credentials from multiple sources...");
    let mut credential_manager = CredentialManager::load_all_credentials()?;
    
    println!("✅ Credential manager initialized successfully\n");
    
    // Show agent information
    println!("👤 Agent Information:");
    if let Some(name) = credential_manager.get_agent_info("AGENT_NAME") {
        println!("   Name: {}", name);
    }
    if let Some(email) = credential_manager.get_agent_info("AGENT_EMAIL") {
        println!("   Email: {}", email);
    }
    if let Some(resume) = credential_manager.get_agent_info("RESUME_PATH") {
        println!("   Resume: {}", resume);
    }
    if let Some(cover_letter) = credential_manager.get_agent_info("COVER_LETTER_PATH") {
        println!("   Cover Letter: {}", cover_letter);
    }
    println!();
    
    // Show available platforms
    let available_platforms = credential_manager.get_available_platforms();
    println!("📋 Available Platforms ({}):", available_platforms.len());
    for platform in &available_platforms {
        if let Some(creds) = credential_manager.get_platform_credentials(platform) {
            println!("   ✓ {}: {} ({})", 
                platform.to_uppercase(), 
                creds.email,
                if creds.access_token.is_some() { "Has Token" } else { "Password Only" }
            );
        }
    }
    
    if available_platforms.is_empty() {
        println!("   ⚠️  No valid credentials found");
        println!("   💡 Set environment variables or update AgentSecrets.template with your credentials");
    }
    println!();
    
    // Show all platforms (including those without credentials)
    println!("🌐 All Supported Platforms:");
    let all_platforms = vec!["linkedin", "indeed", "glassdoor", "monster", "ziprecruiter", "careerbuilder"];
    for platform in all_platforms {
        let has_creds = credential_manager.has_valid_credentials(platform);
        let status = if has_creds { "✅" } else { "❌" };
        println!("   {} {}", status, platform.to_uppercase());
    }
    println!();
    
    // Test authentication if we have credentials
    if !available_platforms.is_empty() {
        println!("🔐 Testing Authentication...");
        match authenticate_all_platforms(&mut credential_manager).await {
            Ok(authenticated) => {
                println!("✅ Authentication completed for {} platforms", authenticated.len());
                for platform in authenticated {
                    println!("   ✓ {}", platform.to_uppercase());
                }
            },
            Err(e) => {
                println!("⚠️  Authentication failed: {}", e);
            }
        }
    } else {
        println!("⏭️  Skipping authentication (no credentials available)");
    }
    
    println!("\n=== Test completed ===");
    Ok(())
}