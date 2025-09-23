# CLAUDE.md (dev-docs/)

## Authority

- Constitution: ../.specify/memory/constitution.md (Articles III, VII, IX)
- SDD Integration: ../.specify/CLAUDE.md (operational context)
- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- Normative docs: ../CONTRIBUTING.md, ../.specify/memory/lifecycle.md

## agent-client-protocol (ACP) protocol

- MUST SEE: (~/dev-space/agent-client-protocol)

## Purpose

Development documentation and evidence collection for the ACPLazyBridge project. This directory supports the SDD workflow through evidence management and technical documentation.

**IMPORTANT**: Evidence collection lives under `_artifacts/` at the repository root. Historical runs are preserved under the corresponding `_artifacts/<type>/legacy/` directories.

## SDD Integration

For comprehensive SDD workflow details, see **[../.specify/CLAUDE.md](../.specify/CLAUDE.md)**

### Constitutional Requirements for Documentation

- **Article III (Test-First)**: All evidence must show RED→GREEN→REFACTOR cycle
- **Article VII (Simplicity)**: Documentation should be clear and concise
- **Article IX (Integration-First)**: Document contracts before implementation

### Evidence Requirements

Every SDD task requires:

1. Test evidence showing initial failure (RED)
2. Implementation evidence showing success (GREEN)
3. Links to specs/<NNN>-<slug>/ artifacts
4. Traceability to constitutional articles

## What to do here

### Documentation Organization

```tree
dev-docs/
├── README.md              # Index of active collections
├── CLAUDE.md              # Developer playbook (this document)
├── _issues_drafts/        # Issue research drafts (pre-issue notes)
├── _projects/             # Project management plans
├── _requirements/         # Roadmap and requirement documents
├── architecture/          # Current architecture references
├── core_servers/          # Runtime/server design documentation
├── changelogs/            # Release and change history
└── references/            # External protocol and integration references
```

## Evidence Collection

### Evidence Locations

#### Primary Location: `_artifacts/`

```bash
_artifacts/
├── tests/<task>/
├── logs/<task>/
├── reports/<task>/
├── jq/<task>/
└── meta/<task>/
```

#### Legacy Archives

Historical artefacts remain available under the `legacy` subdirectories for each evidence area (e.g., `_artifacts/tests/legacy/`, `_artifacts/logs/legacy/`). Use them only when auditing past work.

### Migration Guidance

- Capture new evidence under `_artifacts/<task>/...`.
- Reference `_artifacts/<type>/legacy/` only for historical audits or regression analysis.

### Evidence Naming Conventions

```bash
# Test evidence
test_<YYYYMMDD>_<HHMMSS>.log
test_<module>_<YYYYMMDD>.json

# Execution logs
run_<YYYYMMDD>_<HHMMSS>.log
acp_protocol_<YYYYMMDD>.jsonl

# Reports
coverage_<YYYYMMDD>.html
performance_<YYYYMMDD>.md
```

### Collecting Evidence

```bash
# PRIMARY: Run tests with evidence capture (new location)
cargo test --workspace 2>&1 | tee _artifacts/tests/<task>/test_$(date +%Y%m%d_%H%M%S).log

# Capture ACP protocol testing (primary)
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp 2>&1 | \
  tee _artifacts/logs/<task>/acp_$(date +%Y%m%d_%H%M%S).log

# Generate coverage report (primary)
cargo tarpaulin --out Html --output-dir _artifacts/reports/<task>/
```

## Issue Management (`_issues_drafts/`)

### Issue States (SDD Workflow)

- Root (`_issues_drafts/*.md`): Drafts under analysis prior to opening GitHub issues.
- `closed/`: Historical drafts linked to merged issues/PRs (read-only).

### Issue Template (SDD-Aligned)

Use `dev-docs/_issues_drafts/TEMPLATE.md` when preparing a draft before opening a GitHub issue. The snippet below mirrors the template for quick reference.

```markdown
# Issue: <Title>

## Metadata
- Issue: #<number>
- Type: feature|bug|chore
- Priority: high|medium|low
- Spec: specs/<NNN>-<slug>/

## Description
What needs to be done

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Dependencies
- Blocked by: #<issue>
- Blocks: #<issue>

## Evidence
- Tests: `_artifacts/tests/<task>/`
- Logs: `_artifacts/logs/<task>/`
```

## Requirements Documentation

### Requirements Traceability (SDD)

Link requirements to:

1. **Specifications**: specs/<NNN>/spec.md (source of truth)
2. **Implementation**: crates/*/src/ (follows spec)
3. **Tests**: crates/*/tests/ (written first per Article III)
4. **Evidence**: `_artifacts/<task>/...` (see `_artifacts/<type>/legacy/` for historical runs)

### Requirement Format

```markdown
REQ-<NNN>: <Description>
- Source: specs/<NNN>/spec.md
- Constitutional: Article <X> compliance
- Implementation: <file:line>
- Test: <test file> (must fail first)
- Evidence: `_artifacts/<task>/`
```

## Design Documentation

### Architecture Decision Records (ADR)

```markdown
# ADR-<NNN>: <Title>

## Status
proposed|accepted|deprecated|superseded

## Context
Why this decision is needed

## Decision
What we decided

## Consequences
What happens as a result
```

### Design Patterns

Document recurring patterns:

- Protocol handling patterns
- Error management patterns
- Testing patterns
- CI/CD patterns

## Engineering Guides

### Non-Normative References

Engineering guides provide helpful context but are not authoritative:

- Always link back to normative documents
- Mark as "Guide" or "Reference"
- Include disclaimer when appropriate

### Guide Format

```markdown
# Guide: <Topic>

**Note**: This is a non-normative guide. See [normative doc] for authoritative information.

## Overview
Context and purpose

## Examples
Practical examples

## References
- Normative: [link to authority]
- Related: [other guides]
```

## Review Artifacts

### Code Review Checklist

For each PR, collect:

- [ ] Test results showing RED→GREEN cycle
- [ ] Evidence in _artifacts/<task>/ (primary) or_artifacts/legacy/<task>/ (legacy)
- [ ] Links to specs/<NNN>-<slug>/ artifacts
- [ ] Constitutional compliance verified
- [ ] Performance metrics if applicable
- [ ] Coverage reports if required

### Change Documentation

```markdown
# Changes: <PR Title>

## Summary
What changed and why

## Files Modified
- file1.rs: Description
- file2.rs: Description

## Tests Added/Modified
- test1: Purpose
- test2: Purpose

## Evidence
- Before: _artifacts/logs/<task>/before_<date>.log
- After: _artifacts/logs/<task>/after_<date>.log
- Specs: Link to specs/<NNN>-<slug>/
```

## SDD Documentation

### Maintaining SDD Docs

- Keep synchronized with specs/<NNN>-<slug>/
- Update when specifications change
- Link to current artifacts in both paths
- Follow Constitution version (currently 1.0.1)

### SDD References

- Constitution: ../.specify/memory/constitution.md
- Lifecycle: ../.specify/memory/lifecycle.md
- Principles: ../.specify/spec-driven.md
- Templates: ../.specify/templates/*-template.md
- Commands: ../.specify/commands/{specify,plan,tasks}.md

## Quick Commands

### Evidence Collection

```bash
# PRIMARY: Create task evidence directories
mkdir -p _artifacts/tests/<task> _artifacts/logs/<task> _artifacts/reports/<task> _artifacts/jq/<task>

# Run with evidence capture (primary)
<command> 2>&1 | tee _artifacts/<type>/<task>/<name>_$(date +%Y%m%d_%H%M%S).log

# Test-First Development (Article III)
cargo test <new_test> 2>&1 | tee _artifacts/tests/<task>/red_$(date +%Y%m%d_%H%M%S).log
# ... implement feature ...
cargo test <new_test> 2>&1 | tee _artifacts/tests/<task>/green_$(date +%Y%m%d_%H%M%S).log
```

### Issue Management

```bash
# Archive a completed issue draft
mv dev-docs/_issues_drafts/<draft>.md dev-docs/_issues_drafts/closed/
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "claude-memory"
    path: "./dev-docs/CLAUDE.md"
    version: "1.0.1"
    last_updated: "2025-09-17T08:26:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - "sdd-rules/rules/README.md"
        - ".specify/templates/spec-template.md"
        - ".specify/templates/plan-template.md"
        - ".specify/templates/tasks-template.md"
        - "./CLAUDE.md"
```
