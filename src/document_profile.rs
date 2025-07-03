use serde::{Deserialize, Serialize};

/// Represents a document profile (resume or cover letter)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentProfile {
    /// Name of the profile
    pub name: String,
    /// Path to the document file
    pub path: String,
    /// Keywords associated with this profile
    pub keywords: Vec<String>,
    /// Industries associated with this profile
    #[serde(default)]
    pub industries: Vec<String>,
    /// Seniority levels associated with this profile
    #[serde(default)]
    pub seniority: Vec<String>,
}

impl DocumentProfile {
    /// Creates a new DocumentProfile
    pub fn new(name: &str, path: &str, keywords: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            keywords,
            industries: Vec::new(),
            seniority: Vec::new(),
        }
    }

    /// Creates a new DocumentProfile with industries and seniority
    pub fn new_with_details(
        name: &str, 
        path: &str, 
        keywords: Vec<String>,
        industries: Vec<String>,
        seniority: Vec<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            keywords,
            industries,
            seniority,
        }
    }
}
