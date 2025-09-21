# Research: CI Markdown Style Verification

## Decision Summary

### 1. Local-First vs CI-Fixing Approach

**Decision**: Local-first with CI verification only
**Rationale**:

- Faster feedback loop (seconds vs minutes)
- No context switching for developers
- Avoids "fix markdown" commits polluting PRs
- Aligns with SDD Article VII (Simplicity)

**Alternatives Considered**:

- CI auto-fixing: Rejected due to PR pollution and slower feedback
- Pre-commit hooks only: Rejected due to bypass potential and setup complexity

### 2. Markdown Linting Configuration

**Decision**: Disable MD013 (line length checking)
**Rationale**:

- GitHub Flavored Markdown has no line length requirements
- Modern renderers handle wrapping elegantly
- Reduces unnecessary friction (519 violations, 96% are MD007)
- Current config already uses 1024 (effectively unlimited)

**Alternatives Considered**:

- Keep 120 character limit: Rejected as it adds no value for GitHub rendering
- Keep 80 character limit: Rejected as overly restrictive for documentation

### 3. CI Implementation Strategy

**Decision**: Start with continue-on-error: true (report-only)
**Rationale**:

- Allows gradual team adoption
- Identifies scope of required fixes
- Non-disruptive rollout

**Migration Path**:

- Week 1: Deploy report-only
- Weeks 2-3: Fix violations locally
- Week 4: Remove continue-on-error

### 4. Tool Selection

**Decision**: markdownlint-cli2 over markdownlint-cli v1
**Rationale**:

- Better performance on large file sets
- Native .markdownlintignore support
- Glob pattern support
- Already preferred by run-markdown-style.sh script

**Alternatives Considered**:

- markdownlint-cli v1: Works but slower
- Other linters (remark, textlint): Different rule sets, migration complexity

### 5. GitHub Actions Configuration

**Decision**: Path filtering on **/*.md
**Rationale**:

- Reduces unnecessary CI runs
- Faster PR feedback when no docs changed
- Conserves CI minutes

**Alternatives Considered**:

- Run on all PRs: Wasteful for non-doc changes
- Manual trigger only: Misses automatic validation

## Current State Analysis

### Existing Violations

- **Total**: 519 violations across 139 markdown files
- **Breakdown**:
    - MD007 (list indentation): 498 (96%)
    - MD022 (heading blanks): 10
    - MD032 (list blanks): 8
    - MD047 (file ending): 1
    - MD009 (trailing spaces): 1
    - MD003 (heading style): 1

### Auto-fixable Issues

- 20 violations can be auto-fixed via fix-markdown.sh
- Remaining 499 require manual intervention (mostly MD007)

### Integration Points

- `scripts/ci/run-local-ci.sh` already includes markdown checks
- `scripts/sdd/fix-markdown.sh` available for auto-fixes
- `.markdownlint.json` and `.markdownlintignore` already configured

## Implementation Recommendations

1. **Phase 1**: Update .markdownlint.json to disable MD013
2. **Phase 2**: Add GitHub Actions workflow in report-only mode
3. **Phase 3**: Run fix-markdown.sh for quick wins
4. **Phase 4**: Document local workflow in contributing guides
5. **Phase 5**: Gradual enforcement after team adaptation

## Risk Analysis

### Low Risks

- Configuration changes are easily reversible
- Report-only mode prevents disruption
- Existing scripts handle the validation

### Mitigations

- Clear documentation of local workflow
- Gradual transition period
- Team communication about changes

## References

- [GitHub Flavored Markdown Spec](https://github.github.com/gfm/)
- [Google Markdown Style Guide](https://google.github.io/styleguide/docguide/style.html)
- [markdownlint Rules](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md)
- Issue #28 and comment analysis
