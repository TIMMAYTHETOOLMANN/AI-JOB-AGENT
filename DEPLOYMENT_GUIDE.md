# AI Job Agent Deployment Guide

This guide provides step-by-step instructions for deploying the AI Job Agent application in a production environment with a browser GUI.

## Prerequisites

1. **Rust and Cargo**: Ensure Rust and Cargo are installed (version 1.88.0 or later)
2. **Web Browser**: Chrome or Chromium is required for browser automation
3. **PDF Files**: Prepare your resume.pdf and cover_letter.pdf files

## Configuration Steps

### 1. Configure AgentConfig.toml

The AgentConfig.toml file has been updated with the following settings:
- Name: "Timothy Roessel"
- Email: "timothyroessel@gmail.com"
- Resume path: "resume.pdf"
- Cover letter path: "cover_letter.pdf"

You may need to adjust these settings based on your preferences.

### 2. Create .env File

An .env file has been created from the AgentSecrets.template file with the following essential settings:
- Agent name and email
- Paths to resume and cover letter files
- Placeholders for job site credentials (LinkedIn, Indeed, Glassdoor, Monster, ZipRecruiter)
- Browser automation settings (BROWSER_HEADLESS="false")

You should update this file with your actual credentials for the job sites you want to use.

### 3. Prepare PDF Files

You need to provide actual PDF files for your resume and cover letter:
1. Create or obtain your resume in PDF format and save it as "resume.pdf" in the project root directory
2. Create or obtain your cover letter in PDF format and save it as "cover_letter.pdf" in the project root directory

## Deployment Issues

When attempting to deploy the application using deploy_production.bat, the following issues were encountered:

1. **Missing PDF Files**: The deployment script detected that the resume.pdf and cover_letter.pdf files are missing. You need to provide actual PDF files, not the placeholder .txt files.

2. **Compilation Errors**: The build failed with the following errors:
   - Error in job_application.rs: Use of moved value (E0382)
   - Error in main_gui.rs: Closure requires unique access to `*self` but it is already borrowed (E0500)
   - Various warnings about unused imports and variables

## Fixing the Issues

### 1. PDF Files
Replace the placeholder .txt files with actual PDF files:
```
resume.pdf.txt -> resume.pdf
cover_letter.pdf.txt -> cover_letter.pdf
```

### 2. Compilation Errors
The compilation errors require code changes. Here are some suggestions:

#### For job_application.rs (E0382):
The error occurs because the browser variable is moved in the loop. You might need to clone or reference the browser in each iteration.

#### For main_gui.rs (E0500):
The error occurs due to borrowing issues. You might need to restructure the code to avoid multiple borrows of self.

## Deployment Steps (After Fixing Issues)

1. Run the deployment script:
   ```
   .\deploy_production.bat
   ```

2. If the build is successful, the script will create:
   - Production directories (state, logs, screenshots)
   - A start_jarvis.bat script to run the application
   - A setup_environment.bat script to help with environment setup

3. Run the application:
   ```
   .\start_jarvis.bat
   ```

## Using the Application

Once deployed and running, the application will:
1. Initialize the visible browser
2. Search for jobs based on your criteria in AgentConfig.toml
3. Apply to jobs using your resume and cover letter
4. Pause for manual intervention when needed (e.g., for login/MFA)

You can control the application using:
- Ctrl+P: Pause automation
- Ctrl+R: Resume automation
- Ctrl+Q: Quit application

## Troubleshooting

If you encounter issues during deployment or execution:
1. Check the logs in the logs directory
2. Ensure all prerequisites are met
3. Verify that your PDF files are valid and accessible
4. Make sure your credentials in the .env file are correct

## Additional Resources

- README.md: Contains comprehensive information about the application
- AgentConfig.toml: Configuration for job search criteria and automation settings
- AgentSecrets.template: Template for credentials and secrets