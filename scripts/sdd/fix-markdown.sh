#!/usr/bin/env bash
set -euo pipefail

# Auto-fix markdown issues using markdownlint-cli2
# - Non-destructive: creates backup before fixing
# - Uses .markdownlint.json and .markdownlintignore
# - Shows before/after statistics

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT_DIR"

info() { echo "[MD-FIX] $*"; }
err() { echo "[MD-FIX] ERROR: $*" >&2; }
warn() { echo "[MD-FIX] WARNING: $*" >&2; }

# Parse arguments
DRY_RUN=false
NO_BACKUP=false
TARGET_PATH="**/*.md"

while [[ $# -gt 0 ]]; do
  case $1 in
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    --no-backup)
      NO_BACKUP=true
      shift
      ;;
    --path)
      TARGET_PATH="$2"
      shift 2
      ;;
    --help)
      cat <<EOF
Usage: $0 [OPTIONS]

Auto-fix markdown formatting issues.

Options:
  --dry-run     Show what would be fixed without making changes
  --no-backup   Don't create backup (use with caution)
  --path PATH   Target specific path/pattern (default: "**/*.md")
  --help        Show this help message

Examples:
  $0                     # Fix all markdown files
  $0 --dry-run           # Preview fixes without applying
  $0 --path "docs/*.md"  # Fix only docs directory
EOF
      exit 0
      ;;
    *)
      err "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Check for markdownlint-cli2
if ! command -v markdownlint-cli2 >/dev/null 2>&1; then
  if command -v npx >/dev/null 2>&1; then
    info "markdownlint-cli2 not found, using npx"
    RUNNER=(npx --yes markdownlint-cli2@latest)
  else
    err "markdownlint-cli2 not installed"
    err "install with: npm install -g markdownlint-cli2"
    exit 1
  fi
else
  RUNNER=(markdownlint-cli2)
fi

# Check configuration
if [ ! -f .markdownlint.json ]; then
  err "missing .markdownlint.json configuration"
  exit 1
fi

# Count current issues
info "scanning for markdown issues..."
OUTPUT=$("${RUNNER[@]}" "$TARGET_PATH" --config .markdownlint.json 2>&1 || true)
if echo "$OUTPUT" | grep -q "Summary:"; then
  ISSUES_BEFORE=$(echo "$OUTPUT" | grep "Summary:" | sed 's/.*Summary: \([0-9]*\).*/\1/')
else
  ISSUES_BEFORE=0
fi
info "found $ISSUES_BEFORE issues"

if [ "$ISSUES_BEFORE" -eq 0 ]; then
  info "no issues found - markdown is already clean!"
  exit 0
fi

# Create backup if requested
if [ "$NO_BACKUP" = false ] && [ "$DRY_RUN" = false ]; then
  BACKUP_DIR=".markdown-backup/$(date +%Y%m%d-%H%M%S)"
  info "creating backup in $BACKUP_DIR"
  mkdir -p "$BACKUP_DIR"
  
  # Find and backup markdown files (align with .markdownlintignore)
  find . -name "*.md" -type f \
    ! -path "./.git/*" \
    ! -path "./.worktrees/*" \
    ! -path "./node_modules/*" \
    ! -path "./target/*" \
    ! -path "./dist/*" \
    ! -path "./build/*" \
    ! -path "./.venv/*" \
    ! -path "./venv/*" \
    ! -path "./.codeql-db/*" \
    ! -path "./docs/api/*" \
    ! -path "./coverage/*" \
    ! -path "./tmp/*" \
    ! -path "./temp/*" \
    ! -path "./.cache/*" \
    ! -path "./.vscode/*" \
    ! -path "./.idea/*" \
    ! -path "./archive/*" \
    ! -path "./deprecated/*" \
    ! -path "./.markdown-backup/*" \
    -exec cp --parents {} "$BACKUP_DIR" \; 2>/dev/null || true
  
  info "backup created (restore with: cp -r $BACKUP_DIR/* .)"
fi

# Run fix or dry-run
if [ "$DRY_RUN" = true ]; then
  info "DRY RUN - showing what would be fixed:"
  "${RUNNER[@]}" "$TARGET_PATH" --config .markdownlint.json 2>&1 | head -20
  echo "..."
  info "run without --dry-run to apply fixes"
else
  info "applying auto-fixes..."
  "${RUNNER[@]}" "$TARGET_PATH" --config .markdownlint.json --fix
  
  # Count remaining issues
  OUTPUT_AFTER=$("${RUNNER[@]}" "$TARGET_PATH" --config .markdownlint.json 2>&1 || true)
  if echo "$OUTPUT_AFTER" | grep -q "Summary:"; then
    ISSUES_AFTER=$(echo "$OUTPUT_AFTER" | grep "Summary:" | sed 's/.*Summary: \([0-9]*\).*/\1/')
  else
    ISSUES_AFTER=0
  fi
  
  # Report results
  FIXED=$((ISSUES_BEFORE - ISSUES_AFTER))
  info "fixed $FIXED issues"
  
  if [ "$ISSUES_AFTER" -gt 0 ]; then
    warn "$ISSUES_AFTER issues require manual fixing:"
    "${RUNNER[@]}" "$TARGET_PATH" --config .markdownlint.json 2>&1 | head -10
    
    if [ "$ISSUES_AFTER" -gt 10 ]; then
      echo "... and $((ISSUES_AFTER - 10)) more"
    fi
    
    info "run 'scripts/sdd/check-markdown.sh' for full report"
    exit 1
  else
    info "all issues fixed successfully!"
  fi
fi