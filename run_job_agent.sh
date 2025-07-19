#!/bin/bash
# Generated run script for AI Job Agent

export RUST_LOG=info
export RUST_BACKTRACE=1

# Ensure directories exist
mkdir -p state logs temp

# Run the application
cd "$(dirname "$0")"

if [ "$1" = "--cli" ] || [ "$1" = "--headless" ]; then
    echo "Starting AI Job Agent in CLI mode..."
    ./target/release/autonomous_job_agent --cli 2>&1 | tee logs/job_agent_cli_$(date +%Y%m%d_%H%M%S).log
else
    echo "Starting AI Job Agent in GUI mode..."
    ./target/release/autonomous_job_agent 2>&1 | tee logs/job_agent_gui_$(date +%Y%m%d_%H%M%S).log
fi
