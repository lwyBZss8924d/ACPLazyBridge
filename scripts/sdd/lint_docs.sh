#!/usr/bin/env bash
set -euo pipefail

# Documentation Linting Script - Wrapper for check-markdown.sh
# This script now delegates to check-markdown.sh which uses markdownlint-cli2
# Maintained for backward compatibility

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "📝 Documentation Linting"
echo "========================="
echo ""
echo "Note: This script now uses check-markdown.sh with markdownlint-cli2"
echo ""

# Call check-markdown.sh with verbose flag to show details
exec "$SCRIPT_DIR/check-markdown.sh" --verbose "$@"