#!/usr/bin/env bash
# validate-claude-md.sh - Validate CLAUDE.md file consistency across the repository
# Part of the SDD (Specification-Driven Development) workflow

set -euo pipefail

# Get repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Configuration
CONSTITUTION_VERSION="1.0.1"  # Current constitution version
CLAUDE_MD_FILES=(
    "CLAUDE.md"
    ".github/CLAUDE.md"
    ".specify/CLAUDE.md"
    "sdd-rules/CLAUDE.md"
    "scripts/CLAUDE.md"
    "crates/CLAUDE.md"
    "crates/acp-lazy-core/CLAUDE.md"
    "crates/codex-cli-acp/CLAUDE.md"
    "dev-docs/CLAUDE.md"
    "specs/CLAUDE.md"
    "queries/CLAUDE.md"
    "dev-docs/review/_artifacts/CLAUDE.md"
)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script options
JSON_OUTPUT=false
UPDATE_MODE=false
SPECIFIC_FILE=""
VERBOSE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --json)
            JSON_OUTPUT=true
            shift
            ;;
        --update)
            UPDATE_MODE=true
            shift
            ;;
        --file)
            SPECIFIC_FILE="$2"
            shift 2
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            cat << EOF
Usage: $0 [OPTIONS]

Validate CLAUDE.md file consistency across the repository.

Options:
    --json          Output results in JSON format for CI integration
    --update        Auto-update metadata to current version (use with caution)
    --file FILE     Check specific file only
    --verbose, -v   Show detailed output
    --help, -h      Show this help message

Examples:
    $0                           # Check all CLAUDE.md files
    $0 --json                    # Output JSON for CI
    $0 --file crates/CLAUDE.md   # Check specific file
    $0 --update                  # Update all metadata

EOF
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Function to extract metadata from CLAUDE.md file
extract_metadata() {
    local file="$1"
    if [[ ! -f "$file" ]]; then
        echo "FILE_NOT_FOUND"
        return
    fi

    # Check for YAML format first (new format)
    local yaml_block=$(tail -20 "$file" | sed -n '/^```yaml$/,/^```$/p' 2>/dev/null)
    if [[ -n "$yaml_block" ]]; then
        echo "YAML:$yaml_block"
        return
    fi

    # Fall back to old format (for backward compatibility)
    local metadata=$(tail -3 "$file" | grep -E "Based on Constitution.*\|.*Last Updated" || echo "NO_METADATA")
    echo "$metadata"
}

# Function to parse metadata components
parse_metadata() {
    local metadata="$1"
    local constitution_version=""
    local file_version=""
    local last_updated=""

    # Check if this is YAML format
    if [[ "$metadata" =~ ^YAML: ]]; then
        # Remove YAML: prefix and parse YAML content
        local yaml_content="${metadata#YAML:}"

        # Extract from YAML format
        constitution_version=$(echo "$yaml_content" | grep "Constitution version:" | sed 's/.*Constitution version: *//' | tr -d '\r')

        # Try different version field names
        file_version=$(echo "$yaml_content" | grep -E "(Document version:|Rules version:|Version:)" | head -1 | sed 's/.*: *//' | tr -d '\r')

        # Extract last updated date
        last_updated=$(echo "$yaml_content" | grep "Last Updated:" | sed 's/.*Last Updated: *//' | tr -d '\r')

    elif [[ "$metadata" != "NO_METADATA" ]] && [[ "$metadata" != "FILE_NOT_FOUND" ]]; then
        # Parse old format (backward compatibility)
        # Extract Constitution version - handle both ":" and without
        constitution_version=$(echo "$metadata" | sed -n 's/.*Based on Constitution:* \([0-9.]*\).*/\1/p')
        # Extract file version - handle both with and without parentheses
        # Pattern 1: (path/CLAUDE.md) : version
        file_version=$(echo "$metadata" | sed -n 's/.*(.*CLAUDE\.md.*) *: *\([0-9.]*\).*/\1/p')
        # Pattern 2: CLAUDE.md: version (without parentheses)
        if [[ -z "$file_version" ]]; then
            file_version=$(echo "$metadata" | sed -n 's/.*CLAUDE\.md: *\([0-9.]*\).*/\1/p')
        fi
        # Extract last updated date
        last_updated=$(echo "$metadata" | sed -n 's/.*Last Updated: \([0-9-]*\).*/\1/p')
    fi

    echo "$constitution_version|$file_version|$last_updated"
}

# Function to validate a single CLAUDE.md file
validate_file() {
    local file="$1"
    local errors=()
    local warnings=()

    if [[ ! -f "$file" ]]; then
        errors+=("File not found: $file")
        echo "${errors[@]}"
        return 1
    fi

    local metadata=$(extract_metadata "$file")

    if [[ "$metadata" == "NO_METADATA" ]]; then
        errors+=("Missing metadata in $file")
    elif [[ "$metadata" == "FILE_NOT_FOUND" ]]; then
        errors+=("File not found: $file")
    else
        local parsed=$(parse_metadata "$metadata")
        IFS='|' read -r const_ver file_ver last_updated <<< "$parsed"

        # Check Constitution version
        if [[ "$const_ver" != "$CONSTITUTION_VERSION" ]]; then
            errors+=("Constitution version mismatch in $file: found '$const_ver', expected '$CONSTITUTION_VERSION'")
        fi

        # Check if file version exists
        if [[ -z "$file_ver" ]]; then
            warnings+=("Missing file version in $file")
        fi

        # Check if last updated exists
        if [[ -z "$last_updated" ]]; then
            warnings+=("Missing Last Updated date in $file")
        fi

        # Check date format (YYYY-MM-DD)
        if [[ ! "$last_updated" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
            warnings+=("Invalid date format in $file: '$last_updated'")
        fi
    fi

    # Return results
    if [[ ${#errors[@]} -gt 0 ]]; then
        for error in "${errors[@]}"; do
            echo "ERROR: $error"
        done
    fi

    if [[ ${#warnings[@]} -gt 0 ]]; then
        for warning in "${warnings[@]}"; do
            echo "WARNING: $warning"
        done
    fi

    if [[ ${#errors[@]} -eq 0 ]] && [[ ${#warnings[@]} -eq 0 ]]; then
        echo "OK"
    fi

    return $(( ${#errors[@]} ))
}

# Function to update metadata in a file
update_metadata() {
    local file="$1"
    local today=$(date +%Y-%m-%d)
    local dir=$(dirname "$file")
    local filename=$(basename "$file")

    # Determine the appropriate path format for metadata
    local path_format="$dir/$filename"
    if [[ "$dir" == "." ]]; then
        path_format="$filename"
    fi

    # Remove old metadata
    local temp_file=$(mktemp)

    # Check if there's existing metadata to remove
    local has_yaml=$(tail -20 "$file" | grep -c '```yaml' || true)
    local has_old_format=$(tail -5 "$file" | grep -c "Based on Constitution" || true)

    if [[ $has_yaml -gt 0 ]]; then
        # Remove existing YAML block
        sed '/^---$/,/^```$/d' "$file" | sed '/^[[:space:]]*$/d' > "$temp_file"
    elif [[ $has_old_format -gt 0 ]]; then
        # Remove old format metadata
        local line_count=$(wc -l < "$file")
        local check_lines=5
        for i in $(seq 1 $check_lines); do
            local line=$(tail -$i "$file" | head -1)
            if echo "$line" | grep -q "Based on Constitution"; then
                head -n -$i "$file" > "$temp_file"
                break
            fi
        done
    else
        cp "$file" "$temp_file"
    fi

    # Remove trailing blank lines
    sed -i.bak '/^[[:space:]]*$/d' "$temp_file" && rm "$temp_file.bak"

    # Add new YAML metadata
    cat >> "$temp_file" << EOF

---

\`\`\`yaml
Constitution version: $CONSTITUTION_VERSION
Document: $path_format
Document version: 1.0.1
Last Updated: $today
\`\`\`
EOF

    mv "$temp_file" "$file"
    echo "Updated: $file"
}

# Main validation logic
main() {
    local total_files=0
    local valid_files=0
    local error_files=0
    local warning_files=0
    local results=()

    # Determine which files to check
    local files_to_check=()
    if [[ -n "$SPECIFIC_FILE" ]]; then
        files_to_check=("$SPECIFIC_FILE")
    else
        files_to_check=("${CLAUDE_MD_FILES[@]}")
    fi

    # Validate each file
    for file in "${files_to_check[@]}"; do
        if [[ -f "$file" ]]; then
            total_files=$((total_files + 1))

            if [[ "$VERBOSE" == true ]] && [[ "$JSON_OUTPUT" == false ]]; then
                echo -e "${BLUE}Checking: $file${NC}"
            fi

            result=$(validate_file "$file")

            if [[ "$result" == "OK" ]]; then
                valid_files=$((valid_files + 1))
                if [[ "$VERBOSE" == true ]] && [[ "$JSON_OUTPUT" == false ]]; then
                    echo -e "${GREEN}✓ Valid${NC}"
                fi
                results+=("{\"file\":\"$file\",\"status\":\"valid\"}")
            elif echo "$result" | grep -q "ERROR"; then
                error_files=$((error_files + 1))
                if [[ "$JSON_OUTPUT" == false ]]; then
                    echo -e "${RED}✗ $file${NC}"
                    echo "$result" | while read -r line; do
                        echo "  $line"
                    done
                fi
                results+=("{\"file\":\"$file\",\"status\":\"error\",\"message\":\"$(echo "$result" | tr '\n' ' ')\"}")

                # Update if in update mode
                if [[ "$UPDATE_MODE" == true ]]; then
                    update_metadata "$file"
                fi
            else
                warning_files=$((warning_files + 1))
                if [[ "$JSON_OUTPUT" == false ]]; then
                    echo -e "${YELLOW}⚠ $file${NC}"
                    echo "$result" | while read -r line; do
                        echo "  $line"
                    done
                fi
                results+=("{\"file\":\"$file\",\"status\":\"warning\",\"message\":\"$(echo "$result" | tr '\n' ' ')\"}")
            fi
        fi
    done

    # Output results
    if [[ "$JSON_OUTPUT" == true ]]; then
        echo -n "{"
        echo -n "\"constitution_version\":\"$CONSTITUTION_VERSION\","
        echo -n "\"total_files\":$total_files,"
        echo -n "\"valid_files\":$valid_files,"
        echo -n "\"error_files\":$error_files,"
        echo -n "\"warning_files\":$warning_files,"
        echo -n "\"files\":["

        first=true
        for result in "${results[@]}"; do
            if [[ "$first" == false ]]; then
                echo -n ","
            fi
            echo -n "$result"
            first=false
        done

        echo "]}"
    else
        echo ""
        echo "Summary:"
        echo "  Total files checked: $total_files"
        echo -e "  ${GREEN}Valid files: $valid_files${NC}"
        if [[ $error_files -gt 0 ]]; then
            echo -e "  ${RED}Files with errors: $error_files${NC}"
        fi
        if [[ $warning_files -gt 0 ]]; then
            echo -e "  ${YELLOW}Files with warnings: $warning_files${NC}"
        fi

        if [[ $error_files -gt 0 ]]; then
            echo ""
            echo "Run with --update to fix metadata issues automatically."
            exit 1
        fi
    fi
}

# Run main function
main