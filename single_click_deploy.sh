#!/bin/bash

# ===================================================================
# AI Job Agent - Single-Click Production Deployment Script
# ===================================================================
# This script provides complete one-click deployment for the AI Job Agent
# Consolidates GUI, backend, and web interface into a unified system

set -e  # Exit on any error

# Color codes for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
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

print_success() {
    echo -e "${GREEN}$1${NC}"
}

print_command() {
    echo -e "${PURPLE}[CMD]${NC} $1"
}

# Print banner
clear
print_header "======================================================================="
print_header "    AI JOB AGENT - PRODUCTION DEPLOYMENT & CONTROL CENTER"
print_header "======================================================================="
echo
print_status "Single-click deployment with integrated GUI and backend services"
echo

# Parse command line arguments
GUI_MODE=true
CLI_MODE=false
FORCE_BUILD=false
PRODUCTION_MODE=true
DOCKER_MODE=false
PORT_WEB=3000
PORT_API=8080

while [[ $# -gt 0 ]]; do
    case $1 in
        --cli|--headless)
            GUI_MODE=false
            CLI_MODE=true
            shift
            ;;
        --gui)
            GUI_MODE=true
            CLI_MODE=false
            shift
            ;;
        --force-build)
            FORCE_BUILD=true
            shift
            ;;
        --docker)
            DOCKER_MODE=true
            shift
            ;;
        --dev)
            PRODUCTION_MODE=false
            shift
            ;;
        --web-port)
            PORT_WEB="$2"
            shift 2
            ;;
        --api-port)
            PORT_API="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --cli, --headless    Run in CLI mode without GUI"
            echo "  --gui               Run with GUI interface (default)"
            echo "  --force-build       Force rebuild even if binary exists"
            echo "  --docker            Use Docker deployment"
            echo "  --dev               Development mode"
            echo "  --web-port PORT     Web UI port (default: 3000)"
            echo "  --api-port PORT     API port (default: 8080)"
            echo "  --help              Show this help message"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Validation and setup
print_header "=== SYSTEM VALIDATION ==="

# Check if we're in the correct directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the root of the AI-JOB-AGENT repository"
    exit 1
fi

print_status "✓ Repository root confirmed"

# Create necessary directories
mkdir -p state logs temp
print_status "✓ Required directories created"

# Check if configuration exists
if [ ! -f "AgentConfig.toml" ]; then
    print_warning "AgentConfig.toml not found"
    print_status "Creating default configuration file..."
    cat > AgentConfig.toml << 'EOF'
# AI Job Agent Configuration File
name = "AI Job Agent"
email = "your-email@example.com"
location = ["Remote", "San Francisco", "New York"]
open_to_relocation = true
resume_path = "resume.pdf"
cover_letter_path = "cover_letter.pdf"

job_keywords = ["software engineer", "developer", "programmer"]
skills = ["Python", "Rust", "JavaScript", "React", "Docker"]
industries = ["Technology", "Software", "AI/ML"]
seniority_levels = ["Mid-level", "Senior"]

job_boards = [
    "https://jobs.lever.co",
    "https://boards.greenhouse.io",
    "https://www.linkedin.com/jobs",
    "https://stackoverflow.com/jobs"
]

intelligent_scheduling = true
max_concurrent_applications = 3
application_rate_limit = 10
EOF
    print_status "✓ Default configuration created - please update AgentConfig.toml with your details"
fi

print_status "✓ Configuration file exists"

# Rust and Cargo check
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

print_status "✓ Rust/Cargo available"

# Build the application
print_header "=== BUILDING APPLICATION ==="

BINARY_PATH="target/release/autonomous_job_agent"

if [ "$FORCE_BUILD" = true ] || [ ! -f "$BINARY_PATH" ]; then
    print_status "Building AI Job Agent in release mode..."
    print_command "cargo build --release"
    
    if cargo build --release; then
        print_success "✓ Build completed successfully"
    else
        print_error "Build failed"
        exit 1
    fi
else
    print_status "✓ Binary already exists, skipping build (use --force-build to rebuild)"
fi

# Docker deployment option
if [ "$DOCKER_MODE" = true ]; then
    print_header "=== DOCKER DEPLOYMENT ==="
    
    print_status "Building and starting Docker containers..."
    print_command "docker-compose up --build -d"
    
    if docker-compose up --build -d; then
        print_success "✓ Docker services started successfully"
        print_status "Web UI available at: http://localhost:$PORT_WEB"
        print_status "API available at: http://localhost:$PORT_API"
        print_status "Use 'docker-compose logs -f' to view logs"
        print_status "Use 'docker-compose down' to stop services"
        exit 0
    else
        print_error "Docker deployment failed"
        exit 1
    fi
fi

# Native deployment
print_header "=== STARTING SERVICES ==="

# Create systemd service file for production
if [ "$PRODUCTION_MODE" = true ] && command -v systemctl &> /dev/null; then
    print_status "Setting up systemd service for production..."
    
    SERVICE_FILE="/etc/systemd/system/ai-job-agent.service"
    if [ ! -f "$SERVICE_FILE" ] && [ "$EUID" -eq 0 ]; then
        cat > "$SERVICE_FILE" << EOF
[Unit]
Description=AI Job Agent
After=network.target

[Service]
Type=simple
User=$SUDO_USER
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/$BINARY_PATH
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF
        systemctl daemon-reload
        systemctl enable ai-job-agent
        print_status "✓ Systemd service created and enabled"
    fi
fi

# Function to start web server for GUI
start_web_server() {
    if command -v python3 &> /dev/null; then
        print_status "Starting web server on port $PORT_WEB..."
        cd web
        python3 -m http.server $PORT_WEB > ../logs/web_server.log 2>&1 &
        WEB_PID=$!
        cd ..
        echo $WEB_PID > logs/web_server.pid
        print_success "✓ Web server started (PID: $WEB_PID)"
    elif command -v npx &> /dev/null; then
        print_status "Starting web server with npx..."
        cd web
        npx serve -p $PORT_WEB > ../logs/web_server.log 2>&1 &
        WEB_PID=$!
        cd ..
        echo $WEB_PID > logs/web_server.pid
        print_success "✓ Web server started (PID: $WEB_PID)"
    else
        print_warning "No suitable web server found (python3 or npx), web UI may not be available"
    fi
}

# Start appropriate mode
if [ "$GUI_MODE" = true ]; then
    print_header "=== STARTING GUI MODE ==="
    
    # Start web server
    start_web_server
    
    print_status "Starting AI Job Agent with GUI..."
    print_command "./$BINARY_PATH"
    
    # Store main process PID
    echo $$ > logs/main_process.pid
    
    print_success "✓ AI Job Agent started successfully!"
    print_status "Web Dashboard: http://localhost:$PORT_WEB"
    print_status "API Endpoint: http://localhost:$PORT_API"
    print_status "Logs: tail -f logs/*.log"
    echo
    print_status "Press Ctrl+C to stop all services"
    
    # Cleanup function
    cleanup() {
        print_warning "Shutting down services..."
        
        # Stop web server
        if [ -f logs/web_server.pid ]; then
            WEB_PID=$(cat logs/web_server.pid)
            if kill -0 $WEB_PID 2>/dev/null; then
                kill $WEB_PID
                print_status "✓ Web server stopped"
            fi
            rm -f logs/web_server.pid
        fi
        
        # Stop main process
        if [ -f logs/main_process.pid ]; then
            rm -f logs/main_process.pid
        fi
        
        print_success "✓ All services stopped"
        exit 0
    }
    
    # Set up signal handlers
    trap cleanup SIGINT SIGTERM
    
    # Run the agent
    ./$BINARY_PATH
    
else
    print_header "=== STARTING CLI MODE ==="
    
    print_status "Starting AI Job Agent in CLI mode..."
    print_command "./$BINARY_PATH --cli"
    
    print_success "✓ AI Job Agent started in headless mode!"
    print_status "API Endpoint: http://localhost:$PORT_API"
    print_status "Logs: tail -f logs/*.log"
    echo
    
    # Run the agent in CLI mode
    ./$BINARY_PATH --cli
fi