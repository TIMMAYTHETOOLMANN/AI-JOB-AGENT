use std::time::Duration;
use tracing_subscriber::EnvFilter;

use autonomous_job_agent::visible_browser::VisibleBrowser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    println!("🚀 Testing Visible Browser with Real-Time Monitoring");
    println!("=================================================");

    // Initialize the visible browser
    let browser = VisibleBrowser::new()?;

    println!("\n🌐 Testing Navigation and Job Application Flow");
    
    // Test LinkedIn job application flow
    println!("\n--- LinkedIn Job Application Test ---");
    browser.navigate_to("https://linkedin.com/jobs/12345")?;
    
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Find and click apply button
    match browser.click_element("button[aria-label*='Easy Apply']") {
        Ok(()) => println!("✅ Successfully clicked LinkedIn Easy Apply button"),
        Err(e) => println!("⚠️ LinkedIn apply button click failed: {}", e),
    }
    
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Fill out application form
    browser.type_text("input[type='email']", "test@example.com")?;
    browser.type_text("input[name='name']", "Test Agent")?;
    
    // Test Nike career site
    println!("\n--- Nike Career Application Test ---");
    browser.navigate_to("https://nike.com/careers/job/12345")?;
    
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    match browser.click_element(".apply-btn") {
        Ok(()) => println!("✅ Successfully clicked Nike Apply button"),
        Err(e) => println!("⚠️ Nike apply button click failed: {}", e),
    }
    
    // Test Intel career site
    println!("\n--- Intel Career Application Test ---");
    browser.navigate_to("https://intel.com/careers/job/12345")?;
    
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    match browser.click_element(".apply-button") {
        Ok(()) => println!("✅ Successfully clicked Intel Apply button"),
        Err(e) => println!("⚠️ Intel apply button click failed: {}", e),
    }
    
    // Test Indeed job application
    println!("\n--- Indeed Job Application Test ---");
    browser.navigate_to("https://indeed.com/job/12345")?;
    
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    match browser.click_element(".indeed-apply-btn") {
        Ok(()) => println!("✅ Successfully clicked Indeed Apply button"),
        Err(e) => println!("⚠️ Indeed apply button click failed: {}", e),
    }
    
    // Test English teacher job in Japan
    println!("\n--- English Teacher Japan Application Test ---");
    browser.navigate_to("https://indeed.com/job/english-teacher-tokyo")?;
    
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Fill out Japan-specific application
    browser.type_text("input[name='email']", "teacher@example.com")?;
    browser.type_text("input[name='name']", "English Teacher")?;
    browser.type_text("input[name='visa-status']", "Work Visa Available")?;
    browser.type_text("input[name='japanese-level']", "Conversational")?;
    
    println!("\n📸 Taking screenshot of current page");
    browser.screenshot("test_browser_screenshot.txt")?;
    
    println!("\n✅ Visible Browser Test Completed Successfully!");
    println!("🔍 All browser actions were monitored and logged in real-time");
    println!("🛠️ Interactive control was available throughout the test");
    
    Ok(())
}