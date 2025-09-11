#!/usr/bin/env bash
set -euo pipefail

# Heuristic language check: normative artifacts under sdd-rules/ must be English-only
ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || echo ".")"
cd "$ROOT_DIR"

fail=false
err() { echo "[LANG] ERROR: $*" >&2; fail=true; }
info() { echo "[LANG] $*"; }

# Use Python to detect CJK characters in sdd-rules/*.md
python3 - <<'PY'
import sys, pathlib, re
root = pathlib.Path('.')
cjk = re.compile(r"[\u4E00-\u9FFF]")
violations = []
for p in sorted((root / 'sdd-rules').glob('*.md')):
    text = p.read_text(encoding='utf-8', errors='ignore')
    if cjk.search(text):
        violations.append(str(p))
if violations:
    print('[LANG] ERROR: CJK characters found in normative artifacts:')
    for v in violations:
        print('[LANG]  -', v)
    sys.exit(1)
print('[LANG] sdd-rules English-only check passed')
PY

$fail && exit 1 || exit 0

