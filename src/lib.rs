pub mod config;
pub mod application_state;
pub mod job_search_engine;
pub mod job_application;
pub mod document_profile;
pub mod job_listing;
pub mod secrets;
pub mod linkedin_login;
pub mod nike_career;
pub mod company_career;
pub mod profile_selector;
pub mod globalization;
pub mod adaptive_browser;
pub mod job_filter;
pub mod reporting;
pub mod account_creator;

// GUI module only available with GUI features
#[cfg(feature = "production-gui")]
pub mod main_gui;

// PRODUCTION BROWSER - SINGLE VISIBLE WINDOW ONLY
pub mod visible_browser;

// WEB SERVER FOR API ACCESS (Optional)
#[cfg(feature = "web-server")]
pub mod web_server;

// ADVANCED OPTIMIZATION MODULES
pub mod firewall_bypass;
pub mod credential_validator;
pub mod mock_data_purger;

// REMOVE ALL MOCK/HEADLESS DEPENDENCIES
// Production builds ONLY use visible_browser with real automation
