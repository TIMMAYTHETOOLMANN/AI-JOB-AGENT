# Autonomous Job Agent

Automated, self-healing, adaptive job search and application agent with comprehensive global coverage.  
**Profile:** Timothy Roessel (Comprehensive)

## Features
- **All-platform coverage** - 50+ international job boards and major company career pages
- **Global job site support** - LinkedIn, Indeed, Glassdoor, Monster, ZipRecruiter, CareerBuilder and many more
- **International expansion** - European sites (StepStone, Xing), UK (Reed, Totaljobs), Canadian, Australian, Asian job boards
- **Tech-specific platforms** - Stack Overflow Jobs, AngelList, Dice, GitHub Jobs
- **Major company career pages** - Intel, Nike, Apple, Microsoft, Google, Amazon, Meta, Tesla, and more
- **Automated account creation** - Creates accounts using resume/cover letter data automatically
- **Advanced profile-driven matching** (see AgentConfig.toml)
- **CLI and GUI modes** - Run headless or with interactive GUI
- **Browser automation** - Uses Chromium/Playwright for realistic job applications
- **Search Engine Reconnaissance** - Directly scan search engines for jobs at major employers
- **Adaptive automation** - Self-healing browser automation with performance monitoring

## Quick Start

### 1. Configure Your Credentials (AgentSecrets.template)

The system now supports 50+ job sites globally. Rename `AgentSecrets.template` to `.env` or `AgentSecrets.toml` and fill in your credentials:

```bash
# Copy the template
cp AgentSecrets.template .env

# Edit with your credentials
nano .env  # or use your preferred editor
```

**Key credential categories:**
- **Primary Global Sites**: LinkedIn, Indeed, Glassdoor, Monster, ZipRecruiter, CareerBuilder
- **European Sites**: StepStone, Xing, Jobs.eu, EuroJobs
- **UK Sites**: Reed, Totaljobs, CV-Library, JobSite
- **Canadian Sites**: Workopolis, Job Bank Canada, Monster Canada
- **Australian Sites**: Seek, CareerOne, Jora
- **Asian Sites**: JobsDB, JobStreet, Zhaopin
- **Tech Sites**: Stack Overflow, AngelList, Dice, GitHub Jobs
- **Company Career Pages**: Intel, Nike, Apple, Microsoft, Google, Amazon, Meta, Tesla, and more

### 2. Configure Job Search Criteria (AgentConfig.toml)

The configuration now includes comprehensive international job boards:

```toml
name = "Your Name"
email = "your.email@example.com"
location = ["Your City, State", "Remote", "International"]

# Enable account creation
[account_creation]
enabled = true
use_resume_data = true
use_cover_letter_data = true
auto_verify_email = false
create_professional_profiles = true
upload_documents_automatically = true

# Enable international search
enable_international_search = true
preferred_locales = ["en-US", "en-GB", "en-CA"]
preferred_currency = "USD"
```

### 3. Set Up Resume and Cover Letter

Update the paths in AgentConfig.toml:
```toml
resume_path = "/path/to/your/resume.pdf"
cover_letter_path = "/path/to/your/cover_letter.pdf"
```

### 4. Run the Agent

**CLI Mode (Headless):**
```bash
cargo run --bin autonomous_job_agent -- --cli
```

**GUI Mode (Interactive):**
```bash
cargo run --bin autonomous_job_agent
```

## New Features in This Version

### 🌍 Global Job Site Coverage

The system now supports 50+ job sites across:
- **North America**: US, Canada, Mexico
- **Europe**: UK, Germany, France, Netherlands, and more
- **Asia-Pacific**: Singapore, Malaysia, Hong Kong, Australia
- **Tech-Specific**: Platforms for developers and tech professionals

### 🤖 Automated Account Creation

The agent can automatically create accounts on job sites using your resume and cover letter data:

```toml
[account_creation]
enabled = true                          # Enable account creation
use_resume_data = true                  # Extract info from resume
use_cover_letter_data = true            # Extract info from cover letter
create_professional_profiles = true     # Set up complete profiles
upload_documents_automatically = true   # Upload resume/cover letter
```

### 🏢 Major Company Career Pages

Direct integration with career pages of major employers:
- **Tech Giants**: Apple, Microsoft, Google, Amazon, Meta, Tesla
- **Manufacturing**: Intel, Nike, Adidas, Boeing, GE, Caterpillar  
- **Automotive**: Tesla, Ford, GM, BMW, Mercedes-Benz
- **International**: IBM, HP, Oracle, SAP, Siemens, Philips

### 🔧 Enhanced Browser Automation

- **Adaptive automation** - Adjusts behavior based on success rates
- **Rate limiting** - Prevents triggering anti-spam measures
- **Intelligent scheduling** - Optimizes application timing
- **Performance monitoring** - Tracks and improves success rates

## Configuration Examples

### International Job Search Setup

```toml
location = ["Remote", "New York, NY", "London, UK", "Toronto, CA"]
enable_international_search = true
preferred_locales = ["en-US", "en-GB", "en-CA"]

[regional_job_boards]
EU = ["https://stepstone.com", "https://jobs.eu", "https://xing.com"]
UK = ["https://reed.co.uk", "https://totaljobs.com"]
CA = ["https://workopolis.com", "https://jobbank.gc.ca"]
```

### Tech-Focused Job Search

```toml
industries = ["Software", "Technology", "Engineering", "Startups"]
targets = [
    "https://stackoverflow.com/jobs",
    "https://angellist.com", 
    "https://github.com/jobs",
    "https://dice.com"
]

[company_career_pages]
TECH_GIANTS = [
    "https://jobs.apple.com",
    "https://careers.google.com",
    "https://amazon.jobs"
]
```

### Manufacturing/Engineering Focus

```toml
industries = ["Manufacturing", "Engineering", "Automotive", "Aerospace"]

[company_career_pages]
MANUFACTURING = [
    "https://jobs.intel.com",
    "https://boeing.com/careers",
    "https://ge.com/careers"
]
AUTOMOTIVE = [
    "https://tesla.com/careers",
    "https://ford.com/careers",
    "https://gm.com/careers"
]
```

## Advanced Features

### Search Engine Reconnaissance

The agent scans search engines for job postings:

```toml
search_engines = ["https://www.google.com/search?q=", "https://www.bing.com/search?q="]
company_targets = ["Intel", "Nike", "Tesla", "Microsoft"]
search_locations = ["Your City", "Remote"]
```

### Adaptive Automation Settings

```toml
enable_adaptive_automation = true
max_concurrent_applications = 5
application_rate_limit = 15  # Applications per hour
intelligent_scheduling = true
performance_monitoring = true
```

## Security and Privacy

- **Credential Protection**: All credentials stored locally in `.env` file
- **No API Dependencies**: Uses browser automation instead of APIs
- **Local Operation**: All processing happens on your machine
- **Resume Privacy**: Documents never leave your system except for applications

## Support

The system now provides comprehensive global coverage for job applications with automated account creation and intelligent browser automation. All major international job sites and company career pages are supported.

For issues or questions, check the logs in CLI mode or the GUI real-time monitor.
