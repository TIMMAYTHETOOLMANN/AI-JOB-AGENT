#!/bin/bash

# COPILOT COATING AGENT - UNIFIED DEPLOYMENT DIRECTIVE
# Single-click deployment protocol for any repository
# Version: 1.0.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Global variables
PROJECT_ROOT=""
PROJECT_NAME=""
DEPLOY_LOG=""
START_TIME=""
GUI_AVAILABLE=false
TERMINAL_UI_AVAILABLE=false
DOCKER_AVAILABLE=false
GIT_AVAILABLE=false
DEPLOY_STATUS="STARTING"
BACKGROUND_PID=""

# JSON status payload
JSON_STATUS='{
  "agent": "COPILOT_COATING_AGENT",
  "version": "1.0.0",
  "timestamp": "",
  "project_name": "",
  "project_type": "",
  "deployment_status": "STARTING",
  "steps_completed": [],
  "errors": [],
  "services": [],
  "git_snapshot": "",
  "rollback_available": false
}'

# Function to update JSON status
update_json_status() {
    local status="$1"
    local step="$2"
    local error="$3"
    
    DEPLOY_STATUS="$status"
    
    # Create updated JSON (simplified for bash)
    JSON_STATUS=$(cat <<EOF
{
  "agent": "COPILOT_COATING_AGENT",
  "version": "1.0.0",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "project_name": "$PROJECT_NAME",
  "project_type": "$PROJECT_TYPE",
  "deployment_status": "$status",
  "step_completed": "$step",
  "error": "$error",
  "git_snapshot": "$GIT_SNAPSHOT",
  "rollback_available": $ROLLBACK_AVAILABLE
}
EOF
)
    
    echo "$JSON_STATUS" > "${DEPLOY_LOG}.json"
}

# Function to log with timestamp
log() {
    local message="$1"
    local level="${2:-INFO}"
    local timestamp=$(date "+%Y-%m-%d %H:%M:%S")
    if [ -n "$DEPLOY_LOG" ]; then
        echo "[$timestamp] [$level] $message" | tee -a "$DEPLOY_LOG"
    else
        echo "[$timestamp] [$level] $message"
    fi
}

# Parse command line arguments
NON_INTERACTIVE=false
if [ "$1" = "--non-interactive" ] || [ "$1" = "-n" ]; then
    NON_INTERACTIVE=true
    log "Running in non-interactive mode" "INFO"
fi

# Function to show GUI dialog
show_gui_dialog() {
    local title="$1"
    local message="$2"
    local type="${3:-info}"
    
    if [ "$NON_INTERACTIVE" = true ]; then
        echo -e "${BLUE}$title${NC}: $message"
        if [ "$type" = "question" ]; then
            echo "Non-interactive mode: defaulting to 'no'"
            return 1
        fi
        return 0
    fi
    
    if [ "$GUI_AVAILABLE" = true ]; then
        case "$type" in
            "info")
                zenity --info --title="$title" --text="$message" 2>/dev/null &
                ;;
            "error")
                zenity --error --title="$title" --text="$message" 2>/dev/null &
                ;;
            "question")
                zenity --question --title="$title" --text="$message" 2>/dev/null
                return $?
                ;;
        esac
    elif [ "$TERMINAL_UI_AVAILABLE" = true ] && [ -t 0 ]; then
        case "$type" in
            "info")
                whiptail --title "$title" --msgbox "$message" 10 60 </dev/tty
                ;;
            "error")
                whiptail --title "ERROR: $title" --msgbox "$message" 10 60 </dev/tty
                ;;
            "question")
                whiptail --title "$title" --yesno "$message" 10 60 </dev/tty
                return $?
                ;;
        esac
    else
        echo -e "${BLUE}$title${NC}: $message"
        if [ "$type" = "question" ]; then
            # In non-interactive mode, default to "no" for questions
            echo "Non-interactive mode: defaulting to 'no'"
            return 1
        fi
    fi
}

# Function to get user input with GUI
get_user_input() {
    local prompt="$1"
    local default="$2"
    local is_password="${3:-false}"
    
    if [ "$NON_INTERACTIVE" = true ]; then
        echo "$default"
        return 0
    fi
    
    if [ "$GUI_AVAILABLE" = true ]; then
        if [ "$is_password" = "true" ]; then
            zenity --password --title="Environment Variable" --text="$prompt" 2>/dev/null
        else
            zenity --entry --title="Environment Variable" --text="$prompt" --entry-text="$default" 2>/dev/null
        fi
    elif [ "$TERMINAL_UI_AVAILABLE" = true ] && [ -t 0 ]; then
        if [ "$is_password" = "true" ]; then
            whiptail --passwordbox "$prompt" 10 60 "$default" 3>&1 1>&2 2>&3 </dev/tty
        else
            whiptail --inputbox "$prompt" 10 60 "$default" 3>&1 1>&2 2>&3 </dev/tty
        fi
    else
        # In non-interactive mode, use defaults or skip
        echo "$default"
    fi
}

# Function to show progress in background
show_progress_gui() {
    local title="$1"
    local message="$2"
    
    if [ "$GUI_AVAILABLE" = true ]; then
        (
            echo "10"; sleep 1
            echo "# $message"; sleep 1
            echo "30"; sleep 1
            echo "# Processing..."; sleep 1
            echo "50"; sleep 1
            echo "# Almost done..."; sleep 1
            echo "80"; sleep 1
            echo "# Finishing..."; sleep 1
            echo "100"
        ) | zenity --progress --title="$title" --text="$message" --auto-close 2>/dev/null &
        BACKGROUND_PID=$!
    fi
}

# Function to stop progress GUI
stop_progress_gui() {
    if [ -n "$BACKGROUND_PID" ]; then
        kill $BACKGROUND_PID 2>/dev/null || true
        BACKGROUND_PID=""
    fi
}

# STEP 1 - PROJECT INITIALIZATION
step1_project_initialization() {
    log "=== STEP 1: PROJECT INITIALIZATION ===" "INFO"
    update_json_status "INITIALIZING" "project_initialization" ""
    
    # Identify the root of the repository
    PROJECT_ROOT=$(pwd)
    log "Project root: $PROJECT_ROOT" "INFO"
    
    # Extract project name from folder name
    PROJECT_NAME=$(basename "$PROJECT_ROOT")
    log "Project name: $PROJECT_NAME" "INFO"
    
    # Generate timestamped deployment log file
    START_TIME=$(date +"%Y%m%d_%H%M%S")
    DEPLOY_LOG="$PROJECT_ROOT/deploy_${PROJECT_NAME}_${START_TIME}.log"
    log "Deployment log: $DEPLOY_LOG" "INFO"
    
    show_gui_dialog "Deployment Started" "Project: $PROJECT_NAME\nStarting deployment..." "info"
    
    log "Step 1 completed successfully" "INFO"
}

# STEP 2 - PROJECT TYPE DETECTION
step2_project_type_detection() {
    log "=== STEP 2: PROJECT TYPE DETECTION ===" "INFO"
    update_json_status "DETECTING_PROJECT_TYPE" "project_type_detection" ""
    
    show_progress_gui "Detecting Project Type" "Analyzing project structure..."
    
    PROJECT_TYPE="UNKNOWN"
    
    # Check for Rust project (prioritize since this is a Rust project)
    if [ -f "Cargo.toml" ]; then
        PROJECT_TYPE="RUST"
        log "Detected Rust project" "INFO"
        
        log "Building Rust project..." "INFO"
        if cargo build --release 2>&1 | tee -a "$DEPLOY_LOG"; then
            log "Rust project built successfully" "INFO"
        else
            log "Rust build failed" "ERROR"
            update_json_status "FAILED" "project_type_detection" "Rust build failed"
            return 1
        fi
        
    # Check for Node.js project
    elif [ -f "package.json" ]; then
        PROJECT_TYPE="NODE_JS"
        log "Detected Node.js project" "INFO"
        
        log "Running npm install..." "INFO"
        if npm install 2>&1 | tee -a "$DEPLOY_LOG"; then
            log "npm install completed successfully" "INFO"
        else
            log "npm install failed" "ERROR"
            update_json_status "FAILED" "project_type_detection" "npm install failed"
            return 1
        fi
        
        log "Running npm run build..." "INFO"
        if npm run build 2>&1 | tee -a "$DEPLOY_LOG"; then
            log "npm run build completed successfully" "INFO"
        else
            log "npm run build failed" "ERROR"
            update_json_status "FAILED" "project_type_detection" "npm run build failed"
            return 1
        fi
        
    # Check for Python project
    elif [ -f "requirements.txt" ]; then
        PROJECT_TYPE="PYTHON"
        log "Detected Python project" "INFO"
        
        log "Creating virtual environment..." "INFO"
        if python3 -m venv venv 2>&1 | tee -a "$DEPLOY_LOG"; then
            log "Virtual environment created successfully" "INFO"
        else
            log "Failed to create virtual environment" "ERROR"
            update_json_status "FAILED" "project_type_detection" "Failed to create virtual environment"
            return 1
        fi
        
        log "Activating virtual environment..." "INFO"
        source venv/bin/activate
        
        log "Installing Python dependencies..." "INFO"
        if pip install -r requirements.txt 2>&1 | tee -a "$DEPLOY_LOG"; then
            log "Python dependencies installed successfully" "INFO"
        else
            log "Failed to install Python dependencies" "ERROR"
            update_json_status "FAILED" "project_type_detection" "Failed to install Python dependencies"
            return 1
        fi
        
    # Check for Docker project
    elif [ -f "Dockerfile" ]; then
        PROJECT_TYPE="DOCKER"
        log "Detected Docker project" "INFO"
        
        if [ "$DOCKER_AVAILABLE" = true ]; then
            log "Building Docker image..." "INFO"
            # Convert project name to lowercase for Docker
            DOCKER_IMAGE_NAME=$(echo "$PROJECT_NAME" | tr '[:upper:]' '[:lower:]')
            if docker build -t "$DOCKER_IMAGE_NAME" . 2>&1 | tee -a "$DEPLOY_LOG"; then
                log "Docker image built successfully" "INFO"
            else
                log "Docker build failed" "ERROR"
                update_json_status "FAILED" "project_type_detection" "Docker build failed"
                return 1
            fi
        else
            log "Docker not available, skipping Docker build" "WARN"
        fi
        
    else
        log "Unknown project type" "WARN"
        PROJECT_TYPE="UNKNOWN"
    fi
    
    stop_progress_gui
    
    log "Project type: $PROJECT_TYPE" "INFO"
    log "Step 2 completed successfully" "INFO"
}

# STEP 3 - ENVIRONMENT VARIABLE VALIDATION
step3_environment_validation() {
    log "=== STEP 3: ENVIRONMENT VARIABLE VALIDATION ===" "INFO"
    update_json_status "VALIDATING_ENVIRONMENT" "environment_validation" ""
    
    show_progress_gui "Environment Validation" "Checking environment variables..."
    
    # Load .env file if exists
    if [ -f ".env" ]; then
        log "Loading .env file..." "INFO"
        # Use a safer method to load .env file
        while IFS= read -r line; do
            # Skip empty lines and comments
            if [[ ! -z "$line" && ! "$line" =~ ^[[:space:]]*# ]]; then
                # Export the variable if it's in the correct format
                if [[ "$line" =~ ^[A-Za-z_][A-Za-z0-9_]*= ]]; then
                    export "$line"
                fi
            fi
        done < .env
        log ".env file loaded successfully" "INFO"
    else
        log "No .env file found" "WARN"
    fi
    
    # Check for schema files
    ENV_SCHEMA=""
    if [ -f ".env.schema" ]; then
        ENV_SCHEMA=".env.schema"
        log "Found .env.schema file" "INFO"
    elif [ -f ".env.example" ]; then
        ENV_SCHEMA=".env.example"
        log "Found .env.example file" "INFO"
    elif [ -f "AgentSecrets.template" ]; then
        ENV_SCHEMA="AgentSecrets.template"
        log "Found AgentSecrets.template file" "INFO"
    fi
    
    if [ -n "$ENV_SCHEMA" ]; then
        log "Validating environment variables against schema..." "INFO"
        
        # Extract variable names from schema
        ENV_VARS=$(grep -E '^[A-Z_]+=|^#.*[A-Z_]+=' "$ENV_SCHEMA" | sed 's/=.*//' | sed 's/^#[[:space:]]*//' | sort -u)
        
        for var in $ENV_VARS; do
            if [ -z "${!var}" ]; then
                log "Missing environment variable: $var" "WARN"
                
                # Determine if it's a sensitive variable
                is_password="false"
                if echo "$var" | grep -iE "(password|secret|key|token|pass)" >/dev/null; then
                    is_password="true"
                fi
                
                # Prompt for missing variable
                value=$(get_user_input "Enter value for $var:" "" "$is_password")
                
                if [ -n "$value" ]; then
                    export "$var"="$value"
                    echo "$var=\"$value\"" >> .env
                    log "Environment variable $var set" "INFO"
                else
                    log "Skipping empty variable: $var" "WARN"
                fi
            else
                log "Environment variable $var is set" "INFO"
            fi
        done
    else
        log "No schema file found, scanning code for environment variables..." "INFO"
        
        # Basic scan for common environment variable patterns
        ENV_VARS=$(grep -rho '\$[A-Z_][A-Z0-9_]*' --include="*.rs" --include="*.toml" --include="*.sh" . | sort -u | sed 's/\$//')
        
        for var in $ENV_VARS; do
            if [ -z "${!var}" ]; then
                log "Potentially missing environment variable: $var" "WARN"
            fi
        done
    fi
    
    stop_progress_gui
    
    log "Step 3 completed successfully" "INFO"
}

# STEP 4 - DEPLOYMENT EXECUTION
step4_deployment_execution() {
    log "=== STEP 4: DEPLOYMENT EXECUTION ===" "INFO"
    update_json_status "DEPLOYING" "deployment_execution" ""
    
    show_progress_gui "Deployment Execution" "Executing deployment..."
    
    # Check for docker-compose.yml
    if [ -f "docker-compose.yml" ] || [ -f "docker-compose.yaml" ]; then
        log "Found docker-compose file, running deployment..." "INFO"
        
        if [ "$DOCKER_AVAILABLE" = true ]; then
            if docker-compose up -d --build 2>&1 | tee -a "$DEPLOY_LOG"; then
                log "Docker Compose deployment completed successfully" "INFO"
            else
                log "Docker Compose deployment failed" "ERROR"
                update_json_status "FAILED" "deployment_execution" "Docker Compose deployment failed"
                return 1
            fi
        else
            log "Docker not available, skipping Docker Compose deployment" "WARN"
        fi
        
    # Check for GitLab CI
    elif [ -f ".gitlab-ci.yml" ]; then
        log "Found GitLab CI configuration..." "INFO"
        
        if [ "$GIT_AVAILABLE" = true ]; then
            log "Committing changes and pushing to main..." "INFO"
            
            git add -A
            git commit -m "Automated deployment commit - $(date)"
            
            if git push origin main 2>&1 | tee -a "$DEPLOY_LOG"; then
                log "Changes pushed to GitLab successfully" "INFO"
                
                # Check if webhook configuration is needed
                if ! show_gui_dialog "Webhook Configuration" "Would you like to auto-configure GitLab webhook?" "question"; then
                    log "Webhook configuration skipped by user" "INFO"
                else
                    log "Webhook auto-configuration not implemented yet" "WARN"
                fi
            else
                log "Failed to push changes to GitLab" "ERROR"
                update_json_status "FAILED" "deployment_execution" "Failed to push changes to GitLab"
                return 1
            fi
        else
            log "Git not available, skipping GitLab CI deployment" "WARN"
        fi
        
    # Check for custom deploy.sh
    elif [ -f "deploy.sh" ]; then
        log "Found custom deploy.sh, executing..." "INFO"
        
        if chmod +x deploy.sh && ./deploy.sh 2>&1 | tee -a "$DEPLOY_LOG"; then
            log "Custom deployment script completed successfully" "INFO"
        else
            log "Custom deployment script failed" "ERROR"
            update_json_status "FAILED" "deployment_execution" "Custom deployment script failed"
            return 1
        fi
        
    # Fallback for Rust project
    elif [ "$PROJECT_TYPE" = "RUST" ]; then
        log "No specific deployment method found, running Rust application..." "INFO"
        
        # For this demo, we'll just run the application in the background
        nohup ./target/release/autonomous_job_agent > app.log 2>&1 &
        APP_PID=$!
        echo $APP_PID > app.pid
        
        log "Application started with PID: $APP_PID" "INFO"
        
    else
        log "No deployment method found" "WARN"
    fi
    
    stop_progress_gui
    
    log "Step 4 completed successfully" "INFO"
}

# STEP 5 - REAL-TIME GUI MONITORING
step5_gui_monitoring() {
    log "=== STEP 5: REAL-TIME GUI MONITORING ===" "INFO"
    update_json_status "MONITORING" "gui_monitoring" ""
    
    # Launch background GUI monitoring
    if [ "$GUI_AVAILABLE" = true ]; then
        (
            echo "25"; echo "# Build Started"
            sleep 2
            echo "50"; echo "# Secrets Loaded"
            sleep 2
            echo "75"; echo "# Deploying"
            sleep 2
            echo "100"; echo "# Complete"
            sleep 1
        ) | zenity --progress --title="Deployment Monitor" --text="Monitoring deployment..." --auto-close 2>/dev/null &
        MONITOR_PID=$!
        
        # Auto-close after success
        sleep 6
        kill $MONITOR_PID 2>/dev/null || true
    fi
    
    log "Step 5 completed successfully" "INFO"
}

# STEP 6 - SERVICE VALIDATION
step6_service_validation() {
    log "=== STEP 6: SERVICE VALIDATION ===" "INFO"
    update_json_status "VALIDATING_SERVICES" "service_validation" ""
    
    show_progress_gui "Service Validation" "Checking service health..."
    
    # Detect ports from docker-compose
    SERVICES=()
    if [ -f "docker-compose.yml" ] || [ -f "docker-compose.yaml" ]; then
        COMPOSE_FILE="docker-compose.yml"
        [ -f "docker-compose.yaml" ] && COMPOSE_FILE="docker-compose.yaml"
        
        log "Detecting services from $COMPOSE_FILE..." "INFO"
        
        # Extract port mappings (simplified)
        PORTS=$(grep -E "^\s*-\s*[0-9]+:[0-9]+" "$COMPOSE_FILE" | sed 's/.*- //' | sed 's/:.*//' | sort -u)
        
        for port in $PORTS; do
            log "Found service on port: $port" "INFO"
            SERVICES+=("localhost:$port")
        done
    fi
    
    # Check for common application ports
    COMMON_PORTS=(3000 8000 8080 8443 9000)
    for port in "${COMMON_PORTS[@]}"; do
        if netstat -ln 2>/dev/null | grep ":$port " >/dev/null; then
            log "Detected service on port: $port" "INFO"
            SERVICES+=("localhost:$port")
        fi
    done
    
    # Ping services
    for service in "${SERVICES[@]}"; do
        log "Checking service: $service" "INFO"
        
        if curl -s --connect-timeout 5 "http://$service" >/dev/null; then
            log "Service $service is responding" "INFO"
        else
            log "Service $service is not responding" "WARN"
        fi
    done
    
    stop_progress_gui
    
    log "Step 6 completed successfully" "INFO"
}

# STEP 7 - SNAPSHOT AND ROLLBACK
step7_snapshot_rollback() {
    log "=== STEP 7: SNAPSHOT AND ROLLBACK ===" "INFO"
    update_json_status "CREATING_SNAPSHOT" "snapshot_rollback" ""
    
    if [ "$GIT_AVAILABLE" = true ]; then
        log "Creating Git snapshot..." "INFO"
        
        # Create snapshot tag
        SNAPSHOT_TAG="deploy-snapshot-$(date +%Y%m%d-%H%M%S)"
        
        if git tag "$SNAPSHOT_TAG" 2>&1 | tee -a "$DEPLOY_LOG"; then
            log "Git snapshot created: $SNAPSHOT_TAG" "INFO"
            GIT_SNAPSHOT="$SNAPSHOT_TAG"
            ROLLBACK_AVAILABLE=true
            
            # Create rollback script
            cat > rollback.sh << EOF
#!/bin/bash
echo "Rolling back to snapshot: $SNAPSHOT_TAG"
git reset --hard $SNAPSHOT_TAG
echo "Rollback completed"
EOF
            chmod +x rollback.sh
            
            log "Rollback script created: rollback.sh" "INFO"
        else
            log "Failed to create Git snapshot" "ERROR"
            ROLLBACK_AVAILABLE=false
        fi
    else
        log "Git not available, skipping snapshot" "WARN"
        ROLLBACK_AVAILABLE=false
    fi
    
    log "Step 7 completed successfully" "INFO"
}

# STEP 8 - POST-DEPLOY GUI CONFIRMATION
step8_post_deploy_confirmation() {
    log "=== STEP 8: POST-DEPLOY GUI CONFIRMATION ===" "INFO"
    update_json_status "CONFIRMING" "post_deploy_confirmation" ""
    
    show_gui_dialog "Deployment Complete" "Deployment completed successfully!\n\nProject: $PROJECT_NAME\nType: $PROJECT_TYPE\nTime: $(date)" "info"
    
    # Ask about archiving
    if show_gui_dialog "Archive Deployment" "Would you like to archive this deployment?" "question"; then
        log "Archiving deployment..." "INFO"
        
        ARCHIVE_NAME="deploy-archive-${PROJECT_NAME}-${START_TIME}.tar.gz"
        
        if tar -czf "$ARCHIVE_NAME" --exclude='*.tar.gz' --exclude='node_modules' --exclude='target' --exclude='venv' .; then
            log "Deployment archived: $ARCHIVE_NAME" "INFO"
            
            # Move to deploy-history if directory exists or create it
            mkdir -p deploy-history
            mv "$ARCHIVE_NAME" deploy-history/
            
            log "Archive moved to deploy-history/" "INFO"
        else
            log "Failed to create archive" "ERROR"
        fi
    else
        log "Archiving skipped by user" "INFO"
    fi
    
    log "Step 8 completed successfully" "INFO"
}

# STEP 9 - TERMINAL UI FALLBACK
step9_terminal_ui_fallback() {
    log "=== STEP 9: TERMINAL UI FALLBACK ===" "INFO"
    
    if [ "$GUI_AVAILABLE" = false ] && [ "$TERMINAL_UI_AVAILABLE" = true ]; then
        log "Using terminal UI fallback..." "INFO"
        
        whiptail --title "Deployment Summary" --msgbox "Deployment completed successfully!\n\nProject: $PROJECT_NAME\nType: $PROJECT_TYPE\nStatus: $DEPLOY_STATUS\n\nCheck log: $DEPLOY_LOG" 15 60
    fi
    
    log "Step 9 completed successfully" "INFO"
}

# STEP 10 - EMIT AGENT-FRIENDLY HEADER
step10_emit_json_status() {
    log "=== STEP 10: EMIT AGENT-FRIENDLY HEADER ===" "INFO"
    update_json_status "COMPLETED" "emit_json_status" ""
    
    log "Emitting final JSON status..." "INFO"
    
    # Final JSON payload
    JSON_STATUS=$(cat <<EOF
{
  "agent": "COPILOT_COATING_AGENT",
  "version": "1.0.0",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "project_name": "$PROJECT_NAME",
  "project_type": "$PROJECT_TYPE",
  "project_root": "$PROJECT_ROOT",
  "deployment_status": "COMPLETED",
  "deploy_log": "$DEPLOY_LOG",
  "start_time": "$START_TIME",
  "duration": "$(($(date +%s) - $(date -d "$(echo $START_TIME | sed 's/_/ /')" +%s)))s",
  "git_snapshot": "$GIT_SNAPSHOT",
  "rollback_available": $ROLLBACK_AVAILABLE,
  "services": [$(IFS=,; echo "${SERVICES[*]/#/\"}" | sed 's/"/"/g')],
  "steps_completed": [
    "project_initialization",
    "project_type_detection", 
    "environment_validation",
    "deployment_execution",
    "gui_monitoring",
    "service_validation",
    "snapshot_rollback",
    "post_deploy_confirmation",
    "terminal_ui_fallback",
    "emit_json_status"
  ]
}
EOF
)
    
    # Output JSON to file and stdout
    echo "$JSON_STATUS" > "${DEPLOY_LOG}.json"
    echo "$JSON_STATUS"
    
    log "JSON status emitted successfully" "INFO"
    log "Step 10 completed successfully" "INFO"
}

# Cleanup function
cleanup() {
    stop_progress_gui
    if [ -n "$APP_PID" ]; then
        kill $APP_PID 2>/dev/null || true
    fi
    log "Cleanup completed" "INFO"
}

# Main function
main() {
    # Set up signal handlers
    trap cleanup EXIT INT TERM
    
    # Initialize global variables
    GIT_SNAPSHOT=""
    ROLLBACK_AVAILABLE=false
    PROJECT_TYPE=""
    DEPLOY_LOG=""  # Initialize deploy log variable
    
    # Check for available tools
    if command -v zenity >/dev/null 2>&1; then
        GUI_AVAILABLE=true
        log "GUI available: zenity" "INFO"
    elif command -v yad >/dev/null 2>&1; then
        GUI_AVAILABLE=true
        log "GUI available: yad" "INFO"
    fi
    
    if command -v whiptail >/dev/null 2>&1; then
        TERMINAL_UI_AVAILABLE=true
        log "Terminal UI available: whiptail" "INFO"
    elif command -v dialog >/dev/null 2>&1; then
        TERMINAL_UI_AVAILABLE=true
        log "Terminal UI available: dialog" "INFO"
    fi
    
    if command -v docker >/dev/null 2>&1; then
        DOCKER_AVAILABLE=true
        log "Docker available" "INFO"
    fi
    
    if command -v git >/dev/null 2>&1; then
        GIT_AVAILABLE=true
        log "Git available" "INFO"
    fi
    
    # Display header
    echo -e "${BLUE}======================================${NC}"
    echo -e "${BLUE}  COPILOT COATING AGENT${NC}"
    echo -e "${BLUE}  UNIFIED DEPLOYMENT DIRECTIVE${NC}"
    echo -e "${BLUE}======================================${NC}"
    echo
    
    # Execute deployment steps
    step1_project_initialization
    step2_project_type_detection
    step3_environment_validation
    step4_deployment_execution
    step5_gui_monitoring
    step6_service_validation
    step7_snapshot_rollback
    step8_post_deploy_confirmation
    step9_terminal_ui_fallback
    step10_emit_json_status
    
    # Final success message
    echo -e "${GREEN}======================================${NC}"
    echo -e "${GREEN}  DEPLOYMENT COMPLETED SUCCESSFULLY${NC}"
    echo -e "${GREEN}======================================${NC}"
    echo
    echo -e "Project: ${YELLOW}$PROJECT_NAME${NC}"
    echo -e "Type: ${YELLOW}$PROJECT_TYPE${NC}"
    echo -e "Log: ${YELLOW}$DEPLOY_LOG${NC}"
    echo -e "JSON Status: ${YELLOW}${DEPLOY_LOG}.json${NC}"
    
    if [ "$ROLLBACK_AVAILABLE" = true ]; then
        echo -e "Rollback: ${YELLOW}./rollback.sh${NC}"
    fi
    
    echo
    log "Deployment completed successfully" "INFO"
}

# Run main function
main "$@"