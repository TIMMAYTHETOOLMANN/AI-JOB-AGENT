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
pub mod main_gui;
pub mod job_filter;
pub mod reporting;
pub mod visible_browser;
pub mod web_server;
pub mod account_creator;

// Production browser implementation - always available
pub mod headless_browser;

// Optional mock browser for testing
#[cfg(feature = "mock-browser")]
pub mod mock_browser;
