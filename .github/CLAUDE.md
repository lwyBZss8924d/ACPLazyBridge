# CLAUDE.md (.github/)

## Authority

- See ../.specify/memory/constitution.md (project Constitution), ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- SDD Integration: ../.specify/CLAUDE.md (Claude Code operational context)
- PR Template: PULL_REQUEST_TEMPLATE.md
- Issue Templates: ISSUE_TEMPLATE/

## Purpose

GitHub-specific configuration and automation for the ACPLazyBridge project. This directory contains workflows, templates, and policies that integrate with the SDD workflow.

## SDD Integration

For comprehensive SDD workflow execution details, see **[../.specify/CLAUDE.md](../.specify/CLAUDE.md)**

Key GitHub-specific SDD integration points:

- **PR Creation**: Must link to specs/<NNN>-<slug>/ artifacts (spec.md, plan.md, tasks.md)
- **Evidence Collection**: Store in `_artifacts/` or `_artifacts/legacy/` for historical data
- **Constitutional Gates**: Enforce via PR template checkboxes and CI checks
- **Branch Strategy**: Always use worktrees from origin/main, never develop on main
- **Commit Format**: Include [TASK-NNN] or [BUG-NNN] references

SDD Workflow for GitHub:

```text
Feature Request (Issue) → /sdd-task or /specify → spec.md → /plan → plan.md → /tasks → tasks.md → PR → Review → Merge
```

All PRs must pass:

- Constitutional gates (Articles I, III, VII, VIII, IX)
- Quality gates (fmt, clippy, test)
- SDD structure validation
- Evidence documentation

## What to do here

### GitHub Actions Workflows

```yaml
# Example workflow structure
name: CI
on: [push, pull_request]
jobs:
  sdd-validation:
    - Check SDD structure
    - Validate language policy
    - Run quality gates
```

### Current Workflows

- **ci.yml**: Main CI pipeline
- **claude.yml**: Claude Code specific checks
- **claude-code-review.yml**: Automated code review

## Pull Request Standards

### PR Title Format

```text
<type>(<scope>): <description>
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `perf`: Performance improvement
- `chore`: Maintenance
- `docs`: Documentation

### PR Description Template

```markdown
## Summary
Implements [feature] as specified in specs/<NNN>-<slug>/

## Changes
- Added X to handle Y
- Modified Z for compatibility

## SDD Compliance
- [ ] Specification: specs/<NNN>-<slug>/spec.md
- [ ] Plan: specs/<NNN>-<slug>/plan.md
- [ ] Tasks: specs/<NNN>-<slug>/tasks.md
- [ ] Evidence: _artifacts/<task>/ (primary) or _artifacts/legacy/<task>/ (historical)

## Testing
- [ ] All quality gates pass
- [ ] Local CI run: ✅
- [ ] Test evidence collected

## Constitutional Gates
- [ ] Article I (Library-First): Library interface changes? [yes/no]
- [ ] Article II (CLI): CLI surface changes? [yes/no]
- [ ] Article VII (Simplicity): ≤3 projects maintained
- [ ] Article VIII (Anti-Abstraction): Framework used directly

## References
- Issue: #<number>
- Commits include [TASK-NNN] or [BUG-NNN]
```

## Issue Templates

### Feature Request

Required fields:

- Title with feature description
- User story format
- Acceptance criteria
- Link to draft spec (if exists)

### Bug Report

Required fields:

- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Link to evidence/logs

### Engineering Task

Required fields:

- Task description
- Dependencies
- Definition of done
- Link to parent spec/plan

## GitHub CLI Integration

### Common PR Operations

```bash
# Create PR with SDD compliance
gh pr create \
  --title "feat(codex): implement initialize handler" \
  --body "$(cat pr_description.md)" \
  --assignee @me

# Link PR to issue
gh pr edit <pr-number> --add-label "sdd-compliant"

# Review PR checklist
gh pr view <pr-number> --json statusCheckRollup
```

### Issue Management

```bash
# Create issue from spec
gh issue create \
  --title "Implement <feature>" \
  --body "Spec: specs/<NNN>-<slug>/spec.md" \
  --label "feature"

# Link issue to PR
gh issue develop <issue-number> --branch feature/<NNN>-<slug>
```

## Branch Protection Rules

### Main Branch

- Require PR reviews
- Require status checks:
    - SDD structure validation
    - Language policy check
    - Quality gates (fmt, clippy, test)
    - CI passing
- Require up-to-date branches
- Include administrators

### Feature Branches

Naming: `<category>/<NNN>-<module>-<description>`

- No direct commits
- Delete after merge

## CODEOWNERS

Define review requirements:

```text
# Global owners
* @team-lead

# SDD rules
/sdd-rules/ @sdd-maintainers

# Specifications
/specs/ @product-team @sdd-maintainers

# Core implementation
/crates/ @engineering-team
```

## CI/CD Integration

### Required Checks

```yaml
# Core jobs (GitHub Actions workflow)
build-test:
  - cargo-fmt
  - cargo-clippy
  - cargo-test
  - cross-platform (ubuntu, macos)

sdd-checks:
  - structure-lint
  - language-policy
  - protocol-scenarios
  - semantic-checks

ast-grep-scan:
  - code-quality-scan
  - sarif-conversion
  - github-security-upload
  - report-only-mode (current)

typos-check:
  - documentation-quality

docs-style:
  - markdown-style-verification
  - report-only-mode (current)
  - enforcement-mode (future)
```

### AST-grep Code Scanning

**Current Mode**: Report-only (continue-on-error: true)
**Future Mode**: Enforcement (after Issue #31)

Active rule categories:

- Rust: no-unwrap, no-dbg, todo-comment, mutex-lock
- JavaScript: no-console-log, no-only-in-tests
- Python: no-print, no-pdb
- Go: no-fmt-println

Results uploaded to GitHub Security tab via SARIF format.

### Markdown Style Verification

**Current Mode**: Report-only (continue-on-error: true)
**Future Mode**: Enforcement (after team adaptation period)

Configuration:

- Workflow: `.github/workflows/docs-style.yml`
- Script: `scripts/ci/run-markdown-style.sh`
- Config: `.markdownlint.json` (MD013 disabled for GitHub compatibility)
- Triggers on: Pull requests with `**/*.md` changes

Local development (verification-only approach):

- Run checks: `./scripts/ci/run-markdown-style.sh`
- Auto-fix issues: `./scripts/sdd/fix-markdown.sh`
- Detailed report: `./scripts/sdd/check-markdown.sh --verbose`

See `specs/036-ci-markdown-style/quickstart.md` for troubleshooting guide.

### Enhanced SDD Gates

`scripts/ci/run-sdd-gates.sh` orchestrates:

1. Structure validation
2. Language policy checks
3. Markdown linting
4. Semantic validation
5. Template drift detection
6. Error aggregation

### Artifact Management

```yaml
# Upload evidence artifacts
- uses: actions/upload-artifact@v4
  with:
    name: <category>-<timestamp>
    path: <artifact-path>

# Categories:
# - protocol-outputs: JSONL validation results
# - ast-grep-results: JSON findings and SARIF reports
# - test-evidence: Execution logs and coverage
```

## GitHub Actions Secrets

Required secrets:

- `GITHUB_TOKEN`: Automation
- `CARGO_REGISTRY_TOKEN`: Publishing
- `CODECOV_TOKEN`: Coverage reporting

## Labels

### SDD Labels

- `sdd-spec`: Specification ready
- `sdd-plan`: Plan documented
- `sdd-tasks`: Tasks defined
- `sdd-compliant`: Full SDD compliance
- `needs-clarification`: Has [NEEDS CLARIFICATION] markers

### Priority Labels

- `priority-high`: Blocking
- `priority-medium`: Important
- `priority-low`: Nice to have

### Status Labels

- `status-blocked`: Waiting on dependency
- `status-ready`: Ready for work
- `status-in-progress`: Being worked on
- `status-review`: In review

## Automation Scripts

### PR Validation

```bash
# Check SDD compliance
.github/workflows/scripts/check-sdd-compliance.sh

# Validate evidence
.github/workflows/scripts/validate-evidence.sh
```

### Auto-labeling

Based on:

- File paths changed
- PR title format
- Issue template used

## Quick Reference

### Creating Compliant PR

1. Ensure spec/plan/tasks exist
2. Run local CI: `scripts/ci/run-local-ci.sh`
3. Collect evidence in _artifacts/
4. Create PR with template
5. Link to issue and spec

### Reviewing PR

1. Check SDD compliance checkboxes
2. Verify evidence links
3. Review constitutional gates
4. Validate test results
5. Approve or request changes

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-20T16:00:00Z"
document:
    type: "claude-memory"
    path: ".github/CLAUDE.md"
    version: "1.1.0"
    last_updated: "2025-09-20T16:00:00Z"
    changelog: "Updated CI/CD Integration section following PR #36"
    dependencies:
        - ".specify/memory/constitution.md"
        - "./CLAUDE.md"
        - ".github/workflows/ci.yml"
```
