#!/usr/bin/env bash
# validate-sdd-docs.sh - Comprehensive SDD document validator
# Part of the SDD (Specification-Driven Development) workflow
# Uses yq, ast-grep, and ripgrep for multi-layer validation

set -euo pipefail

# Get repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

# Source metadata utilities
source "$REPO_ROOT/scripts/sdd/lib/metadata-utils.sh"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_ERRORS=0
TOTAL_WARNINGS=0
TOTAL_SPECS=0

# Logging functions
info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

success() {
    echo -e "${GREEN}[✓]${NC} $*"
}

warn() {
    echo -e "${YELLOW}[⚠]${NC} $*"
    ((TOTAL_WARNINGS++)) || true
}

error() {
    echo -e "${RED}[✗]${NC} $*"
    ((TOTAL_ERRORS++)) || true
}

# Extract YAML frontmatter from markdown file
extract_yaml_frontmatter() {
    local file="$1"

    # Check if file exists and is readable
    if [[ ! -r "$file" ]]; then
        return 1
    fi

    # Extract content between ```yaml and ```
    # Use awk for more reliable processing
    awk '/^```yaml$/{flag=1;next}/^```$/{flag=0}flag' "$file" 2>/dev/null || echo ""
}

# Validate YAML frontmatter against template requirements
validate_yaml_frontmatter() {
    local file="$1"
    local doc_type="$2"  # spec, plan, or tasks

    info "Checking YAML metadata in $(basename "$file")"

    # Extract YAML content
    local yaml_content
    yaml_content=$(extract_yaml_frontmatter "$file")

    if [[ -z "$yaml_content" ]]; then
        error "No YAML frontmatter found in $file"
        return 1
    fi

    # Validate YAML syntax (with timeout to prevent hanging)
    if ! echo "$yaml_content" | timeout 2 yq eval '.' - > /dev/null 2>&1; then
        error "Invalid YAML syntax in $file"
        return 1
    fi

    # Check required fields based on document type
    local required_fields=()

    # Common fields for all documents
    required_fields+=("worktree" "feature_branch" "status")

    # Type-specific fields
    case "$doc_type" in
        spec)
            required_fields+=("created" "last_updated" "input" "specs.constitution" "specs.type" "specs.feature_number")
            ;;
        plan)
            required_fields+=("spec_uri" "plan_uri" "tasks_uri" "evidence_uris")
            ;;
        tasks)
            required_fields+=("spec_uri" "plan_uri" "tasks_uri" "evidence_uris")
            ;;
    esac

    # Validate each required field
    local missing_fields=()
    for field in "${required_fields[@]}"; do
        local value
        value=$(echo "$yaml_content" | timeout 2 yq eval ".$field" - 2>/dev/null || echo "null")

        if [[ "$value" == "null" ]] || [[ -z "$value" ]]; then
            missing_fields+=("$field")
        elif [[ "$value" == *"["*"]"* ]]; then
            # Check for placeholder patterns like [PLACEHOLDER]
            warn "Placeholder value in field '$field': $value"
        fi
    done

    if [[ ${#missing_fields[@]} -gt 0 ]]; then
        error "Missing required fields in $file: ${missing_fields[*]}"
        return 1
    fi

    success "YAML metadata valid"
    return 0
}

# Validate document sections match template
validate_sections() {
    local doc="$1"
    local template="$2"
    local doc_type="$3"

    info "Checking document structure against template"

    if [[ ! -f "$template" ]]; then
        warn "Template not found: $template"
        return 1
    fi

    # Extract required sections from template (## level headers)
    local template_sections=()
    while IFS= read -r line; do
        if [[ "$line" =~ ^##[[:space:]]+(.+)$ ]]; then
            local section="${BASH_REMATCH[1]}"
            # Skip optional section markers
            if [[ "$section" != *"(optional)"* ]] && [[ "$section" != *"_(optional)_"* ]]; then
                # Remove any trailing markers like _(mandatory)_
                section=$(echo "$section" | sed 's/ *_(mandatory)_//')
                template_sections+=("$section")
            fi
        fi
    done < "$template"

    # Check each required section exists in document
    local missing_sections=()
    for section in "${template_sections[@]}"; do
        # Handle section variations (e.g., "Requirements" vs "Functional Requirements")
        if ! grep -Fq "## $section" "$doc"; then
            missing_sections+=("$section")
        fi
    done

    if [[ ${#missing_sections[@]} -gt 0 ]]; then
        warn "Missing sections in $doc: ${missing_sections[*]}"
    else
        success "Document structure matches template"
    fi
}

# Check for unresolved placeholders and clarifications
check_placeholders() {
    local file="$1"

    info "Checking for unresolved placeholders"

    # Check for [NEEDS CLARIFICATION] markers
    local clarifications
    clarifications=$(grep -o '\[NEEDS CLARIFICATION[^]]*\]' "$file" 2>/dev/null || true)
    if [[ -n "$clarifications" ]]; then
        warn "Unresolved clarifications in $file:"
        echo "$clarifications" | while IFS= read -r line; do
            echo "    $line"
        done
    fi

    # Check for generic placeholders
    local placeholders
    placeholders=$(grep -o '\[PLACEHOLDER[^]]*\]' "$file" 2>/dev/null || true)
    if [[ -n "$placeholders" ]]; then
        warn "Unresolved placeholders in $file:"
        echo "$placeholders" | while IFS= read -r line; do
            echo "    $line"
        done
    fi

    # Check for template markers that should be replaced
    if grep -q '\[FEATURE NAME\]\|\[FEATURE\]\|\[DATE\]\|\[STATUS\]' "$file"; then
        warn "Unreplaced template markers found in $file"
    fi
}

# Validate task numbering and structure
validate_tasks() {
    local file="$1"

    info "Checking task structure"

    # Check for proper task numbering (T001, T002, etc.)
    if [[ -f "$file" ]]; then
        if ! grep -q 'T[0-9][0-9][0-9]' "$file"; then
            warn "No standard task numbering (T001, T002...) found in $file"
        fi

        # Check for [P] parallel task markers
        local parallel_count
        # grep -c prints 0 even when exit status is 1 (no matches), so avoid
        # appending an extra "0" which would result in "0\n0". Use `|| true`
        # to prevent set -e from exiting on status 1 while keeping grep output.
        parallel_count=$(grep -c '\[P\]' "$file" 2>/dev/null || true)
        if [[ "$parallel_count" -gt 0 ]]; then
            info "Found $parallel_count parallel tasks marked with [P]"
        fi

        # Check for evidence paths
        if ! grep -q '_artifacts/\|_artifacts/legacy/' "$file"; then
            warn "No evidence paths found in tasks"
        fi
    fi
}

# Use ast-grep for additional validation if available
check_with_astgrep() {
    local file="$1"

    # Check if ast-grep is available
    if ! command -v ast-grep &> /dev/null; then
        return 0
    fi

    # Try to find patterns with ast-grep
    # Note: ast-grep may not support markdown/yaml directly, so we use it opportunistically

    # Check for TODO patterns
    local todos
    todos=$(ast-grep -p 'TODO' "$file" 2>/dev/null || rg 'TODO|FIXME|XXX' "$file" 2>/dev/null || true)
    if [[ -n "$todos" ]]; then
        warn "TODO/FIXME markers found in $file"
    fi
}

# Validate cross-references between spec/plan/tasks
validate_cross_references() {
    local spec_dir="$1"

    info "Checking cross-references"

    # Extract URIs from plan.md
    if [[ -f "$spec_dir/plan.md" ]]; then
        local yaml_content
        yaml_content=$(extract_yaml_frontmatter "$spec_dir/plan.md")

        if [[ -n "$yaml_content" ]]; then
            local spec_uri plan_uri tasks_uri
            spec_uri=$(echo "$yaml_content" | yq eval '.spec_uri' - 2>/dev/null)
            plan_uri=$(echo "$yaml_content" | yq eval '.plan_uri' - 2>/dev/null)
            tasks_uri=$(echo "$yaml_content" | yq eval '.tasks_uri' - 2>/dev/null)

            # Validate URIs point to correct files
            if [[ "$spec_uri" != "null" ]] && [[ ! "$spec_uri" =~ $(basename "$spec_dir") ]]; then
                warn "spec_uri doesn't match directory: $spec_uri"
            fi
        fi
    fi
}

# Main validation for a spec directory
validate_spec_directory() {
    local spec_dir="$1"
    local spec_name=$(basename "$spec_dir")

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📁 Validating: $spec_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    ((TOTAL_SPECS+=1))

    # Check spec.md
    if [[ -f "$spec_dir/spec.md" ]]; then
        if ! validate_yaml_frontmatter "$spec_dir/spec.md" "spec"; then :; fi
        if ! validate_sections "$spec_dir/spec.md" "$REPO_ROOT/.specify/templates/spec-template.md" "spec"; then :; fi
        if ! check_placeholders "$spec_dir/spec.md"; then :; fi
        if ! check_with_astgrep "$spec_dir/spec.md"; then :; fi
    else
        error "Missing spec.md"
    fi

    # Check plan.md
    if [[ -f "$spec_dir/plan.md" ]]; then
        if ! validate_yaml_frontmatter "$spec_dir/plan.md" "plan"; then :; fi
        if ! validate_sections "$spec_dir/plan.md" "$REPO_ROOT/.specify/templates/plan-template.md" "plan"; then :; fi
        if ! check_placeholders "$spec_dir/plan.md"; then :; fi
        if ! check_with_astgrep "$spec_dir/plan.md"; then :; fi
    else
        warn "Missing plan.md (may not be created yet)"
    fi

    # Check tasks.md
    if [[ -f "$spec_dir/tasks.md" ]]; then
        if ! validate_yaml_frontmatter "$spec_dir/tasks.md" "tasks"; then :; fi
        if ! validate_sections "$spec_dir/tasks.md" "$REPO_ROOT/.specify/templates/tasks-template.md" "tasks"; then :; fi
        if ! validate_tasks "$spec_dir/tasks.md"; then :; fi
        if ! check_placeholders "$spec_dir/tasks.md"; then :; fi
        if ! check_with_astgrep "$spec_dir/tasks.md"; then :; fi
    else
        warn "Missing tasks.md (may not be created yet)"
    fi

    # Validate cross-references
    if ! validate_cross_references "$spec_dir"; then :; fi
}

# Main function
main() {
    echo "🔍 SDD Document Validation"
    echo "Version: 1.0.0"
    echo "Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo ""

    # Check dependencies
    if ! command -v yq &> /dev/null; then
        error "yq is required but not installed"
        echo "Install with: brew install yq (macOS) or check https://github.com/mikefarah/yq"
        exit 2
    fi

    if ! command -v rg &> /dev/null; then
        warn "ripgrep (rg) not found, some checks will be limited"
    fi

    if ! command -v ast-grep &> /dev/null; then
        info "ast-grep not found, structural checks will be skipped"
    fi

    # Find all spec directories
    local spec_dirs=()
    if [[ -d "$REPO_ROOT/specs" ]]; then
        while IFS= read -r -d '' dir; do
            spec_dirs+=("$dir")
        done < <(find "$REPO_ROOT/specs" -mindepth 1 -maxdepth 1 -type d -print0 | sort -z)
    fi

    if [[ ${#spec_dirs[@]} -eq 0 ]]; then
        warn "No spec directories found in $REPO_ROOT/specs/"
        exit 0
    fi

    # Validate each spec directory
    local spec_filter="${SPEC_FILTER:-}"

    for spec_dir in "${spec_dirs[@]}"; do
        local spec_name
        spec_name=$(basename "$spec_dir")

        if [[ -n "$spec_filter" && ! "$spec_name" == $spec_filter ]]; then
            info "Skipping spec directory due to filter ($spec_filter): $spec_name"
            continue
        fi

        # Skip example directories that intentionally omit metadata
        if [[ "$spec_name" == "000-example" ]]; then
            info "Skipping example directory: $spec_name"
            continue
        fi

        validate_spec_directory "$spec_dir"
    done

    # Print summary
    echo ""
    echo "════════════════════════════════════════"
    echo "📊 Validation Summary"
    echo "════════════════════════════════════════"
    echo "Specs validated: $TOTAL_SPECS"

    if [[ $TOTAL_ERRORS -gt 0 ]]; then
        echo -e "${RED}Errors: $TOTAL_ERRORS${NC}"
    else
        echo -e "${GREEN}Errors: 0${NC}"
    fi

    if [[ $TOTAL_WARNINGS -gt 0 ]]; then
        echo -e "${YELLOW}Warnings: $TOTAL_WARNINGS${NC}"
    else
        echo -e "${GREEN}Warnings: 0${NC}"
    fi

    echo ""

    # Exit with appropriate code
    if [[ $TOTAL_ERRORS -gt 0 ]]; then
        echo -e "${RED}❌ Validation failed - please fix errors${NC}"
        exit 1
    elif [[ $TOTAL_WARNINGS -gt 0 ]]; then
        echo -e "${YELLOW}⚠️  Validation passed with warnings${NC}"
        exit 0
    else
        echo -e "${GREEN}✅ All SDD documents valid${NC}"
        exit 0
    fi
}

# Parse command line options
while [[ $# -gt 0 ]]; do
    case "$1" in
        --help|-h)
            cat << EOF
Usage: $0 [OPTIONS]

Validate SDD specification documents against templates.

Options:
    --help, -h    Show this help message
    --verbose     Enable verbose output
    --strict      Treat warnings as errors

This script validates:
  - YAML frontmatter syntax and required fields
  - Document structure matches templates
  - No unresolved placeholders or clarifications
  - Proper task numbering and evidence paths
  - Cross-references between spec/plan/tasks

Dependencies:
  - yq (required): YAML parser
  - ripgrep (optional): Enhanced text search
  - ast-grep (optional): Structural pattern matching

Exit codes:
  0 - Validation passed (with or without warnings)
  1 - Validation errors found
  2 - Missing dependencies

EOF
            exit 0
            ;;
        --verbose)
            set -x
            shift
            ;;
        --strict)
            # In strict mode, warnings become errors
            warn() {
                error "$@"
            }
            shift
            ;;
        *)
            error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Run main validation
main
