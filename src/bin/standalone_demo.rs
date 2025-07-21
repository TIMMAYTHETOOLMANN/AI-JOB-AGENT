/// Standalone demo for optimization features - no GUI dependencies
use std::collections::{HashMap, HashSet};
use anyhow::{Result, anyhow};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 AI JOB AGENT - OPTIMIZATION FEATURES DEMO");
    println!("═══════════════════════════════════════════════");
    
    // Test 1: Mock URL Detection
    println!("\n🌐 TESTING MOCK URL DETECTION");
    test_mock_url_detection();
    
    // Test 2: Credential Validation  
    println!("\n👤 TESTING CREDENTIAL VALIDATION");
    test_credential_validation();
    
    // Test 3: Firewall Pattern Detection
    println!("\n🛡️ TESTING FIREWALL PATTERN DETECTION");
    test_firewall_detection();
    
    // Test 4: Mock Data Patterns
    println!("\n🧹 TESTING MOCK DATA PATTERNS");
    test_mock_data_patterns();
    
    // Test 5: Environment Variable Scanning
    println!("\n🔍 TESTING ENVIRONMENT SCANNING");
    test_environment_scanning();
    
    println!("\n✅ OPTIMIZATION DEMO COMPLETED SUCCESSFULLY");
    println!("═══════════════════════════════════════════════");
    
    Ok(())
}

fn test_mock_url_detection() {
    let mock_domains = get_mock_domains();
    let test_urls = vec![
        ("https://linkedin.com/jobs/123", false),
        ("https://indeed.com/job/456", false),
        ("https://test.com/fake-job", true),
        ("http://localhost:3000/mock-job", true),
        ("https://example.com/sample-job", true),
        ("https://fake-jobs.com/position", true),
        ("https://glassdoor.com/reviews", false),
    ];
    
    for (url, should_be_mock) in test_urls {
        let is_mock = is_mock_url(url, &mock_domains);
        if is_mock == should_be_mock {
            if is_mock {
                println!("✅ Mock URL correctly blocked: {}", url);
            } else {
                println!("✅ Real URL correctly allowed: {}", url);
            }
        } else {
            println!("❌ URL detection failed: {}", url);
        }
    }
}

fn test_credential_validation() {
    let test_credentials = vec![
        ("real.user@gmail.com", "John Smith", false),
        ("john.doe@example.com", "John Doe", true),
        ("test@test.com", "Test User", true),
        ("actual.person@company.com", "Jane Wilson", false),
        ("admin@localhost", "Admin User", true),
    ];
    
    for (email, name, should_be_fake) in test_credentials {
        let is_fake = is_fake_credential(email, name);
        if is_fake == should_be_fake {
            if is_fake {
                println!("✅ Fake credentials correctly blocked: {} ({})", email, name);
            } else {
                println!("✅ Real credentials correctly allowed: {} ({})", email, name);
            }
        } else {
            println!("❌ Credential validation failed: {} ({})", email, name);
        }
    }
}

fn test_firewall_detection() {
    let test_cases = vec![
        ("Attention Required! Cloudflare", "Cloudflare", true),
        ("DDoS protection by Cloudflare", "Cloudflare", true),
        ("Access Denied - Web Application Firewall", "WAF", true),
        ("Rate Limited - Too Many Requests", "Rate Limit", true),
        ("Welcome to our job portal", "None", false),
        ("Please complete the CAPTCHA", "Challenge", true),
    ];
    
    for (content, expected_type, should_detect) in test_cases {
        let detected = detect_firewall(content);
        if detected == should_detect {
            if detected {
                println!("✅ Firewall correctly detected: {} ({})", expected_type, content);
            } else {
                println!("✅ No firewall correctly identified: {}", content);
            }
        } else {
            println!("❌ Firewall detection failed: {}", content);
        }
    }
}

fn test_mock_data_patterns() {
    let test_results = vec![
        ("Successfully applied to Software Engineer position", false),
        ("Application submitted successfully", false),
        ("Successfully simulated job application", true),
        ("Mock success - pretend application completed", true),
        ("Test application successful", true),
        ("Demo completed successfully", true),
    ];
    
    for (result, should_be_mock) in test_results {
        let is_mock = is_mock_result(result);
        if is_mock == should_be_mock {
            if is_mock {
                println!("✅ Mock result correctly blocked: {}", result);
            } else {
                println!("✅ Real result correctly allowed: {}", result);
            }
        } else {
            println!("❌ Result validation failed: {}", result);
        }
    }
}

fn test_environment_scanning() {
    // Check for some common environment variables
    let critical_vars = vec!["AGENT_NAME", "AGENT_EMAIL", "RESUME_PATH", "COVER_LETTER_PATH"];
    let optional_vars = vec!["LINKEDIN_EMAIL", "INDEED_EMAIL", "GLASSDOOR_EMAIL"];
    
    println!("Critical environment variables:");
    for var in critical_vars {
        match env::var(var) {
            Ok(value) => {
                let preview = create_safe_preview(&value);
                println!("  ✅ {}: {}", var, preview);
            }
            Err(_) => {
                println!("  ❌ {}: Not set", var);
            }
        }
    }
    
    println!("Optional environment variables:");
    for var in optional_vars {
        match env::var(var) {
            Ok(value) => {
                let preview = create_safe_preview(&value);
                println!("  ✅ {}: {}", var, preview);
            }
            Err(_) => {
                println!("  ⚠️  {}: Not set", var);
            }
        }
    }
}

// Helper functions for the demo

fn get_mock_domains() -> HashSet<String> {
    let mut domains = HashSet::new();
    domains.insert("localhost".to_string());
    domains.insert("test.com".to_string());
    domains.insert("example.com".to_string());
    domains.insert("fake-jobs.com".to_string());
    domains.insert("mock-careers.com".to_string());
    domains.insert("demo-jobs.org".to_string());
    domains
}

fn is_mock_url(url: &str, mock_domains: &HashSet<String>) -> bool {
    let url_lower = url.to_lowercase();
    
    // Check for localhost
    if url_lower.contains("localhost") || url_lower.contains("127.0.0.1") {
        return true;
    }
    
    // Check for mock keywords
    let mock_keywords = ["mock", "test", "fake", "demo", "sample"];
    for keyword in mock_keywords {
        if url_lower.contains(keyword) {
            return true;
        }
    }
    
    // Check against mock domains
    for domain in mock_domains {
        if url_lower.contains(&domain.to_lowercase()) {
            return true;
        }
    }
    
    false
}

fn is_fake_credential(email: &str, name: &str) -> bool {
    let fake_patterns = vec![
        "john.doe@example.com",
        "test@test.com",
        "admin@localhost",
        "user@domain.com",
    ];
    
    let email_lower = email.to_lowercase();
    let name_lower = name.to_lowercase();
    
    // Check for exact fake patterns
    for pattern in fake_patterns {
        if email_lower == pattern {
            return true;
        }
    }
    
    // Check for obvious test patterns
    if email_lower.contains("test") || email_lower.contains("example") || 
       name_lower.contains("test") || name_lower.contains("doe") {
        return true;
    }
    
    false
}

fn detect_firewall(content: &str) -> bool {
    let firewall_patterns = vec![
        "cloudflare",
        "attention required",
        "access denied",
        "web application firewall",
        "rate limited",
        "too many requests",
        "captcha",
        "ddos protection",
    ];
    
    let content_lower = content.to_lowercase();
    
    for pattern in firewall_patterns {
        if content_lower.contains(pattern) {
            return true;
        }
    }
    
    false
}

fn is_mock_result(result: &str) -> bool {
    let mock_indicators = vec![
        "simulated",
        "mock",
        "test",
        "demo",
        "pretend",
        "fake",
        "placeholder",
    ];
    
    let result_lower = result.to_lowercase();
    
    for indicator in mock_indicators {
        if result_lower.contains(indicator) {
            return true;
        }
    }
    
    false
}

fn create_safe_preview(value: &str) -> String {
    if value.len() <= 4 {
        "****".to_string()
    } else if value.contains('@') {
        // Email - show first few chars and domain
        if let Some(at_pos) = value.find('@') {
            let (local, domain) = value.split_at(at_pos);
            format!("{}***{}", &local[..std::cmp::min(2, local.len())], domain)
        } else {
            format!("{}***", &value[..2])
        }
    } else {
        // Other values - show first 2 and last 2 chars
        format!("{}***{}", &value[..2], &value[value.len()-2..])
    }
}