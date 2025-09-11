# CLAUDE.md (specs/)

## Authority

- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- Templates: ../sdd-rules/spec-template.md, ../sdd-rules/plan-template.md, ../sdd-rules/tasks-template.md

## Purpose

This directory contains all feature specifications following the SDD lifecycle. Each feature has its own numbered subdirectory containing specification artifacts.

## What to do here

### Creating a New Feature

1. **Use /specify command** to auto-create spec structure
2. **Follow template structure** from sdd-rules/spec-template.md
3. **Mark ambiguities** with `[NEEDS CLARIFICATION]`
4. **Include metadata block**:

   ```yaml
   Issue-URI: https://github.com/org/repo/issues/NNN
   Spec-URI: specs/NNN-feature/spec.md
   Plan-URI: specs/NNN-feature/plan.md
   Tasks-URI: specs/NNN-feature/tasks.md
   Evidence-URIs: dev-docs/review/_artifacts/NNN-feature/
   ```

### Directory Structure

```tree
specs/
├── 000-example/           # Example specification
├── 001-feature-name/      # Feature specification
│   ├── spec.md           # WHAT/WHY - Requirements
│   ├── plan.md           # HOW - Technical approach
│   ├── tasks.md          # Executable task breakdown
│   ├── data-model.md     # Data structures (if needed)
│   ├── contracts/        # API contracts (if needed)
│   ├── research.md       # Research findings (if needed)
│   └── quickstart.md     # Validation scenarios
└── NNN-next-feature/
```

## Specification Writing Guidelines

### Focus on WHAT and WHY, not HOW

✅ **Good**: "Users need real-time message delivery"
❌ **Bad**: "Implement WebSocket connection using Socket.io"

### Be Testable and Measurable

✅ **Good**: "Messages must be delivered within 100ms"
❌ **Bad**: "Messages should be fast"

### Mark Uncertainties

When information is missing or unclear:

```markdown
[NEEDS CLARIFICATION: What is the expected message throughput?]
```

## Plan Writing Guidelines

### Follow Constitutional Gates

Before implementation, verify:

- [ ] **Simplicity Gate** (Article VII): ≤3 projects, no future-proofing
- [ ] **Anti-Abstraction Gate** (Article VIII): Using framework directly
- [ ] **Integration-First Gate** (Article IX): Contracts defined
- [ ] **Test-First Gate** (Article III): Tests written before code

### Structure Technical Decisions

For each decision, document:

1. **Requirement traced**: Which spec requirement drives this?
2. **Options considered**: What alternatives exist?
3. **Rationale**: Why this approach?
4. **Trade-offs**: What are we gaining/losing?

## Task Writing Guidelines

### Task Format

```markdown
- [ ] [P] Task description (parallelizable)
- [ ] Task description (sequential)
  - [ ] Subtask 1
  - [ ] Subtask 2
```

### Task Ordering

1. **Contract definition** tasks first
2. **Test writing** tasks before implementation
3. **Implementation** tasks follow TDD cycle
4. **Integration** tasks last

## Common Patterns

### Feature Specification Pattern

```markdown
# Specification: Feature Name

## Metadata
Issue-URI: ...
Spec-URI: ...

## Overview
Brief description

## User Stories
As a [role], I want [feature], so that [benefit]

## Functional Requirements
- REQ-001: Requirement description
- REQ-002: Another requirement

## Non-Functional Requirements
- NFR-001: Performance requirement
- NFR-002: Security requirement

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Out of Scope
- What this feature does NOT include
```

### Implementation Plan Pattern

```markdown
# Implementation Plan: Feature Name

## Phase -1: Pre-Implementation Gates
### Simplicity Gate
- [ ] Checked

### Anti-Abstraction Gate
- [ ] Checked

## Phase 0: Research & Design
### Technical Context
- Architecture decisions
- Technology choices

## Phase 1: Core Implementation
### Components
- Component descriptions

### Data Model
- Entity definitions

## Phase 2: Integration & Testing
### Test Strategy
- Test approach

### Integration Points
- External dependencies
```

## Links and References

- **Templates**: ../sdd-rules/{spec,plan,tasks}-template.md
- **Lifecycle**: ../sdd-rules/lifecycle.md
- **Commands**: ../sdd-rules/commands/
- **Evidence**: ../dev-docs/review/_artifacts/

## Quick Commands

```bash
# Start new feature (from repo root)
/specify "Feature description here"

# Generate plan (after spec exists)
/plan "Technical approach details"

# Create tasks (after plan exists)
/tasks
```

---

Specification Version: 1.0.3 | specs/CLAUDE.md Format: 1.0 | Last Updated: 2025-09-11
