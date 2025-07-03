# Autonomous Job Agent - Build and Deployment Guide

This document provides instructions for building and deploying the Autonomous Job Agent application.

## Prerequisites

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- Git (optional, for version control)

## Configuration

Before building or deploying the application, ensure that the `AgentConfig.toml` file is properly configured with your settings:

- Personal information (name, email, location)
- Job search parameters (keywords, skills, industries, etc.)
- Resume and cover letter paths
- Search engine reconnaissance configuration

## Building the Application

### Debug Build

To build the application in debug mode (with debugging symbols and less optimization):

```bash
cargo build
```

The debug executable will be located at `target/debug/autonomous_job_agent.exe` (Windows) or `target/debug/autonomous_job_agent` (Linux/macOS).

### Release Build

For deployment, it's recommended to build the application in release mode, which includes optimizations for better performance:

```bash
cargo build --release
```

The release executable will be located at `target/release/autonomous_job_agent.exe` (Windows) or `target/release/autonomous_job_agent` (Linux/macOS).

## Running the Application

### Running with Cargo

You can run the application directly using Cargo:

```bash
# Debug mode
cargo run

# Release mode
cargo run --release
```

### Running the Executable Directly

You can also run the compiled executable directly:

```bash
# Windows (Debug)
.\target\debug\autonomous_job_agent.exe

# Windows (Release)
.\target\release\autonomous_job_agent.exe

# Linux/macOS (Debug)
./target/debug/autonomous_job_agent

# Linux/macOS (Release)
./target/release/autonomous_job_agent
```

## Deployment

### Local Deployment

For local deployment, simply copy the release executable and the `AgentConfig.toml` file to the desired location. Ensure both files are in the same directory.

### Scheduled Execution

You can set up scheduled execution using your operating system's task scheduler:

#### Windows (Task Scheduler)

1. Open Task Scheduler
2. Create a new task
3. Set the trigger (when to run)
4. Set the action to start the program and provide the path to the executable
5. Set any additional conditions or settings as needed

#### Linux/macOS (cron)

1. Open the crontab file: `crontab -e`
2. Add a line to schedule the execution, for example:
   ```
   0 9 * * * cd /path/to/application && ./autonomous_job_agent
   ```
   (This runs the application daily at 9 AM)

## Logging

The application uses the `env_logger` crate for logging. By default, it logs at the "info" level. You can control the log level using the `RUST_LOG` environment variable:

```bash
# Windows
set RUST_LOG=debug
.\target\release\autonomous_job_agent.exe

# Linux/macOS
RUST_LOG=debug ./target/release/autonomous_job_agent
```

Log levels from most to least verbose: trace, debug, info, warn, error.

## Troubleshooting

If you encounter issues:

1. Check the log output for error messages
2. Verify that the `AgentConfig.toml` file is properly formatted and contains all required fields
3. Ensure that the paths to the resume and cover letter files are correct
4. Check that you have internet connectivity for the job search functionality

## Updating the Application

To update the application:

1. Pull the latest code (if using Git)
2. Rebuild the application using the instructions above
3. Replace the existing executable with the new one
