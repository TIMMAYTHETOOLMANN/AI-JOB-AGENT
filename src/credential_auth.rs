use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};
use reqwest::Client;
use crate::secrets::{PlatformCredentials, CredentialManager};

/// Token response structure for OAuth and API authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
}

/// Credential authenticator that fetches API tokens
#[derive(Debug)]
pub struct CredentialAuthenticator {
    pub client: Client,
    pub platform_configs: HashMap<String, PlatformConfig>,
}

/// Platform-specific authentication configuration
#[derive(Debug, Clone)]
pub struct PlatformConfig {
    pub auth_url: String,
    pub token_url: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
    pub auth_method: AuthMethod,
}

/// Authentication methods supported
#[derive(Debug, Clone)]
pub enum AuthMethod {
    OAuth2,
    ApiKey,
    BasicAuth,
    SessionCookie,
    Custom(String),
}

impl CredentialAuthenticator {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("AI-Job-Agent/1.0")
            .cookie_store(true)
            .build()
            .expect("Failed to create HTTP client");

        let mut platform_configs = HashMap::new();
        
        // Initialize platform configurations
        Self::initialize_platform_configs(&mut platform_configs);
        
        Self {
            client,
            platform_configs,
        }
    }

    /// Initialize authentication configurations for supported platforms
    fn initialize_platform_configs(configs: &mut HashMap<String, PlatformConfig>) {
        // LinkedIn OAuth2 configuration
        configs.insert("linkedin".to_string(), PlatformConfig {
            auth_url: "https://www.linkedin.com/oauth/v2/authorization".to_string(),
            token_url: Some("https://www.linkedin.com/oauth/v2/accessToken".to_string()),
            client_id: None, // Will be loaded from environment
            client_secret: None,
            redirect_uri: Some("http://localhost:8080/callback".to_string()),
            scope: Some("r_liteprofile r_emailaddress w_member_social".to_string()),
            auth_method: AuthMethod::OAuth2,
        });

        // Indeed API configuration
        configs.insert("indeed".to_string(), PlatformConfig {
            auth_url: "https://secure.indeed.com/account/login".to_string(),
            token_url: Some("https://apis.indeed.com/ads/apisearch".to_string()),
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
            auth_method: AuthMethod::BasicAuth,
        });

        // Glassdoor API configuration
        configs.insert("glassdoor".to_string(), PlatformConfig {
            auth_url: "https://www.glassdoor.com/profile/login_input.htm".to_string(),
            token_url: Some("https://api.glassdoor.com/api/api.htm".to_string()),
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
            auth_method: AuthMethod::ApiKey,
        });

        // Monster configuration (basic auth)
        configs.insert("monster".to_string(), PlatformConfig {
            auth_url: "https://accounts.monster.com/login".to_string(),
            token_url: None,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
            auth_method: AuthMethod::BasicAuth,
        });

        // ZipRecruiter configuration
        configs.insert("ziprecruiter".to_string(), PlatformConfig {
            auth_url: "https://www.ziprecruiter.com/login".to_string(),
            token_url: None,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
            auth_method: AuthMethod::SessionCookie,
        });

        // CareerBuilder configuration
        configs.insert("careerbuilder".to_string(), PlatformConfig {
            auth_url: "https://www.careerbuilder.com/login".to_string(),
            token_url: None,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            scope: None,
            auth_method: AuthMethod::SessionCookie,
        });
    }

    /// Authenticate and fetch API tokens for a platform
    pub async fn authenticate_platform(&self, platform: &str, credentials: &PlatformCredentials) -> Result<TokenResponse, Error> {
        let platform_config = self.platform_configs.get(platform)
            .ok_or_else(|| anyhow::anyhow!("Platform {} not supported", platform))?;

        match platform_config.auth_method {
            AuthMethod::OAuth2 => self.oauth2_authentication(platform, credentials, platform_config).await,
            AuthMethod::ApiKey => self.api_key_authentication(platform, credentials, platform_config).await,
            AuthMethod::BasicAuth => self.basic_authentication(platform, credentials, platform_config).await,
            AuthMethod::SessionCookie => self.session_cookie_authentication(platform, credentials, platform_config).await,
            AuthMethod::Custom(ref method) => self.custom_authentication(platform, credentials, platform_config, method).await,
        }
    }

    /// OAuth2 authentication flow
    async fn oauth2_authentication(&self, platform: &str, credentials: &PlatformCredentials, config: &PlatformConfig) -> Result<TokenResponse, Error> {
        if let (Some(token_url), Some(client_id), Some(client_secret)) = (&config.token_url, &config.client_id, &config.client_secret) {
            let mut params = HashMap::new();
            params.insert("grant_type", "password");
            params.insert("username", &credentials.email);
            params.insert("password", &credentials.password);
            params.insert("client_id", client_id);
            params.insert("client_secret", client_secret);
            
            if let Some(scope) = &config.scope {
                params.insert("scope", scope);
            }

            let response = self.client
                .post(token_url)
                .form(&params)
                .send()
                .await?;

            if response.status().is_success() {
                let token_response: TokenResponse = response.json().await?;
                Ok(token_response)
            } else {
                Err(anyhow::anyhow!("OAuth2 authentication failed for {}: {}", platform, response.status()))
            }
        } else {
            Err(anyhow::anyhow!("OAuth2 configuration incomplete for {}", platform))
        }
    }

    /// API key authentication
    async fn api_key_authentication(&self, platform: &str, credentials: &PlatformCredentials, _config: &PlatformConfig) -> Result<TokenResponse, Error> {
        if let Some(api_key) = &credentials.api_key {
            // For API key auth, we return the key as the access token
            Ok(TokenResponse {
                access_token: api_key.clone(),
                refresh_token: None,
                expires_in: None,
                token_type: Some("ApiKey".to_string()),
                scope: None,
            })
        } else {
            Err(anyhow::anyhow!("API key not found for platform {}", platform))
        }
    }

    /// Basic authentication (username/password)
    async fn basic_authentication(&self, platform: &str, credentials: &PlatformCredentials, config: &PlatformConfig) -> Result<TokenResponse, Error> {
        // For basic auth, we simulate a login to get session tokens
        let response = self.client
            .post(&config.auth_url)
            .form(&[
                ("username", &credentials.email),
                ("password", &credentials.password),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            // Extract session tokens from cookies or response headers
            let session_token = response.headers()
                .get("set-cookie")
                .and_then(|cookie| cookie.to_str().ok())
                .unwrap_or("session_token_placeholder")
                .to_string();

            Ok(TokenResponse {
                access_token: session_token,
                refresh_token: None,
                expires_in: Some(3600), // 1 hour default
                token_type: Some("Session".to_string()),
                scope: None,
            })
        } else {
            Err(anyhow::anyhow!("Basic authentication failed for {}: {}", platform, response.status()))
        }
    }

    /// Session cookie authentication
    async fn session_cookie_authentication(&self, platform: &str, credentials: &PlatformCredentials, config: &PlatformConfig) -> Result<TokenResponse, Error> {
        if let Some(session_cookie) = &credentials.session_cookie {
            // Use existing session cookie
            Ok(TokenResponse {
                access_token: session_cookie.clone(),
                refresh_token: None,
                expires_in: None,
                token_type: Some("SessionCookie".to_string()),
                scope: None,
            })
        } else {
            // Perform login to get session cookie
            self.basic_authentication(platform, credentials, config).await
        }
    }

    /// Custom authentication method
    async fn custom_authentication(&self, platform: &str, credentials: &PlatformCredentials, config: &PlatformConfig, method: &str) -> Result<TokenResponse, Error> {
        match method {
            "linkedin_scraping" => self.linkedin_scraping_auth(credentials, config).await,
            _ => Err(anyhow::anyhow!("Custom authentication method '{}' not implemented for {}", method, platform))
        }
    }

    /// LinkedIn scraping authentication (fallback method)
    async fn linkedin_scraping_auth(&self, credentials: &PlatformCredentials, config: &PlatformConfig) -> Result<TokenResponse, Error> {
        // This would implement browser-based login simulation
        // For now, return a placeholder response
        Ok(TokenResponse {
            access_token: format!("linkedin_session_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
            refresh_token: None,
            expires_in: Some(7200), // 2 hours
            token_type: Some("ScrapingSession".to_string()),
            scope: None,
        })
    }

    /// Refresh an expired token
    pub async fn refresh_token(&self, platform: &str, refresh_token: &str) -> Result<TokenResponse, Error> {
        let platform_config = self.platform_configs.get(platform)
            .ok_or_else(|| anyhow::anyhow!("Platform {} not supported", platform))?;

        if let Some(token_url) = &platform_config.token_url {
            let mut params = HashMap::new();
            params.insert("grant_type", "refresh_token");
            params.insert("refresh_token", refresh_token);

            let response = self.client
                .post(token_url)
                .form(&params)
                .send()
                .await?;

            if response.status().is_success() {
                let token_response: TokenResponse = response.json().await?;
                Ok(token_response)
            } else {
                Err(anyhow::anyhow!("Token refresh failed for {}: {}", platform, response.status()))
            }
        } else {
            Err(anyhow::anyhow!("Token refresh not supported for platform {}", platform))
        }
    }

    /// Update credential manager with fetched tokens
    pub fn update_credentials(&self, manager: &mut CredentialManager, platform: &str, token_response: &TokenResponse) {
        if let Some(creds) = manager.platforms.get_mut(&platform.to_lowercase()) {
            creds.access_token = Some(token_response.access_token.clone());
            creds.refresh_token = token_response.refresh_token.clone();
        }
    }

    /// Validate if tokens are still valid
    pub async fn validate_token(&self, platform: &str, token: &str) -> bool {
        // Implementation would depend on platform-specific validation endpoints
        // For now, return true as a placeholder
        true
    }
}

/// Convenience function to authenticate all available platforms
pub async fn authenticate_all_platforms(manager: &mut CredentialManager) -> Result<Vec<String>, Error> {
    let authenticator = CredentialAuthenticator::new();
    let mut authenticated_platforms = Vec::new();

    for (platform, credentials) in &manager.platforms.clone() {
        if !credentials.email.is_empty() && !credentials.password.is_empty() {
            match authenticator.authenticate_platform(platform, credentials).await {
                Ok(token_response) => {
                    authenticator.update_credentials(manager, platform, &token_response);
                    authenticated_platforms.push(platform.clone());
                    println!("✅ Successfully authenticated with {}", platform);
                }
                Err(e) => {
                    eprintln!("⚠️  Failed to authenticate with {}: {}", platform, e);
                }
            }
        }
    }

    Ok(authenticated_platforms)
}