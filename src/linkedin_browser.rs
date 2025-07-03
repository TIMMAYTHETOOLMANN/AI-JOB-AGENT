use thirtyfour::prelude::*;
use anyhow::{Result, anyhow};
use std::time::Duration;

pub async fn linkedin_browser_login(email: &str, password: &str) -> Result<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://www.linkedin.com/login").await?;
    driver.find(By::Id("username")).await?.send_keys(email).await?;
    driver.find(By::Id("password")).await?.send_keys(password).await?;
    driver.find(By::XPath("//button[@type='submit']")).await?.click().await?;
    driver.set_implicit_wait_timeout(Duration::from_secs(10)).await?;
    // Check for successful login by looking for the profile icon or home page
    let logged_in = driver.find(By::Css("img.global-nav__me-photo")).await.is_ok();
    if !logged_in {
        return Err(anyhow!("LinkedIn login failed. Check credentials or 2FA."));
    }
    Ok(driver)
}

