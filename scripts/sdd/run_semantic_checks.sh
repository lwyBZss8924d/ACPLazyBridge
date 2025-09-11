#!/usr/bin/env bash

# SDD Semantic Consistency Checker
# Validates semantic consistency between docs and code

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔍 Running Semantic Consistency Checks..."
echo "========================================="

# Check if we have semantic tools available
check_semantic_tools() {
    if command -v rg &> /dev/null; then
        echo -e "${GREEN}✓${NC} ripgrep (rg) available"
        return 0
    else
        echo -e "${YELLOW}⚠${NC} ripgrep not found - install for better semantic search"
        echo "  Install with: brew install ripgrep (macOS) or apt install ripgrep (Linux)"
        return 1
    fi
}

# Check consistency between specs and implementation
check_spec_implementation() {
    echo ""
    echo "Checking Spec-Implementation Consistency..."
    
    if [ ! -d "specs" ]; then
        echo -e "${YELLOW}ℹ${NC} No specs directory found (expected for new projects)"
        return 0
    fi
    
    # Find all spec files
    SPEC_FILES=$(find specs -name "*.md" -type f 2>/dev/null || true)
    
    if [ -z "$SPEC_FILES" ]; then
        echo -e "${YELLOW}ℹ${NC} No specification files found yet"
        return 0
    fi
    
    # Check for common spec terms in code
    echo "  Checking for spec references in code..."
    
    # Look for TASK references
    if command -v rg &> /dev/null; then
        TASK_REFS=$(rg -c "TASK-[0-9]{3}" --type py --type js --type ts 2>/dev/null || echo "0")
        echo "  Found task references in code: $TASK_REFS"
    fi
    
    return 0
}

# Check documentation cross-references
check_doc_references() {
    echo ""
    echo "Checking Documentation Cross-References..."
    
    # Check if constitution references are valid
    if [ -f "dev-docs/sdd/constitution.md" ]; then
        echo "  Validating constitution references..."
        
        # Check for broken internal links
        BROKEN_LINKS=0
        for doc in dev-docs/**/*.md; do
            if [ -f "$doc" ]; then
                # Simple check for markdown links to non-existent files
                grep -o '\[.*\]([^)]*\.md)' "$doc" 2>/dev/null | while read -r link; do
                    FILE=$(echo "$link" | sed 's/.*(\(.*\))/\1/')
                    if [[ "$FILE" == /* ]]; then
                        # Absolute path
                        if [ ! -f ".$FILE" ]; then
                            echo -e "${YELLOW}⚠${NC} Possible broken link in $doc: $FILE"
                            ((BROKEN_LINKS++))
                        fi
                    fi
                done
            fi
        done
        
        if [ $BROKEN_LINKS -eq 0 ]; then
            echo -e "${GREEN}✓${NC} No broken documentation links found"
        fi
    fi
    
    return 0
}

# Check agent context consistency
check_agent_contexts() {
    echo ""
    echo "Checking Agent Context Consistency..."
    
    # Verify CLAUDE.md files have consistent structure
    CLAUDE_FILES=$(find dev-docs -name "CLAUDE.md" -type f 2>/dev/null || true)
    
    if [ -n "$CLAUDE_FILES" ]; then
        echo "  Found $(echo "$CLAUDE_FILES" | wc -l) CLAUDE.md files"
        
        # Check for required sections
        for file in $CLAUDE_FILES; do
            if ! grep -q "## Role" "$file"; then
                echo -e "${YELLOW}⚠${NC} Missing '## Role' section in $file"
            fi
            if ! grep -q "## Allowed Tools" "$file"; then
                echo -e "${YELLOW}⚠${NC} Missing '## Allowed Tools' section in $file"
            fi
        done
    fi
    
    # Similar check for AGENTS.md
    AGENTS_FILES=$(find dev-docs -name "AGENTS.md" -type f 2>/dev/null || true)
    
    if [ -n "$AGENTS_FILES" ]; then
        echo "  Found $(echo "$AGENTS_FILES" | wc -l) AGENTS.md files"
    fi
    
    return 0
}

# Check workflow consistency
check_workflow_consistency() {
    echo ""
    echo "Checking Workflow Consistency..."
    
    if [ -d ".github/workflows" ]; then
        WORKFLOW_COUNT=$(ls -1 .github/workflows/*.yml 2>/dev/null | wc -l)
        echo "  Found $WORKFLOW_COUNT workflow files"
        
        # Check if workflows reference scripts that exist
        for workflow in .github/workflows/*.yml; do
            # Extract script references
            SCRIPTS=$(grep -o 'scripts/[^"]*' "$workflow" 2>/dev/null || true)
            
            for script in $SCRIPTS; do
                if [ ! -f "$script" ]; then
                    echo -e "${YELLOW}⚠${NC} Workflow $workflow references non-existent script: $script"
                fi
            done
        done
        
        echo -e "${GREEN}✓${NC} Workflow consistency check complete"
    fi
    
    return 0
}

# Check template placeholders
check_template_placeholders() {
    echo ""
    echo "Checking for Unresolved Template Placeholders..."
    
    PLACEHOLDERS=(
        "\[PLACEHOLDER\]"
        "\[TODO\]"
        "{{[^}]*}}"
        "\[PRINCIPLE_[0-9]_NAME\]"
    )
    
    FOUND_PLACEHOLDERS=0
    
    for pattern in "${PLACEHOLDERS[@]}"; do
        if command -v rg &> /dev/null; then
            MATCHES=$(rg "$pattern" dev-docs --type md -c 2>/dev/null || true)
            if [ -n "$MATCHES" ]; then
                echo -e "${YELLOW}⚠${NC} Found placeholder pattern '$pattern':"
                echo "$MATCHES" | head -5
                ((FOUND_PLACEHOLDERS++))
            fi
        fi
    done
    
    if [ $FOUND_PLACEHOLDERS -eq 0 ]; then
        echo -e "${GREEN}✓${NC} No unresolved placeholders found"
    fi
    
    return 0
}

# Main execution
main() {
    local CHECKS_PASSED=true
    
    # Check for semantic tools
    if ! check_semantic_tools; then
        echo -e "${YELLOW}⚠${NC} Some semantic checks will be limited without ripgrep"
    fi
    
    # Run all checks
    check_spec_implementation || CHECKS_PASSED=false
    check_doc_references || CHECKS_PASSED=false
    check_agent_contexts || CHECKS_PASSED=false
    check_workflow_consistency || CHECKS_PASSED=false
    check_template_placeholders || CHECKS_PASSED=false
    
    echo ""
    echo "========================================="
    if [ "$CHECKS_PASSED" = true ]; then
        echo -e "${GREEN}✅ Semantic consistency checks complete${NC}"
        exit 0
    else
        echo -e "${YELLOW}⚠️  Semantic checks completed with warnings${NC}"
        exit 0  # Don't fail CI for warnings
    fi
}

# Run main
main "$@"