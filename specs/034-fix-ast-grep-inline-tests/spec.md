# Specification: Fix ast-grep Inline Test False Positives

## Metadata

```yaml
worktree: /acplb-worktrees/fix-ast-grep-inline-tests
feature_branch: fix/ast-grep-inline-tests
created: 2025-09-19
last_updated: 2025-09-19T04:32:00Z
status: processing
input: GitHub Issue #34
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/34
specs:
    constitution: 1.0.1
    type: spec
    feature_number: 034
```

## Overview

After merging PR #31, the codebase still shows 86 ast-grep warnings in the IDE. Most of these are false positives from test code - both in test files and inline tests within src files. This creates noise that obscures real issues and reduces developer productivity.

## Problem Statement

The ast-grep rules for `rust-no-unwrap` and `rust-mutex-lock` are flagging legitimate uses of `unwrap()` and `expect()` in test code. While the file-based exclusion patterns work for separate test files, they don't exclude inline tests marked with `#[test]` or within `#[cfg(test)]` modules in src files.

## User Stories

As a developer, I want:

- Clean ast-grep output that only shows real issues in production code
- The ability to use `unwrap()` freely in test code without warnings
- Clear guidance on how to suppress warnings when needed

## Functional Requirements

- REQ-001: ast-grep rules must exclude test files from analysis
- REQ-002: ast-grep must support suppression comments for inline tests
- REQ-003: The solution must be documented in contributing guidelines
- REQ-004: The solution must be maintainable and not require per-test annotations

## Non-Functional Requirements

- NFR-001: The solution should reduce false positives by >90%
- NFR-002: The solution should not impact CI/CD performance
- NFR-003: The solution should be compatible with IDE integrations
- NFR-004: Documentation must be clear for new contributors

## Acceptance Criteria

- [x] ast-grep warnings reduced from 86 to <10 false positives (achieved: 0 Rust warnings)
- [x] File-based exclusion patterns comprehensive and documented (documented limitations)
- [x] Suppression comment syntax documented and working
- [x] CONTRIBUTING.md updated with ast-grep guidelines
- [x] All existing tests still pass
- [x] No production code affected by changes

## Technical Context

### Current State

- 86 warnings after PR #31
- Most warnings from test files and inline tests
- AST-based exclusion patterns don't work for inline tests

### Constraints

- ast-grep doesn't support complex AST exclusion for test attributes
- Must use built-in ast-grep features (file patterns, suppression comments)
- Cannot modify upstream ast-grep tool

## Out of Scope

- Modifying ast-grep tool itself
- Creating custom AST parsers
- Changing test structure or moving inline tests
- Addressing non-test-related warnings

## Dependencies

- ast-grep tool and configuration
- Existing rule files in sdd-rules/rules/code-analysis/ast-grep/rust/
- IDE ast-grep integrations

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Over-suppression hiding real issues | High | Use targeted suppression comments |
| Maintenance burden of suppressions | Medium | Document patterns clearly |
| IDE compatibility issues | Low | Test with multiple IDEs |

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
