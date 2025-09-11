#!/usr/bin/env bash

# Documentation Linting Script
# Validates markdown documentation against style guidelines

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}📝 Documentation Linting${NC}"
echo "========================="

# Check if markdownlint is installed
check_markdownlint() {
    if command -v markdownlint &> /dev/null; then
        echo -e "${GREEN}✓${NC} markdownlint is installed"
        markdownlint --version
        return 0
    else
        echo -e "${YELLOW}⚠${NC} markdownlint not found"
        echo "  Install with: npm install -g markdownlint-cli"
        return 1
    fi
}

# Create markdownlint config if missing
create_config() {
    if [ ! -f ".markdownlint.json" ]; then
        echo -e "${YELLOW}ℹ${NC} Creating .markdownlint.json with Google style guidelines..."
        
        cat > .markdownlint.json << 'EOF'
{
  "default": true,
  "MD003": { "style": "atx" },
  "MD004": { "style": "dash" },
  "MD007": { "indent": 2 },
  "MD013": false,
  "MD024": { "siblings_only": true },
  "MD025": true,
  "MD026": { "punctuation": ".,;:!" },
  "MD029": { "style": "ordered" },
  "MD033": false,
  "MD035": { "style": "---" },
  "MD036": false,
  "MD041": false,
  "MD046": { "style": "fenced" },
  "MD048": { "style": "backtick" },
  "MD049": { "style": "underscore" },
  "MD050": { "style": "asterisk" },
  "line-length": false,
  "no-bare-urls": false,
  "no-emphasis-as-heading": true,
  "no-trailing-spaces": true,
  "no-hard-tabs": true,
  "no-duplicate-heading": { "siblings_only": true }
}
EOF
        echo -e "${GREEN}✓${NC} Created .markdownlint.json"
    else
        echo -e "${GREEN}✓${NC} Using existing .markdownlint.json"
    fi
}

# Run markdownlint
run_linting() {
    echo ""
    echo "Running markdownlint..."
    echo "-----------------------"
    
    # Define directories to lint
    LINT_DIRS=(
        "dev-docs"
        "specs"
        "templates"
        "."
    )
    
    # Files to explicitly include at root
    ROOT_FILES=(
        "README.md"
        "CONTRIBUTING.md"
        "CHANGELOG.md"
        "CODE_OF_CONDUCT.md"
        "SECURITY.md"
        "SUPPORT.md"
    )
    
    local LINT_FAILED=false
    local TOTAL_ISSUES=0
    
    # Lint directories
    for dir in "${LINT_DIRS[@]}"; do
        if [ -d "$dir" ]; then
            echo ""
            echo "Linting $dir/..."
            
            if [ "$dir" = "." ]; then
                # For root, only lint specific files
                for file in "${ROOT_FILES[@]}"; do
                    if [ -f "$file" ]; then
                        if ! markdownlint "$file" 2>/dev/null; then
                            LINT_FAILED=true
                            ((TOTAL_ISSUES++))
                        fi
                    fi
                done
            else
                # For other directories, lint all .md files
                if ! markdownlint "$dir/**/*.md" --ignore node_modules --ignore venv 2>/dev/null; then
                    LINT_FAILED=true
                    ((TOTAL_ISSUES++))
                fi
            fi
        else
            echo -e "${YELLOW}ℹ${NC} Directory $dir not found - skipping"
        fi
    done
    
    echo ""
    if [ "$LINT_FAILED" = true ]; then
        echo -e "${YELLOW}⚠${NC} Markdown linting found issues"
        echo "  Run 'markdownlint --fix' to auto-fix some issues"
        return 1
    else
        echo -e "${GREEN}✓${NC} All markdown files pass linting"
        return 0
    fi
}

# Check for common documentation issues
check_doc_issues() {
    echo ""
    echo "Checking for common documentation issues..."
    echo "------------------------------------------"
    
    local ISSUES_FOUND=false
    
    # Check for broken links (simple check)
    echo "  Checking for potentially broken links..."
    BROKEN_LINKS=$(find . -name "*.md" -type f | while read -r file; do
        grep -o '\[.*\](\.\.*/.*\.md)' "$file" 2>/dev/null | while read -r link; do
            LINK_PATH=$(echo "$link" | sed 's/.*](\(.*\))/\1/')
            DIR=$(dirname "$file")
            FULL_PATH="$DIR/$LINK_PATH"
            RESOLVED=$(realpath "$FULL_PATH" 2>/dev/null || echo "")
            
            if [ -n "$RESOLVED" ] && [ ! -f "$RESOLVED" ]; then
                echo "    $file: $LINK_PATH"
            fi
        done
    done)
    
    if [ -n "$BROKEN_LINKS" ]; then
        echo -e "${YELLOW}⚠${NC} Potentially broken links found:"
        echo "$BROKEN_LINKS"
        ISSUES_FOUND=true
    fi
    
    # Check for missing front matter in key docs
    echo "  Checking for documentation metadata..."
    for file in README.md CONTRIBUTING.md; do
        if [ -f "$file" ]; then
            if ! head -1 "$file" | grep -q "^#" && ! head -1 "$file" | grep -q "^---"; then
                echo -e "${YELLOW}⚠${NC} $file might be missing a title or front matter"
                ISSUES_FOUND=true
            fi
        fi
    done
    
    if [ "$ISSUES_FOUND" = false ]; then
        echo -e "${GREEN}✓${NC} No common documentation issues found"
    fi
}

# Generate documentation report
generate_report() {
    echo ""
    echo "Documentation Statistics"
    echo "------------------------"
    
    # Count documentation files
    MD_COUNT=$(find . -name "*.md" -type f | grep -v node_modules | grep -v venv | wc -l)
    echo "  Total markdown files: $MD_COUNT"
    
    # Count lines of documentation
    if command -v wc &> /dev/null; then
        TOTAL_LINES=$(find . -name "*.md" -type f | grep -v node_modules | grep -v venv | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}')
        echo "  Total documentation lines: $TOTAL_LINES"
    fi
    
    # List largest documentation files
    echo ""
    echo "  Largest documentation files:"
    find . -name "*.md" -type f | grep -v node_modules | grep -v venv | xargs wc -l 2>/dev/null | sort -rn | head -5 | while read -r line; do
        echo "    $line"
    done
}

# Main execution
main() {
    local EXIT_CODE=0
    
    # Check for markdownlint
    if ! check_markdownlint; then
        echo -e "${YELLOW}⚠${NC} Skipping markdown linting - tool not installed"
        echo "  Documentation linting is recommended but not required"
    else
        # Create config if needed
        create_config
        
        # Run linting
        if ! run_linting; then
            EXIT_CODE=1
        fi
    fi
    
    # Check for other issues
    check_doc_issues
    
    # Generate report
    generate_report
    
    echo ""
    echo "========================="
    if [ $EXIT_CODE -eq 0 ]; then
        echo -e "${GREEN}✅ Documentation check complete${NC}"
    else
        echo -e "${YELLOW}⚠️  Documentation check found issues (non-blocking)${NC}"
        # Don't fail CI for documentation issues
        EXIT_CODE=0
    fi
    
    exit $EXIT_CODE
}

# Run main
main "$@"