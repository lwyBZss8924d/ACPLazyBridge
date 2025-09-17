#!/usr/bin/env bash
# migrate-to-yaml-metadata.sh - Migrate SDD document metadata to YAML format
# Part of the SDD (Specification-Driven Development) workflow

set -euo pipefail

# Get repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script options
DRY_RUN=false
VERBOSE=false
SPECIFIC_FILE=""
BACKUP=true
FORCE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --file)
            SPECIFIC_FILE="$2"
            shift 2
            ;;
        --no-backup)
            BACKUP=false
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --help|-h)
            cat << EOF
Usage: $0 [OPTIONS]

Migrate SDD document metadata from various formats to unified YAML format.

Options:
    --dry-run       Show what would be changed without modifying files
    --verbose, -v   Show detailed output
    --file FILE     Process specific file only
    --no-backup     Don't create backup files
    --force         Force migration even if already in YAML format
    --help, -h      Show this help message

Examples:
    $0 --dry-run                    # Preview changes
    $0                               # Migrate all files
    $0 --file sdd-rules/CLAUDE.md   # Migrate specific file

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

# Function to detect metadata format type
detect_format() {
    local file="$1"
    local last_lines=$(tail -10 "$file" 2>/dev/null || echo "")

    # Check for existing YAML format
    if echo "$last_lines" | grep -q '```yaml'; then
        echo "yaml"
    # Check for "Based on Constitution" format (CLAUDE.md files)
    elif echo "$last_lines" | grep -q "Based on Constitution:"; then
        echo "constitution"
    # Check for "specification_version" format (SDD rules)
    elif echo "$last_lines" | grep -q "specification_version:"; then
        echo "specification"
    # Check for simple version format
    elif echo "$last_lines" | grep -q "^Version:"; then
        echo "simple"
    else
        echo "none"
    fi
}

# Function to extract metadata from "Based on Constitution" format
extract_constitution_metadata() {
    local file="$1"
    local metadata=$(tail -5 "$file" | grep -E "Based on Constitution.*\|.*Last Updated" || echo "")

    if [[ -n "$metadata" ]]; then
        # Extract Constitution version
        local const_ver=$(echo "$metadata" | sed -n 's/.*Based on Constitution:* \([0-9.]*\).*/\1/p')
        # Extract document path and version
        local doc_info=$(echo "$metadata" | sed -n 's/.*| *\((.*)\) *: *\([0-9.]*\).*/\1|\2/p')
        if [[ -z "$doc_info" ]]; then
            doc_info=$(echo "$metadata" | sed -n 's/.*| *\([^:|]*\) *: *\([0-9.]*\).*/\1|\2/p')
        fi
        local doc_path=$(echo "$doc_info" | cut -d'|' -f1 | tr -d '()')
        local doc_ver=$(echo "$doc_info" | cut -d'|' -f2)
        # Extract last updated date
        local last_updated=$(echo "$metadata" | sed -n 's/.*Last Updated: \([0-9-]*\).*/\1/p')

        echo "$const_ver|$doc_path|$doc_ver|$last_updated"
    fi
}

# Function to extract metadata from "specification_version" format
extract_specification_metadata() {
    local file="$1"
    # Try single line format first
    local metadata=$(tail -5 "$file" | grep -E "specification_version:.*\|.*Last Updated" || echo "")

    # If not found, try multi-line format
    if [[ -z "$metadata" ]]; then
        local spec_line=$(tail -5 "$file" | grep "specification_version:")
        local last_line=$(tail -1 "$file")
        if [[ -n "$spec_line" ]] && [[ -n "$last_line" ]]; then
            metadata="$spec_line $last_line"
        fi
    fi

    if [[ -n "$metadata" ]]; then
        # Extract specification version
        local spec_ver=$(echo "$metadata" | sed -n 's/.*specification_version: \([0-9.]*\).*/\1/p')
        # Extract rules name and version
        local rules_info=$(echo "$metadata" | sed -n 's/.*| *\([^:|]*\) Format: *\([0-9.]*\).*/\1|\2/p')
        local rules_name=$(echo "$rules_info" | cut -d'|' -f1)
        local rules_ver=$(echo "$rules_info" | cut -d'|' -f2)
        # Extract last updated date
        local last_updated=$(echo "$metadata" | sed -n 's/.*Last Updated: \([0-9-]*\).*/\1/p')

        echo "$spec_ver|$rules_name|$rules_ver|$last_updated"
    fi
}

# Function to generate YAML metadata based on file type
generate_yaml_metadata() {
    local file="$1"
    local format="$2"
    local today=$(date +%Y-%m-%d)

    case "$format" in
        constitution)
            local metadata=$(extract_constitution_metadata "$file")
            if [[ -n "$metadata" ]]; then
                IFS='|' read -r const_ver doc_path doc_ver last_updated <<< "$metadata"
                # Use existing date if available, otherwise use today
                [[ -z "$last_updated" ]] && last_updated="$today"
                [[ -z "$const_ver" ]] && const_ver="1.0.1"
                [[ -z "$doc_ver" ]] && doc_ver="1.0.0"
                [[ -z "$doc_path" ]] && doc_path="$file"

                cat << EOF

---

\`\`\`yaml
Constitution version: $const_ver
Document: $doc_path
Document version: $doc_ver
Last Updated: $last_updated
\`\`\`
EOF
            fi
            ;;

        specification)
            local metadata=$(extract_specification_metadata "$file")
            if [[ -n "$metadata" ]]; then
                IFS='|' read -r spec_ver rules_name rules_ver last_updated <<< "$metadata"
                # Use existing date if available, otherwise use today
                [[ -z "$last_updated" ]] && last_updated="$today"
                [[ -z "$spec_ver" ]] && spec_ver="1.0.0"
                [[ -z "$rules_ver" ]] && rules_ver="1.0"
                [[ -z "$rules_name" ]] && rules_name=$(basename "$file" .md)

                # Default constitution version for rules
                local const_ver="1.0.1"

                cat << EOF

---

\`\`\`yaml
Constitution version: $const_ver
Rules: $rules_name
Rules version: $rules_ver
Last Updated: $last_updated
\`\`\`
EOF
            fi
            ;;

        none)
            # Generate default metadata based on file location
            local doc_type="Document"
            local doc_path="$file"

            if [[ "$file" == *"/rules/"* ]]; then
                doc_type="Rules"
                doc_path=$(basename "$file" .md)
            elif [[ "$file" == *"CLAUDE.md" ]]; then
                doc_path="${file#./}"
            fi

            cat << EOF

---

\`\`\`yaml
Constitution version: 1.0.1
$doc_type: $doc_path
${doc_type} version: 1.0.0
Last Updated: $today
\`\`\`
EOF
            ;;
    esac
}

# Function to remove old metadata
remove_old_metadata() {
    local file="$1"
    local format="$2"
    local temp_file=$(mktemp)

    case "$format" in
        constitution|specification)
            # Remove the metadata line(s) at the end
            local line_count=$(wc -l < "$file")
            local check_lines=5
            local found_metadata=false
            local lines_to_remove=0

            # Check last few lines for metadata pattern
            for i in $(seq 1 $check_lines); do
                local line=$(tail -$i "$file" | head -1)
                if echo "$line" | grep -qE "(Based on Constitution:|specification_version:)"; then
                    found_metadata=true
                    lines_to_remove=$i
                    # For multi-line specification format, check if we need to remove more lines
                    if echo "$line" | grep -q "specification_version:"; then
                        # Check if next line has "Last Updated"
                        local next_line=$(tail -$((i-1)) "$file" | head -1 2>/dev/null)
                        if echo "$next_line" | grep -q "Last Updated:"; then
                            lines_to_remove=$((i+1))
                        fi
                    fi
                    # Copy all but the last lines (macOS compatible)
                    local total_lines=$(wc -l < "$file")
                    local keep_lines=$((total_lines - lines_to_remove))
                    if [[ $keep_lines -gt 0 ]]; then
                        head -n $keep_lines "$file" > "$temp_file"
                        # Remove only trailing blank lines at the end
                        local temp_clean=$(mktemp)
                        # Use perl for reliable trailing blank line removal
                        perl -pe 'chomp if eof' "$temp_file" > "$temp_clean"
                        mv "$temp_clean" "$temp_file"
                    else
                        touch "$temp_file"
                    fi
                    break
                fi
            done

            if [[ "$found_metadata" == false ]]; then
                cp "$file" "$temp_file"
            fi
            ;;

        yaml)
            # Already in YAML format, return as-is unless forced
            if [[ "$FORCE" == true ]]; then
                # Remove existing YAML block while preserving content
                local in_yaml_block=false
                local yaml_started=false
                while IFS= read -r line; do
                    if [[ "$line" == "---" ]] && [[ "$yaml_started" == false ]]; then
                        yaml_started=true
                        in_yaml_block=true
                    elif [[ "$line" == '```' ]] && [[ "$in_yaml_block" == true ]]; then
                        in_yaml_block=false
                        continue
                    elif [[ "$in_yaml_block" == false ]]; then
                        echo "$line" >> "$temp_file"
                    fi
                done < "$file"
                # Remove trailing blank lines
                if [[ -s "$temp_file" ]]; then
                    local temp_clean=$(mktemp)
                    perl -pe 'chomp if eof' "$temp_file" > "$temp_clean"
                    mv "$temp_clean" "$temp_file"
                fi
            else
                cp "$file" "$temp_file"
            fi
            ;;

        *)
            cp "$file" "$temp_file"
            ;;
    esac

    echo "$temp_file"
}

# Function to migrate a single file
migrate_file() {
    local file="$1"
    local format=$(detect_format "$file")

    if [[ "$VERBOSE" == true ]]; then
        echo -e "${BLUE}Processing: $file (format: $format)${NC}"
    fi

    # Skip if already in YAML format and not forced
    if [[ "$format" == "yaml" ]] && [[ "$FORCE" == false ]]; then
        if [[ "$VERBOSE" == true ]]; then
            echo -e "${GREEN}  ✓ Already in YAML format${NC}"
        fi
        return 0
    fi

    # Skip if no metadata found and not a target file
    if [[ "$format" == "none" ]]; then
        # Check if this is a file that should have metadata
        if [[ ! "$file" =~ (CLAUDE\.md|sdd-rules/.*\.md|\.specify/.*\.md) ]]; then
            if [[ "$VERBOSE" == true ]]; then
                echo -e "${YELLOW}  ⚠ No metadata found, skipping${NC}"
            fi
            return 0
        fi
    fi

    # Generate new YAML metadata
    local new_metadata=$(generate_yaml_metadata "$file" "$format")

    if [[ -z "$new_metadata" ]]; then
        echo -e "${YELLOW}  ⚠ Could not generate metadata for $file${NC}"
        return 1
    fi

    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${YELLOW}Would migrate: $file${NC}"
        echo "New metadata:"
        echo "$new_metadata"
        return 0
    fi

    # Create backup if requested
    if [[ "$BACKUP" == true ]]; then
        cp "$file" "${file}.bak"
    fi

    # Remove old metadata and add new
    local clean_file=$(remove_old_metadata "$file" "$format")

    # Verify the clean file has content (not just empty or metadata)
    local clean_line_count=$(wc -l < "$clean_file")
    if [[ ! -s "$clean_file" ]] || [[ $clean_line_count -lt 2 ]]; then
        echo -e "${RED}  ✗ Error: File content would be lost for $file${NC}"
        echo -e "${RED}    Clean file has $clean_line_count lines${NC}" >&2
        rm "$clean_file"
        return 1
    fi

    # Create new file with preserved content and new metadata
    local temp_output=$(mktemp)
    cat "$clean_file" > "$temp_output"
    echo "$new_metadata" >> "$temp_output"

    # Only overwrite if the new file is valid
    if [[ -s "$temp_output" ]]; then
        mv "$temp_output" "$file"
    else
        echo -e "${RED}  ✗ Error: Failed to create valid output for $file${NC}"
        rm "$temp_output" "$clean_file" 2>/dev/null
        return 1
    fi

    rm "$clean_file" 2>/dev/null

    echo -e "${GREEN}✓ Migrated: $file${NC}"
    return 0
}

# Main migration logic
main() {
    local total_files=0
    local migrated_files=0
    local skipped_files=0
    local error_files=0

    # Determine which files to process
    local files_to_process=()

    if [[ -n "$SPECIFIC_FILE" ]]; then
        if [[ ! -f "$SPECIFIC_FILE" ]]; then
            echo -e "${RED}Error: File not found: $SPECIFIC_FILE${NC}"
            exit 1
        fi
        files_to_process=("$SPECIFIC_FILE")
    else
        # Find all markdown files that should have metadata
        while IFS= read -r file; do
            files_to_process+=("$file")
        done < <(find . -name "*.md" -type f | grep -E "(CLAUDE\.md|sdd-rules/|\.specify/)" | grep -v ".bak" | sort)
    fi

    echo -e "${BLUE}Starting metadata migration...${NC}"
    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${YELLOW}DRY RUN MODE - No files will be modified${NC}"
    fi
    echo ""

    # Process each file
    for file in "${files_to_process[@]}"; do
        total_files=$((total_files + 1))

        if migrate_file "$file"; then
            migrated_files=$((migrated_files + 1))
        else
            error_files=$((error_files + 1))
        fi
    done

    # Summary
    echo ""
    echo "Migration Summary:"
    echo "  Total files processed: $total_files"
    echo -e "  ${GREEN}Successfully migrated: $migrated_files${NC}"
    if [[ $error_files -gt 0 ]]; then
        echo -e "  ${RED}Errors: $error_files${NC}"
    fi

    if [[ "$DRY_RUN" == true ]]; then
        echo ""
        echo "This was a dry run. Run without --dry-run to apply changes."
    fi

    if [[ $error_files -gt 0 ]]; then
        exit 1
    fi
}

# Run main function
main