use std::collections::HashMap;
use std::fs;
use std::env;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};

/// Platform credentials structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCredentials {
    pub email: String,
    pub password: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub session_cookie: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl Default for PlatformCredentials {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            api_key: None,
            api_secret: None,
            session_cookie: None,
            access_token: None,
            refresh_token: None,
        }
    }
}

/// Credential manager that handles secure storage and retrieval
#[derive(Debug, Clone)]
pub struct CredentialManager {
    pub platforms: HashMap<String, PlatformCredentials>,
    pub agent_info: HashMap<String, String>,
}

impl CredentialManager {
    pub fn new() -> Self {
        Self {
            platforms: HashMap::new(),
            agent_info: HashMap::new(),
        }
    }

    /// Load credentials from multiple sources with priority:
    /// 1. Environment variables (highest priority)
    /// 2. .env file
    /// 3. AgentSecrets.toml
    /// 4. AgentSecrets.template (fallback)
    pub fn load_all_credentials() -> Result<Self, Error> {
        let mut manager = CredentialManager::new();
        
        // Load from files first (lowest priority)
        for file_path in &["AgentSecrets.template", "AgentSecrets.toml", ".env"] {
            if std::path::Path::new(file_path).exists() {
                let file_secrets = load_secrets(file_path);
                manager.merge_secrets(file_secrets);
            }
        }
        
        // Load from environment variables (highest priority)
        let env_secrets = load_environment_variables();
        manager.merge_secrets(env_secrets);
        
        // Initialize platform credentials
        manager.initialize_platform_credentials();
        
        Ok(manager)
    }

    /// Merge secrets into the credential manager
    fn merge_secrets(&mut self, secrets: HashMap<String, String>) {
        for (key, value) in secrets {
            if value.is_empty() {
                continue;
            }
            
            // Agent info
            if matches!(key.as_str(), "AGENT_NAME" | "AGENT_EMAIL" | "RESUME_PATH" | "COVER_LETTER_PATH") {
                self.agent_info.insert(key, value);
                continue;
            }
            
            // Platform credentials
            if let Some(platform) = extract_platform_from_key(&key) {
                let creds = self.platforms.entry(platform.to_lowercase()).or_insert_with(PlatformCredentials::default);
                
                match key.split('_').last().unwrap_or("").to_lowercase().as_str() {
                    "email" => creds.email = value,
                    "password" => creds.password = value,
                    "key" | "api_key" => creds.api_key = Some(value),
                    "secret" | "api_secret" => creds.api_secret = Some(value),
                    "cookie" | "session_cookie" => creds.session_cookie = Some(value),
                    "token" | "access_token" => creds.access_token = Some(value),
                    "refresh_token" => creds.refresh_token = Some(value),
                    _ => {}
                }
            }
        }
    }

    /// Initialize platform credentials for known platforms
    fn initialize_platform_credentials(&mut self) {
        let platforms = vec!["linkedin", "indeed", "glassdoor", "monster", "ziprecruiter", "careerbuilder"];
        for platform in platforms {
            self.platforms.entry(platform.to_string()).or_insert_with(PlatformCredentials::default);
        }
    }

    /// Get credentials for a specific platform
    pub fn get_platform_credentials(&self, platform: &str) -> Option<&PlatformCredentials> {
        self.platforms.get(&platform.to_lowercase())
    }

    /// Get agent information
    pub fn get_agent_info(&self, key: &str) -> Option<&String> {
        self.agent_info.get(key)
    }

    /// Check if platform has valid credentials
    pub fn has_valid_credentials(&self, platform: &str) -> bool {
        if let Some(creds) = self.get_platform_credentials(platform) {
            !creds.email.is_empty() && (!creds.password.is_empty() || creds.access_token.is_some())
        } else {
            false
        }
    }

    /// Get all platforms with valid credentials
    pub fn get_available_platforms(&self) -> Vec<String> {
        self.platforms.keys()
            .filter(|platform| self.has_valid_credentials(platform))
            .cloned()
            .collect()
    }

    /// Prompt user for minimal credentials if not available
    pub fn prompt_for_credentials(&mut self, platform: &str) -> Result<(), Error> {
        if self.has_valid_credentials(platform) {
            return Ok(());
        }

        println!("Credentials needed for {}", platform);
        
        let mut email = String::new();
        let mut password = String::new();
        
        print!("Email for {}: ", platform);
        std::io::stdin().read_line(&mut email)?;
        email = email.trim().to_string();
        
        print!("Password for {}: ", platform);
        // In a real implementation, you'd want to use a secure password input
        std::io::stdin().read_line(&mut password)?;
        password = password.trim().to_string();
        
        let creds = PlatformCredentials {
            email,
            password,
            ..Default::default()
        };
        
        self.platforms.insert(platform.to_lowercase(), creds);
        Ok(())
    }
}

/// Load secrets from a file (existing function enhanced)
pub fn load_secrets(path: &str) -> HashMap<String, String> {
    let mut secrets = HashMap::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').to_string();
                secrets.insert(key, value);
            }
        }
    }
    secrets
}

/// Load credentials from environment variables
pub fn load_environment_variables() -> HashMap<String, String> {
    let mut secrets = HashMap::new();
    
    // Agent info environment variables
    let agent_keys = vec!["AGENT_NAME", "AGENT_EMAIL", "RESUME_PATH", "COVER_LETTER_PATH"];
    
    // Platform environment variable patterns
    let platforms = vec!["LINKEDIN", "INDEED", "GLASSDOOR", "MONSTER", "ZIPRECRUITER", "CAREERBUILDER"];
    let suffixes = vec!["EMAIL", "PASSWORD", "API_KEY", "API_SECRET", "SESSION_COOKIE", "ACCESS_TOKEN", "REFRESH_TOKEN"];
    
    // Load agent info
    for key in agent_keys {
        if let Ok(value) = env::var(key) {
            secrets.insert(key.to_string(), value);
        }
    }
    
    // Load platform credentials
    for platform in platforms {
        for suffix in &suffixes {
            let key = format!("{}_{}", platform, suffix);
            if let Ok(value) = env::var(&key) {
                secrets.insert(key, value);
            }
        }
    }
    
    secrets
}

/// Extract platform name from environment variable key
fn extract_platform_from_key(key: &str) -> Option<&str> {
    let platforms = vec!["LINKEDIN", "INDEED", "GLASSDOOR", "MONSTER", "ZIPRECRUITER", "CAREERBUILDER"];
    
    for platform in platforms {
        if key.starts_with(platform) {
            return Some(platform);
        }
    }
    None
}

