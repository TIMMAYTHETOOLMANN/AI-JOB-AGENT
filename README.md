# Autonomous Job Agent

Automated, self-healing, adaptive job search and application agent.  
**Profile:** Timothy Roessel (Comprehensive)

## Features
- All-platform coverage, all industries, all job boards
- Advanced profile-driven matching (see AgentConfig.toml)
- Automated job search and application with personalized payloads
- **NEW: Search Engine Reconnaissance** - Directly scan search engines for jobs at major employers

### Search Engine Reconnaissance
The agent now includes a powerful new capability to scan search engines (Google, Bing, etc.) for job postings at specific companies. This allows you to:

- Discover jobs that might not be posted on traditional job boards
- Target specific companies of interest
- Find jobs using company-specific career sites
- Automatically filter and apply to matching positions

To configure this feature, use the following settings in your AgentConfig.toml:
- `search_engines`: List of search engine URLs to query
- `company_targets`: List of companies to target
- `search_locations`: List of locations to search in

### Enhanced Credential Management
The agent features a comprehensive credential management system that supports:

- **Environment variables** for secure credential storage
- **Multiple credential sources** (env vars, .env file, config files)
- **API token authentication** for supported platforms
- **Automatic credential detection** and validation

See [CREDENTIALS.md](CREDENTIALS.md) for detailed setup instructions.

## Quickstart

1. **Configure credentials** (see [CREDENTIALS.md](CREDENTIALS.md) for details):
   ```bash
   # Set environment variables (recommended)
   export LINKEDIN_EMAIL="your-email@example.com"
   export LINKEDIN_PASSWORD="your-password"
   
   # Or copy and edit the example file
   cp .env.example .env
   ```

2. **Test your credentials**:
   ```bash
   cargo run --bin test_credentials
   ```

3. **Configure AgentConfig.toml** (see below)

4. **Resume and cover letter files are configured**
   - The system is configured to use Timothy's resume and cover letter from:
     - Resume: `C:/OneDrive/Documents/Desktop/FIN RES.pdf`
     - Cover Letter: `C:/OneDrive/Documents/Desktop/FIN CV.pdf`
   - If you need to use different files, update the paths in AgentConfig.toml or set environment variables

5. Build and run:
   ```bash
   cargo build --release
   cargo run --release
   ```

## Sample AgentConfig.toml

```toml
name = "Timothy Roessel"
email = "timothyroessel@gmail.com"
location = ["Beaverton, OR", "Portland, OR", "Remote", "United States", "Any"]
open_to_relocation = true
job_keywords = [
    "Process Optimization", "Manufacturing Support", "Production Support",
    "Project Coordination", "Technical Documentation", "Process Engineer",
    "Operations Analyst", "Root Cause Analysis", "Continuous Improvement",
    "Automation", "AI Integration", "Process Automation", "Team Collaboration",
    "Technical Support", "Project Manager", "Quality Analyst", "Operations Manager"
]
skills = [
    "Process Innovation", "NX CAD", "SolidWorks", "Teamcenter", "MS Office Suite",
    "SharePoint", "Python", "Process Automation", "Technical Documentation"
]
industries = [
    "Manufacturing", "Engineering", "Operations", "Technical Support", "Automation",
    "Business Process", "Automotive", "Insurance", "Retail"
]
seniority_levels = ["Mid", "Senior", "Lead"]
min_salary = 70000
max_salary = 200000
targets = [
    "https://indeed.com", "https://linkedin.com/jobs", "https://glassdoor.com",
    "https://monster.com", "https://ziprecruiter.com", "https://careerbuilder.com",
    "https://simplyhired.com", "Company career pages", "All available job boards"
]
resume_path = "C:/OneDrive/Documents/Desktop/FIN RES.pdf"
cover_letter_path = "C:/OneDrive/Documents/Desktop/FIN CV.pdf"

# Search Engine Reconnaissance Configuration
search_engines = ["https://www.google.com/search?q=", "https://www.bing.com/search?q="]
company_targets = ["Intel", "Nike", "adidas", "Precision Castparts", "Daimler", "Large Companies List"]
search_locations = ["Beaverton, OR", "Portland, OR", "Remote"]
```
