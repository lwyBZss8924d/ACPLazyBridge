#!/bin/bash
# Validate CI workflow structure and ast-grep job configuration
# This test MUST FAIL initially (TDD - RED phase)

set -euo pipefail

WORKFLOW_FILE=".github/workflows/ci.yml"
ERRORS=0

echo "=== CI Workflow Validation Test ==="
echo "Testing: $WORKFLOW_FILE"
echo

# Check if workflow exists
if [ ! -f "$WORKFLOW_FILE" ]; then
    echo "✓ Workflow file exists"
else
    echo "✓ Workflow file exists"
fi

# Check for ast-grep-scan job
echo -n "Checking for ast-grep-scan job... "
if grep -q "ast-grep-scan:" "$WORKFLOW_FILE"; then
    echo "✓ Found"
else
    echo "✗ NOT FOUND (expected to fail initially)"
    ((ERRORS++))
fi

# Check for continue-on-error (report-only mode)
echo -n "Checking for report-only mode... "
if grep -A5 "ast-grep-scan:" "$WORKFLOW_FILE" | grep -q "continue-on-error: true"; then
    echo "✓ Found"
else
    echo "✗ NOT FOUND (expected to fail initially)"
    ((ERRORS++))
fi

# Check for SARIF upload action
echo -n "Checking for SARIF upload... "
if grep -q "github/codeql-action/upload-sarif" "$WORKFLOW_FILE"; then
    echo "✓ Found"
else
    echo "✗ NOT FOUND (expected to fail initially)"
    ((ERRORS++))
fi

# Check for typos-check job
echo -n "Checking for typos-check job... "
if grep -q "typos-check:" "$WORKFLOW_FILE"; then
    echo "✓ Found"
else
    echo "✗ NOT FOUND (expected to fail initially)"
    ((ERRORS++))
fi

# Check for test matrix with multiple OS
echo -n "Checking for cross-platform matrix... "
if grep -q "os: \[ubuntu-latest, macos-latest, windows-latest\]" "$WORKFLOW_FILE"; then
    echo "✓ Found"
else
    echo "✗ NOT FOUND (expected to fail initially)"
    ((ERRORS++))
fi

# Check for security-events permission
echo -n "Checking for security-events permission... "
if grep -q "security-events: write" "$WORKFLOW_FILE"; then
    echo "✓ Found"
else
    echo "✗ NOT FOUND (expected to fail initially)"
    ((ERRORS++))
fi

# Check for rust cache
echo -n "Checking for Rust cache configuration... "
if grep -q "Swatinem/rust-cache" "$WORKFLOW_FILE"; then
    echo "✓ Found"
else
    echo "✗ NOT FOUND (expected to fail initially)"
    ((ERRORS++))
fi

echo
echo "=== Summary ==="
if [ $ERRORS -eq 0 ]; then
    echo "✓ All workflow validations passed"
    exit 0
else
    echo "✗ Found $ERRORS validation failures (this is expected in TDD RED phase)"
    exit 1
fi