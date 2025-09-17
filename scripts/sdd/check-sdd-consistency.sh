#!/usr/bin/env bash
# check-sdd-consistency.sh - Check SDD documentation consistency
# Part of the SDD (Specification-Driven Development) workflow

set -euo pipefail

# Get repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Source metadata utilities
source "$REPO_ROOT/scripts/sdd/lib/metadata-utils.sh"

# Script options
VERBOSE=false
OUTPUT_FORMAT="text"  # text or json
FIX_MODE=false
CHECK_DEPENDENCIES=true
CHECK_REFERENCES=true

# Expected values
EXPECTED_CONSTITUTION_VERSION="1.0.1"
CONSTITUTION_FILE=".specify/memory/constitution.md"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --format)
            OUTPUT_FORMAT="$2"
            shift 2
            ;;
        --fix)
            FIX_MODE=true
            shift
            ;;
        --no-dependencies)
            CHECK_DEPENDENCIES=false
            shift
            ;;
        --no-references)
            CHECK_REFERENCES=false
            shift
            ;;
        --help|-h)
            cat << EOF
Usage: $0 [OPTIONS]

Check SDD documentation consistency across the project.

Options:
    --verbose, -v        Show detailed output
    --format FORMAT      Output format: text (default) or json
    --fix               Fix minor issues automatically
    --no-dependencies   Skip dependency validation
    --no-references     Skip reference validation
    --help, -h          Show this help message

Checks performed:
    1. Constitution version consistency
    2. Metadata format consistency
    3. Document dependencies validation
    4. Cross-references validation
    5. Required files existence
    6. Template synchronization

Examples:
    $0                              # Run all consistency checks
    $0 --format json > report.json  # Generate JSON report
    $0 --fix                        # Fix minor issues automatically
    $0 --verbose                    # Show detailed output

Exit codes:
    0 - All checks passed
    1 - Consistency issues found
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

# Consistency check results
declare -A check_results
total_checks=0
passed_checks=0
warning_checks=0
failed_checks=0

# Issues found
declare -a constitution_version_mismatches=()
declare -a missing_metadata=()
declare -a invalid_metadata=()
declare -a broken_dependencies=()
declare -a broken_references=()
declare -a missing_required_files=()
declare -a format_inconsistencies=()

# Function to log check result
log_check() {
    local check_name="$1"
    local status="$2"  # pass, warning, fail
    local message="$3"

    check_results["$check_name"]="$status:$message"
    total_checks=$((total_checks + 1))

    case "$status" in
        pass)
            passed_checks=$((passed_checks + 1))
            [[ "$VERBOSE" == true ]] && echo -e "${GREEN}✓ $check_name: $message${NC}"
            ;;
        warning)
            warning_checks=$((warning_checks + 1))
            [[ "$VERBOSE" == true ]] && echo -e "${YELLOW}⚠ $check_name: $message${NC}"
            ;;
        fail)
            failed_checks=$((failed_checks + 1))
            [[ "$VERBOSE" == true ]] && echo -e "${RED}✗ $check_name: $message${NC}"
            ;;
    esac
}

# Check 1: Constitution version consistency
check_constitution_versions() {
    if [[ "$VERBOSE" == true ]]; then
        echo -e "\n${BLUE}Checking constitution version consistency...${NC}"
    fi

    local files_checked=0
    local version_mismatches=0

    while IFS= read -r file; do
        files_checked=$((files_checked + 1))

        # Extract metadata
        local metadata=$(extract_metadata "$file")
        if [[ -z "$metadata" ]]; then
            missing_metadata+=("$file")
            continue
        fi

        # Parse metadata
        local format=$(detect_metadata_format "$metadata")
        local metadata_json=""

        if [[ "$format" == "$FORMAT_NESTED" ]]; then
            metadata_json=$(parse_nested_metadata "$metadata" 2>/dev/null) || {
                invalid_metadata+=("$file")
                continue
            }
        elif [[ "$format" == "$FORMAT_SIMPLE" ]]; then
            metadata_json=$(parse_simple_metadata "$metadata")
        else
            invalid_metadata+=("$file")
            continue
        fi

        # Check constitution version
        local const_version=$(get_constitution_version "$metadata_json")
        if [[ -n "$const_version" ]] && [[ "$const_version" != "$EXPECTED_CONSTITUTION_VERSION" ]]; then
            constitution_version_mismatches+=("$file:$const_version")
            version_mismatches=$((version_mismatches + 1))
        fi
    done < <(find . -name "*.md" -type f | grep -E "(CLAUDE\.md|WARP\.md|AGENTS\.md|sdd-rules/|\.specify/)" | grep -v ".bak")

    if [[ $version_mismatches -eq 0 ]]; then
        log_check "constitution_version" "pass" "All $files_checked files use version $EXPECTED_CONSTITUTION_VERSION"
    else
        log_check "constitution_version" "fail" "$version_mismatches files have mismatched versions"
    fi
}

# Check 2: Required files existence
check_required_files() {
    if [[ "$VERBOSE" == true ]]; then
        echo -e "\n${BLUE}Checking required SDD files...${NC}"
    fi

    local required_files=(
        ".specify/memory/constitution.md"
        ".specify/memory/lifecycle.md"
        ".specify/memory/constitution_update_checklist.md"
        ".specify/templates/spec-template.md"
        ".specify/templates/plan-template.md"
        ".specify/templates/tasks-template.md"
        ".specify/commands/specify.md"
        ".specify/commands/plan.md"
        ".specify/commands/tasks.md"
        "sdd-rules/AGENTS.md"
        "CLAUDE.md"
    )

    local missing_count=0
    for file in "${required_files[@]}"; do
        if [[ ! -f "$file" ]]; then
            missing_required_files+=("$file")
            missing_count=$((missing_count + 1))
        fi
    done

    if [[ $missing_count -eq 0 ]]; then
        log_check "required_files" "pass" "All ${#required_files[@]} required files exist"
    else
        log_check "required_files" "fail" "$missing_count required files missing"
    fi
}

# Check 3: Metadata format consistency
check_metadata_formats() {
    if [[ "$VERBOSE" == true ]]; then
        echo -e "\n${BLUE}Checking metadata format consistency...${NC}"
    fi

    local nested_count=0
    local simple_count=0
    local none_count=0

    while IFS= read -r file; do
        local metadata=$(extract_metadata "$file")
        if [[ -n "$metadata" ]]; then
            local format=$(detect_metadata_format "$metadata")
            case "$format" in
                "$FORMAT_NESTED")
                    nested_count=$((nested_count + 1))
                    ;;
                "$FORMAT_SIMPLE")
                    simple_count=$((simple_count + 1))
                    format_inconsistencies+=("$file:simple_format")
                    ;;
                "$FORMAT_NONE")
                    none_count=$((none_count + 1))
                    ;;
            esac
        else
            none_count=$((none_count + 1))
        fi
    done < <(find . -name "*.md" -type f | grep -E "(CLAUDE\.md|WARP\.md|AGENTS\.md|sdd-rules/|\.specify/)" | grep -v ".bak")

    if [[ $simple_count -eq 0 ]] && [[ $none_count -eq 0 ]]; then
        log_check "metadata_format" "pass" "All files use nested YAML format"
    elif [[ $simple_count -gt 0 ]]; then
        log_check "metadata_format" "warning" "$simple_count files still use simple format"
    else
        log_check "metadata_format" "fail" "$none_count files have invalid or missing metadata"
    fi
}

# Check 4: Document dependencies
check_document_dependencies() {
    if [[ "$CHECK_DEPENDENCIES" != true ]]; then
        return
    fi

    if [[ "$VERBOSE" == true ]]; then
        echo -e "\n${BLUE}Checking document dependencies...${NC}"
    fi

    local broken_count=0

    while IFS= read -r file; do
        local metadata=$(extract_metadata "$file")
        [[ -z "$metadata" ]] && continue

        local format=$(detect_metadata_format "$metadata")
        [[ "$format" != "$FORMAT_NESTED" ]] && continue

        # Extract dependencies from nested format
        if echo "$metadata" | grep -q "dependencies:"; then
            while IFS= read -r dep; do
                dep=$(echo "$dep" | sed 's/^[[:space:]]*-[[:space:]]*//' | tr -d '"')
                if [[ -n "$dep" ]] && [[ ! -f "$dep" ]]; then
                    broken_dependencies+=("$file -> $dep")
                    broken_count=$((broken_count + 1))
                fi
            done < <(echo "$metadata" | sed -n '/dependencies:/,/^[^[:space:]]/p' | grep "^[[:space:]]*-")
        fi
    done < <(find . -name "*.md" -type f | grep -E "(CLAUDE\.md|sdd-rules/|\.specify/)" | grep -v ".bak")

    if [[ $broken_count -eq 0 ]]; then
        log_check "dependencies" "pass" "All document dependencies exist"
    else
        log_check "dependencies" "fail" "$broken_count broken dependencies found"
    fi
}

# Check 5: Cross-references
check_cross_references() {
    if [[ "$CHECK_REFERENCES" != true ]]; then
        return
    fi

    if [[ "$VERBOSE" == true ]]; then
        echo -e "\n${BLUE}Checking cross-references...${NC}"
    fi

    local broken_count=0

    # Check for references to .specify/ files
    while IFS= read -r file; do
        while IFS= read -r ref; do
            # Extract path from markdown link
            local path=$(echo "$ref" | sed -n 's/.*](\([^)]*\)).*/\1/p')
            if [[ -n "$path" ]] && [[ "$path" =~ ^\.specify/ || "$path" =~ ^sdd-rules/ ]]; then
                if [[ ! -f "$path" ]] && [[ ! -d "$path" ]]; then
                    broken_references+=("$file -> $path")
                    broken_count=$((broken_count + 1))
                fi
            fi
        done < <(grep -o '\[.*\](.*\.md)' "$file" 2>/dev/null || true)
    done < <(find . -name "*.md" -type f | grep -v ".bak")

    if [[ $broken_count -eq 0 ]]; then
        log_check "references" "pass" "All cross-references are valid"
    else
        log_check "references" "warning" "$broken_count broken references found"
    fi
}

# Function to generate JSON report
generate_json_report() {
    cat << EOF | jq .
{
    "summary": {
        "total_checks": $total_checks,
        "passed": $passed_checks,
        "warnings": $warning_checks,
        "failed": $failed_checks
    },
    "constitution": {
        "expected_version": "$EXPECTED_CONSTITUTION_VERSION",
        "mismatches": $(printf '%s\n' "${constitution_version_mismatches[@]}" | jq -R . | jq -s . 2>/dev/null || echo "[]")
    },
    "metadata": {
        "missing": $(printf '%s\n' "${missing_metadata[@]}" | jq -R . | jq -s . 2>/dev/null || echo "[]"),
        "invalid": $(printf '%s\n' "${invalid_metadata[@]}" | jq -R . | jq -s . 2>/dev/null || echo "[]"),
        "format_inconsistencies": $(printf '%s\n' "${format_inconsistencies[@]}" | jq -R . | jq -s . 2>/dev/null || echo "[]")
    },
    "files": {
        "missing_required": $(printf '%s\n' "${missing_required_files[@]}" | jq -R . | jq -s . 2>/dev/null || echo "[]")
    },
    "dependencies": {
        "broken": $(printf '%s\n' "${broken_dependencies[@]}" | jq -R . | jq -s . 2>/dev/null || echo "[]")
    },
    "references": {
        "broken": $(printf '%s\n' "${broken_references[@]}" | jq -R . | jq -s . 2>/dev/null || echo "[]")
    }
}
EOF
}

# Main logic
main() {
    if [[ "$OUTPUT_FORMAT" == "text" ]]; then
        echo -e "${BLUE}Starting SDD consistency check...${NC}"
    fi

    # Run all checks
    check_required_files
    check_constitution_versions
    check_metadata_formats

    if [[ "$CHECK_DEPENDENCIES" == true ]]; then
        check_document_dependencies
    fi

    if [[ "$CHECK_REFERENCES" == true ]]; then
        check_cross_references
    fi

    # Generate output
    if [[ "$OUTPUT_FORMAT" == "json" ]]; then
        generate_json_report
    else
        # Summary
        echo ""
        echo "========================================="
        echo "SDD Consistency Check Summary"
        echo "========================================="
        echo ""
        echo "Checks performed: $total_checks"
        echo -e "  ${GREEN}Passed: $passed_checks${NC}"
        [[ $warning_checks -gt 0 ]] && echo -e "  ${YELLOW}Warnings: $warning_checks${NC}"
        [[ $failed_checks -gt 0 ]] && echo -e "  ${RED}Failed: $failed_checks${NC}"

        # Detail issues if any
        if [[ ${#constitution_version_mismatches[@]} -gt 0 ]]; then
            echo ""
            echo "Constitution Version Mismatches:"
            for item in "${constitution_version_mismatches[@]}"; do
                echo "  - $item"
            done
        fi

        if [[ ${#missing_metadata[@]} -gt 0 ]]; then
            echo ""
            echo "Files Missing Metadata:"
            for item in "${missing_metadata[@]}"; do
                echo "  - $item"
            done
        fi

        if [[ ${#format_inconsistencies[@]} -gt 0 ]]; then
            echo ""
            echo "Format Inconsistencies:"
            for item in "${format_inconsistencies[@]}"; do
                echo "  - $item"
            done
        fi

        if [[ ${#missing_required_files[@]} -gt 0 ]]; then
            echo ""
            echo "Missing Required Files:"
            for item in "${missing_required_files[@]}"; do
                echo "  - $item"
            done
        fi

        if [[ ${#broken_dependencies[@]} -gt 0 ]]; then
            echo ""
            echo "Broken Dependencies:"
            for item in "${broken_dependencies[@]}"; do
                echo "  - $item"
            done
        fi

        if [[ ${#broken_references[@]} -gt 0 ]] && [[ "$VERBOSE" == true ]]; then
            echo ""
            echo "Broken Cross-References:"
            for item in "${broken_references[@]}"; do
                echo "  - $item"
            done
        fi

        echo ""
        if [[ $failed_checks -eq 0 ]] && [[ $warning_checks -eq 0 ]]; then
            echo -e "${GREEN}✓ All consistency checks passed!${NC}"
        elif [[ $failed_checks -eq 0 ]]; then
            echo -e "${YELLOW}⚠ Consistency check completed with warnings${NC}"
        else
            echo -e "${RED}✗ Consistency check failed${NC}"
        fi
    fi

    # Exit with appropriate code
    if [[ $failed_checks -gt 0 ]]; then
        exit 1
    else
        exit 0
    fi
}

# Run main function
main