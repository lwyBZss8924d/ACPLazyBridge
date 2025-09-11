#!/usr/bin/env bash
set -euo pipefail

# Local CI runner (pre-PR): structure, language, markdown style, semantic checks
# Fails fast on blocking checks. Intended for local validation.

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT_DIR"

info() { echo "[LOCAL-CI] $*"; }
err() { echo "[LOCAL-CI] ERROR: $*" >&2; }

STEPS=(
  scripts/ci/run-sdd-structure-lint.sh
  scripts/ci/check-language-policy.sh
  scripts/ci/run-markdown-style.sh
  scripts/sdd/run_semantic_checks.sh
)

for step in "${STEPS[@]}"; do
  if [ ! -x "$step" ]; then
    # allow non-executable, but must exist
    if [ -f "$step" ]; then
      info "running $step"
      bash "$step"
    else
      err "missing $step"
      exit 1
    fi
  else
    info "running $step"
    "$step"
  fi
  info "$step OK"
  echo "----------------------------------------"
done

info "All local CI checks completed successfully"

