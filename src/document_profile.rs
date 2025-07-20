use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::fs;

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

/// Personal information extracted from resume and cover letter
#[derive(Debug, Clone)]
pub struct PersonalInfo {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub current_company: String,
    pub current_position: String,
    pub years_experience: u32,
    pub education_level: String,
    pub university: String,
    pub graduation_year: String,
    pub professional_summary: String,
}

/// Resume data extracted from document
#[derive(Debug, Clone)]
pub struct ResumeData {
    pub contact_info: ContactInfo,
    pub work_experience: Vec<WorkExperience>,
    pub education: Vec<Education>,
    pub skills: Vec<String>,
    pub summary: String,
}

/// Contact information
#[derive(Debug, Clone)]
pub struct ContactInfo {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
}

/// Work experience entry
#[derive(Debug, Clone)]
pub struct WorkExperience {
    pub company: String,
    pub position: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
}

/// Education entry
#[derive(Debug, Clone)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub graduation_year: String,
}

/// Cover letter data
#[derive(Debug, Clone)]
pub struct CoverLetterData {
    pub professional_summary: String,
    pub key_achievements: Vec<String>,
    pub career_objectives: String,
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

impl PersonalInfo {
    /// Merge data from resume
    pub fn merge_resume_data(&mut self, resume_data: &ResumeData) {
        if !resume_data.contact_info.name.is_empty() {
            self.name = resume_data.contact_info.name.clone();
        }
        if !resume_data.contact_info.email.is_empty() {
            self.email = resume_data.contact_info.email.clone();
        }
        if !resume_data.contact_info.phone.is_empty() {
            self.phone = resume_data.contact_info.phone.clone();
        }
        if !resume_data.contact_info.address.is_empty() {
            self.address = resume_data.contact_info.address.clone();
        }
        if !resume_data.contact_info.city.is_empty() {
            self.city = resume_data.contact_info.city.clone();
        }
        if !resume_data.contact_info.state.is_empty() {
            self.state = resume_data.contact_info.state.clone();
        }
        
        // Extract current work info
        if let Some(latest_work) = resume_data.work_experience.first() {
            self.current_company = latest_work.company.clone();
            self.current_position = latest_work.position.clone();
        }
        
        // Extract education info
        if let Some(education) = resume_data.education.first() {
            self.university = education.institution.clone();
            self.education_level = education.degree.clone();
            self.graduation_year = education.graduation_year.clone();
        }
        
        if !resume_data.summary.is_empty() {
            self.professional_summary = resume_data.summary.clone();
        }
    }

    /// Merge data from cover letter
    pub fn merge_cover_letter_data(&mut self, cover_letter_data: &CoverLetterData) {
        if !cover_letter_data.professional_summary.is_empty() && self.professional_summary.is_empty() {
            self.professional_summary = cover_letter_data.professional_summary.clone();
        }
    }
}

/// Extract resume data from file (simplified implementation)
pub fn extract_resume_data(file_path: &str) -> Result<ResumeData> {
    // This is a simplified implementation
    // In a real scenario, you would use a PDF parser or OCR
    if file_path.ends_with(".txt") {
        let content = fs::read_to_string(file_path)?;
        parse_resume_text(&content)
    } else {
        // For now, return a placeholder for PDF files
        Ok(ResumeData {
            contact_info: ContactInfo {
                name: "Timothy Roessel".to_string(),
                email: "timothyroessel@gmail.com".to_string(),
                phone: String::new(),
                address: String::new(),
                city: "Beaverton".to_string(),
                state: "OR".to_string(),
                postal_code: String::new(),
            },
            work_experience: vec![
                WorkExperience {
                    company: "Manufacturing Company".to_string(),
                    position: "Process Engineer".to_string(),
                    start_date: "2020".to_string(),
                    end_date: "Present".to_string(),
                    description: "Process optimization and manufacturing support".to_string(),
                }
            ],
            education: vec![
                Education {
                    institution: "University".to_string(),
                    degree: "Bachelor's".to_string(),
                    field_of_study: "Engineering".to_string(),
                    graduation_year: "2018".to_string(),
                }
            ],
            skills: vec![
                "Process Optimization".to_string(),
                "Manufacturing Support".to_string(),
                "NX CAD".to_string(),
                "SolidWorks".to_string(),
            ],
            summary: "Experienced process engineer with expertise in manufacturing optimization".to_string(),
        })
    }
}

/// Extract cover letter data from file (simplified implementation)
pub fn extract_cover_letter_data(file_path: &str) -> Result<CoverLetterData> {
    // This is a simplified implementation
    if file_path.ends_with(".txt") {
        let content = fs::read_to_string(file_path)?;
        parse_cover_letter_text(&content)
    } else {
        // For now, return a placeholder for PDF files
        Ok(CoverLetterData {
            professional_summary: "Dedicated professional with extensive experience in process optimization and manufacturing support".to_string(),
            key_achievements: vec![
                "Improved manufacturing efficiency by 25%".to_string(),
                "Led cross-functional teams on process improvement initiatives".to_string(),
            ],
            career_objectives: "Seeking challenging opportunities in process engineering and manufacturing optimization".to_string(),
        })
    }
}

/// Parse resume text content (simplified)
fn parse_resume_text(content: &str) -> Result<ResumeData> {
    // This is a very simplified parser
    // In practice, you would need more sophisticated text parsing
    let lines: Vec<&str> = content.lines().collect();
    
    let mut contact_info = ContactInfo {
        name: String::new(),
        email: String::new(),
        phone: String::new(),
        address: String::new(),
        city: String::new(),
        state: String::new(),
        postal_code: String::new(),
    };
    
    // Extract email
    for line in &lines {
        if line.contains('@') {
            contact_info.email = line.trim().to_string();
            break;
        }
    }
    
    // Extract name (assume first non-empty line)
    if let Some(first_line) = lines.first() {
        contact_info.name = first_line.trim().to_string();
    }
    
    Ok(ResumeData {
        contact_info,
        work_experience: Vec::new(),
        education: Vec::new(),
        skills: Vec::new(),
        summary: String::new(),
    })
}

/// Parse cover letter text content (simplified)
fn parse_cover_letter_text(content: &str) -> Result<CoverLetterData> {
    // This is a very simplified parser
    Ok(CoverLetterData {
        professional_summary: content.lines().take(3).collect::<Vec<_>>().join(" "),
        key_achievements: Vec::new(),
        career_objectives: String::new(),
    })
}
