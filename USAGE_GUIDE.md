# AI Job Agent - Usage Guide

## Quick Setup for Global Job Applications

### Step 1: Set Up Your Credentials

1. Copy the credentials template:
   ```bash
   cp AgentSecrets.template .env
   ```

2. Edit `.env` with your job site credentials:
   ```bash
   # Essential credentials for major sites
   LINKEDIN_EMAIL="your-email@example.com"
   LINKEDIN_PASSWORD="your-password"
   
   INDEED_EMAIL="your-email@example.com"
   INDEED_PASSWORD="your-password"
   
   GLASSDOOR_EMAIL="your-email@example.com"
   GLASSDOOR_PASSWORD="your-password"
   
   # Add more sites as needed...
   ```

### Step 2: Configure Your Profile

Edit `AgentConfig.toml`:

```toml
name = "Your Full Name"
email = "your.email@example.com"
location = ["Your City, State", "Remote"]

# Update these paths to your actual files
resume_path = "/path/to/your/resume.pdf"
cover_letter_path = "/path/to/your/cover_letter.pdf"

# Enable account creation
[account_creation]
enabled = true
use_resume_data = true
use_cover_letter_data = true
```

### Step 3: Run the Agent

**For CLI mode (headless):**
```bash
cargo run --bin autonomous_job_agent -- --cli
```

**For GUI mode (with interface):**
```bash
cargo run --bin autonomous_job_agent
```

## Supported Job Sites (50+)

### Global Platforms
- LinkedIn Jobs
- Indeed (Global)
- Glassdoor
- Monster
- ZipRecruiter
- CareerBuilder

### European Sites
- StepStone (Europe)
- Xing (Germany/DACH)
- Jobs.eu
- EuroJobs

### UK Sites
- Reed
- Totaljobs
- CV-Library
- JobSite

### Canadian Sites
- Workopolis
- Job Bank Canada
- Monster Canada

### Australian Sites
- Seek
- CareerOne
- Jora

### Tech-Specific
- Stack Overflow Jobs
- AngelList/Wellfound
- Dice
- GitHub Jobs

### Major Company Career Pages
- Intel, Nike, Apple, Microsoft
- Google, Amazon, Meta, Tesla
- Boeing, GE, IBM, HP
- And many more...

## What the Agent Does

1. **Account Creation**: Automatically creates accounts on job sites using your resume data
2. **Job Search**: Searches all configured job boards for matching positions
3. **Smart Filtering**: Filters jobs based on your criteria (location, salary, keywords)
4. **Application**: Automatically applies to matching jobs with your resume and cover letter
5. **Tracking**: Keeps track of applications to avoid duplicates
6. **Reporting**: Provides real-time status and performance metrics

## Key Features

- **Global Coverage**: Applies to jobs worldwide across 50+ platforms
- **Intelligent Automation**: Adapts behavior based on success rates
- **Rate Limiting**: Prevents triggering anti-spam measures
- **Resume Matching**: Uses multiple resume profiles for different job types
- **Company Targeting**: Direct applications to major company career pages
- **Search Engine Reconnaissance**: Finds jobs through Google/Bing searches

## Security Notes

- All credentials are stored locally in your `.env` file
- No data is sent to third parties except for job applications
- Resume and cover letter files remain on your local system
- Browser automation mimics human behavior to avoid detection

## Troubleshooting

### If the agent fails to start:
1. Ensure `AgentConfig.toml` exists and is properly formatted
2. Check that resume and cover letter paths are correct
3. Verify credentials in `.env` file are properly formatted

### If applications are not working:
1. Check browser automation in the logs
2. Verify job site credentials are correct
3. Ensure rate limits are not being exceeded

### For international job search:
1. Set `enable_international_search = true` in config
2. Add appropriate `preferred_locales`
3. Configure regional job boards for your target countries

## Tips for Best Results

1. **Use specific job keywords** that match your field
2. **Configure multiple resume profiles** for different job types
3. **Set realistic salary ranges** for your target markets
4. **Enable international search** to maximize opportunities
5. **Monitor the logs** to ensure applications are successful
6. **Update credentials regularly** as job sites change their interfaces

The system is designed to be comprehensive and autonomous - once configured, it will continuously search and apply for jobs across all major platforms globally.