#!/usr/bin/env bash
# Enhanced SDD quality gates - comprehensive validation suite
# Combines structure, language, markdown, and semantic checks

set -euo pipefail

# Setup
ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || echo ".")"
cd "$ROOT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Error tracking
TOTAL_ERRORS=0
TOTAL_WARNINGS=0

# Logging functions
info() {
    echo -e "${GREEN}[SDD-GATES]${NC} $*"
}

warn() {
    echo -e "${YELLOW}[SDD-GATES] WARNING:${NC} $*" >&2
    ((TOTAL_WARNINGS++))
}

err() {
    echo -e "${RED}[SDD-GATES] ERROR:${NC} $*" >&2
    ((TOTAL_ERRORS++))
}

section() {
    echo
    echo "========================================"
    echo "$1"
    echo "========================================"
}

# Check if script exists and run it
run_check() {
    local script="$1"
    local description="$2"

    if [ -f "$script" ] && [ -x "$script" ]; then
        info "Running: $description"
        if ! "$script"; then
            err "$description failed"
            return 1
        fi
    else
        warn "Script not found or not executable: $script"
        return 1
    fi
    return 0
}

# Main execution
main() {
    info "Starting enhanced SDD quality gates validation"
    info "Repository root: $ROOT_DIR"

    # 1. SDD Structure Validation
    section "1. SDD STRUCTURE VALIDATION"
    if [ -f "scripts/ci/run-sdd-structure-lint.sh" ]; then
        run_check "scripts/ci/run-sdd-structure-lint.sh" "SDD structure lint"
    else
        warn "SDD structure lint script not found"
    fi

    # 2. Language Policy Check
    section "2. LANGUAGE POLICY CHECK"
    if [ -f "scripts/ci/check-language-policy.sh" ]; then
        run_check "scripts/ci/check-language-policy.sh" "Language policy (English-only normative)"
    else
        warn "Language policy script not found"
    fi

    # 3. Markdown Style Check
    section "3. MARKDOWN STYLE CHECK"
    if [ -f "scripts/ci/run-markdown-style.sh" ]; then
        run_check "scripts/ci/run-markdown-style.sh" "Markdown style validation"
    else
        warn "Markdown style script not found, checking with basic validation"
        # Basic markdown validation as fallback
        if command -v markdownlint >/dev/null 2>&1; then
            info "Running markdownlint on documentation"
            if ! markdownlint "**/*.md" 2>/dev/null; then
                warn "Markdown linting found issues"
            fi
        fi
    fi

    # 4. Python SDD Structure Validation
    section "4. PYTHON SDD STRUCTURE VALIDATION"
    if [ -f "scripts/sdd/validate_structure.py" ]; then
        info "Running Python structure validation"
        if ! python3 scripts/sdd/validate_structure.py .; then
            err "Python structure validation failed"
        fi
    else
        warn "Python structure validation script not found"
    fi

    # 5. Semantic Checks
    section "5. SEMANTIC VALIDATION"
    if [ -f "scripts/sdd/run_semantic_checks.sh" ]; then
        run_check "scripts/sdd/run_semantic_checks.sh" "Semantic validation (cross-references)"
    else
        warn "Semantic validation script not found"
    fi

    # 6. Check for required SDD artifacts
    section "6. SDD ARTIFACT VALIDATION"
    info "Checking for required SDD artifacts..."

    # Check .specify directory
    if [ -d ".specify" ]; then
        info "✓ .specify directory exists"

        # Check for constitution
        if [ -f ".specify/memory/constitution.md" ]; then
            info "✓ Constitution found"
        else
            err "Missing .specify/memory/constitution.md"
        fi

        # Check for lifecycle
        if [ -f ".specify/memory/lifecycle.md" ]; then
            info "✓ Lifecycle document found"
        else
            err "Missing .specify/memory/lifecycle.md"
        fi

        # Check for templates
        for template in spec-template.md plan-template.md tasks-template.md; do
            if [ -f ".specify/templates/$template" ]; then
                info "✓ Template found: $template"
            else
                err "Missing template: .specify/templates/$template"
            fi
        done
    else
        err ".specify directory not found"
    fi

    # Check sdd-rules directory
    if [ -d "sdd-rules" ]; then
        info "✓ sdd-rules directory exists"

        if [ -f "sdd-rules/CLAUDE.md" ]; then
            info "✓ Claude rules found"
        else
            err "Missing sdd-rules/CLAUDE.md"
        fi

        if [ -f "sdd-rules/AGENTS.md" ]; then
            info "✓ Agents configuration found"
        else
            err "Missing sdd-rules/AGENTS.md"
        fi
    else
        err "sdd-rules directory not found"
    fi

    # 7. Check for specs directory with at least one spec
    section "7. SPECIFICATION DIRECTORY CHECK"
    if [ -d "specs" ]; then
        info "✓ specs directory exists"
        spec_count=$(find specs -maxdepth 1 -type d -name "[0-9]*" 2>/dev/null | wc -l | tr -d ' ')
        if [ "$spec_count" -gt 0 ]; then
            info "✓ Found $spec_count specification(s)"
        else
            warn "No numbered specifications found in specs/"
        fi
    else
        err "specs directory not found"
    fi

    # 8. Check for evidence directory
    section "8. EVIDENCE DIRECTORY CHECK"
    if [ -d "_artifacts" ] || [ -d "dev-docs/review/_artifacts" ]; then
        info "✓ Evidence directory exists"
    else
        warn "No evidence directory found (_artifacts or dev-docs/review/_artifacts)"
    fi

    # 9. Check for NEEDS CLARIFICATION markers
    section "9. CHECKING FOR UNRESOLVED CLARIFICATIONS"
    if grep -r "\[NEEDS CLARIFICATION\]" specs/ --include="*.md" 2>/dev/null; then
        warn "Found [NEEDS CLARIFICATION] markers in specifications"
    else
        info "✓ No unresolved clarification markers"
    fi

    # 10. Metadata validation
    section "10. METADATA VALIDATION"
    if [ -f "scripts/sdd/validate-metadata.sh" ]; then
        run_check "scripts/sdd/validate-metadata.sh" "YAML metadata validation"
    else
        warn "Metadata validation script not found"
    fi

    # Summary
    section "VALIDATION SUMMARY"
    info "Total Errors: $TOTAL_ERRORS"
    info "Total Warnings: $TOTAL_WARNINGS"

    if [ $TOTAL_ERRORS -gt 0 ]; then
        err "SDD quality gates FAILED with $TOTAL_ERRORS error(s)"
        exit 1
    elif [ $TOTAL_WARNINGS -gt 0 ]; then
        warn "SDD quality gates passed with $TOTAL_WARNINGS warning(s)"
        exit 0
    else
        info "✓ All SDD quality gates PASSED"
        exit 0
    fi
}

# Run main function
main "$@"