#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <file>" >&2; exit 2
fi
FILE="$1"

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

if [[ -f ./sgconfig.yml ]]; then
  CONFIG=( -c ./sgconfig.yml )
else
  CONFIG=()
fi

echo "[sg-scan-file] $FILE"
/opt/homebrew/bin/ast-grep scan "${CONFIG[@]}" --format=github "$FILE",
