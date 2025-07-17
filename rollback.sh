#!/bin/bash
echo "Rolling back to snapshot: deploy-snapshot-20250717-010451"
git reset --hard deploy-snapshot-20250717-010451
echo "Rollback completed"
