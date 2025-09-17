#!/usr/bin/env bash
# validate-metadata.sh - Validate SDD document YAML metadata
# Part of the SDD (Specification-Driven Development) workflow

set -euo pipefail

# Get repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Source metadata utilities
source "$REPO_ROOT/scripts/sdd/lib/metadata-utils.sh"

# Script options
VERBOSE=false
SPECIFIC_FILE=""
OUTPUT_FORMAT="text"  # text or json
STRICT_MODE=false
CHECK_CONSISTENCY=false

# Current constitution version
EXPECTED_CONSTITUTION_VERSION="1.0.1"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --file)
            SPECIFIC_FILE="$2"
            shift 2
            ;;
        --format)
            OUTPUT_FORMAT="$2"
            shift 2
            ;;
        --strict)
            STRICT_MODE=true
            shift
            ;;
        --check-consistency)
            CHECK_CONSISTENCY=true
            shift
            ;;
        --help|-h)
            cat << EOF
Usage: $0 [OPTIONS]

Validate YAML metadata in SDD documents.

Options:
    --verbose, -v         Show detailed validation output
    --file FILE          Validate specific file only
    --format FORMAT      Output format: text (default) or json
    --strict             Strict mode: fail on any warning
    --check-consistency  Check constitution version consistency
    --help, -h           Show this help message

Examples:
    $0                                    # Validate all files
    $0 --file CLAUDE.md                  # Validate specific file
    $0 --format json > report.json       # Generate JSON report
    $0 --check-consistency --strict      # Strict consistency check

Exit codes:
    0 - All validations passed
    1 - Validation errors found
    2 - Missing dependencies

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

# Check dependencies
if ! check_dependencies; then
    exit 2
fi

# Validation results
declare -A validation_results
total_files=0
valid_files=0
warning_files=0
error_files=0
no_metadata_files=0

# Function to validate a single file
validate_file() {
    local file="$1"
    local errors=()
    local warnings=()

    if [[ "$VERBOSE" == true ]]; then
        echo -e "${BLUE}Validating: $file${NC}"
    fi

    # Extract metadata
    local metadata=$(extract_metadata "$file")

    if [[ -z "$metadata" ]]; then
        no_metadata_files=$((no_metadata_files + 1))
        errors+=("No metadata found")
        validation_results["$file"]="no_metadata"
        return 1
    fi

    # Detect format
    local format=$(detect_metadata_format "$metadata")

    if [[ "$format" == "$FORMAT_NONE" ]]; then
        errors+=("Invalid metadata format")
        validation_results["$file"]="invalid_format"
        return 1
    fi

    # Parse metadata based on format
    local metadata_json=""
    if [[ "$format" == "$FORMAT_NESTED" ]]; then
        metadata_json=$(parse_nested_metadata "$metadata" 2>/dev/null) || {
            errors+=("Failed to parse nested YAML metadata")
            validation_results["$file"]="parse_error"
            return 1
        }
    else
        metadata_json=$(parse_simple_metadata "$metadata")
    fi

    # Check for parse errors
    if echo "$metadata_json" | grep -q '"error"'; then
        errors+=("YAML parse error: $(echo "$metadata_json" | jq -r .error)")
        validation_results["$file"]="parse_error"
        return 1
    fi

    # Validate structure
    local validation_errors
    if ! validation_errors=$(validate_metadata_structure "$metadata_json" "$file" 2>&1); then
        while IFS= read -r error; do
            errors+=("$error")
        done <<< "$validation_errors"
    fi

    # Check constitution version consistency
    if [[ "$CHECK_CONSISTENCY" == true ]]; then
        local const_version=$(get_constitution_version "$metadata_json")
        if [[ -n "$const_version" ]] && [[ "$const_version" != "$EXPECTED_CONSTITUTION_VERSION" ]]; then
            warnings+=("Constitution version mismatch: expected $EXPECTED_CONSTITUTION_VERSION, found $const_version")
        fi
    fi

    # Check for old format in files that should use nested format
    if [[ "$format" == "$FORMAT_SIMPLE" ]] && [[ "$file" =~ CLAUDE\.md$ || "$file" =~ sdd-rules/.*\.md$ ]]; then
        warnings+=("Using old simple format, should migrate to nested format")
    fi

    # Validate date format
    local last_updated=$(get_last_updated "$metadata_json")
    if [[ -n "$last_updated" ]]; then
        # Check if date is in ISO format (YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ)
        if ! echo "$last_updated" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}(T[0-9]{2}:[0-9]{2}:[0-9]{2}(Z|[+-][0-9]{2}:[0-9]{2}))?$'; then
            warnings+=("Invalid date format: $last_updated (should be ISO 8601)")
        fi
    fi

    # Report results
    if [[ ${#errors[@]} -gt 0 ]]; then
        validation_results["$file"]="error"
        if [[ "$VERBOSE" == true ]]; then
            echo -e "${RED}  ✗ Errors:${NC}"
            for error in "${errors[@]}"; do
                echo -e "${RED}    - $error${NC}"
            done
        fi
        return 1
    elif [[ ${#warnings[@]} -gt 0 ]]; then
        validation_results["$file"]="warning"
        if [[ "$VERBOSE" == true ]]; then
            echo -e "${YELLOW}  ⚠ Warnings:${NC}"
            for warning in "${warnings[@]}"; do
                echo -e "${YELLOW}    - $warning${NC}"
            done
        fi
        if [[ "$STRICT_MODE" == true ]]; then
            return 1
        fi
        return 0
    else
        validation_results["$file"]="valid"
        if [[ "$VERBOSE" == true ]]; then
            echo -e "${GREEN}  ✓ Valid${NC}"
        fi
        return 0
    fi
}

# Function to generate JSON report
generate_json_report() {
    local json="{"
    json+="\"summary\":{"
    json+="\"total_files\":$total_files,"
    json+="\"valid_files\":$valid_files,"
    json+="\"warning_files\":$warning_files,"
    json+="\"error_files\":$error_files,"
    json+="\"no_metadata_files\":$no_metadata_files"
    json+="},"
    json+="\"expected_constitution_version\":\"$EXPECTED_CONSTITUTION_VERSION\","
    json+="\"results\":{"

    local first=true
    for file in "${!validation_results[@]}"; do
        if [[ "$first" == false ]]; then
            json+=","
        fi
        first=false
        json+="\"$file\":\"${validation_results[$file]}\""
    done

    json+="}}"
    echo "$json" | python3 -m json.tool
}

# Main validation logic
main() {
    # Determine which files to validate
    local files_to_validate=()

    if [[ -n "$SPECIFIC_FILE" ]]; then
        if [[ ! -f "$SPECIFIC_FILE" ]]; then
            echo -e "${RED}Error: File not found: $SPECIFIC_FILE${NC}"
            exit 1
        fi
        files_to_validate=("$SPECIFIC_FILE")
    else
        # Find all markdown files in SDD-related directories
        while IFS= read -r file; do
            files_to_validate+=("$file")
        done < <(find . -name "*.md" -type f | grep -E "(CLAUDE\.md|WARP\.md|AGENTS\.md|sdd-rules/|\.specify/|specs/)" | grep -v ".bak" | sort)
    fi

    if [[ "$OUTPUT_FORMAT" == "text" ]]; then
        echo -e "${BLUE}Starting metadata validation...${NC}"
        echo ""
    fi

    # Validate each file
    for file in "${files_to_validate[@]}"; do
        total_files=$((total_files + 1))

        if validate_file "$file"; then
            if [[ "${validation_results[$file]}" == "valid" ]]; then
                valid_files=$((valid_files + 1))
            elif [[ "${validation_results[$file]}" == "warning" ]]; then
                warning_files=$((warning_files + 1))
            fi
        else
            error_files=$((error_files + 1))
        fi
    done

    # Generate output
    if [[ "$OUTPUT_FORMAT" == "json" ]]; then
        generate_json_report
    else
        # Summary
        echo ""
        echo "Validation Summary:"
        echo "  Total files checked: $total_files"
        echo -e "  ${GREEN}Valid: $valid_files${NC}"
        if [[ $warning_files -gt 0 ]]; then
            echo -e "  ${YELLOW}Warnings: $warning_files${NC}"
        fi
        if [[ $error_files -gt 0 ]]; then
            echo -e "  ${RED}Errors: $error_files${NC}"
        fi
        if [[ $no_metadata_files -gt 0 ]]; then
            echo -e "  ${YELLOW}No metadata: $no_metadata_files${NC}"
        fi

        if [[ "$CHECK_CONSISTENCY" == true ]]; then
            echo ""
            echo "Constitution Version: $EXPECTED_CONSTITUTION_VERSION"
        fi
    fi

    # Exit with appropriate code
    if [[ $error_files -gt 0 ]]; then
        exit 1
    elif [[ $warning_files -gt 0 ]] && [[ "$STRICT_MODE" == true ]]; then
        exit 1
    else
        exit 0
    fi
}

# Run main function
main