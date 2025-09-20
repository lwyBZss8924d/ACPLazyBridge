#!/bin/bash
# PreToolUse hook to validate SDD compliance for critical operations
# Ensures constitutional requirements are met before executing tools

# Read JSON input from stdin
INPUT=$(cat)

# Validate JSON input
if ! echo "$INPUT" | jq empty 2>/dev/null; then
    echo "Error: Invalid JSON input" >&2
    exit 1
fi

# Extract tool name and input
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name')
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // .tool_input.file_path // ""')
CONTENT=$(echo "$INPUT" | jq -r '.tool_input.content // .tool_input.new_string // ""')

# Initialize issues array
ISSUES=()

# Function to add issue
add_issue() {
    ISSUES+=("$1")
}

# Check if this is an SDD-critical operation
if [[ "$TOOL_NAME" == "Write" || "$TOOL_NAME" == "Edit" || "$TOOL_NAME" == "MultiEdit" ]]; then

    # Check if operating on SDD artifacts
    if [[ "$FILE_PATH" == *"specs/"* ]]; then

        # Validate specs directory structure (NNN-slug format)
        if ! echo "$FILE_PATH" | grep -qE "specs/[0-9]{3}-[a-z-]+/"; then
            add_issue "SDD specs must follow naming convention: specs/NNN-slug/"
        fi

        # Check for required metadata in spec/plan/task files
        if [[ "$FILE_PATH" == *"spec.md" || "$FILE_PATH" == *"plan.md" || "$FILE_PATH" == *"tasks.md" ]]; then

            # Check for constitutional version
            if ! echo "$CONTENT" | grep -q "constitution:"; then
                add_issue "${FILE_PATH} must include constitutional version in YAML metadata"
            fi

            # Check for issue_uri in spec.md
            if [[ "$FILE_PATH" == *"spec.md" ]] && ! echo "$CONTENT" | grep -q "issue_uri:"; then
                add_issue "spec.md must include issue_uri in metadata"
            fi
        fi
    fi

    # Check Test-First principle for implementation files
    if [[ "$FILE_PATH" == *"src/"* || "$FILE_PATH" == *"lib/"* ]] && \
       [[ "$FILE_PATH" != *"test"* ]]; then
        add_issue "WARNING: Tests must be written BEFORE implementation (RED → GREEN → REFACTOR)"
    fi

    # Check for stdout misuse in Rust code
    if [[ "$FILE_PATH" == *.rs ]] && echo "$CONTENT" | grep -qE "(println!|print!)"; then
        # Allow println! in tests
        if [[ "$FILE_PATH" != *"test"* ]] && [[ "$FILE_PATH" != *"tests/"* ]]; then
            add_issue "stdout is reserved for protocol/JSONL only; use stderr for logs (eprintln!)"
        fi
    fi
fi

# If there are issues, return them as validation feedback
if [ ${#ISSUES[@]} -gt 0 ]; then
    # Build issues string
    ISSUES_TEXT="SDD Compliance Check:"
    for issue in "${ISSUES[@]}"; do
        ISSUES_TEXT="${ISSUES_TEXT}
• ${issue}"
    done

    # Output JSON with permission decision
    jq -n \
        --arg reason "$ISSUES_TEXT" \
        '{
            hookSpecificOutput: {
                hookEventName: "PreToolUse",
                permissionDecision: "ask",
                permissionDecisionReason: $reason
            }
        }'
else
    # No issues, allow the operation
    exit 0
fi