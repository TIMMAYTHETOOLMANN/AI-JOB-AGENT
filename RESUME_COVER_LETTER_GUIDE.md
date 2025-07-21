# Resume and Cover Letter Configuration Guide

## Overview

This guide explains how to properly configure the AI Job Agent to use your resume and cover letter files. The application has been updated to use the proprietary resume and cover letter paths specified in the `AgentSecrets.template` file.

## Current Configuration

The application has been configured to use the following paths for your resume and cover letter:

- **Resume Path**: `C:/OneDrive/Documents/Desktop/FIN RES.pdf`
- **Cover Letter Path**: `C:/OneDrive/Documents/Desktop/FIN CV.pdf`

These paths were extracted from the `AgentSecrets.template` file, which contains the proprietary information for your resume and cover letter.

## Changes Made

The following changes have been made to ensure the application uses the correct resume and cover letter files:

1. **AgentConfig.toml**: Updated to use the proprietary paths from AgentSecrets.template:
   ```toml
   resume_path = "C:/OneDrive/Documents/Desktop/FIN RES.pdf"
   cover_letter_path = "C:/OneDrive/Documents/Desktop/FIN CV.pdf"
   ```

2. **.env File**: Updated to use the proprietary paths from AgentSecrets.template:
   ```
   RESUME_PATH="C:/OneDrive/Documents/Desktop/FIN RES.pdf"
   COVER_LETTER_PATH="C:/OneDrive/Documents/Desktop/FIN CV.pdf"
   ```

3. **Deployment Script**: Created an updated deployment script (`deploy_production_updated.bat`) that:
   - Checks for the presence of resume_path and cover_letter_path in AgentConfig.toml
   - Removes the checks for local resume.pdf and cover_letter.pdf files
   - Updates the production checklist to emphasize the importance of correct paths

## How to Use

### Option 1: Use the Proprietary Files (Recommended)

1. Ensure that your resume and cover letter files exist at the specified paths:
   - `C:/OneDrive/Documents/Desktop/FIN RES.pdf`
   - `C:/OneDrive/Documents/Desktop/FIN CV.pdf`

2. Run the updated deployment script:
   ```
   .\deploy_production_updated.bat
   ```

3. Follow the instructions in the deployment script to complete the deployment.

### Option 2: Update the Paths to Your Files

If your resume and cover letter files are in different locations:

1. Update the paths in both `AgentConfig.toml` and `.env`:
   - In `AgentConfig.toml`:
     ```toml
     resume_path = "path/to/your/resume.pdf"
     cover_letter_path = "path/to/your/cover_letter.pdf"
     ```
   - In `.env`:
     ```
     RESUME_PATH="path/to/your/resume.pdf"
     COVER_LETTER_PATH="path/to/your/cover_letter.pdf"
     ```

2. Run the updated deployment script:
   ```
   .\deploy_production_updated.bat
   ```

3. Follow the instructions in the deployment script to complete the deployment.

## Troubleshooting

If you encounter issues with the resume or cover letter files:

1. **File Not Found**: Ensure that the files exist at the specified paths.
2. **Path Format**: Ensure that the paths use the correct format for your operating system (Windows uses backslashes `\` or forward slashes `/`).
3. **File Format**: Ensure that the files are in PDF format.
4. **File Permissions**: Ensure that the application has permission to read the files.

## Additional Notes

- The application will use the paths specified in `AgentConfig.toml` for the resume and cover letter files.
- The environment variables in `.env` are used as a fallback if the paths in `AgentConfig.toml` are not valid.
- The `AgentSecrets.template` file contains the proprietary information for your resume and cover letter, but it is not used directly by the application.