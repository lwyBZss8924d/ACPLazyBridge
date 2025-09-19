#!/bin/bash
# Validate SARIF format conversion from ast-grep JSON
# This test MUST FAIL initially (TDD - RED phase)

set -euo pipefail

SARIF_SCRIPT="scripts/ci/json-to-sarif.jq"
ERRORS=0

echo "=== SARIF Format Validation Test ==="
echo

# Check if SARIF conversion script exists
echo -n "Checking for SARIF conversion script... "
if [ -f "$SARIF_SCRIPT" ]; then
    echo "✓ Found"
else
    echo "✗ NOT FOUND (expected to fail initially)"
    ((ERRORS++))
    exit 1
fi

# Create sample ast-grep JSON output for testing
SAMPLE_JSON=$(cat <<'EOF'
[
  {
    "rule_id": "rust-no-unwrap",
    "severity": "warning",
    "message": "Avoid using unwrap() which can panic",
    "file": "src/main.rs",
    "start_line": 10,
    "start_column": 5,
    "end_line": 10,
    "end_column": 15
  }
]
EOF
)

# Test SARIF conversion
echo -n "Testing SARIF conversion... "
SARIF_OUTPUT=$(echo "$SAMPLE_JSON" | jq -f "$SARIF_SCRIPT" 2>/dev/null || true)

if [ -z "$SARIF_OUTPUT" ]; then
    echo "✗ Conversion failed"
    ((ERRORS++))
else
    echo "✓ Conversion succeeded"
fi

# Validate SARIF schema compliance
echo -n "Checking SARIF schema version... "
SCHEMA_VERSION=$(echo "$SARIF_OUTPUT" | jq -r '.version' 2>/dev/null || echo "")
if [ "$SCHEMA_VERSION" = "2.1.0" ]; then
    echo "✓ Valid (2.1.0)"
else
    echo "✗ Invalid or missing version"
    ((ERRORS++))
fi

# Check for required SARIF fields
echo -n "Checking for tool information... "
TOOL_NAME=$(echo "$SARIF_OUTPUT" | jq -r '.runs[0].tool.driver.name' 2>/dev/null || echo "")
if [ "$TOOL_NAME" = "ast-grep" ]; then
    echo "✓ Valid"
else
    echo "✗ Invalid or missing tool name"
    ((ERRORS++))
fi

# Check for results array
echo -n "Checking for results array... "
RESULTS_COUNT=$(echo "$SARIF_OUTPUT" | jq '.runs[0].results | length' 2>/dev/null || echo "0")
if [ "$RESULTS_COUNT" -gt 0 ]; then
    echo "✓ Results present"
else
    echo "✗ No results found"
    ((ERRORS++))
fi

# Validate result structure
echo -n "Checking result structure... "
RULE_ID=$(echo "$SARIF_OUTPUT" | jq -r '.runs[0].results[0].ruleId' 2>/dev/null || echo "")
if [ "$RULE_ID" = "rust-no-unwrap" ]; then
    echo "✓ Valid"
else
    echo "✗ Invalid result structure"
    ((ERRORS++))
fi

# Check physical location
echo -n "Checking physical location mapping... "
FILE_URI=$(echo "$SARIF_OUTPUT" | jq -r '.runs[0].results[0].locations[0].physicalLocation.artifactLocation.uri' 2>/dev/null || echo "")
if [ "$FILE_URI" = "src/main.rs" ]; then
    echo "✓ Valid"
else
    echo "✗ Invalid location mapping"
    ((ERRORS++))
fi

echo
echo "=== Summary ==="
if [ $ERRORS -eq 0 ]; then
    echo "✓ All SARIF validations passed"
    exit 0
else
    echo "✗ Found $ERRORS validation failures (this is expected in TDD RED phase)"
    exit 1
fi