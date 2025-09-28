# AGENTS.md (specs/)

- ⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
- ⚠️ _Fllow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
- ⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_

📌 When AI-Engineer SDD-TASKs Cooking Workflow can follow the BASELINE TEMPLATES work in (specs/): [AI-Engineer-SDD-Workflow-Baseline-templates](.specify/memory/AI-Engineer-SDD-Workflow-Baseline-templates.txt)

## Authority

- Constitution: ../.specify/memory/constitution.md (Articles I, III, VII, VIII, IX)
- SDD Integration: ../.specify/CLAUDE.md (operational context)
- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- Templates: ../.specify/templates/{spec,plan,tasks}-template.md
- Lifecycle: ../.specify/memory/lifecycle.md

## Purpose

This directory contains all feature specifications following the SDD lifecycle. Each feature has its own numbered subdirectory containing specification artifacts that drive the implementation according to the Specification-Driven Development principles.

## SDD Integration

For comprehensive SDD workflow details, see **[../.specify/AGENTS.md](../.specify/AGENTS.md)**
Key SDD principles for specifications:

- **Library-First (Article I)**: All features start as library specifications
- **Test-First (Article III)**: Specifications drive test creation before implementation
- **Simplicity (Article VII)**: Specifications avoid over-engineering
- **Anti-Abstraction (Article VIII)**: Specifications use direct, concrete requirements
- **Integration-First (Article IX)**: Specifications define contracts upfront

## What to Do Here

### Creating a New Feature

1. **Use /specify command** to auto-create spec structure
2. **Follow template structure** from ../.specify/templates/spec-template.md
3. **Mark ambiguities** with `[NEEDS CLARIFICATION]`
4. **Include metadata block**:

   ```yaml
   Issue-URI: https://github.com/org/repo/issues/NNN
   Spec-URI: specs/NNN-feature/spec.md
   Plan-URI: specs/NNN-feature/plan.md
   Tasks-URI: specs/NNN-feature/tasks.md
   Evidence-URIs:
     - _artifacts/<task>/ (primary)
     - _artifacts/legacy/<task>/ (legacy)
   ```

### Directory Structure

```tree
specs/
├── CLAUDE.md              # This guidance file
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

### Best practice**: (specs/<NNN>-<slug>) in SDD TASKs WORKTREE

(/ACPLazyBridge) | worktree: (acplb-worktrees/038-adopt-acp-runtime)

```tree
acplb-worktrees/038-adopt-acp-runtime/specs/038-adopt-acp-runtime
❯ tree
.
├── contracts
│   └── runtime_api.md
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
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

- [ ] **Library-First Gate** (Article I): Feature as library with optional CLI
- [ ] **Test-First Gate** (Article III): Tests written before code
- [ ] **Simplicity Gate** (Article VII): ≤3 projects, no future-proofing
- [ ] **Anti-Abstraction Gate** (Article VIII): Using framework directly
- [ ] **Integration-First Gate** (Article IX): Contracts defined

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
2. **Test writing** tasks before implementation (RED→GREEN→REFACTOR)
3. **Implementation** tasks follow TDD cycle
4. **Integration** tasks last

### Evidence Collection

Tasks must include evidence collection:

```markdown
- [ ] Write failing test
  Evidence: _artifacts/tests/<task>/red_YYYYMMDD.log
- [ ] Implement to pass test
  Evidence: _artifacts/tests/<task>/green_YYYYMMDD.log
```

## Common Patterns

### Feature Specification Pattern

```markdown
# Specification: Feature Name
## Metadata
Issue-URI: ...
Spec-URI: ...
Evidence-URIs:
  - _artifacts/<task>/ (primary)
  - _artifacts/legacy/<task>/ (legacy)
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
### Constitutional Compliance
- [ ] Library-First (Article I)
- [ ] Test-First (Article III)
- [ ] Simplicity (Article VII)
- [ ] Anti-Abstraction (Article VIII)
- [ ] Integration-First (Article IX)
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
- Test approach (RED→GREEN→REFACTOR)
### Integration Points
- External dependencies
```

## SDD Workflow Commands

```bash
# Start new feature (from repo root)
./scripts/sdd/create-new-feature.sh "Feature description"
# Or use command:
/specify "Feature description here"
# Generate plan (after spec exists)
./scripts/sdd/setup-plan.sh
# Or use command:
/plan "Technical approach details"
# Create tasks (after plan exists)
./scripts/sdd/check-task-prerequisites.sh
# Then use command:
/tasks
```

## Links and References

### Templates

- [Spec Template](../.specify/templates/spec-template.md)
- [Plan Template](../.specify/templates/plan-template.md)
- [Tasks Template](../.specify/templates/tasks-template.md)
- [Agent Template](../.specify/templates/agent-file-template.md)

### SDD Documentation

- [Constitution](../.specify/memory/constitution.md) - Core SDD principles
- [Lifecycle](../.specify/memory/lifecycle.md) - SDD workflow lifecycle
- [SDD Commands](../.specify/commands/) - Command implementations
- [SDD Scripts](../scripts/sdd/) - Automation tools

### Evidence Paths

- Primary: `_artifacts/{tests,logs,reports,jq}/<task>/`
- Legacy: `_artifacts/legacy/{tests,logs,reports,jq}/<task>/`

### Related Documentation

- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
- [sdd-rules/AGENTS.md](../sdd-rules/AGENTS.md) - Team coordination
- [sdd-rules/CLAUDE.md](../sdd-rules/CLAUDE.md) - Claude-specific rules

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "codex-memory"
    path: "./specs/AGENTS.md"
    version: "1.0.1"
    last_updated: "2025-09-27T10:05:00Z"
    dependencies:
        - "./AGENTS.md"
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - ".specify/memory/AI-Engineer-SDD-Workflow-Baseline-templates.txt"
        - "sdd-rules/rules/README.md"
        - ".specify/templates/spec-template.md"
        - ".specify/templates/plan-template.md"
        - ".specify/templates/tasks-template.md"
```
