#!/usr/bin/env bash
# Baseline TODO/FIXME/HACK comments in ACP rust/
set -euo pipefail
TARGET="/Users/arthur/dev-space/agent-client-protocol/rust"
rg -n --no-heading -e 'TODO|FIXME|HACK' "$TARGET" --glob '*.rs' \
| awk -F: '{print "{\"file\":\""$1"\",\"line\":"$2",\"text\":\""substr($0, index($0,$3))"\"}"}' \
| jq -s 'group_by(.file) | map({file: .[0].file, count: length, items: map({line: .line, text: .text})}) | sort_by(-.count)'
