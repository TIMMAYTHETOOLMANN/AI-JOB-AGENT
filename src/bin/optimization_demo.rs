use anyhow::Result;
use autonomous_job_agent::{
    firewall_bypass::FirewallBypass,
    credential_validator::CredentialValidator,
    mock_data_purger::MockDataPurger,
};

/// Demo script to test the new optimization features
#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 AI JOB AGENT - OPTIMIZATION DEMO");
    println!("═══════════════════════════════════════");
    
    // Test 1: Credential Validation
    println!("\n🔐 TESTING CREDENTIAL VALIDATION");
    let mut validator = CredentialValidator::new();
    match validator.validate_instantly().await {
        Ok(report) => {
            println!("✅ Credential validation completed");
            println!("   Completion: {:.1}%", report.completion_percentage);
        }
        Err(e) => {
            println!("⚠️  Credential validation failed: {}", e);
        }
    }
    
    // Test 2: Mock Data Purging
    println!("\n🧹 TESTING MOCK DATA PURGING");
    let mut purger = MockDataPurger::new();
    match purger.purge_all_mock_data().await {
        Ok(report) => {
            println!("✅ Mock data purge completed");
            println!("   Items scanned: {}", report.total_scanned);
            println!("   Mock items found: {}", report.mock_items_found);
        }
        Err(e) => {
            println!("⚠️  Mock data purge failed: {}", e);
        }
    }
    
    // Test 3: URL Validation
    println!("\n🌐 TESTING URL VALIDATION");
    let test_urls = vec![
        "https://linkedin.com/jobs/123",
        "https://indeed.com/job/456",
        "https://test.com/fake-job",
        "http://localhost:3000/mock-job",
        "https://example.com/sample-job",
    ];
    
    for url in test_urls {
        match purger.validate_real_url(url) {
            Ok(_) => println!("✅ Valid URL: {}", url),
            Err(_) => println!("❌ Mock URL blocked: {}", url),
        }
    }
    
    // Test 4: Firewall Detection
    println!("\n🛡️ TESTING FIREWALL DETECTION");
    let mut firewall = FirewallBypass::new();
    
    let test_content = "Attention Required! Cloudflare Ray ID: 12345";
    match firewall.detect_firewall_block(test_content, "https://test.com", "Attention Required").await {
        Ok(detection) => {
            if detection.is_blocked {
                println!("🚨 Firewall detected: {:?} (confidence: {:.1}%)", detection.firewall_type, detection.confidence);
            } else {
                println!("✅ No firewall detected");
            }
        }
        Err(e) => {
            println!("⚠️  Firewall detection error: {}", e);
        }
    }
    
    // Test 5: Credential Validation
    println!("\n👤 TESTING CREDENTIAL VALIDATION");
    let test_credentials = vec![
        ("real.user@gmail.com", "John Smith"),
        ("john.doe@example.com", "John Doe"),
        ("test@test.com", "Test User"),
    ];
    
    for (email, name) in test_credentials {
        match purger.validate_real_credentials(email, name) {
            Ok(_) => println!("✅ Valid credentials: {} ({})", email, name),
            Err(_) => println!("❌ Fake credentials blocked: {} ({})", email, name),
        }
    }
    
    println!("\n✅ OPTIMIZATION DEMO COMPLETED");
    println!("═══════════════════════════════════════");
    
    Ok(())
}