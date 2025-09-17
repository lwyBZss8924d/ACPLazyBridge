#!/usr/bin/env bash
# Baseline dbg! occurrences in ACP rust/ (non-test excluded by convention)
set -euo pipefail
CFG="/Users/arthur/dev-space/ACPLazyBridge/sgconfig.yml"
TARGET="/Users/arthur/dev-space/agent-client-protocol/rust"
/opt/homebrew/bin/ast-grep -p 'dbg!($$$ARGS)' -l rust "$TARGET" \
| jq -R -c 'split(":") | {file: .[0], line: (.[1]|tonumber)}' \
| jq -s 'group_by(.file) | map({file: .[0].file, count: length, lines: map(.line)}) | sort_by(-.count)'
