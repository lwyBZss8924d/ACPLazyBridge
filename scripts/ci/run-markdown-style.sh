#!/usr/bin/env bash
set -euo pipefail

# CI markdown style check (blocking)
# - Uses .markdownlint.json at repo root
# - Runs over repository markdown files (via git ls-files)
# - Exits non-zero on violations

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT_DIR"

info() { echo "[DOC-STYLE] $*"; }
err() { echo "[DOC-STYLE] ERROR: $*" >&2; }

if [ ! -f .markdownlint.json ]; then
  err "missing .markdownlint.json at repository root"
  exit 1
fi

# Collect markdown files tracked by git, excluding common vendor/build dirs
mapfile -t FILES < <(git ls-files '*.md' \
  | grep -Ev '^(node_modules/|.git/|target/|dist/|build/|.venv/|venv/)' || true)

if [ ${#FILES[@]} -eq 0 ]; then
  info "no markdown files found; nothing to lint"
  exit 0
fi

# Determine runner: prefer npx (no prior install), else global markdownlint
if command -v npx >/dev/null 2>&1; then
  RUNNER=(npx --yes markdownlint-cli@0.39.0)
elif command -v markdownlint >/dev/null 2>&1; then
  RUNNER=(markdownlint)
else
  err "neither npx nor markdownlint-cli found; install Node (npx) or markdownlint-cli"
  err "e.g., npm install -g markdownlint-cli"
  exit 1
fi

info "running markdownlint over ${#FILES[@]} files"
"${RUNNER[@]}" --config .markdownlint.json "${FILES[@]}"
info "markdown style checks passed"

