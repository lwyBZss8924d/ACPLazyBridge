#!/usr/bin/env bash
set -euo pipefail

# CI markdown style check (blocking) - optimized for markdownlint-cli2
# - Uses .markdownlint.json at repo root
# - Uses .markdownlintignore for exclusions
# - Exits non-zero on violations

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT_DIR"

info() { echo "[DOC-STYLE] $*"; }
err() { echo "[DOC-STYLE] ERROR: $*" >&2; }

# Check for configuration file
if [ ! -f .markdownlint.json ]; then
  err "missing .markdownlint.json at repository root"
  exit 1
fi

# Determine runner: prefer markdownlint-cli2, fall back to cli v1 or npx
if command -v markdownlint-cli2 >/dev/null 2>&1; then
  info "using markdownlint-cli2 (preferred)"
  RUNNER=(markdownlint-cli2)
  USE_CLI2=true
elif command -v markdownlint >/dev/null 2>&1; then
  info "using markdownlint-cli (v1) - consider upgrading to markdownlint-cli2"
  RUNNER=(markdownlint)
  USE_CLI2=false
elif command -v npx >/dev/null 2>&1; then
  info "using npx with markdownlint-cli2"
  RUNNER=(npx --yes markdownlint-cli2@latest)
  USE_CLI2=true
else
  err "no markdown linter found"
  err "install markdownlint-cli2: npm install -g markdownlint-cli2"
  err "or ensure npx is available (comes with Node.js)"
  exit 1
fi

# Build file list based on CLI version
if [ "$USE_CLI2" = true ]; then
  # cli2 supports globs and .markdownlintignore
  info "running markdownlint-cli2 with glob patterns"
  "${RUNNER[@]}" "**/*.md" --config .markdownlint.json
else
  # cli v1 needs explicit file list
  info "collecting markdown files for markdownlint-cli v1"
  mapfile -t FILES < <(git ls-files '*.md' \
    | grep -Ev '^(node_modules/|.git/|target/|dist/|build/|.venv/|venv/|.worktrees/|.codeql-db/)' \
    | grep -v 'sdd-rules/rules/changelog/semver.md' || true)
  
  if [ ${#FILES[@]} -eq 0 ]; then
    info "no markdown files found; nothing to lint"
    exit 0
  fi
  
  info "running markdownlint over ${#FILES[@]} files"
  "${RUNNER[@]}" --config .markdownlint.json "${FILES[@]}"
fi

info "markdown style checks passed"