#!/usr/bin/env bash
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

if [[ -f ./sgconfig.yml ]]; then
  CONFIG=( -c ./sgconfig.yml )
else
  CONFIG=()
fi

echo "[sg-scan] Using config: ${CONFIG[*]:-(none)}"
/opt/homebrew/bin/ast-grep scan "${CONFIG[@]}" --inspect summary .,
