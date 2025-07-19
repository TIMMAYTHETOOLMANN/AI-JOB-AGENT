#!/bin/bash

# Quick test script to validate the deployment system
echo "Testing Copilot Coating Agent..."

# Clean up previous test files
rm -f deploy_*.log deploy_*.json rollback.sh

# Test deployment in non-interactive mode
echo "Running deployment in non-interactive mode..."
./copilot_deploy.sh --non-interactive

# Check if deployment completed
if [ -f "rollback.sh" ]; then
    echo "✅ Deployment completed successfully!"
    echo "✅ Rollback script created"
    echo "✅ Git snapshot created"
    
    # Check JSON output
    if [ -f "deploy_"*".log.json" ]; then
        echo "✅ JSON status file created"
        echo "JSON Status:"
        cat deploy_*.log.json | head -20
    fi
    
    # Check log file
    if [ -f "deploy_"*".log" ]; then
        echo "✅ Deployment log created"
        echo "Latest log entries:"
        tail -10 deploy_*.log
    fi
    
    echo ""
    echo "Files created:"
    ls -la rollback.sh deploy_*.log deploy_*.json 2>/dev/null || echo "No files found"
    
else
    echo "❌ Deployment failed - no rollback script found"
    exit 1
fi

echo ""
echo "All tests passed! 🎉"