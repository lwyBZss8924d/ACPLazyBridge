#!/usr/bin/env bash
# Language policy enforcement script
# Checks that normative artifacts are written in English only

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Paths considered normative artifacts
NORMATIVE_PATHS=("specs" "templates" "dev-docs" "memory")
NORMATIVE_FILES=("CLAUDE.md" "AGENTS.md" "WARP.md")

# Check for CJK characters (Chinese, Japanese, Korean) and Cyrillic
# These are common non-English character ranges
CJK_PATTERN='[\x{4E00}-\x{9FFF}\x{3040}-\x{30FF}\x{AC00}-\x{D7AF}\x{0400}-\x{04FF}]'

FOUND_VIOLATIONS=0
VIOLATION_FILES=""

echo "🔍 Checking language policy compliance..."
echo "   Normative artifacts must be written in English only"
echo ""

# Function to check a single file
check_file() {
    local file="$1"
    
    # Skip binary files and images
    if file "$file" | grep -q "binary\|image\|data"; then
        return 0
    fi
    
    # Check for non-English characters (excluding code blocks)
    if grep -qP "$CJK_PATTERN" "$file" 2>/dev/null; then
        echo -e "${RED}✗${NC} Non-English characters found in: $file"
        FOUND_VIOLATIONS=1
        VIOLATION_FILES="$VIOLATION_FILES\n  - $file"
        
        # Show first occurrence with line number for debugging
        if command -v grep &> /dev/null && grep --version | grep -q GNU; then
            FIRST_LINE=$(grep -nP "$CJK_PATTERN" "$file" 2>/dev/null | head -1 | cut -d: -f1)
            if [ -n "$FIRST_LINE" ]; then
                echo "    First occurrence at line $FIRST_LINE"
            fi
        fi
        return 1
    fi
    
    return 0
}

# Check directories
for dir in "${NORMATIVE_PATHS[@]}"; do
    if [ -d "$dir" ]; then
        echo "Checking directory: $dir/"
        
        # Find all markdown and text files
        while IFS= read -r -d '' file; do
            check_file "$file"
        done < <(find "$dir" -type f \( -name "*.md" -o -name "*.txt" -o -name "*.rst" \) -print0 2>/dev/null)
    fi
done

# Check root-level normative files
for file in "${NORMATIVE_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "Checking file: $file"
        check_file "$file"
    fi
done

echo ""

# Report results
if [ $FOUND_VIOLATIONS -eq 1 ]; then
    echo -e "${RED}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${RED}Language Policy Violation${NC}"
    echo -e "${RED}═══════════════════════════════════════════════════════════${NC}"
    echo ""
    echo "Non-English characters were detected in normative artifacts."
    echo "All specifications, plans, tasks, and documentation must be in English."
    echo ""
    echo -e "Files with violations:${VIOLATION_FILES}"
    echo ""
    echo "Please translate the content to English before committing."
    echo -e "${RED}═══════════════════════════════════════════════════════════${NC}"
    exit 1
else
    echo -e "${GREEN}✓${NC} All normative artifacts are in English"
    echo -e "${GREEN}Language policy check passed!${NC}"
    exit 0
fi
