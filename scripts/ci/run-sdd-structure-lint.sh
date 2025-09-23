#!/usr/bin/env bash
set -euo pipefail

# Minimal SDD structure lint
ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || echo ".")"
cd "$ROOT_DIR"

fail=false
err() { echo "[SDD-LINT] ERROR: $*" >&2; fail=true; }
info() { echo "[SDD-LINT] $*"; }

# 1) Required authoritative files (support legacy sdd-rules/ and new .specify/*)
require_either() {
  local primary="$1"; shift
  local alt="$1"; shift || true
  if [ -f "$primary" ]; then
    return 0
  fi
  if [ -n "$alt" ] && [ -f "$alt" ]; then
    info "found alternative for $primary: $alt"
    return 0
  fi
  err "missing $primary (and alternative ${alt:-none})"
}

require_either sdd-rules/AGENTS.md "sdd-rules/AGENTS.md"
require_either sdd-rules/CLAUDE.md "sdd-rules/CLAUDE.md"
require_either sdd-rules/lifecycle.md ".specify/memory/lifecycle.md"
require_either sdd-rules/spec-driven.md ".specify/spec-driven.md"
require_either sdd-rules/spec-template.md ".specify/templates/spec-template.md"
require_either sdd-rules/plan-template.md ".specify/templates/plan-template.md"
require_either sdd-rules/tasks-template.md ".specify/templates/tasks-template.md"

# 2) No outdated protocol version examples
if grep -Rni --include='*.md' 'protocolVersion":"2024-11-05"' . >/dev/null; then
  err "found outdated protocolVersion examples (2024-11-05); use integer 1"
fi

# 3) Evidence path hints (at least appears somewhere)
if ! grep -Rni --include='*.md' '_artifacts/legacy' . >/dev/null; then
  err "evidence path '_artifacts/legacy' not referenced in docs"
fi

# 4) Worktree example from origin/main -b (weak check)
if ! grep -Rni --include='*.md' 'worktree add .* origin/main -b' . >/dev/null; then
  err "missing canonical worktree example with 'origin/main -b'"
fi

$fail && exit 1 || { info "SDD structure lint passed"; exit 0; }

