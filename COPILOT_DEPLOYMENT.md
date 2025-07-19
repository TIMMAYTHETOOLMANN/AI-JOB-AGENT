# COPILOT COATING AGENT - Unified Deployment System

## Overview

The Copilot Coating Agent is a comprehensive, single-click deployment system that automatically detects project type, validates environment variables, and provides streamlined deployment with rollback capabilities.

## Features

- **🔍 Automatic Project Detection**: Supports Rust, Node.js, Python, and Docker projects
- **🔧 Environment Variable Validation**: Validates and prompts for missing environment variables
- **📱 GUI/Terminal UI Support**: Uses zenity/yad for GUI or whiptail/dialog for terminal
- **🚀 Multiple Deployment Methods**: Docker Compose, GitLab CI, custom scripts
- **📊 Real-time Monitoring**: Background GUI monitoring with status updates
- **🔄 Git Snapshots & Rollback**: Automatic snapshots with one-click rollback
- **📋 JSON Status Output**: Agent-friendly JSON status for integration
- **💾 Deployment Archiving**: Optional compression and archiving of deployments

## Quick Start

### Basic Usage

```bash
# Interactive mode (with GUI prompts)
./copilot_deploy.sh

# Non-interactive mode (uses defaults)
./copilot_deploy.sh --non-interactive
```

### Test the System

```bash
# Run the test suite
./test_deploy.sh
```

## Deployment Process

The system follows a 10-step deployment process:

1. **Project Initialization** - Identify project root and generate logs
2. **Project Type Detection** - Detect and build project (Rust/Node.js/Python/Docker)
3. **Environment Variable Validation** - Load and validate environment variables
4. **Deployment Execution** - Execute deployment based on project type
5. **Real-time GUI Monitoring** - Show progress in background GUI
6. **Service Validation** - Check service health and connectivity
7. **Snapshot and Rollback** - Create Git snapshots for rollback
8. **Post-deploy Confirmation** - Show success and archive options
9. **Terminal UI Fallback** - Use terminal UI if GUI unavailable
10. **JSON Status Output** - Emit agent-friendly status JSON

## Project Type Support

### Rust Projects (Current)
- Detects `Cargo.toml`
- Runs `cargo build --release`
- Supports Docker deployment via `docker-compose.yml`

### Node.js Projects
- Detects `package.json`
- Runs `npm install` and `npm run build`

### Python Projects
- Detects `requirements.txt`
- Creates virtual environment and installs dependencies

### Docker Projects
- Detects `Dockerfile`
- Builds Docker image with proper naming

## Environment Variables

The system automatically validates environment variables against:
- `.env.schema` (if present)
- `.env.example` (if present)
- `AgentSecrets.template` (if present)

Missing variables are prompted via GUI or terminal interface.

## Deployment Methods

### Docker Compose
```bash
# If docker-compose.yml exists
docker-compose up -d --build
```

### GitLab CI
```bash
# If .gitlab-ci.yml exists
git add -A && git commit -m "Automated deployment" && git push origin main
```

### Custom Scripts
```bash
# If deploy.sh exists
./deploy.sh
```

## GUI Support

The system supports multiple UI modes:

- **GUI Mode**: Uses `zenity` or `yad` for graphical prompts
- **Terminal Mode**: Uses `whiptail` or `dialog` for text-based UI
- **Non-interactive Mode**: Uses defaults for automated deployment

## Rollback System

Automatic Git snapshots are created before deployment:

```bash
# View rollback script
cat rollback.sh

# Execute rollback
./rollback.sh
```

## JSON Status Output

The system outputs JSON status for agent integration:

```json
{
  "agent": "COPILOT_COATING_AGENT",
  "version": "1.0.0",
  "timestamp": "2025-07-17T00:55:32Z",
  "project_name": "AI-JOB-AGENT",
  "project_type": "RUST",
  "deployment_status": "COMPLETED",
  "git_snapshot": "deploy-snapshot-20250717-005532",
  "rollback_available": true
}
```

## Files Created

- `deploy_PROJECT_TIMESTAMP.log` - Deployment log
- `deploy_PROJECT_TIMESTAMP.log.json` - JSON status
- `rollback.sh` - Rollback script
- `deploy-archive-PROJECT-TIMESTAMP.tar.gz` - Optional archive

## Configuration

The system automatically detects configuration from:
- `AgentConfig.toml` (for Rust projects)
- `package.json` (for Node.js projects)
- `requirements.txt` (for Python projects)
- `docker-compose.yml` (for Docker projects)

## Dependencies

### Required
- `bash` - Shell environment
- `git` - Version control (for snapshots)

### Optional (Enhanced Features)
- `docker` - For Docker deployment
- `zenity` or `yad` - For GUI prompts
- `whiptail` or `dialog` - For terminal UI
- `curl` - For service validation
- `netstat` - For port detection

## Examples

### Successful Deployment
```bash
$ ./copilot_deploy.sh --non-interactive
======================================
  COPILOT COATING AGENT
  UNIFIED DEPLOYMENT DIRECTIVE
======================================

[2025-07-17 00:55:32] [INFO] === STEP 1: PROJECT INITIALIZATION ===
[2025-07-17 00:55:32] [INFO] Project root: /path/to/project
[2025-07-17 00:55:32] [INFO] Project name: AI-JOB-AGENT
[2025-07-17 00:55:32] [INFO] Detected Rust project
[2025-07-17 00:55:32] [INFO] Rust project built successfully
[2025-07-17 00:55:32] [INFO] Git snapshot created: deploy-snapshot-20250717-005532
[2025-07-17 00:55:32] [INFO] Deployment completed successfully

======================================
  DEPLOYMENT COMPLETED SUCCESSFULLY
======================================
```

### Rollback Example
```bash
$ ./rollback.sh
Rolling back to snapshot: deploy-snapshot-20250717-005532
HEAD is now at abc1234 Previous working state
Rollback completed
```

## Troubleshooting

### Common Issues

1. **Docker not found**: Install Docker or use `--non-interactive` mode
2. **GUI not available**: System will fallback to terminal UI
3. **Permission denied**: Ensure script is executable (`chmod +x copilot_deploy.sh`)
4. **Git not found**: Install Git for snapshot functionality

### Debug Mode

Add debug logging by setting environment variable:
```bash
export DEBUG=1
./copilot_deploy.sh
```

## Integration

The system is designed for integration with CI/CD pipelines and automation tools:

```bash
# CI/CD Integration
./copilot_deploy.sh --non-interactive
if [ $? -eq 0 ]; then
    echo "Deployment successful"
    # Parse JSON status
    cat deploy_*.log.json | jq '.deployment_status'
else
    echo "Deployment failed"
    exit 1
fi
```

## License

This deployment system is part of the AI Job Agent project and follows the same license terms.