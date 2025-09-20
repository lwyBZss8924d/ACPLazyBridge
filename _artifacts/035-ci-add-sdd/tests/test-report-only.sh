#!/bin/bash
# Test report-only mode configuration
# This test MUST FAIL initially (TDD - RED phase)

set -euo pipefail

WORKFLOW_FILE=".github/workflows/ci.yml"
ERRORS=0

echo "=== Report-Only Mode Validation Test ==="
echo

# Function to check if a job has continue-on-error
check_report_only() {
    local job_name=$1
    echo -n "Checking $job_name for report-only mode... "

    # Extract job configuration
    if grep -A2 "$job_name:" "$WORKFLOW_FILE" | grep -q "continue-on-error: true"; then
        echo "✓ Configured as report-only"
        return 0
    else
        echo "✗ NOT configured as report-only"
        return 1
    fi
}

# Check ast-grep-scan job specifically
if ! check_report_only "ast-grep-scan"; then
    ((ERRORS++))
fi

# Verify job won't block PR merging
echo -n "Checking if ast-grep findings block PR... "
if grep -A10 "ast-grep-scan:" "$WORKFLOW_FILE" | grep -q "continue-on-error: true"; then
    echo "✓ Won't block (report-only)"
else
    echo "✗ WILL BLOCK (not report-only)"
    ((ERRORS++))
fi

# Check for proper SARIF upload even on failure
echo -n "Checking SARIF upload configuration... "
if grep -A20 "ast-grep-scan:" "$WORKFLOW_FILE" | grep -q "if: always()"; then
    echo "✓ Uploads even on failure"
else
    # Also accept if: success() || failure() pattern
    if grep -A20 "ast-grep-scan:" "$WORKFLOW_FILE" | grep -q "if: success() || failure()"; then
        echo "✓ Uploads even on failure"
    else
        echo "✗ May not upload on failure"
        ((ERRORS++))
    fi
fi

# Verify other jobs are NOT in report-only mode
echo -n "Checking other jobs remain enforced... "
OTHER_JOBS_ENFORCED=true
for job in "build-test" "sdd-checks" "quality-gates"; do
    if grep -A5 "$job:" "$WORKFLOW_FILE" 2>/dev/null | grep -q "continue-on-error: true"; then
        echo "✗ $job should NOT be report-only"
        OTHER_JOBS_ENFORCED=false
        ((ERRORS++))
        break
    fi
done
if [ "$OTHER_JOBS_ENFORCED" = true ]; then
    echo "✓ Other jobs properly enforced"
fi

# Check for documentation of report-only mode
echo -n "Checking for report-only documentation... "
if grep -q "# Report-only mode" "$WORKFLOW_FILE" || grep -q "# Non-blocking" "$WORKFLOW_FILE"; then
    echo "✓ Documented"
else
    echo "✗ NOT DOCUMENTED (expected to fail initially)"
    ((ERRORS++))
fi

echo
echo "=== Summary ==="
if [ $ERRORS -eq 0 ]; then
    echo "✓ All report-only validations passed"
    exit 0
else
    echo "✗ Found $ERRORS validation failures (this is expected in TDD RED phase)"
    exit 1
fi