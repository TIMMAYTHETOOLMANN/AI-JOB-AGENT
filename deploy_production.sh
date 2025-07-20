#!/bin/bash

# ========================================================
# PRODUCTION DEPLOYMENT SCRIPT - FULL GUI & BACKEND
# ========================================================

echo "🚀 AI Job Agent - Production Deployment System"
echo "=============================================="

# Create necessary directories
echo "📁 Creating production directories..."
mkdir -p state
mkdir -p logs
mkdir -p screenshots
mkdir -p config
mkdir -p data

# Build all production binaries
echo "🔨 Building production binaries..."
cargo build --release --features production-gui,web-server,visible-chrome

if [ $? -ne 0 ]; then
    echo "❌ Build failed! Check compilation errors."
    exit 1
fi

echo "✅ Build completed successfully!"

# Set up configuration if not exists
if [ ! -f "AgentConfig.toml" ]; then
    echo "📝 Creating default configuration..."
    cat > AgentConfig.toml << 'EOF'
name = "Production Agent"
email = "agent@example.com"
location = ["Remote", "United States"]
resume_path = "resume.pdf"
cover_letter_path = "cover_letter.pdf"

[job_search]
keywords = ["software engineer", "developer", "programmer"]
industries = ["Technology", "Software"]
seniority_levels = ["Mid-level", "Senior"]

[automation]
max_applications_per_day = 50
application_rate_limit = 10
enable_international_search = true
enable_adaptive_automation = true

[account_creation]
enabled = true
use_resume_data = true
use_cover_letter_data = true
auto_verify_email = false
create_professional_profiles = true

[web_server]
port = 8080
enable_api = true
enable_websocket = true
EOF
fi

# Set up secrets template if not exists
if [ ! -f "AgentSecrets.template" ]; then
    echo "🔐 Creating secrets template..."
    cat > AgentSecrets.template << 'EOF'
# AI Job Agent - Production Secrets Configuration
# Copy this file to .env and fill in your credentials

# Agent Identity
AGENT_NAME="Your Full Name"
AGENT_EMAIL="your.email@example.com"
AGENT_PHONE="+1-555-123-4567"

# Document Paths
RESUME_PATH="/path/to/your/resume.pdf"
COVER_LETTER_PATH="/path/to/your/cover_letter.pdf"

# LinkedIn Credentials
LINKEDIN_EMAIL="linkedin@example.com"
LINKEDIN_PASSWORD="your_linkedin_password"

# Indeed Credentials
INDEED_EMAIL="indeed@example.com"
INDEED_PASSWORD="your_indeed_password"

# Glassdoor Credentials
GLASSDOOR_EMAIL="glassdoor@example.com"
GLASSDOOR_PASSWORD="your_glassdoor_password"

# Monster Credentials
MONSTER_EMAIL="monster@example.com"
MONSTER_PASSWORD="your_monster_password"

# ZipRecruiter Credentials
ZIPRECRUITER_EMAIL="ziprecruiter@example.com"
ZIPRECRUITER_PASSWORD="your_ziprecruiter_password"

# Web Server Security
WEB_SERVER_SECRET="random_secret_key_here"
API_ACCESS_TOKEN="secure_api_token_here"
EOF
fi

echo "🎯 Production system ready! Choose deployment mode:"
echo ""
echo "1. 🎮 GUI Mode:    ./target/release/autonomous_job_agent --gui"
echo "2. 🌐 Web Mode:    ./target/release/autonomous_job_agent --web"
echo "3. ⚡ CLI Mode:    ./target/release/autonomous_job_agent --cli"
echo "4. 🚀 Hybrid:      ./target/release/autonomous_job_agent"
echo ""
echo "📋 Specialized binaries:"
echo "   • GUI Only:     ./target/release/production_gui"
echo "   • Web Only:     ./target/release/web_server"
echo ""
echo "🔧 Configuration:"
echo "   • Edit AgentConfig.toml for job search settings"
echo "   • Copy AgentSecrets.template to .env and add credentials"
echo ""
echo "✅ Production deployment complete!"
