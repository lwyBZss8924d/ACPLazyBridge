---
name: specify
description: Create or update the feature specification from a natural language feature description.
argument-hint: "<feature description>"
allowed-tools: Bash(scripts/sdd/create-new-feature.sh:*), Bash(git checkout:*), Bash(git branch:*), Read, Write, Edit, Glob
---

Given the feature description provided as an argument, do this:

1. Run the script `.specify/scripts/bash/create-new-feature.sh --json "$ARGUMENTS"` from repo root and parse its JSON output for BRANCH_NAME and SPEC_FILE. All file paths must be absolute.
2. Load `.specify/templates/spec-template.md` to understand required sections.
3. Write the specification to SPEC_FILE using the template structure, replacing placeholders with concrete details derived from the feature description (arguments) while preserving section order and headings.
4. Report completion with branch name, spec file path, and readiness for the next phase.

Note: The script creates and checks out the new branch and initializes the spec file before writing.

---

// ⚠️ _Whether initializing, modifying, or updating this SDD TASKs file (specs/xxxx-xxxx/spec.md), YOU MUST ULTRATHINK Analyze ISSUES [input] then edit the template file for the task first!_
// ⚠️ MUST follow and get UTC time NOW! `{{YYYY-MM-DD}}T{{HH:MM:SS}}Z` (date -u '+%Y-%m-%dT%H:%M:%SZ') first for specs file in the header's metadata-date yaml code block.
