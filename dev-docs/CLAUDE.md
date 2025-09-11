# CLAUDE.md (dev-docs/)

## Authority

- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- Normative docs: ../CONTRIBUTING.md, ../sdd-rules/lifecycle.md

## Purpose

Development documentation and evidence collection for the ACPLazyBridge project. This directory supports the SDD workflow through evidence management and technical documentation.

## What to do here

### Documentation Organization

```tree
dev-docs/
├── design/              # Architecture and design docs
├── engineering/         # Engineering guides (non-normative)
├── plan/               # Planning documents
│   └── issues/         # Issue tracking
│       ├── open/       # Active issues
│       ├── waiting/    # Blocked issues
│       └── closed/     # Completed issues
├── references/         # External references
├── requirements/       # Requirements documentation
├── review/             # Code review artifacts
│   ├── changes/        # Change logs
│   └── _artifacts/     # Evidence collection
└── sdd/                # SDD methodology docs
```

## Evidence Collection (_artifacts/)

### Directory Structure

```bash
dev-docs/review/_artifacts/
├── tests/              # Test results
│   └── <task>/        # Per-task test evidence
├── logs/              # Execution logs
│   └── <task>/        # Per-task logs
├── jq/                # JSON query results
│   └── <task>/        # Per-task JSON analysis
└── reports/           # Analysis reports
    └── <task>/        # Per-task reports
```

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
# Run tests with evidence capture
cargo test --workspace 2>&1 | tee dev-docs/review/_artifacts/tests/<task>/test_$(date +%Y%m%d_%H%M%S).log

# Capture ACP protocol testing
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp 2>&1 | \
  tee dev-docs/review/_artifacts/logs/<task>/acp_$(date +%Y%m%d_%H%M%S).log

# Generate coverage report
cargo tarpaulin --out Html --output-dir dev-docs/review/_artifacts/reports/<task>/
```

## Issue Management (plan/issues/)

### Issue States

- **open/**: Active issues being worked on
- **waiting/**: Blocked on dependencies or decisions
- **closed/**: Completed and archived

### Issue Template

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
- Tests: dev-docs/review/_artifacts/tests/<task>/
- Logs: dev-docs/review/_artifacts/logs/<task>/
```

## Requirements Documentation

### Requirements Traceability

Link requirements to:

1. **Specifications**: specs/<NNN>/spec.md
2. **Implementation**: crates/*/src/
3. **Tests**: crates/*/tests/
4. **Evidence**: dev-docs/review/_artifacts/

### Requirement Format

```markdown
REQ-<NNN>: <Description>
- Source: <spec reference>
- Implementation: <file:line>
- Test: <test file>
- Evidence: <artifact path>
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

- [ ] Test results in _artifacts/tests/
- [ ] Execution logs in _artifacts/logs/
- [ ] Performance metrics in _artifacts/reports/
- [ ] Coverage reports in _artifacts/reports/

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
- Before: _artifacts/logs/before_<date>.log
- After: _artifacts/logs/after_<date>.log
```

## SDD Documentation

### Maintaining SDD Docs

- Keep synchronized with implementation
- Update when specs change
- Link to current artifacts
- Version appropriately

### SDD References

- Principles: ../sdd-rules/spec-driven.md
- Lifecycle: ../sdd-rules/lifecycle.md
- Templates: ../sdd-rules/*-template.md

## Quick Commands

### Evidence Collection

```bash
# Create task evidence directory
mkdir -p dev-docs/review/_artifacts/<task>/{tests,logs,jq,reports}

# Run with evidence capture
<command> 2>&1 | tee dev-docs/review/_artifacts/<type>/<task>/<name>_$(date +%Y%m%d_%H%M%S).log
```

### Issue Management

```bash
# Move issue through states
mv dev-docs/plan/issues/open/issue.md dev-docs/plan/issues/waiting/
mv dev-docs/plan/issues/waiting/issue.md dev-docs/plan/issues/closed/
```

---

Specification Version: 1.0.3 | dev-docs/CLAUDE.md Format: 1.0 | Last Updated: 2025-09-11
