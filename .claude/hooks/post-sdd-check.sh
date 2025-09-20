#!/bin/bash
# PostToolUse hook for SDD workflow validation
# Runs after spec/plan/task files are created or modified

# Read JSON input from stdin
INPUT=$(cat)

# Validate JSON input
if ! echo "$INPUT" | jq empty 2>/dev/null; then
    echo "Error: Invalid JSON input" >&2
    exit 1
fi

# Extract tool name and file path from response
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name')
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_response.filePath // .tool_response.file_path // ""')

# Only process Write/Edit operations on SDD files
if [[ "$TOOL_NAME" != "Write" && "$TOOL_NAME" != "Edit" && "$TOOL_NAME" != "MultiEdit" ]]; then
    exit 0
fi

# Check if operating on specs directory
if [[ "$FILE_PATH" != *"specs/"* ]]; then
    exit 0
fi

# Initialize results array
RESULTS=()

# Function to add result
add_result() {
    RESULTS+=("$1")
}

# Determine which validation scripts to run based on file
if [[ "$FILE_PATH" == *"spec.md" ]]; then
    # Run SDD document validation for spec files
    if [ -f "scripts/sdd/validate-sdd-docs.sh" ]; then
        OUTPUT=$(scripts/sdd/validate-sdd-docs.sh 2>&1)
        if [ $? -ne 0 ]; then
            add_result "SDD validation: Failed - $OUTPUT"
        fi
    fi

elif [[ "$FILE_PATH" == *"plan.md" ]]; then
    # Run SDD document validation and semantic checks for plan files
    if [ -f "scripts/sdd/validate-sdd-docs.sh" ]; then
        OUTPUT=$(scripts/sdd/validate-sdd-docs.sh 2>&1)
        if [ $? -ne 0 ]; then
            add_result "SDD validation: Failed"
        fi
    fi

    if [ -f "scripts/sdd/run_semantic_checks.sh" ]; then
        OUTPUT=$(scripts/sdd/run_semantic_checks.sh 2>&1)
        if [ $? -ne 0 ]; then
            add_result "Semantic checks: Failed"
        fi
    fi

elif [[ "$FILE_PATH" == *"tasks.md" ]]; then
    # Check task prerequisites
    if [ -f "scripts/sdd/check-task-prerequisites.sh" ]; then
        OUTPUT=$(scripts/sdd/check-task-prerequisites.sh 2>&1)
        if [ $? -ne 0 ]; then
            add_result "Task prerequisites: Missing"
        fi
    fi
fi

# If there are validation results, report them
if [ ${#RESULTS[@]} -gt 0 ]; then
    # Build results string
    RESULTS_TEXT="SDD Validation Results:"
    for result in "${RESULTS[@]}"; do
        RESULTS_TEXT="${RESULTS_TEXT}
${result}"
    done

    # Output JSON with additional context
    jq -n \
        --arg context "$RESULTS_TEXT" \
        '{
            hookSpecificOutput: {
                hookEventName: "PostToolUse",
                additionalContext: $context
            }
        }'
fi

exit 0