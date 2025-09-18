# Feature Specification: Fix ast-grep Rust Warnings

```yaml
worktree: /acplb-worktrees/fix-ast-grep-unwrap-mutex
feature_branch: fix/ast-grep-unwrap-mutex
created: 2025-09-18
last_updated: 2025-09-19
status: done
input: GitHub Issue #31
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/31
specs:
    constitution: 1.0.1
    type: spec
    feature_number: 031
```

## Execution Flow (main)

```text
1. Parse issue description from Input
   → Issue #31: Fix ast-grep rust warnings
2. Extract key concepts from description
   → Actors: developers, CI system
   → Actions: exclude inline tests, refactor unwrap usage
   → Data: ast-grep rules, Rust source files
   → Constraints: maintain functionality, pass quality gates
3. No unclear aspects identified
4. Fill User Scenarios & Testing section
5. Generate Functional Requirements
6. Identify Key Entities
7. Run Review Checklist
8. Return: SUCCESS (spec ready for planning)
```

## User Scenarios & Testing

### Primary User Story

As a developer working on ACPLazyBridge, I want ast-grep to only report legitimate code issues and not flag test code that appropriately uses `unwrap()`, so that I can focus on real problems that need fixing.

### Acceptance Criteria

- [x] **AC-001**: `ast-grep scan` produces zero warnings for code inside `#[cfg(test)]` modules
    - Command: `ast-grep scan -c ./sgconfig.yml --filter "rust-no-unwrap"`
    - Evidence: `_artifacts/reports/fix-ast-grep-unwrap-mutex/after-rules.log`
- [x] **AC-002**: `ast-grep scan` produces zero warnings for functions with `#[test]` attribute
    - Command: `ast-grep scan -c ./sgconfig.yml --filter "rust-no-unwrap"`
    - Evidence: `_artifacts/reports/fix-ast-grep-unwrap-mutex/after-rules.log`
- [x] **AC-003**: All production `unwrap()` calls replaced with explicit error handling
    - Command: `grep -r "unwrap()" crates/*/src --exclude-dir=tests`
    - Evidence: `_artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.log`
- [x] **AC-004**: Required `expect()` calls include descriptive context messages
    - Command: `ast-grep scan -c ./sgconfig.yml --filter "rust-mutex-lock"`
    - Evidence: Code review of `transport.rs` line 132
- [x] **AC-005**: Quality gates pass: cargo fmt
    - Command: `cargo fmt --all -- --check`
    - Evidence: `_artifacts/reports/fix-ast-grep-unwrap-mutex/clippy.log`
- [x] **AC-006**: Quality gates pass: cargo clippy
    - Command: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
    - Evidence: `_artifacts/reports/fix-ast-grep-unwrap-mutex/clippy.log`
- [x] **AC-007**: Quality gates pass: cargo test
    - Command: `cargo test --workspace --all-features --locked`
    - Evidence: `_artifacts/reports/fix-ast-grep-unwrap-mutex/test.log`
- [x] **AC-008**: Before/after comparison shows warning reduction
    - Evidence: `_artifacts/reports/fix-ast-grep-unwrap-mutex/before-summary.txt` vs `after-summary.txt`

### Edge Cases

- What happens when test code is in a separate `tests/` directory? (Should still allow unwrap)
- How does system handle nested `#[cfg(test)]` modules? (Should exclude all nested content)
- What about `#[bench]` and other test-like attributes? (Consider for future expansion)

## Requirements

### Functional Requirements

- **FR-001**: System MUST exclude inline test code (`#[cfg(test)]` modules) from rust-no-unwrap rule
- **FR-002**: System MUST exclude test functions (`#[test]` attribute) from rust-no-unwrap rule
- **FR-003**: System MUST exclude inline test code from rust-mutex-lock rule
- **FR-004**: System MUST continue to flag `unwrap()` in production code
- **FR-004a**: System MUST flag `expect()` without descriptive context in production code
- **FR-005**: System MUST handle errors explicitly in production code using `?` operator or proper error handling
- **FR-006**: System MUST provide meaningful context for `expect()` calls where they remain necessary (e.g., Mutex poisoning)
- **FR-007**: System MUST pass all quality gates (cargo fmt, clippy, test)
- **FR-008**: System MUST generate evidence of before/after ast-grep scan results

### Key Entities

- **AST-grep Rules**: YAML configuration files that define code patterns to detect
- **Inline Tests**: Rust code blocks marked with `#[cfg(test)]` or `#[test]` attributes
- **Production Code**: Non-test Rust source files that should follow strict error handling
- **Evidence Artifacts**: Scan outputs and test results stored for validation

## Review & Acceptance Checklist

### Content Quality

- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness

- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable with checkboxes
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

### Acceptance Criteria Validation

- [x] AC-001: Test module exclusion verified
- [x] AC-002: Test function exclusion verified
- [x] AC-003: Production unwrap() fixed (3 files)
- [x] AC-004: Expect messages added where needed
- [x] AC-005: cargo fmt passed
- [x] AC-006: cargo clippy passed
- [x] AC-007: cargo test passed (57 tests)
- [x] AC-008: Warnings reduced from 104 to 51

## Execution Status

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked (none found)
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
