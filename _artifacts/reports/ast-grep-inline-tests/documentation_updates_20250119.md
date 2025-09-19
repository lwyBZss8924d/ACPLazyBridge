# Documentation Updates from Issue #34 Learnings

## Summary
Updated all ast-grep related documentation to include critical limitations discovered while fixing Issue #34 (85+ false positives in test files).

## Key Learning Documented
**When rules are loaded via `ruleDirs` in sgconfig.yml, the `files:` field in individual rule YAML files is COMPLETELY IGNORED.**

This undocumented limitation means file exclusion patterns like `"!**/tests/**"` don't work, requiring suppression comments as the only reliable workaround.

## Files Updated

### 1. sdd-rules/rules/tools-cli/sdd-rules-tools-cli-astgrep.md (v1.0.2)
- Added CRITICAL LIMITATION warning in Files globs section (line 115)
- Added comprehensive "Suppression Comments" section with examples (line 224)
- Added real-world Issue #34 example showing the problem and solution (line 708)
- Added troubleshooting table for common issues (line 805)
- Added verification commands to test suppressions
- Updated version to 1.0.2 with changelog

### 2. sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md (v1.0.2)
- Added "Important Limitations Discovered" section (line 12)
- Documented file exclusion pattern limitation
- Documented suppression comment requirements
- Added verification commands
- Updated version to 1.0.2 with changelog

### 3. .claude/agents/code-retriever.md
- Added "Important Limitations" section (line 120)
- Warned about file pattern limitations
- Recommended using `--rule-file` for testing
- Added note about suppression comments

### 4. .claude/agents/code-analyzer.md
- Added "Important Limitations" section (line 109)
- Included example of non-working exclusion pattern
- Added verification command
- Noted expectation of higher warning counts in repos with tests

## Impact
These documentation updates will:
- Prevent future engineers from wasting time on non-working file patterns
- Provide clear workarounds with suppression comments
- Help sub-agents understand and work around the limitations
- Save debugging time for teams using ast-grep with test code

## Verification
All documentation now references Issue #34 and includes:
- Clear warning about the limitation
- Practical workarounds
- Real examples from our fix
- Verification commands

---
Generated: 2025-09-19
Related to: Issue #34 - Fix ast-grep inline test false positives