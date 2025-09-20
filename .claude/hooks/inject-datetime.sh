#!/bin/bash
# UserPromptSubmit hook to inject current UTC date/time
# Eliminates manual date/time entry in prompts

# Read JSON input from stdin
INPUT=$(cat)

# Validate JSON input
if ! echo "$INPUT" | jq empty 2>/dev/null; then
    echo "Error: Invalid JSON input" >&2
    exit 1
fi

# Get current UTC date/time components
DATE_UTC=$(date -u +"%Y-%m-%d")
TIME_UTC=$(date -u +"%H:%M:%S")
ISO_UTC=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
DAY_OF_WEEK=$(date -u +"%A")

# Create the date context message
DATE_CONTEXT="⚠️ **UTC TIME:** ${ISO_UTC} (${DAY_OF_WEEK})

**Reference Mapping:**
• 'today'/'current' → ${DATE_UTC}
• 'latest'/'recent' → ≤ ${ISO_UTC}
• past → < ${DATE_UTC} | future → > ${DATE_UTC}

_Use this timestamp for any temporal references in user's request._"

# Output JSON with additionalContext for Claude
jq -n \
    --arg context "$DATE_CONTEXT" \
    '{
        hookSpecificOutput: {
            hookEventName: "UserPromptSubmit",
            additionalContext: $context
        }
    }'

exit 0