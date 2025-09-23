# Claude Memory and SDD Rules Index Alignment

## Metadata

- Issue-URI: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/26
- Spec-URI: specs/001-claude-memory-sdd-alignment/spec.md
- Evidence-URIs: _artifacts/tests/legacy/001-claude-memory-sdd-alignment/

## Overview

Align Claude Code memory documents (CLAUDE.md) with SDD Developer Team workflow requirements and create a centralized rules index for improved navigation and consistency.

## Problem Statement

1. Claude Code memory hierarchy is not fully aligned with SDD workflow requirements
2. Missing per-directory CLAUDE.md files for module-specific context
3. No centralized index for SDD rules documentation
4. Inconsistent references between agent documentation and rules

## Requirements

### Functional Requirements

1. Root CLAUDE.md must:
   - Define repository-level governance and SDD workflow
   - Include normative authority chain references
   - Provide project navigation structure
   - Specify ACP protocol version consistency (v1 with integer 1)

2. Per-directory CLAUDE.md files must:
   - Be created for crates/acp-lazy-core, crates/codex-cli-acp, scripts/
   - Define module-specific build, test, and operation conventions
   - Reference parent CLAUDE.md for inherited rules
   - Include local evidence paths

3. Rules index (sdd-rules/rules/README.md) must:
   - Provide categorized navigation to all rule documents
   - Include brief descriptions for each category
   - Link to both normative and non-normative references

### Non-Functional Requirements

1. All documentation must be in English (per language policy)
2. Must pass SDD structure lint checks
3. Must maintain consistency with existing normative documents

## Acceptance Criteria

1. [ ] Root CLAUDE.md updated with SDD workflow and authority chain
2. [ ] Per-directory CLAUDE.md files created for specified modules
3. [ ] Rules index created and linked from AGENTS.md and WARP.md
4. [ ] All local CI checks pass (lint, language policy)
5. [ ] Evidence artifacts stored under _artifacts/legacy/

## Constraints

- Must not modify existing normative documents (CONTRIBUTING.md, sdd-rules/spec-driven.md)
- Must maintain backward compatibility with existing workflows
- Must follow canonical branch naming (docs/001-claude-memory-sdd-alignment)

## Dependencies

- sdd-rules/spec-driven.md (SDD workflow definition)
- sdd-rules/AGENTS.md (agent roles and workflows)
- CONTRIBUTING.md (engineering ground rules)

## Risks

- Risk: Inconsistent updates across distributed CLAUDE.md files
    - Mitigation: Clear inheritance hierarchy and regular consistency checks
- Risk: Rules index becoming outdated
    - Mitigation: CI validation for broken links and missing references
