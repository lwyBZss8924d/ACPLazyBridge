#!/usr/bin/env bash
# Calibrated, reproducible baseline for ACP rust/ unwrap/expect using ACPLazyBridge config
# Outputs JSON grouped counts by file for rust-no-unwrap
set -euo pipefail
CFG="/Users/arthur/dev-space/ACPLazyBridge/sgconfig.yml"
TARGET="/Users/arthur/dev-space/agent-client-protocol/rust"
/opt/homebrew/bin/ast-grep scan -c "$CFG" --filter '^rust-no-unwrap$' "$TARGET" --json=stream \
| jq -c '{file: .file}' \
| jq -s 'group_by(.file) | map({file: .[0].file, count: length}) | sort_by(-.count)'
