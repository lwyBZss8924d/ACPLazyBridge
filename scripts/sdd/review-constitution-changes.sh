#!/usr/bin/env bash
# review-constitution-changes.sh - Review constitution changes with CodeRabbit
# Part of the SDD (Specification-Driven Development) workflow
# Uses CodeRabbit CLI to ensure constitution update consistency

set -euo pipefail

# Get repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Script metadata
SCRIPT_NAME="$(basename "$0")"
SCRIPT_VERSION="1.0.0"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CODERABBIT_CONFIG="coderabbit.yaml"
CONSTITUTION_FILE=".specify/memory/constitution.md"
CHECKLIST_FILE=".specify/memory/constitution_update_checklist.md"
OUTPUT_DIR="_artifacts/reviews/constitution"
TIMESTAMP="$(date +%Y%m%d_%H%M%S)"

# Review options
REVIEW_TYPE="uncommitted"  # Default to uncommitted changes
BASE_BRANCH="main"
OUTPUT_FORMAT="plain"
VERBOSE=false
DRY_RUN=false

# List of CLAUDE.md files that must be synchronized
CLAUDE_FILES=(
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

# Template files that may need updates
TEMPLATE_FILES=(
    ".specify/templates/spec-template.md"
    ".specify/templates/plan-template.md"
    ".specify/templates/tasks-template.md"
)

# Command documentation files
COMMAND_FILES=(
    ".specify/commands/specify.md"
    ".specify/commands/plan.md"
    ".specify/commands/tasks.md"
    ".specify/commands/sdd-task.md"
)

# Function to print colored output
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to print header
print_header() {
    echo ""
    print_color "$BLUE" "╔══════════════════════════════════════════════════════════════╗"
    print_color "$BLUE" "║         CodeRabbit Constitution Review Tool v${SCRIPT_VERSION}         ║"
    print_color "$BLUE" "╚══════════════════════════════════════════════════════════════╝"
    echo ""
}

# Function to show usage
usage() {
    cat << EOF
Usage: $SCRIPT_NAME [OPTIONS]

Review constitution changes using CodeRabbit CLI to ensure consistency
across all CLAUDE.md files and dependent documents.

Options:
    -t, --type TYPE        Review type: all, committed, uncommitted (default: uncommitted)
    -b, --base BRANCH      Base branch for comparison (default: main)
    -f, --format FORMAT    Output format: plain, interactive (default: plain)
    -o, --output DIR       Output directory for review reports (default: _artifacts/reviews/constitution)
    -v, --verbose          Enable verbose output
    -d, --dry-run          Show what would be done without executing
    -h, --help             Display this help message

Examples:
    # Review uncommitted constitution changes
    $SCRIPT_NAME

    # Review all changes against main branch
    $SCRIPT_NAME --type all --base main

    # Review with verbose output
    $SCRIPT_NAME --verbose

    # Interactive review mode
    $SCRIPT_NAME --format interactive

    # Dry run to see what would be checked
    $SCRIPT_NAME --dry-run

EOF
    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--type)
            REVIEW_TYPE="$2"
            shift 2
            ;;
        -b|--base)
            BASE_BRANCH="$2"
            shift 2
            ;;
        -f|--format)
            OUTPUT_FORMAT="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -d|--dry-run)
            DRY_RUN=true
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo "Unknown option: $1"
            usage
            ;;
    esac
done

# Function to check prerequisites
check_prerequisites() {
    local errors=0

    # Check for CodeRabbit CLI
    if ! command -v coderabbit >/dev/null 2>&1; then
        print_color "$RED" "✗ CodeRabbit CLI not found. Please install it first:"
        echo "  curl -fsSL https://cli.coderabbit.ai/install.sh | sh"
        ((errors++))
    else
        print_color "$GREEN" "✓ CodeRabbit CLI found"
    fi

    # Check for configuration file
    if [[ ! -f "$CODERABBIT_CONFIG" ]]; then
        print_color "$RED" "✗ Configuration file not found: $CODERABBIT_CONFIG"
        ((errors++))
    else
        print_color "$GREEN" "✓ Configuration file found: $CODERABBIT_CONFIG"
    fi

    # Check authentication status
    if command -v coderabbit >/dev/null 2>&1; then
        if ! coderabbit auth status >/dev/null 2>&1; then
            print_color "$YELLOW" "⚠ CodeRabbit authentication may be required"
            echo "  Run: coderabbit auth login"
        fi
    fi

    # Check git repository
    if ! git rev-parse --git-dir >/dev/null 2>&1; then
        print_color "$RED" "✗ Not in a git repository"
        ((errors++))
    else
        print_color "$GREEN" "✓ Git repository found"
    fi

    if [[ $errors -gt 0 ]]; then
        print_color "$RED" "\n✗ Prerequisites check failed. Please fix the issues above."
        exit 1
    fi
}

# Function to detect constitution changes
detect_constitution_changes() {
    local has_changes=false

    print_color "$BLUE" "\n📋 Detecting constitution-related changes..."

    # Check for constitution file changes
    if git diff --name-only "$BASE_BRANCH" | grep -q "$CONSTITUTION_FILE"; then
        print_color "$YELLOW" "  ⚠ Constitution file modified: $CONSTITUTION_FILE"
        has_changes=true
    fi

    # Check for CLAUDE.md changes
    for file in "${CLAUDE_FILES[@]}"; do
        if [[ -f "$file" ]] && git diff --name-only "$BASE_BRANCH" | grep -q "$file"; then
            print_color "$YELLOW" "  ⚠ CLAUDE.md modified: $file"
            has_changes=true
        fi
    done

    # Check for template changes
    for file in "${TEMPLATE_FILES[@]}"; do
        if [[ -f "$file" ]] && git diff --name-only "$BASE_BRANCH" | grep -q "$file"; then
            print_color "$YELLOW" "  ⚠ Template modified: $file"
            has_changes=true
        fi
    done

    if [[ "$has_changes" == "false" ]]; then
        print_color "$GREEN" "  ✓ No constitution-related changes detected"
    fi

    echo "$has_changes"
}

# Function to check constitution version consistency
check_constitution_versions() {
    local current_version=""
    local inconsistent_files=()

    print_color "$BLUE" "\n🔍 Checking constitution version consistency..."

    # Get current constitution version
    if [[ -f "$CONSTITUTION_FILE" ]]; then
        current_version=$(grep -A1 "^constitution:" "$CONSTITUTION_FILE" 2>/dev/null | \
                          grep "version:" | sed 's/.*version:[[:space:]]*"\(.*\)"/\1/' | head -1)

        if [[ -n "$current_version" ]]; then
            print_color "$GREEN" "  Constitution version: $current_version"
        else
            print_color "$RED" "  ✗ Could not determine constitution version"
            return 1
        fi
    fi

    # Check each CLAUDE.md file
    for file in "${CLAUDE_FILES[@]}"; do
        if [[ -f "$file" ]]; then
            local file_version=$(grep -A1 "^constitution:" "$file" 2>/dev/null | \
                                grep "version:" | sed 's/.*version:[[:space:]]*"\(.*\)"/\1/' | head -1)

            if [[ -z "$file_version" ]]; then
                print_color "$YELLOW" "  ⚠ No version found in: $file"
                inconsistent_files+=("$file")
            elif [[ "$file_version" != "$current_version" ]]; then
                print_color "$RED" "  ✗ Version mismatch in $file: $file_version (expected: $current_version)"
                inconsistent_files+=("$file")
            elif [[ "$VERBOSE" == "true" ]]; then
                print_color "$GREEN" "  ✓ $file: $file_version"
            fi
        fi
    done

    if [[ ${#inconsistent_files[@]} -eq 0 ]]; then
        print_color "$GREEN" "  ✓ All CLAUDE.md files have consistent constitution version"
        return 0
    else
        print_color "$RED" "  ✗ Found ${#inconsistent_files[@]} files with inconsistent versions"
        return 1
    fi
}

# Function to run CodeRabbit review
run_coderabbit_review() {
    local review_output=""
    local review_file="${OUTPUT_DIR}/coderabbit_review_${TIMESTAMP}.txt"

    print_color "$BLUE" "\n🤖 Running CodeRabbit review..."

    # Create output directory
    mkdir -p "$OUTPUT_DIR"

    # Build CodeRabbit command
    local cmd="coderabbit review"
    cmd="$cmd --config $CODERABBIT_CONFIG"
    cmd="$cmd --type $REVIEW_TYPE"

    if [[ "$REVIEW_TYPE" != "uncommitted" ]]; then
        cmd="$cmd --base $BASE_BRANCH"
    fi

    if [[ "$OUTPUT_FORMAT" == "plain" ]]; then
        cmd="$cmd --plain"
    fi

    # Show command if verbose or dry-run
    if [[ "$VERBOSE" == "true" ]] || [[ "$DRY_RUN" == "true" ]]; then
        print_color "$BLUE" "  Command: $cmd"
    fi

    # Execute or simulate
    if [[ "$DRY_RUN" == "true" ]]; then
        print_color "$YELLOW" "  [DRY RUN] Would execute: $cmd"
        return 0
    fi

    # Run the review and capture output
    if [[ "$OUTPUT_FORMAT" == "plain" ]]; then
        # For plain output, save to file and display
        if $cmd > "$review_file" 2>&1; then
            print_color "$GREEN" "  ✓ Review completed successfully"
            print_color "$BLUE" "  Review saved to: $review_file"

            if [[ "$VERBOSE" == "true" ]]; then
                echo ""
                cat "$review_file"
            fi
        else
            print_color "$RED" "  ✗ Review failed"
            if [[ -f "$review_file" ]]; then
                cat "$review_file"
            fi
            return 1
        fi
    else
        # For interactive mode, run directly
        if $cmd; then
            print_color "$GREEN" "  ✓ Interactive review completed"
        else
            print_color "$RED" "  ✗ Review failed"
            return 1
        fi
    fi

    return 0
}

# Function to generate summary report
generate_summary_report() {
    local report_file="${OUTPUT_DIR}/constitution_review_summary_${TIMESTAMP}.md"

    print_color "$BLUE" "\n📊 Generating summary report..."

    if [[ "$DRY_RUN" == "true" ]]; then
        print_color "$YELLOW" "  [DRY RUN] Would generate report at: $report_file"
        return 0
    fi

    mkdir -p "$OUTPUT_DIR"

    cat > "$report_file" << EOF
# Constitution Review Summary

**Date:** $(date -u +"%Y-%m-%dT%H:%M:%SZ")
**Review Type:** $REVIEW_TYPE
**Base Branch:** $BASE_BRANCH

## Review Results

### Prerequisites Check
- ✅ CodeRabbit CLI installed
- ✅ Configuration file present
- ✅ Git repository valid

### Constitution Version Consistency
EOF

    # Add version check results
    if check_constitution_versions >/dev/null 2>&1; then
        echo "- ✅ All CLAUDE.md files have consistent constitution version" >> "$report_file"
    else
        echo "- ❌ Constitution version inconsistencies detected" >> "$report_file"
    fi

    cat >> "$report_file" << EOF

### Files Reviewed

#### CLAUDE.md Files (${#CLAUDE_FILES[@]} total)
EOF

    for file in "${CLAUDE_FILES[@]}"; do
        if [[ -f "$file" ]]; then
            echo "- [x] $file" >> "$report_file"
        else
            echo "- [ ] $file (not found)" >> "$report_file"
        fi
    done

    cat >> "$report_file" << EOF

#### Template Files (${#TEMPLATE_FILES[@]} total)
EOF

    for file in "${TEMPLATE_FILES[@]}"; do
        if [[ -f "$file" ]]; then
            echo "- [x] $file" >> "$report_file"
        else
            echo "- [ ] $file (not found)" >> "$report_file"
        fi
    done

    cat >> "$report_file" << EOF

#### Command Documentation (${#COMMAND_FILES[@]} total)
EOF

    for file in "${COMMAND_FILES[@]}"; do
        if [[ -f "$file" ]]; then
            echo "- [x] $file" >> "$report_file"
        else
            echo "- [ ] $file (not found)" >> "$report_file"
        fi
    done

    cat >> "$report_file" << EOF

## Review Artifacts

- CodeRabbit Review: ${OUTPUT_DIR}/coderabbit_review_${TIMESTAMP}.txt
- This Summary: ${report_file}

## Next Steps

1. Review the CodeRabbit feedback
2. Update any files with inconsistent constitution versions
3. Run validation scripts:
   \`\`\`bash
   ./scripts/sdd/validate-metadata.sh
   ./scripts/sdd/check-sdd-consistency.sh
   \`\`\`
4. Commit the synchronized changes

---

_Generated by review-constitution-changes.sh v${SCRIPT_VERSION}_
EOF

    print_color "$GREEN" "  ✓ Summary report saved to: $report_file"

    if [[ "$VERBOSE" == "true" ]]; then
        echo ""
        cat "$report_file"
    fi
}

# Function to run complementary validation scripts
run_validation_scripts() {
    print_color "$BLUE" "\n🔧 Running complementary validation scripts..."

    if [[ "$DRY_RUN" == "true" ]]; then
        print_color "$YELLOW" "  [DRY RUN] Would run validation scripts"
        return 0
    fi

    # Run metadata validation
    if [[ -x "scripts/sdd/validate-metadata.sh" ]]; then
        print_color "$BLUE" "  Running metadata validation..."
        if scripts/sdd/validate-metadata.sh --check-consistency >/dev/null 2>&1; then
            print_color "$GREEN" "    ✓ Metadata validation passed"
        else
            print_color "$YELLOW" "    ⚠ Metadata validation found issues"
        fi
    fi

    # Run consistency check
    if [[ -x "scripts/sdd/check-sdd-consistency.sh" ]]; then
        print_color "$BLUE" "  Running consistency check..."
        if scripts/sdd/check-sdd-consistency.sh >/dev/null 2>&1; then
            print_color "$GREEN" "    ✓ Consistency check passed"
        else
            print_color "$YELLOW" "    ⚠ Consistency check found issues"
        fi
    fi
}

# Main execution
main() {
    print_header

    # Check prerequisites
    print_color "$BLUE" "🔍 Checking prerequisites..."
    check_prerequisites

    # Detect changes
    has_changes=$(detect_constitution_changes)

    if [[ "$has_changes" == "false" ]] && [[ "$REVIEW_TYPE" == "uncommitted" ]]; then
        print_color "$GREEN" "\n✓ No constitution-related changes to review"
        exit 0
    fi

    # Check version consistency
    check_constitution_versions || true

    # Run CodeRabbit review
    run_coderabbit_review || true

    # Run validation scripts
    run_validation_scripts

    # Generate summary report
    generate_summary_report

    print_color "$GREEN" "\n✅ Constitution review completed!"
    print_color "$BLUE" "📁 Review artifacts saved to: $OUTPUT_DIR"
}

# Execute main function
main "$@"