#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "usage: run-hook.sh <crate-name> [args...]" >&2
  exit 1
fi

crate="$1"
shift || true

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$SCRIPT_DIR"
CRATE_DIR="$WORKSPACE_DIR/rs/${crate}"
TARGET_BASE="${CLAUDE_PROJECT_DIR:-$(cd "$SCRIPT_DIR/.." && pwd)}/target/hooks"
BINARY="$TARGET_BASE/release/${crate}"

rebuild=0
if [[ ! -x "$BINARY" ]]; then
  rebuild=1
else
  if [[ "$BINARY" -ot "$WORKSPACE_DIR/Cargo.toml" ]]; then
    rebuild=1
  fi
  while IFS= read -r source; do
    if [[ "$BINARY" -ot "$source" ]]; then
      rebuild=1
      break
    fi
  done < <(find "$CRATE_DIR" -type f \( -name *.rs -o -name Cargo.toml \))
fi

if [[ $rebuild -eq 1 ]]; then
  mkdir -p "$TARGET_BASE"
  CARGO_TARGET_DIR="$TARGET_BASE" cargo build --quiet --manifest-path "$WORKSPACE_DIR/Cargo.toml" --release -p "$crate"
fi

exec "$BINARY" "$@"
