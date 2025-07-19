#!/bin/bash

# ===================================================================
# AI Job Agent - Single Click Deployment Script
# ===================================================================
# This script provides complete one-click deployment for the AI Job Agent
# Supports both GUI and CLI modes with automatic dependency resolution

set -e  # Exit on any error

# Color codes for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}$1${NC}"
}

# Print banner
clear
print_header "====================================================================="
print_header "              AI JOB AGENT - SINGLE CLICK DEPLOYMENT"
print_header "====================================================================="
echo

# Parse command line arguments
GUI_MODE=true
FORCE_BUILD=false
PRODUCTION_MODE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --cli|--headless)
            GUI_MODE=false
            shift
            ;;
        --force-build)
            FORCE_BUILD=true
            shift
            ;;
        --production)
            PRODUCTION_MODE=true
            shift
            ;;
        --help|-h)
            echo "AI Job Agent Deployment Script"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --cli, --headless    Run in CLI mode (no GUI)"
            echo "  --force-build        Force rebuild even if binary exists"
            echo "  --production         Enable production optimizations"
            echo "  --help, -h           Show this help message"
            echo ""
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if we're in the correct directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the root of the AI-JOB-AGENT repository"
    exit 1
fi

print_status "Starting deployment process..."

# Create necessary directories
print_status "Creating necessary directories..."
mkdir -p state
mkdir -p logs
mkdir -p temp

# Check and validate configuration
print_status "Validating configuration..."
if [ ! -f "AgentConfig.toml" ]; then
    print_error "AgentConfig.toml not found. Please ensure it exists and is properly configured."
    exit 1
fi

# Validate required configuration fields
validate_config() {
    local field=$1
    local file=$2
    if ! grep -q "$field = " "$file"; then
        print_error "AgentConfig.toml missing required '$field' field"
        exit 1
    fi
}

validate_config "name" "AgentConfig.toml"
validate_config "email" "AgentConfig.toml"
validate_config "resume_path" "AgentConfig.toml"

print_status "✓ Configuration validation passed"

# Remove conflicting files that could hinder deployment
print_status "Cleaning up conflicting files..."
rm -f Cargo.toml.bak Cargo.toml.complex Cargo.lock.bak
rm -rf target/debug  # Remove debug builds to save space

# Check for Rust and Cargo
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust and Cargo from https://rustup.rs/"
    exit 1
fi

print_status "✓ Rust/Cargo found"

# Determine build mode
RELEASE_FLAG=""
if [ "$PRODUCTION_MODE" = true ]; then
    RELEASE_FLAG="--release"
    print_status "Building in production mode (optimized)..."
else
    print_status "Building in development mode..."
fi

# Check if binary exists and if we need to build
BINARY_PATH="target/release/autonomous_job_agent"
if [ "$PRODUCTION_MODE" = false ]; then
    BINARY_PATH="target/debug/autonomous_job_agent"
fi

if [ ! -f "$BINARY_PATH" ] || [ "$FORCE_BUILD" = true ]; then
    print_status "Building AI Job Agent..."
    
    # Build the application
    if cargo build $RELEASE_FLAG --bin autonomous_job_agent; then
        print_status "✓ Build completed successfully"
    else
        print_error "Build failed"
        exit 1
    fi
else
    print_status "✓ Using existing binary: $BINARY_PATH"
fi

# Check for GUI dependencies if GUI mode is requested
if [ "$GUI_MODE" = true ]; then
    print_status "Checking GUI dependencies..."
    
    # Check for display
    if [ -z "$DISPLAY" ]; then
        print_warning "No DISPLAY environment variable set."
        print_warning "Attempting to start virtual display..."
        
        # Install xvfb if not present
        if ! command -v Xvfb &> /dev/null; then
            print_status "Installing virtual display server..."
            sudo apt-get update -qq && sudo apt-get install -y xvfb
        fi
        
        # Start virtual display
        Xvfb :99 -screen 0 1024x768x24 &
        export DISPLAY=:99
        print_status "✓ Virtual display started on :99"
    fi
    
    # Check for required libraries
    if ! ldconfig -p | grep -q libxkbcommon-x11; then
        print_status "Installing GUI dependencies..."
        sudo apt-get update -qq && sudo apt-get install -y libxkbcommon-x11-0 libxkbcommon-dev
        print_status "✓ GUI dependencies installed"
    fi
fi

# Set up environment variables
print_status "Setting up environment..."
export RUST_LOG=${RUST_LOG:-info}
export RUST_BACKTRACE=${RUST_BACKTRACE:-1}

# Create a run script for easy restart
print_status "Creating run script..."
cat > run_job_agent.sh << EOF
#!/bin/bash
# Generated run script for AI Job Agent

export RUST_LOG=info
export RUST_BACKTRACE=1

# Ensure directories exist
mkdir -p state logs temp

# Run the application
cd "\$(dirname "\$0")"

if [ "\$1" = "--cli" ] || [ "\$1" = "--headless" ]; then
    echo "Starting AI Job Agent in CLI mode..."
    ./$BINARY_PATH --cli 2>&1 | tee logs/job_agent_cli_\$(date +%Y%m%d_%H%M%S).log
else
    echo "Starting AI Job Agent in GUI mode..."
    ./$BINARY_PATH 2>&1 | tee logs/job_agent_gui_\$(date +%Y%m%d_%H%M%S).log
fi
EOF

chmod +x run_job_agent.sh

# Create systemd service for production deployment
if [ "$PRODUCTION_MODE" = true ]; then
    print_status "Creating systemd service..."
    cat > ai-job-agent.service << EOF
[Unit]
Description=AI Job Agent
After=network.target

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/run_job_agent.sh --cli
Restart=on-failure
RestartSec=30
Environment=RUST_LOG=info
Environment=RUST_BACKTRACE=1
Environment=DISPLAY=:99

[Install]
WantedBy=multi-user.target
EOF
    print_status "✓ Systemd service file created: ai-job-agent.service"
fi

# Final deployment status
print_header ""
print_header "====================================================================="
print_header "                    DEPLOYMENT COMPLETED SUCCESSFULLY"
print_header "====================================================================="
echo

print_status "AI Job Agent is ready to run!"
echo

if [ "$GUI_MODE" = true ]; then
    print_header "GUI Mode - Starting with observability interface..."
    print_status "The GUI will show real-time agent activity and job application status"
    print_status "Agent and GUI are fully synchronized via message passing"
    
    # Start the application in GUI mode
    print_status "Launching AI Job Agent with GUI..."
    ./$BINARY_PATH 2>&1 | tee logs/job_agent_gui_$(date +%Y%m%d_%H%M%S).log
else
    print_header "CLI Mode - Starting in headless mode..."
    print_status "Agent will run continuously and log all activities"
    print_status "Check logs in the 'logs' directory for monitoring"
    
    # Start the application in CLI mode
    print_status "Launching AI Job Agent in CLI mode..."
    ./$BINARY_PATH --cli 2>&1 | tee logs/job_agent_cli_$(date +%Y%m%d_%H%M%S).log
fi

# If we reach here, the application has exited
print_status "AI Job Agent has stopped."

if [ "$PRODUCTION_MODE" = true ]; then
    echo
    print_header "Production Deployment Options:"
    echo "• Install as system service: sudo cp ai-job-agent.service /etc/systemd/system/ && sudo systemctl enable ai-job-agent"
    echo "• Start service: sudo systemctl start ai-job-agent"
    echo "• View logs: sudo journalctl -u ai-job-agent -f"
    echo "• Manual restart: ./run_job_agent.sh"
fi

echo
print_header "====================================================================="
print_header "                         DEPLOYMENT COMPLETE"
print_header "====================================================================="