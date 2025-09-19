# Implementation Plan: Fix ast-grep Inline Test False Positives

```yaml
worktree: /acplb-worktrees/fix-ast-grep-inline-tests
feature_branch: fix/ast-grep-inline-tests
created: 2025-09-19
last_updated: 2025-09-19T04:32:00Z
status: processing
input: GitHub Issue #34
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/34
spec_uri: specs/034-fix-ast-grep-inline-tests/spec.md
plan_uri: specs/034-fix-ast-grep-inline-tests/plan.md
tasks_uri: specs/034-fix-ast-grep-inline-tests/tasks.md
evidence_uris: _artifacts/reports/ast-grep-inline-tests/
specs:
    constitution: 1.0.1
    type: plan
    feature_number: 034
```

## Phase -1: Pre-Implementation Gates

### Constitutional Compliance

- [ ] Library-First (Article I): N/A - Configuration changes only
- [ ] Test-First (Article III): Testing configuration effectiveness
- [ ] Simplicity (Article VII): Using built-in ast-grep features
- [ ] Anti-Abstraction (Article VIII): Direct configuration, no wrappers
- [ ] Integration-First (Article IX): Working within existing tooling

## Phase 0: Research & Design

### Technical Context

After extensive research and testing, we've identified that:

1. **AST-based exclusion doesn't work**: Patterns like `inside`, `has`, `follows`, `precedes` fail to exclude inline tests
2. **File-based exclusion works**: Can exclude entire test files successfully
3. **Suppression comments work**: ast-grep supports `// ast-grep-ignore` comments

### Solution Architecture

Two-pronged approach:

1. **File-based exclusion**: Comprehensive patterns in rule YAML files
2. **Suppression comments**: For inline tests in src files

### Technology Choices

- Use existing ast-grep configuration (no new tools)
- Leverage built-in suppression comment feature
- Maintain compatibility with IDE integrations

## Phase 1: Core Implementation

### Rule Configuration Updates

Update both `rust-no-unwrap.yml` and `rust-mutex-lock.yml`:

```yaml
files:
  - "**/*.rs"
  - "!**/tests/**"       # Test directories
  - "!**/*_test.rs"      # Test files ending with _test
  - "!**/*_tests.rs"     # Test files ending with _tests
  - "!**/test_*.rs"      # Test files starting with test_
  - "!**/benches/**"     # Benchmark directories
  - "!**/examples/**"    # Example directories
```

### Suppression Comment Strategy

For inline test modules in src files:

```rust
#[cfg(test)]
mod tests {
    // ast-grep-ignore: rust-no-unwrap, rust-mutex-lock
    use super::*;
    // All tests in this module are suppressed
}
```

### Documentation Updates

- Add `note` field to rule YAML files explaining suppression
- Update CONTRIBUTING.md with ast-grep guidelines
- Document in sdd-rules/rules/code-analysis/

## Phase 2: Integration & Testing

### Test Strategy

1. **Baseline measurement**: Count warnings before changes
2. **Apply file exclusions**: Verify test file warnings removed
3. **Add suppression comments**: Verify inline test warnings removed
4. **Final measurement**: Confirm >90% reduction

### Integration Points

- IDE ast-grep extensions (VS Code, Cursor)
- CI/CD ast-grep scans
- Pre-commit hooks

### Validation Approach

```bash
# Before changes
ast-grep scan -c sgconfig.yml . | grep -c "warning"

# After changes
ast-grep scan -c sgconfig.yml . | grep -c "warning"

# Verify reduction
```

## Phase 3: Documentation & Rollout

### Documentation Requirements

1. **Rule files**: Add inline documentation
2. **CONTRIBUTING.md**: Section on ast-grep usage
3. **SDD rules**: Update code-analysis documentation

### Rollout Strategy

1. Test in worktree first
2. Verify with multiple IDEs
3. Create PR with evidence
4. Merge to main

## Decision Log

| Decision | Rationale | Trade-offs |
|----------|-----------|------------|
| Use file exclusion patterns | Works reliably, simple to maintain | Doesn't handle inline tests |
| Use suppression comments | Built-in feature, targeted control | Requires manual annotation |
| Don't pursue AST patterns | Testing showed they don't work | Would be cleaner if it worked |
| Document in rule files | Discoverable by developers | Adds verbosity to rules |

## Risk Analysis

### Technical Risks

- **Over-suppression**: Mitigated by targeted comments
- **IDE incompatibility**: Mitigated by testing multiple IDEs
- **Maintenance burden**: Mitigated by clear documentation

### Process Risks

- **Developer resistance**: Mitigated by reducing false positives
- **Forgotten suppressions**: Mitigated by module-level comments

## Success Metrics

- Warning count reduced from 86 to <10
- No production code warnings suppressed
- Developer satisfaction with clean output
- Documentation clarity (no questions in first month)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
