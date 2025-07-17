#!/bin/bash
echo "Rolling back to snapshot: deploy-snapshot-20250717-005532"
git reset --hard deploy-snapshot-20250717-005532
echo "Rollback completed"
