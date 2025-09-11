# Specification-Driven Development (SDD) Rules

## Overview

Specification-Driven Development (SDD) is the mandatory development methodology for ACPLazyBridge. Every feature, bug fix, or change must follow the SDD lifecycle to ensure quality, traceability, and maintainability.

## SDD Lifecycle Phases

### 1. Specify Phase (`/specify`)

**Purpose**: Define WHAT needs to be built

**Output**: `specs/<NNN>-<slug>/spec.md`

**Requirements**:

- Clear problem statement
- Functional requirements
- Non-functional requirements
- User stories with acceptance criteria
- Success metrics

**Template Structure**:

```markdown
# Specification: <Feature Name>

## Overview
[Problem statement and context]

## Requirements
### Functional Requirements
- REQ-001: [Requirement]
- REQ-002: [Requirement]

### Non-Functional Requirements
- NFR-001: [Performance/Security/etc]

## User Stories
As a [role], I want [feature], so that [benefit]

## Acceptance Criteria
- [ ] AC-001: [Testable criterion]
- [ ] AC-002: [Testable criterion]
```

### 2. Plan Phase (`/plan`)

**Purpose**: Define HOW to build it

**Output**: `specs/<NNN>-<slug>/plan.md`

**Requirements**:

- Technical approach
- Architecture decisions
- Component breakdown
- Risk assessment
- Dependencies

**Template Structure**:

```markdown
# Implementation Plan: <Feature Name>

## Technical Approach
[Overall strategy]

## Architecture
[Component design and interactions]

## Dependencies
- External: [Libraries, APIs]
- Internal: [Modules, components]

## Risk Assessment
- Risk: [Description] | Mitigation: [Strategy]
```

### 3. Tasks Phase (`/tasks`)

**Purpose**: Break down into executable steps

**Output**: `specs/<NNN>-<slug>/tasks.md`

**Requirements**:

- Atomic, testable tasks
- Clear dependencies
- Time estimates
- Verification criteria

**Template Structure**:

```markdown
# Tasks: <Feature Name>

## Task List
- [ ] TASK-001: [Description] (Est: Xh)
- [ ] TASK-002: [Description] (Est: Xh)

## Dependencies
TASK-002 depends on TASK-001

## Verification
- TASK-001: Run test X
- TASK-002: Check output Y
```

### 4. Implement Phase

**Purpose**: Execute the tasks

**Requirements**:

- Follow worktree-first development
- Maintain evidence trail
- Pass all quality gates
- Document as you go

### 5. Validate Phase

**Purpose**: Ensure quality and compliance

**Requirements**:

- All tests pass
- Code quality checks pass
- SDD structure validated
- Evidence collected

### 6. Review Phase

**Purpose**: Peer review and merge

**Requirements**:

- Complete PR template
- Link to specs and evidence
- Pass CI/CD checks
- Squash merge to main

## Constitutional Gates

### Article III: Test-First Development

- Write failing tests first (RED)
- Implement to pass (GREEN)
- Refactor if needed (REFACTOR)
- No code without tests

### Article VII: Simplicity

- Maximum 3 projects in workspace
- No future-proofing
- Remove unused features
- Prefer deletion over addition

### Article VIII: Anti-Abstraction

- Use framework features directly
- No unnecessary wrappers
- Concrete over abstract
- Explicit over implicit

### Article IX: Integration-First

- Define contracts before implementation
- Test with real protocol messages
- Validate against specifications
- No mocking when avoidable

## Rules Categories

### Documentation Style (`sdd-rules/rules/documentation-style/`)

- Follow Google Developer Documentation Style Guide
- Use markdownlint configuration
- Maintain consistent formatting
- Clear, concise technical writing

### Git Workflow (`sdd-rules/rules/git/`)

#### Worktree Management

- ALWAYS use worktrees for development
- Never develop on main branch
- One worktree per feature/task
- Clean up after merge

#### Pull Requests

- Follow PR template strictly
- Link to specifications
- Include evidence artifacts
- Squash merge only

#### Commit Messages

- Format: `<type>(<scope>): <description>`
- Types: feat, fix, docs, style, refactor, perf, test, chore
- Include references to tasks/issues
- Sign-off when required

### CI/CD (`sdd-rules/rules/ci/`)

- Must pass all local checks before push
- Evidence collection mandatory
- Automated validation in CI
- No merge without green CI

### Testing (`sdd-rules/rules/tests/`)

- Unit tests for all functions
- Integration tests for protocols
- Evidence-based testing
- Test documentation required

### Code Analysis (`sdd-rules/rules/code-analysis/`)

- Static analysis with clippy
- Security scanning with CodeQL
- Performance profiling when needed
- Complexity metrics tracking

## SDD Commands

### `/specify` Command

- Generates specification template
- Validates requirement format
- Creates spec directory structure
- Links to issue tracking

### `/plan` Command

- Generates plan template
- Validates technical approach
- Checks dependency availability
- Risk assessment framework

### `/tasks` Command

- Generates task template
- Creates task breakdown
- Estimates complexity
- Sets up verification criteria

## Quality Checkpoints

### Pre-Implementation

- [ ] Specification approved
- [ ] Plan reviewed
- [ ] Tasks estimated
- [ ] Worktree created

### During Implementation

- [ ] Following TDD cycle
- [ ] Collecting evidence
- [ ] Running local CI
- [ ] Updating documentation

### Post-Implementation

- [ ] All tests pass
- [ ] Quality gates pass
- [ ] Evidence complete
- [ ] PR template filled

## Enforcement

### Automated Checks

```bash
# SDD structure validation
scripts/ci/run-sdd-structure-lint.sh

# Full validation suite
scripts/ci/run-local-ci.sh
```

### Manual Review

- Specification completeness
- Plan feasibility
- Task atomicity
- Evidence traceability

## Language Policy

### Normative (English Required)

- All specifications
- All plans and tasks
- All code and comments
- All commit messages
- All PR descriptions

### Non-Normative (Any Language)

- Development notes
- Team discussions
- Research documents
- Local documentation

## Key Paths

- **Specifications**: `specs/<NNN>-<slug>/`
- **SDD Rules**: `sdd-rules/rules/`
- **Templates**: `sdd-rules/templates/`
- **Commands**: `sdd-rules/commands/`
- **Evidence**: `dev-docs/review/_artifacts/`

---

Specification Version: 1.0.3 | SDD_rules.md ("serena" MCP's memories) Format: 1.0 | Last Updated: 2025-09-11
