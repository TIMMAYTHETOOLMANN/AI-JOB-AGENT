# Credential Management System

The AI Job Agent now includes an enhanced credential management system that allows you to securely store and retrieve credentials for multiple job platforms with minimal input.

## Features

- **Multi-source credential loading** with priority order
- **Environment variable support** for secure credential storage
- **API token authentication** for supported platforms
- **Automatic credential detection** and validation
- **Platform-specific authentication** methods

## Supported Platforms

- **LinkedIn** - OAuth2, API keys, session cookies
- **Indeed** - Basic auth, API keys
- **Glassdoor** - API keys
- **Monster** - Basic authentication
- **ZipRecruiter** - Session cookies
- **CareerBuilder** - Session cookies

## Quick Setup

### Method 1: Environment Variables (Recommended)

Set environment variables for the platforms you want to use:

```bash
# Agent Information
export AGENT_NAME="Your Name"
export AGENT_EMAIL="your.email@example.com"
export RESUME_PATH="/path/to/your/resume.pdf"
export COVER_LETTER_PATH="/path/to/your/cover-letter.pdf"

# LinkedIn
export LINKEDIN_EMAIL="your.linkedin@email.com"
export LINKEDIN_PASSWORD="your-password"

# Indeed
export INDEED_EMAIL="your.indeed@email.com"
export INDEED_PASSWORD="your-password"

# Add more platforms as needed...
```

### Method 2: .env File

Create a `.env` file in the project root:

```bash
# Copy the example file
cp .env.example .env

# Edit .env with your credentials
nano .env
```

### Method 3: AgentSecrets.template

Edit the existing `AgentSecrets.template` file with your credentials.

## Testing Your Setup

Use the credential test utility to verify your setup:

```bash
cargo run --bin test_credentials
```

This will show:
- ✅ Which platforms have valid credentials
- 🔐 Authentication test results
- 📋 Available credential sources

## Credential Priority

The system loads credentials in this priority order:

1. **Environment Variables** (highest priority)
2. **.env file** 
3. **AgentSecrets.toml**
4. **AgentSecrets.template** (fallback)

## Environment Variable Names

### Agent Information
- `AGENT_NAME` - Your full name
- `AGENT_EMAIL` - Your email address  
- `RESUME_PATH` - Path to your resume file
- `COVER_LETTER_PATH` - Path to your cover letter file

### Platform Credentials (Replace PLATFORM with the platform name)
- `PLATFORM_EMAIL` - Account email
- `PLATFORM_PASSWORD` - Account password
- `PLATFORM_API_KEY` - API key (if available)
- `PLATFORM_API_SECRET` - API secret (if available)
- `PLATFORM_SESSION_COOKIE` - Session cookie (if available)
- `PLATFORM_ACCESS_TOKEN` - OAuth access token (if available)
- `PLATFORM_REFRESH_TOKEN` - OAuth refresh token (if available)

### Supported Platform Names
- `LINKEDIN`
- `INDEED`
- `GLASSDOOR`
- `MONSTER`
- `ZIPRECRUITER`
- `CAREERBUILDER`

## Security Best Practices

1. **Use environment variables** for production
2. **Never commit** credential files to version control
3. **Use API tokens** instead of passwords when possible
4. **Rotate credentials** regularly
5. **Limit credential scope** to necessary permissions only

## Troubleshooting

### No credentials found
- Check that environment variables are set correctly
- Verify `.env` file exists and has correct format
- Check `AgentSecrets.template` has values filled in

### Authentication failed
- Verify credentials are correct
- Check if platform requires API keys instead of passwords
- Ensure account has necessary permissions
- Some platforms may require OAuth setup

### Platform not supported
- Check the supported platforms list above
- The system is extensible - new platforms can be added

## API Token Authentication

The system can automatically fetch API tokens for supported platforms using your username/password. This provides:

- **Secure access** without storing passwords long-term
- **Token refresh** capabilities
- **Session management** for platforms that require it

## Example Usage

```bash
# Set basic credentials
export LINKEDIN_EMAIL="john.doe@email.com"
export LINKEDIN_PASSWORD="secure-password"

# Test the credentials
cargo run --bin test_credentials

# Run the job agent
cargo run --release -- --cli
```

The system will automatically:
1. Load your credentials
2. Authenticate with platforms
3. Fetch API tokens as needed
4. Use secure tokens for job searching and applications

This minimal input approach means you only need to set your credentials once, and the system handles the rest automatically.