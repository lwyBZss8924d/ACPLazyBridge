#!/usr/bin/env bash
# metadata-utils.sh - Shared utilities for SDD YAML metadata processing
# Part of the SDD (Specification-Driven Development) workflow

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Metadata format types
FORMAT_SIMPLE="simple"
FORMAT_NESTED="nested"
FORMAT_NONE="none"

# Document types
DOC_TYPE_CLAUDE="claude-memory"
DOC_TYPE_AGENTS="agents-memory"
DOC_TYPE_CONSTITUTION="constitution"
DOC_TYPE_CONSTITUTION_LIFECYCLE="constitution-lifecycle"
DOC_TYPE_SDD_RULE="sdd-rule"
DOC_TYPE_SDD_COMMAND="sdd-command"
DOC_TYPE_SDD_TEMPLATE="sdd-template"
DOC_TYPE_GENERIC="document"

# Function to extract YAML metadata from file
extract_metadata() {
    local file="$1"
    local metadata=""

    # Check if file exists
    if [[ ! -f "$file" ]]; then
        echo "ERROR: File not found: $file" >&2
        return 1
    fi

    # Look for YAML block at the end of file
    local in_yaml=false
    local yaml_content=""

    while IFS= read -r line; do
        if [[ "$line" == '```yaml' ]]; then
            in_yaml=true
            yaml_content=""
        elif [[ "$line" == '```' ]] && [[ "$in_yaml" == true ]]; then
            in_yaml=false
            metadata="$yaml_content"
        elif [[ "$in_yaml" == true ]]; then
            yaml_content+="$line"$'\n'
        fi
    done < <(tail -50 "$file")

    echo "$metadata"
}

# Function to detect metadata format
detect_metadata_format() {
    local metadata="$1"

    if [[ -z "$metadata" ]]; then
        echo "$FORMAT_NONE"
        return
    fi

    # Check for nested format (has constitution: or document: at root level)
    if echo "$metadata" | grep -q "^constitution:" || echo "$metadata" | grep -q "^document:"; then
        echo "$FORMAT_NESTED"
    # Check for simple format (has Version: or similar at root)
    elif echo "$metadata" | grep -q "^Version:" || echo "$metadata" | grep -q "^Constitution version:"; then
        echo "$FORMAT_SIMPLE"
    else
        echo "$FORMAT_NONE"
    fi
}

# Function to parse nested YAML metadata using yq
parse_nested_metadata() {
    local metadata="$1"

    # Use yq to convert YAML to JSON (using -o json for JSON output)
    echo "$metadata" | yq -o json 2>/dev/null || {
        echo '{"error": "Failed to parse YAML"}'
        return 1
    }
}

# Function to parse simple YAML metadata
parse_simple_metadata() {
    local metadata="$1"
    local json="{"
    local first=true

    while IFS= read -r line; do
        if [[ -n "$line" ]] && [[ "$line" =~ ^([^:]+):(.*)$ ]]; then
            local key="${BASH_REMATCH[1]}"
            local value="${BASH_REMATCH[2]}"
            # Trim whitespace
            key=$(echo "$key" | xargs)
            value=$(echo "$value" | xargs)

            if [[ "$first" == false ]]; then
                json+=","
            fi
            first=false

            # Convert key to lowercase and replace spaces with underscores
            key=$(echo "$key" | tr '[:upper:]' '[:lower:]' | tr ' ' '_')
            json+="\"$key\":\"$value\""
        fi
    done <<< "$metadata"

    json+="}"
    echo "$json"
}

# Function to get constitution version from metadata
get_constitution_version() {
    local metadata_json="$1"
    local version=""

    # Try nested format first
    version=$(echo "$metadata_json" | jq -r '.constitution.version // .constitution_version // .version // empty' 2>/dev/null || echo "")

    echo "$version"
}

# Function to get document type from metadata
get_document_type() {
    local metadata_json="$1"
    local doc_type=""

    doc_type=$(echo "$metadata_json" | jq -r '.document.type // empty' 2>/dev/null || echo "")

    echo "$doc_type"
}

# Function to get document version from metadata
get_document_version() {
    local metadata_json="$1"
    local version=""

    version=$(echo "$metadata_json" | jq -r '.document.version // .document_version // .memory_version // empty' 2>/dev/null || echo "")

    echo "$version"
}

# Function to get last updated date from metadata
get_last_updated() {
    local metadata_json="$1"
    local date=""

    date=$(echo "$metadata_json" | jq -r '.document.last_updated // .last_updated // .last_amended // empty' 2>/dev/null || echo "")

    echo "$date"
}

# Function to validate metadata structure
validate_metadata_structure() {
    local metadata_json="$1"
    local file_path="$2"
    local errors=()

    # Check for constitution version
    local const_version=$(get_constitution_version "$metadata_json")
    if [[ -z "$const_version" ]]; then
        errors+=("Missing constitution version")
    fi

    # Check for document type (for nested format)
    if echo "$metadata_json" | grep -q '"document"'; then
        local doc_type=$(get_document_type "$metadata_json")
        if [[ -z "$doc_type" ]]; then
            errors+=("Missing document type")
        fi

        local doc_version=$(get_document_version "$metadata_json")
        if [[ -z "$doc_version" ]]; then
            errors+=("Missing document version")
        fi
    fi

    # Check for last updated date
    local last_updated=$(get_last_updated "$metadata_json")
    if [[ -z "$last_updated" ]]; then
        errors+=("Missing last updated date")
    fi

    # Return errors
    if [[ ${#errors[@]} -gt 0 ]]; then
        printf '%s\n' "${errors[@]}"
        return 1
    fi

    return 0
}

# Function to format metadata for display
format_metadata_display() {
    local metadata_json="$1"
    local file_path="$2"

    echo "File: $file_path"
    echo "  Constitution Version: $(get_constitution_version "$metadata_json")"
    echo "  Document Type: $(get_document_type "$metadata_json")"
    echo "  Document Version: $(get_document_version "$metadata_json")"
    echo "  Last Updated: $(get_last_updated "$metadata_json")"
}

# Function to check if required tools are available
check_dependencies() {
    # Check for yq (YAML processor)
    if ! command -v yq >/dev/null 2>&1; then
        echo -e "${RED}Error: yq is required but not installed${NC}" >&2
        echo -e "${YELLOW}Install with: brew install yq (macOS) or see https://github.com/mikefarah/yq${NC}" >&2
        return 1
    fi

    # Check for jq (JSON processor)
    if ! command -v jq >/dev/null 2>&1; then
        echo -e "${RED}Error: jq is required but not installed${NC}" >&2
        echo -e "${YELLOW}Install with: brew install jq (macOS) or apt-get install jq (Linux)${NC}" >&2
        return 1
    fi

    return 0
}

# Export functions for use in other scripts
export -f extract_metadata
export -f detect_metadata_format
export -f parse_nested_metadata
export -f parse_simple_metadata
export -f get_constitution_version
export -f get_document_type
export -f get_document_version
export -f get_last_updated
export -f validate_metadata_structure
export -f format_metadata_display
export -f check_dependencies