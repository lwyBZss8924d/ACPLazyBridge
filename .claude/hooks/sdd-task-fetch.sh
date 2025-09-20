#!/bin/bash
# Wrapper script to fetch GitHub issue details for /sdd-task command
# Accepts issue number or GitHub URL as argument

# Get the first argument
ISSUE_ARG="$1"

# Check if argument is provided
if [ -z "$ISSUE_ARG" ]; then
    echo '{"error": "No issue number or URL provided"}'
    exit 1
fi

# Fetch issue details using gh CLI
# gh issue view accepts both issue numbers and URLs
gh issue view "$ISSUE_ARG" \
    --json title,body,number,url,state,labels \
    --jq '{
        "number": .number,
        "title": .title,
        "url": .url,
        "state": .state,
        "labels": [.labels[].name],
        "body": .body
    }' 2>&1

# Check if gh command succeeded
if [ $? -ne 0 ]; then
    echo '{"error": "Failed to fetch issue details"}'
    exit 1
fi

exit 0