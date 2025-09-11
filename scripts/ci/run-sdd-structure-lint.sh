#!/usr/bin/env bash
set -euo pipefail

# Minimal SDD structure lint
ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || echo ".")"
cd "$ROOT_DIR"

fail=false
err() { echo "[SDD-LINT] ERROR: $*" >&2; fail=true; }
info() { echo "[SDD-LINT] $*"; }

# 1) Required authoritative files
for f in sdd-rules/AGENTS.md sdd-rules/CLAUDE.md sdd-rules/lifecycle.md sdd-rules/spec-driven.md sdd-rules/spec-template.md sdd-rules/plan-template.md sdd-rules/tasks-template.md; do
  if [ ! -f "$f" ]; then err "missing $f"; fi
done

# 2) No outdated protocol version examples
if grep -Rni --include='*.md' 'protocolVersion":"2024-11-05"' . >/dev/null; then
  err "found outdated protocolVersion examples (2024-11-05); use integer 1"
fi

# 3) Evidence path hints (at least appears somewhere)
if ! grep -Rni --include='*.md' 'dev-docs/review/_artifacts' . >/dev/null; then
  err "evidence path 'dev-docs/review/_artifacts' not referenced in docs"
fi

# 4) Worktree example from origin/main -b (weak check)
if ! grep -Rni --include='*.md' 'worktree add .* origin/main -b' . >/dev/null; then
  err "missing canonical worktree example with 'origin/main -b'"
fi

$fail && exit 1 || { info "SDD structure lint passed"; exit 0; }

