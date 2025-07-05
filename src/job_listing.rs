use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobListing {
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub url: String,
    pub salary: Option<String>,
    pub date_posted: Option<chrono::NaiveDateTime>,
}

