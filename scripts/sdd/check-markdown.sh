#!/usr/bin/env bash
set -euo pipefail

# Check markdown for style issues (non-blocking, informational)
# - Provides detailed report without failing
# - Shows which issues are auto-fixable
# - Suggests next steps

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT_DIR"

info() { echo "[MD-CHECK] $*"; }
warn() { echo "[MD-CHECK] WARNING: $*" >&2; }

# Color codes for terminal output (if supported)
if [ -t 1 ]; then
  RED='\033[0;31m'
  YELLOW='\033[0;33m'
  GREEN='\033[0;32m'
  BLUE='\033[0;34m'
  NC='\033[0m' # No Color
else
  RED=''
  YELLOW=''
  GREEN=''
  BLUE=''
  NC=''
fi

# Parse arguments
VERBOSE=false
OUTPUT_FORMAT="human"
TARGET_PATH="**/*.md"

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
    --path)
      TARGET_PATH="$2"
      shift 2
      ;;
    --help)
      cat <<EOF
Usage: $0 [OPTIONS]

Check markdown files for style issues (non-blocking).

Options:
  -v, --verbose      Show all issues (not just summary)
  --format FORMAT    Output format: human, json, github (default: human)
  --path PATH        Target specific path/pattern (default: "**/*.md")
  --help             Show this help message

Examples:
  $0                        # Check all markdown files
  $0 --verbose              # Show detailed issues
  $0 --format github        # GitHub Actions annotation format
  $0 --path "docs/**/*.md"  # Check only docs directory
EOF
      exit 0
      ;;
    *)
      warn "Unknown option: $1 (ignored)"
      shift
      ;;
  esac
done

# Check for markdownlint-cli2
if ! command -v markdownlint-cli2 >/dev/null 2>&1; then
  if command -v npx >/dev/null 2>&1; then
    info "using npx to run markdownlint-cli2"
    RUNNER=(npx --yes markdownlint-cli2@latest)
  else
    warn "markdownlint-cli2 not installed"
    warn "install with: npm install -g markdownlint-cli2"
    info "falling back to basic check..."
    
    # Basic fallback check
    echo -e "${YELLOW}Basic markdown check:${NC}"
    find . -name "*.md" -type f ! -path "./.git/*" ! -path "./node_modules/*" \
      ! -path "./.worktrees/*" -print | head -20
    echo "..."
    echo "Install markdownlint-cli2 for full analysis"
    exit 0
  fi
else
  RUNNER=(markdownlint-cli2)
fi

# Check configuration
if [ ! -f .markdownlint.json ]; then
  warn "missing .markdownlint.json configuration"
  warn "using default rules"
fi

# Run the check
info "checking markdown files..."

# Capture output
LINT_OUTPUT=$("${RUNNER[@]}" "$TARGET_PATH" --config .markdownlint.json 2>&1 || true)

# Count issues by type
TOTAL_ISSUES=$(echo "$LINT_OUTPUT" | grep -c "^[^:]*\.md:" || echo "0")

if [ "$TOTAL_ISSUES" -eq 0 ]; then
  echo -e "${GREEN}✓ All markdown files are clean!${NC}"
  exit 0
fi

# Analyze issues
declare -A ISSUE_COUNTS
declare -A FIXABLE_RULES

# Rules that can be auto-fixed
FIXABLE_RULES=(
  ["MD022"]="true"  # Blanks around headings
  ["MD032"]="true"  # Blanks around lists
  ["MD047"]="true"  # File ends with newline
  ["MD012"]="true"  # Multiple blank lines
  ["MD009"]="true"  # Trailing spaces
  ["MD010"]="true"  # Hard tabs
  ["MD031"]="true"  # Blanks around fences
)

# Count issues by rule
while IFS= read -r line; do
  if [[ "$line" =~ MD[0-9]+ ]]; then
    RULE="${BASH_REMATCH[0]}"
    ISSUE_COUNTS["$RULE"]=$((${ISSUE_COUNTS["$RULE"]:-0} + 1))
  fi
done <<< "$LINT_OUTPUT"

# Calculate fixable count
FIXABLE_COUNT=0
for rule in "${!ISSUE_COUNTS[@]}"; do
  if [ "${FIXABLE_RULES[$rule]:-false}" = "true" ]; then
    FIXABLE_COUNT=$((FIXABLE_COUNT + ${ISSUE_COUNTS[$rule]}))
  fi
done

MANUAL_COUNT=$((TOTAL_ISSUES - FIXABLE_COUNT))

# Output based on format
case "$OUTPUT_FORMAT" in
  json)
    cat <<EOF
{
  "total_issues": $TOTAL_ISSUES,
  "auto_fixable": $FIXABLE_COUNT,
  "manual_required": $MANUAL_COUNT,
  "issues_by_rule": {
$(for rule in "${!ISSUE_COUNTS[@]}"; do
  echo "    \"$rule\": ${ISSUE_COUNTS[$rule]},"
done | sed '$ s/,$//')
  }
}
EOF
    ;;
    
  github)
    # GitHub Actions annotation format
    echo "$LINT_OUTPUT" | while IFS= read -r line; do
      if [[ "$line" =~ ^(.+\.md):([0-9]+):([0-9]+)?[[:space:]]+(MD[0-9]+) ]]; then
        FILE="${BASH_REMATCH[1]}"
        LINE="${BASH_REMATCH[2]}"
        COL="${BASH_REMATCH[3]:-1}"
        RULE="${BASH_REMATCH[4]}"
        MSG="${line#*$RULE}"
        echo "::warning file=$FILE,line=$LINE,col=$COL::$RULE$MSG"
      fi
    done
    ;;
    
  human|*)
    # Human-readable output
    echo -e "${YELLOW}Markdown Style Report${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "Total issues: ${RED}$TOTAL_ISSUES${NC}"
    echo -e "Auto-fixable: ${BLUE}$FIXABLE_COUNT${NC}"
    echo -e "Manual fixes: ${YELLOW}$MANUAL_COUNT${NC}"
    echo ""
    
    if [ ${#ISSUE_COUNTS[@]} -gt 0 ]; then
      echo "Issues by rule:"
      for rule in $(echo "${!ISSUE_COUNTS[@]}" | tr ' ' '\n' | sort); do
        COUNT=${ISSUE_COUNTS[$rule]}
        if [ "${FIXABLE_RULES[$rule]:-false}" = "true" ]; then
          echo -e "  ${BLUE}$rule${NC}: $COUNT (auto-fixable)"
        else
          echo -e "  ${YELLOW}$rule${NC}: $COUNT (manual)"
        fi
      done
      echo ""
    fi
    
    if [ "$VERBOSE" = true ] || [ "$MANUAL_COUNT" -gt 0 ]; then
      echo "Sample issues requiring attention:"
      echo "────────────────────────────────────────"
      echo "$LINT_OUTPUT" | grep -E "MD[0-9]+" | head -10
      
      if [ "$TOTAL_ISSUES" -gt 10 ]; then
        echo "... and $((TOTAL_ISSUES - 10)) more"
      fi
      echo ""
    fi
    
    # Suggestions
    echo -e "${GREEN}Next steps:${NC}"
    if [ "$FIXABLE_COUNT" -gt 0 ]; then
      echo "  1. Run auto-fix for $FIXABLE_COUNT issues:"
      echo "     ${BLUE}scripts/sdd/fix-markdown.sh${NC}"
    fi
    if [ "$MANUAL_COUNT" -gt 0 ]; then
      echo "  2. Manually fix $MANUAL_COUNT remaining issues"
      echo "     See: ${BLUE}sdd-rules/rules/documentation-style/sdd-rules-documentation-markdownlint.md${NC}"
    fi
    ;;
esac

# Exit with warning status if issues found (but don't fail)
if [ "$TOTAL_ISSUES" -gt 0 ]; then
  exit 0  # Non-blocking, so always exit 0
fi