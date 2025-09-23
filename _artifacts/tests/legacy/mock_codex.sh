#!/bin/bash
# Mock Codex proto mode for testing streaming

# Read the prompt request
read -r line

# Output some mock streaming events
echo '{"type": "agent_message", "message": "Starting to process your request..."}'
sleep 0.1
echo '{"type": "agent_message_delta", "delta": "I am processing"}'
sleep 0.1
echo '{"type": "agent_message_delta", "delta": " your prompt"}'
sleep 0.1
echo '{"type": "agent_message_delta", "delta": " and streaming"}'
sleep 0.1
echo '{"type": "agent_message_delta", "delta": " the response."}'
sleep 0.1
echo '{"type": "tool_call", "id": "tool_123", "name": "test_tool", "arguments": {"foo": "bar"}}'
sleep 0.1
echo '{"type": "task_complete", "reason": "done"}'

# Keep alive briefly then exit
sleep 0.5