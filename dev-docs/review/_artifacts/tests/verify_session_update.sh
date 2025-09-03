#!/bin/bash

# Test script to verify session/update message format
# Runs the ACP server and checks that session updates have the correct structure

set -e

echo "Building codex-cli-acp..."
cd /Users/arthur/dev-space/acplb-worktrees/codex-proto-1
cargo build -p codex-cli-acp 2>/dev/null

echo "Starting ACP server and sending test messages..."
RUST_LOG=debug timeout 5 cargo run -p codex-cli-acp 2>test_debug.log < dev-docs/review/_artifacts/tests/session_update_format.jsonl > test_output.jsonl || true

echo "Checking session/update message format..."
# Extract session/update messages
grep '"method":"session/update"' test_output.jsonl > session_updates.jsonl || true

if [ ! -s session_updates.jsonl ]; then
    echo "No session/update messages found in output"
    exit 1
fi

echo "Found session/update messages:"
cat session_updates.jsonl

echo ""
echo "Validating structure..."
# Check that updates have the correct nested structure with "update" field
while IFS= read -r line; do
    # Check for proper structure: params.update.sessionUpdate
    if echo "$line" | jq -e '.params.update.sessionUpdate' > /dev/null 2>&1; then
        echo "✓ Message has correct nested structure with 'update' field and 'sessionUpdate' discriminator"
        echo "  Type: $(echo "$line" | jq -r '.params.update.sessionUpdate')"
    else
        echo "✗ Message missing proper structure!"
        echo "  Line: $line"
        exit 1
    fi
done < session_updates.jsonl

echo ""
echo "All session/update messages have correct format!"

# Clean up
rm -f test_output.jsonl session_updates.jsonl test_debug.log

echo "Test passed!"