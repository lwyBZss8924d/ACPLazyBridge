#!/bin/bash

# Test script to verify session/update streaming with proper format
# This simulates a Codex proto process for testing

set -e

echo "Building codex-cli-acp..."
cd /Users/arthur/dev-space/acplb-worktrees/codex-proto-1
cargo build -p codex-cli-acp 2>/dev/null

echo "Creating test scenario with mock Codex output..."

# Create a mock Codex proto that outputs test events
cat > mock_codex_proto.sh << 'EOF'
#!/bin/bash
# Mock Codex proto that outputs test events

# Read and ignore input
while IFS= read -r line; do
    :
done

# Output some test events to stdout
echo '{"type":"agent_message","message":"Starting analysis..."}'
sleep 0.1
echo '{"type":"agent_message_delta","delta":"Here is "}'
sleep 0.1
echo '{"type":"agent_message_delta","delta":"the response."}'
sleep 0.1
echo '{"type":"tool_call","id":"tool_123","name":"read_file","arguments":{"path":"/test/file.txt"},"status":"pending"}'
sleep 0.1
echo '{"type":"tool_call","id":"tool_123","name":"read_file","arguments":{"path":"/test/file.txt"},"status":"completed"}'
sleep 0.1
echo '{"type":"task_complete","reason":"end_turn"}'
EOF

chmod +x mock_codex_proto.sh

# Create a JSONL input that uses mock Codex
cat > test_with_mock.jsonl << 'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}
{"jsonrpc":"2.0","id":2,"method":"session/new","params":{"workingDirectory":"/tmp/test"}}
EOF

# First, get the session ID
echo "Getting session ID..."
RESPONSE=$(timeout 2 cargo run -p codex-cli-acp --bin codex-cli-acp 2>/dev/null < test_with_mock.jsonl || true)
SESSION_ID=$(echo "$RESPONSE" | grep '"sessionId"' | head -1 | jq -r '.result.sessionId')

if [ -z "$SESSION_ID" ]; then
    echo "Failed to get session ID"
    exit 1
fi

echo "Session ID: $SESSION_ID"

# Now test with the actual session ID and mock Codex
cat > test_prompt.jsonl << EOF
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}
{"jsonrpc":"2.0","id":2,"method":"session/new","params":{"workingDirectory":"/tmp/test"}}
{"jsonrpc":"2.0","id":3,"method":"session/prompt","params":{"sessionId":"$SESSION_ID","prompt":"test prompt","includeWorkspaceContext":false,"codexCommand":"./mock_codex_proto.sh"}}
EOF

echo ""
echo "Running test with mock Codex output..."
timeout 5 cargo run -p codex-cli-acp --bin codex-cli-acp 2>/dev/null < test_prompt.jsonl > test_output.jsonl || true

echo "Output received:"
cat test_output.jsonl

echo ""
echo "Extracting session/update messages..."
grep '"method":"session/update"' test_output.jsonl > session_updates.jsonl || true

if [ ! -s session_updates.jsonl ]; then
    echo "Warning: No session/update messages found"
else
    echo "Found $(wc -l < session_updates.jsonl) session/update messages"
    echo ""
    echo "Validating structure..."
    
    while IFS= read -r line; do
        # Check for proper structure
        if echo "$line" | jq -e '.params.update.sessionUpdate' > /dev/null 2>&1; then
            UPDATE_TYPE=$(echo "$line" | jq -r '.params.update.sessionUpdate')
            echo "✓ Valid update of type: $UPDATE_TYPE"
            
            # Show content for agent messages
            if [ "$UPDATE_TYPE" = "agent_message_chunk" ]; then
                CONTENT=$(echo "$line" | jq -r '.params.update.content.text // .params.update.content')
                echo "  Content: $CONTENT"
            elif [ "$UPDATE_TYPE" = "tool_call" ]; then
                TOOL_ID=$(echo "$line" | jq -r '.params.update.toolCallId')
                TITLE=$(echo "$line" | jq -r '.params.update.title')
                STATUS=$(echo "$line" | jq -r '.params.update.status')
                echo "  Tool: $TITLE (ID: $TOOL_ID, Status: $STATUS)"
            fi
        else
            echo "✗ Invalid structure in message:"
            echo "$line" | jq .
            exit 1
        fi
    done < session_updates.jsonl
    
    echo ""
    echo "All session/update messages have correct format!"
fi

# Clean up
rm -f mock_codex_proto.sh test_with_mock.jsonl test_prompt.jsonl test_output.jsonl session_updates.jsonl

echo "Test completed!"