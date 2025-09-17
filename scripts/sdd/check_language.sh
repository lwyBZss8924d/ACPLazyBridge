#!/usr/bin/env bash
# Language policy enforcement script (project-level)
# Ensures normative artifacts are English-only across .specify/, sdd-rules/, dev-docs/, and specs/

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Default normative roots (override with --paths "dir1 dir2 ...")
NORMATIVE_ROOTS=(".specify" "sdd-rules" "dev-docs" "specs")
ROOT_LEVEL_FILES=("CLAUDE.md" "AGENTS.md" "WARP.md")

# Parse arguments
if [[ ${1:-} == "--paths" ]]; then
  shift
  IFS=' ' read -r -a NORMATIVE_ROOTS <<< "${1:-}"
  shift || true
fi

echo "🔍 Checking language policy compliance (English-only)"
echo "   Roots: ${NORMATIVE_ROOTS[*]}"

# Use Python for robust Unicode detection (works on macOS/Linux)
python3 - <<'PY'
import sys, pathlib, re, os

roots = os.getenv('NORMATIVE_ROOTS_CSV', '').split(',') if os.getenv('NORMATIVE_ROOTS_CSV') else []
if not roots:
  # Fallback to defaults if env not set
  roots = ['.specify', 'sdd-rules', 'dev-docs', 'specs']

# File globs to include
INCLUDE_EXTS = {'.md', '.txt', '.rst'}

# Unicode ranges: CJK Unified, Extensions A, Hiragana, Katakana, Hangul, Cyrillic
patterns = [
  re.compile(r"[\u4E00-\u9FFF]"),   # CJK Unified
  re.compile(r"[\u3400-\u4DBF]"),   # CJK Ext A
  re.compile(r"[\u3040-\u309F]"),   # Hiragana
  re.compile(r"[\u30A0-\u30FF]"),   # Katakana
  re.compile(r"[\uAC00-\uD7AF]"),   # Hangul
  re.compile(r"[\u0400-\u04FF]")    # Cyrillic
]

violations = []

def scan_file(p: pathlib.Path):
    try:
        text = p.read_text(encoding='utf-8', errors='ignore')
    except Exception:
        return
    for pat in patterns:
        m = pat.search(text)
        if m:
            # Compute line number of first occurrence
            line_no = text.count('\n', 0, m.start()) + 1
            violations.append((str(p), line_no))
            return

for root in roots:
    rp = pathlib.Path(root)
    if not rp.exists():
        continue
    for p in rp.rglob('*'):
        if p.is_file() and p.suffix.lower() in INCLUDE_EXTS:
            scan_file(p)

# Also check select root-level files if present
for name in ["CLAUDE.md", "AGENTS.md", "WARP.md"]:
    p = pathlib.Path(name)
    if p.exists() and p.is_file():
        scan_file(p)

if violations:
    print("\n[LANG] ERROR: Non-English characters detected in normative artifacts:")
    for path, line_no in violations:
        print(f"  - {path}: first occurrence at line {line_no}")
    sys.exit(1)
else:
    print("\n[LANG] ✓ All normative artifacts appear to be English-only")
    sys.exit(0)
PY
