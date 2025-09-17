#!/usr/bin/env bash
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

if [[ -f ./sgconfig.yml ]]; then
  CONFIG=( -c ./sgconfig.yml )
else
  CONFIG=()
fi

echo "[sg-fix] Applying fixes defined by rules (use with care)"
/opt/homebrew/bin/ast-grep scan "${CONFIG[@]}" -U .,
