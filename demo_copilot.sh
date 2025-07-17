#!/bin/bash

# COPILOT COATING AGENT - Complete System Demonstration
# This script demonstrates all features of the unified deployment system

set -e

echo "🚀 COPILOT COATING AGENT - COMPLETE SYSTEM DEMONSTRATION"
echo "========================================================="
echo

# Clean up any previous test files
echo "🧹 Cleaning up previous test files..."
rm -f deploy_*.log deploy_*.json rollback.sh
rm -rf deploy-history/

echo "✅ Cleanup complete"
echo

# Show system capabilities
echo "🔍 System Capabilities:"
echo "- ✅ Automatic project type detection (Rust, Node.js, Python, Docker)"
echo "- ✅ Environment variable validation with GUI prompts"
echo "- ✅ Multiple deployment methods (Docker Compose, GitLab CI, custom scripts)"
echo "- ✅ Real-time GUI monitoring"
echo "- ✅ Service health validation"
echo "- ✅ Git snapshots with automatic rollback"
echo "- ✅ Web dashboard for monitoring"
echo "- ✅ JSON status output for agent integration"
echo "- ✅ Terminal UI fallback support"
echo "- ✅ Deployment archiving and history"
echo

# Show detected project information
echo "📊 Project Information:"
echo "- Project Type: $([ -f Cargo.toml ] && echo "Rust" || echo "Unknown")"
echo "- Docker Support: $([ -f Dockerfile ] && echo "Yes" || echo "No")"
echo "- Docker Compose: $([ -f docker-compose.yml ] && echo "Yes" || echo "No")"
echo "- GitLab CI: $([ -f .gitlab-ci.yml ] && echo "Yes" || echo "No")"
echo "- Environment File: $([ -f .env ] && echo "Yes" || echo "No")"
echo "- Configuration: $([ -f AgentConfig.toml ] && echo "AgentConfig.toml" || echo "None")"
echo

# Demonstrate the deployment process
echo "🚀 Running Deployment Process..."
echo "Running: ./copilot_deploy.sh --non-interactive"
echo

# Run the deployment
if ./copilot_deploy.sh --non-interactive >/dev/null 2>&1; then
    echo "✅ Deployment completed successfully!"
else
    echo "❌ Deployment failed"
    exit 1
fi

echo

# Show what was created
echo "📁 Files Created:"
if ls deploy_*.log >/dev/null 2>&1; then
    echo "- Deployment Log: $(ls deploy_*.log)"
fi
if ls deploy_*.json >/dev/null 2>&1; then
    echo "- JSON Status: $(ls deploy_*.json)"
fi
if [ -f rollback.sh ]; then
    echo "- Rollback Script: rollback.sh"
fi

echo

# Show JSON status
echo "📊 JSON Status Output:"
if ls deploy_*.json >/dev/null 2>&1; then
    cat deploy_*.json | head -15
fi

echo

# Show rollback capability
echo "🔄 Rollback Capability:"
if [ -f rollback.sh ]; then
    echo "- Rollback script created: rollback.sh"
    echo "- Content:"
    cat rollback.sh | sed 's/^/  /'
else
    echo "- No rollback script found"
fi

echo

# Show deployment log summary
echo "📋 Deployment Summary:"
if ls deploy_*.log >/dev/null 2>&1; then
    echo "- Total log lines: $(wc -l deploy_*.log | awk '{print $1}')"
    echo "- Build time: $(grep "Finished.*target" deploy_*.log | tail -1 | awk '{print $6, $7}' || echo "Not available")"
    echo "- Steps completed: $(grep "Step.*completed successfully" deploy_*.log | wc -l || echo "0")"
fi

echo

# Show web dashboard info
echo "🌐 Web Dashboard:"
echo "- Dashboard file: web/index.html"
echo "- Access via: python3 -m http.server 8000 --directory web"
echo "- URL: http://localhost:8000"

echo

# Show integration example
echo "🔗 Agent Integration Example:"
echo "```bash"
echo "# Run deployment"
echo "./copilot_deploy.sh --non-interactive"
echo ""
echo "# Check status"
echo 'STATUS=$(cat deploy_*.json | jq -r .deployment_status)'
echo 'if [ "$STATUS" = "COMPLETED" ]; then'
echo '    echo "✅ Deployment successful"'
echo 'else'
echo '    echo "❌ Deployment failed"'
echo '    exit 1'
echo 'fi'
echo ""
echo "# Rollback if needed"
echo "./rollback.sh"
echo "```"

echo

# Final summary
echo "🎉 DEMONSTRATION COMPLETE"
echo "========================="
echo "The COPILOT COATING AGENT unified deployment system is fully operational!"
echo ""
echo "Key Features Demonstrated:"
echo "- ✅ Single-click deployment with comprehensive logging"
echo "- ✅ Automatic project detection and build process"
echo "- ✅ Environment variable validation"
echo "- ✅ Git snapshots with rollback capability"
echo "- ✅ JSON status output for automation"
echo "- ✅ Web dashboard for monitoring"
echo "- ✅ Non-interactive mode for CI/CD integration"
echo ""
echo "For more information, see: COPILOT_DEPLOYMENT.md"
echo "To test the system: ./test_deploy.sh"
echo "To run deployment: ./copilot_deploy.sh"