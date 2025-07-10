pub mod config;
pub mod application_state;
pub mod job_search_engine;
pub mod job_application;
pub mod document_profile;
pub mod job_listing;
pub mod mock_browser;
pub mod secrets;
pub mod linkedin_login;
pub mod nike_career;
pub mod company_career;
pub mod profile_selector;
pub mod globalization;
pub mod adaptive_browser;
pub mod location_config_gui;

#[cfg(feature = "egui_gui")]
pub mod location_gui_app;
