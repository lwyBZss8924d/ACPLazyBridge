#!/usr/bin/env bash
# query-metadata.sh - Query SDD documents by metadata fields
# Part of the SDD (Specification-Driven Development) workflow

set -euo pipefail

# Get repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Source metadata utilities
source "$REPO_ROOT/scripts/sdd/lib/metadata-utils.sh"

# Script options
QUERY_TYPE=""
QUERY_VALUE=""
OUTPUT_FORMAT="text"  # text, json, or paths
VERBOSE=false
SORT_BY=""  # date, version, type, path

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --type)
            QUERY_TYPE="document_type"
            QUERY_VALUE="$2"
            shift 2
            ;;
        --constitution-version)
            QUERY_TYPE="constitution_version"
            QUERY_VALUE="$2"
            shift 2
            ;;
        --document-version)
            QUERY_TYPE="document_version"
            QUERY_VALUE="$2"
            shift 2
            ;;
        --outdated)
            QUERY_TYPE="outdated"
            QUERY_VALUE="${2:-7}"  # Days, default 7
            shift
            [[ $# -gt 0 ]] && shift
            ;;
        --missing-metadata)
            QUERY_TYPE="missing_metadata"
            shift
            ;;
        --all)
            QUERY_TYPE="all"
            shift
            ;;
        --format)
            OUTPUT_FORMAT="$2"
            shift 2
            ;;
        --sort)
            SORT_BY="$2"
            shift 2
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            cat << EOF
Usage: $0 [OPTIONS]

Query SDD documents by metadata fields.

Options:
    --type TYPE                Query by document type
                              (claude-memory, agents-memory, sdd-rule, etc.)
    --constitution-version VER Query by constitution version
    --document-version VER     Query by document version
    --outdated [DAYS]          Find documents not updated in N days (default: 7)
    --missing-metadata         Find files without metadata
    --all                      List all files with metadata
    --format FORMAT            Output format: text (default), json, or paths
    --sort FIELD              Sort by: date, version, type, or path
    --verbose, -v             Show detailed output
    --help, -h                Show this help message

Document Types:
    claude-memory             Claude agent memory files
    agents-memory            Agent coordination files
    constitution             Constitution documents
    constitution-lifecycle   Lifecycle documents
    sdd-rule                 SDD rule documents
    sdd-command              SDD command documents
    sdd-template            SDD template documents
    document                 Generic documents

Examples:
    $0 --all                                         # List all documents
    $0 --type sdd-rule                              # Find all SDD rules
    $0 --outdated 30                                # Find docs not updated in 30 days
    $0 --constitution-version 1.0.0                 # Find old constitution versions
    $0 --missing-metadata --format paths            # Get paths of files without metadata
    $0 --all --sort date --format json              # JSON output sorted by date

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

# Check if a query type was specified
if [[ -z "$QUERY_TYPE" ]]; then
    echo "Error: No query specified. Use --help for usage information"
    exit 1
fi

# Results storage
declare -A results

# Function to check if document matches query
matches_query() {
    local metadata_json="$1"
    local file_path="$2"

    case "$QUERY_TYPE" in
        document_type)
            local doc_type=$(get_document_type "$metadata_json")
            [[ "$doc_type" == "$QUERY_VALUE" ]]
            ;;
        constitution_version)
            local const_version=$(get_constitution_version "$metadata_json")
            [[ "$const_version" == "$QUERY_VALUE" ]]
            ;;
        document_version)
            local doc_version=$(get_document_version "$metadata_json")
            [[ "$doc_version" == "$QUERY_VALUE" ]]
            ;;
        outdated)
            local last_updated=$(get_last_updated "$metadata_json")
            if [[ -n "$last_updated" ]]; then
                # Convert date to seconds since epoch
                local updated_epoch=$(date -j -f "%Y-%m-%d" "${last_updated%%T*}" "+%s" 2>/dev/null || date -d "${last_updated%%T*}" "+%s" 2>/dev/null || echo "0")
                local current_epoch=$(date "+%s")
                local days_old=$(( (current_epoch - updated_epoch) / 86400 ))
                [[ $days_old -gt $QUERY_VALUE ]]
            else
                false
            fi
            ;;
        all)
            true
            ;;
        *)
            false
            ;;
    esac
}

# Function to process a single file
process_file() {
    local file="$1"

    # Extract metadata
    local metadata=$(extract_metadata "$file")

    if [[ -z "$metadata" ]]; then
        if [[ "$QUERY_TYPE" == "missing_metadata" ]]; then
            results["$file"]="no_metadata"
        fi
        return
    fi

    # Detect format
    local format=$(detect_metadata_format "$metadata")

    if [[ "$format" == "$FORMAT_NONE" ]]; then
        if [[ "$QUERY_TYPE" == "missing_metadata" ]]; then
            results["$file"]="invalid_format"
        fi
        return
    fi

    # Skip if we're only looking for missing metadata
    if [[ "$QUERY_TYPE" == "missing_metadata" ]]; then
        return
    fi

    # Parse metadata based on format
    local metadata_json=""
    if [[ "$format" == "$FORMAT_NESTED" ]]; then
        metadata_json=$(parse_nested_metadata "$metadata" 2>/dev/null) || return
    else
        metadata_json=$(parse_simple_metadata "$metadata")
    fi

    # Check if matches query
    if matches_query "$metadata_json" "$file"; then
        # Store result with metadata for sorting
        local const_version=$(get_constitution_version "$metadata_json")
        local doc_type=$(get_document_type "$metadata_json")
        local doc_version=$(get_document_version "$metadata_json")
        local last_updated=$(get_last_updated "$metadata_json")

        results["$file"]=$(echo "{
            \"constitution_version\": \"$const_version\",
            \"document_type\": \"$doc_type\",
            \"document_version\": \"$doc_version\",
            \"last_updated\": \"$last_updated\"
        }" | tr '\n' ' ')
    fi
}

# Function to sort results
sort_results() {
    local sorted_files=()

    case "$SORT_BY" in
        date)
            # Sort by last updated date (newest first)
            while IFS= read -r file; do
                sorted_files+=("$file")
            done < <(
                for file in "${!results[@]}"; do
                    if [[ "${results[$file]}" != "no_metadata" ]] && [[ "${results[$file]}" != "invalid_format" ]]; then
                        local metadata="${results[$file]}"
                        local date=$(echo "$metadata" | grep -o '"last_updated": "[^"]*"' | cut -d'"' -f4)
                        echo "$date|$file"
                    else
                        echo "|$file"
                    fi
                done | sort -r | cut -d'|' -f2
            )
            ;;
        version)
            # Sort by document version
            while IFS= read -r file; do
                sorted_files+=("$file")
            done < <(
                for file in "${!results[@]}"; do
                    if [[ "${results[$file]}" != "no_metadata" ]] && [[ "${results[$file]}" != "invalid_format" ]]; then
                        local metadata="${results[$file]}"
                        local version=$(echo "$metadata" | grep -o '"document_version": "[^"]*"' | cut -d'"' -f4)
                        echo "$version|$file"
                    else
                        echo "|$file"
                    fi
                done | sort -V | cut -d'|' -f2
            )
            ;;
        type)
            # Sort by document type
            while IFS= read -r file; do
                sorted_files+=("$file")
            done < <(
                for file in "${!results[@]}"; do
                    if [[ "${results[$file]}" != "no_metadata" ]] && [[ "${results[$file]}" != "invalid_format" ]]; then
                        local metadata="${results[$file]}"
                        local type=$(echo "$metadata" | grep -o '"document_type": "[^"]*"' | cut -d'"' -f4)
                        echo "$type|$file"
                    else
                        echo "|$file"
                    fi
                done | sort | cut -d'|' -f2
            )
            ;;
        path|*)
            # Sort by file path (alphabetical)
            while IFS= read -r file; do
                sorted_files+=("$file")
            done < <(printf '%s\n' "${!results[@]}" | sort)
            ;;
    esac

    echo "${sorted_files[@]}"
}

# Function to output results
output_results() {
    local sorted_files
    IFS=' ' read -ra sorted_files <<< "$(sort_results)"

    case "$OUTPUT_FORMAT" in
        json)
            echo "{"
            echo "  \"query\": {"
            echo "    \"type\": \"$QUERY_TYPE\","
            if [[ -n "$QUERY_VALUE" ]]; then
                echo "    \"value\": \"$QUERY_VALUE\","
            fi
            echo "    \"results_count\": ${#results[@]}"
            echo "  },"
            echo "  \"results\": ["

            local first=true
            for file in "${sorted_files[@]}"; do
                if [[ "$first" == false ]]; then
                    echo ","
                fi
                first=false

                echo -n "    {"
                echo -n "\"path\": \"$file\""

                if [[ "${results[$file]}" != "no_metadata" ]] && [[ "${results[$file]}" != "invalid_format" ]]; then
                    local metadata="${results[$file]}"
                    local const_version=$(echo "$metadata" | grep -o '"constitution_version": "[^"]*"' | cut -d'"' -f4)
                    local doc_type=$(echo "$metadata" | grep -o '"document_type": "[^"]*"' | cut -d'"' -f4)
                    local doc_version=$(echo "$metadata" | grep -o '"document_version": "[^"]*"' | cut -d'"' -f4)
                    local last_updated=$(echo "$metadata" | grep -o '"last_updated": "[^"]*"' | cut -d'"' -f4)

                    [[ -n "$const_version" ]] && echo -n ", \"constitution_version\": \"$const_version\""
                    [[ -n "$doc_type" ]] && echo -n ", \"document_type\": \"$doc_type\""
                    [[ -n "$doc_version" ]] && echo -n ", \"document_version\": \"$doc_version\""
                    [[ -n "$last_updated" ]] && echo -n ", \"last_updated\": \"$last_updated\""
                else
                    echo -n ", \"status\": \"${results[$file]}\""
                fi

                echo -n "}"
            done

            echo ""
            echo "  ]"
            echo "}"
            ;;

        paths)
            for file in "${sorted_files[@]}"; do
                echo "$file"
            done
            ;;

        text|*)
            echo "Query Results:"
            echo "  Type: $QUERY_TYPE"
            [[ -n "$QUERY_VALUE" ]] && echo "  Value: $QUERY_VALUE"
            echo "  Found: ${#results[@]} document(s)"
            echo ""

            for file in "${sorted_files[@]}"; do
                if [[ "${results[$file]}" != "no_metadata" ]] && [[ "${results[$file]}" != "invalid_format" ]]; then
                    local metadata="${results[$file]}"
                    local const_version=$(echo "$metadata" | grep -o '"constitution_version": "[^"]*"' | cut -d'"' -f4)
                    local doc_type=$(echo "$metadata" | grep -o '"document_type": "[^"]*"' | cut -d'"' -f4)
                    local doc_version=$(echo "$metadata" | grep -o '"document_version": "[^"]*"' | cut -d'"' -f4)
                    local last_updated=$(echo "$metadata" | grep -o '"last_updated": "[^"]*"' | cut -d'"' -f4)

                    echo "$file"
                    [[ -n "$const_version" ]] && echo "  Constitution: $const_version"
                    [[ -n "$doc_type" ]] && echo "  Type: $doc_type"
                    [[ -n "$doc_version" ]] && echo "  Version: $doc_version"
                    [[ -n "$last_updated" ]] && echo "  Updated: $last_updated"
                    echo ""
                else
                    echo "$file"
                    echo "  Status: ${results[$file]}"
                    echo ""
                fi
            done
            ;;
    esac
}

# Main logic
main() {
    if [[ "$VERBOSE" == true ]]; then
        echo -e "${BLUE}Searching for documents...${NC}"
    fi

    # Find all markdown files
    while IFS= read -r file; do
        process_file "$file"
    done < <(find . -name "*.md" -type f | grep -v ".bak" | sort)

    # Output results
    if [[ ${#results[@]} -eq 0 ]]; then
        if [[ "$OUTPUT_FORMAT" == "json" ]]; then
            echo '{"query": {"type": "'$QUERY_TYPE'", "value": "'$QUERY_VALUE'"}, "results": []}'
        else
            echo "No documents found matching the query"
        fi
        exit 0
    fi

    output_results
}

# Run main function
main