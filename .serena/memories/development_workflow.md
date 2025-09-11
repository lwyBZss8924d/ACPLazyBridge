# ACPLazyBridge Development Workflow

## Specification-Driven Development (SDD) Workflow

The project follows a strict SDD methodology. Every feature or change must follow this workflow:

### 1. Specification Phase (`/specify`)

Create a feature specification under `specs/<NNN>-<slug>/spec.md`:

```markdown
# Specification: <Feature Name>

## Overview
Brief description of the feature

## Requirements
- Functional requirements
- Non-functional requirements
- Constraints

## User Stories
As a [role], I want [feature], so that [benefit]

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2
```

### 2. Planning Phase (`/plan`)

Create technical design in `specs/<NNN>-<slug>/plan.md`:

```markdown
# Implementation Plan: <Feature Name>

## Technical Approach
- Architecture decisions
- Component design
- Integration points

## Dependencies
- External libraries
- Internal modules

## Risk Assessment
- Technical risks
- Mitigation strategies
```

### 3. Task Breakdown (`/tasks`)

Create executable tasks in `specs/<NNN>-<slug>/tasks.md`:

```markdown
# Tasks: <Feature Name>

## Task List
- [ ] Task 1: Description
- [ ] Task 2: Description
- [ ] Task 3: Description

## Task Dependencies
Task 2 depends on Task 1

## Estimates
- Task 1: 2 hours
- Task 2: 4 hours
```

## 4. Implementation Phase

### Worktree-First Development

**ALWAYS** create a new worktree for development:

```bash
# Create worktree from origin/main
git -C /Users/arthur/dev-space/ACPLazyBridge worktree add \
  /Users/arthur/dev-space/acplb-worktrees/<task-dir> \
  origin/main -b <branch>

# Example for SDD feature
git -C /Users/arthur/dev-space/ACPLazyBridge worktree add \
  /Users/arthur/dev-space/acplb-worktrees/001-feature-name \
  origin/main -b feature/001-feature-name

# Optional IDE navigation symlink
ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> \
  /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>
```

### Branch Naming Convention

Format: `<category>/<NNN>-<module>-<description>`

Categories:

- `feature/` - New features
- `fix/` - Bug fixes
- `perf/` - Performance improvements
- `chore/` - Maintenance tasks
- `docs/` - Documentation

Examples:

- `feature/001-codex-initialize`
- `fix/002-streaming-buffer`
- `docs/003-sdd-rules`

## 5. Validation Phase

### Quality Gates (Must Pass)

```bash
# Rust formatting
cargo fmt --all -- --check

# Clippy linting
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Tests
cargo test --workspace --all-features --locked

# SDD structure validation
scripts/ci/run-sdd-structure-lint.sh

# Language policy check
scripts/ci/check-language-policy.sh

# Markdown style check
markdownlint . --config .markdownlint.json

# Full local CI (runs all checks)
scripts/ci/run-local-ci.sh
```

### Evidence Collection

Store evidence in `dev-docs/review/_artifacts/<task>/`:

```bash
# Create task evidence directory
mkdir -p dev-docs/review/_artifacts/<task>/{tests,logs,reports}

# Run tests with evidence
cargo test --workspace 2>&1 | tee dev-docs/review/_artifacts/<task>/logs/test_$(date +%Y%m%d_%H%M%S).log

# Capture ACP protocol tests
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp 2>&1 | \
  tee dev-docs/review/_artifacts/<task>/logs/acp_test_$(date +%Y%m%d_%H%M%S).log
```

## 6. Review Phase

### Pull Request Requirements

**(1) Title Format**:

`<category>(<module>): <description>`

- Example: `feat(codex): implement initialize handler`

**(2) Description Template**:

```markdown
## Summary
Implements [feature] as specified in specs/<NNN>-<slug>/

## Changes
- Added X to handle Y
- Modified Z for compatibility

## Testing
- Test evidence: dev-docs/review/_artifacts/<task>/
- All quality gates pass
- Local CI run: ✅

## SDD Compliance
- [ ] Specification: specs/<NNN>-<slug>/spec.md
- [ ] Plan: specs/<NNN>-<slug>/plan.md
- [ ] Tasks: specs/<NNN>-<slug>/tasks.md
- [ ] Evidence: dev-docs/review/_artifacts/<task>/

## References
- Issue: #<issue-number>
- ACP Spec: Section X.Y
```

**(3) Create PR**:

```bash
# Push to remote
git push -u origin <branch-name>

# Create PR via GitHub CLI
gh pr create --title "<category>(<module>): <description>" \
  --body "$(cat pr_description.md)"
```

## 7. Post-Merge Cleanup

```bash
# Remove worktree after merge
git worktree remove /Users/arthur/dev-space/acplb-worktrees/<task-dir>

# Remove symlink if created
rm /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>

# Update main branch
git checkout main
git pull origin main

# Prune remote branches
git remote prune origin
```

## Constitutional Requirements

### Simplicity (Article VII)

- Maximum 3 projects in workspace
- No future-proofing
- Remove unused features

### Anti-Abstraction (Article VIII)

- Use framework features directly
- No unnecessary wrappers
- Concrete implementations

### Integration-First (Article IX)

- Define contracts before implementation
- Test with real protocol messages
- Validate against ACP spec

### Test-First (Article III)

- Write failing tests first (RED)
- Implement to pass (GREEN)
- Refactor if needed (REFACTOR)

## Language Policy

### Normative Artifacts (English Required)

- Specifications (`specs/`)
- Plans and tasks
- Issues and PRs
- Commit messages
- Code comments

### Non-Normative (Any Language)

- Development notes
- Chinese docs under `dev-docs/zh-CN/`
- Team discussions

## Collaboration Guidelines

- Always start from `origin/main`
- Never commit directly to main
- Use squash merge for clean history
- Keep evidence for traceability
- Coordinate via issues and specs
- Follow SDD lifecycle strictly

---

Specification Version: 1.0.3 | development_workflow.md ("serena" MCP's memories) Format: 1.0 | Last Updated: 2025-09-11
