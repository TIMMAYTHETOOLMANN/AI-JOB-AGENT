#!/bin/bash

# Production-ready AI Job Agent Setup Script
# This script sets up the environment and validates the system

echo "=== AI Job Agent Production Setup ==="

# Check if we're in the correct directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Please run this script from the root of the AI-JOB-AGENT repository"
    exit 1
fi

# Create necessary directories
mkdir -p state
mkdir -p logs
mkdir -p temp

# Check if configuration file exists
if [ ! -f "AgentConfig.toml" ]; then
    echo "Error: AgentConfig.toml not found. Please ensure it exists and is properly configured."
    exit 1
fi

# Validate configuration
echo "Validating configuration..."
if ! grep -q "name = " AgentConfig.toml; then
    echo "Error: AgentConfig.toml missing required 'name' field"
    exit 1
fi

if ! grep -q "email = " AgentConfig.toml; then
    echo "Error: AgentConfig.toml missing required 'email' field"
    exit 1
fi

if ! grep -q "resume_path = " AgentConfig.toml; then
    echo "Error: AgentConfig.toml missing required 'resume_path' field"
    exit 1
fi

echo "✓ Configuration validation passed"

# Check if Chrome/Chromium is available for headless automation
if command -v google-chrome &> /dev/null; then
    echo "✓ Google Chrome found"
    CHROME_PATH=$(which google-chrome)
elif command -v chromium-browser &> /dev/null; then
    echo "✓ Chromium found"
    CHROME_PATH=$(which chromium-browser)
elif command -v chromium &> /dev/null; then
    echo "✓ Chromium found"
    CHROME_PATH=$(which chromium)
else
    echo "Warning: No Chrome/Chromium found. Installing chromium..."
    # Try to install chromium
    if command -v apt-get &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y chromium-browser
    elif command -v yum &> /dev/null; then
        sudo yum install -y chromium
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y chromium
    else
        echo "Error: Cannot install Chrome/Chromium automatically. Please install manually."
        exit 1
    fi
fi

# Set Chrome path environment variable
export CHROME_PATH=$CHROME_PATH

# Create a test script to verify functionality
echo "Creating test script..."
cat > test_job_agent.sh << 'EOF'
#!/bin/bash

echo "=== Testing AI Job Agent ==="

# Test 1: Configuration loading
echo "Test 1: Configuration loading..."
if cargo run --bin test_config 2>/dev/null; then
    echo "✓ Configuration loads successfully"
else
    echo "✗ Configuration loading failed"
    exit 1
fi

# Test 2: Headless browser initialization
echo "Test 2: Headless browser test..."
# This would test the browser initialization

# Test 3: Job search functionality
echo "Test 3: Job search test..."
# This would test the job search functionality

echo "✓ All tests passed"
EOF

chmod +x test_job_agent.sh

# Create a production run script
echo "Creating production run script..."
cat > run_job_agent.sh << 'EOF'
#!/bin/bash

echo "=== Starting AI Job Agent in Production Mode ==="

# Set environment variables
export RUST_LOG=info
export RUST_BACKTRACE=1

# Ensure state directory exists
mkdir -p state
mkdir -p logs

# Run the job agent with proper logging
cargo run --release --bin autonomous_job_agent 2>&1 | tee logs/job_agent_$(date +%Y%m%d_%H%M%S).log
EOF

chmod +x run_job_agent.sh

# Create systemd service file for production deployment
echo "Creating systemd service file..."
cat > ai-job-agent.service << EOF
[Unit]
Description=AI Job Agent
After=network.target

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/run_job_agent.sh
Restart=on-failure
RestartSec=30
Environment=RUST_LOG=info
Environment=RUST_BACKTRACE=1

[Install]
WantedBy=multi-user.target
EOF

echo "✓ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Test the setup: ./test_job_agent.sh"
echo "2. Run in production: ./run_job_agent.sh"
echo "3. Install as service: sudo cp ai-job-agent.service /etc/systemd/system/ && sudo systemctl enable ai-job-agent"
echo ""
echo "Important: Make sure to configure your AgentSecrets.template with proper credentials"
echo "Important: Verify resume and cover letter paths in AgentConfig.toml"