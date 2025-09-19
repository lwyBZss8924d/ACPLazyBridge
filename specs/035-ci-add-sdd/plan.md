# Implementation Plan: CI with SDD Gates and ast-grep Scanning

```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/ci-sdd-gates
feature_branch: 035-ci-add-sdd
created: 2025-09-19
last_updated: 2025-09-19
status: planning
input: Feature specification from `/specs/035-ci-add-sdd/spec.md`
spec_uri: specs/035-ci-add-sdd/spec.md
plan_uri: specs/035-ci-add-sdd/plan.md
tasks_uri: specs/035-ci-add-sdd/tasks.md
evidence_uris: _artifacts/035-ci-add-sdd/
specs:
    constitution: 1.0.1
    type: plan
    feature_number: 035
```

## Execution Flow (/plan command scope)

```text
1. Load feature spec from Input path
   → ✓ Loaded spec.md for CI integration
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → ✓ No NEEDS CLARIFICATION markers found
   → Project Type: CI/CD infrastructure (not web/mobile)
   → Structure Decision: GitHub Actions workflows
3. Fill the Constitution Check section based on the content of the constitution document
   → ✓ Evaluated against Articles I, III, VII, VIII, IX
4. Evaluate Constitution Check section below
   → No violations identified
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → ✓ Research completed on CI tools and patterns
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
   → ✓ Generated CI contracts and validation scenarios
7. Re-evaluate Constitution Check section
   → No new violations
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach
   → ✓ Task strategy defined
9. STOP - Ready for /tasks command
```

## Summary

Implementing comprehensive CI workflow with SDD quality gates and ast-grep code analysis for the ACPLazyBridge repository. The solution uses GitHub Actions to orchestrate multiple validation jobs including SDD structure checks, ast-grep security scanning with SARIF integration, and Rust quality gates. Implementation follows a two-stage rollout: report-only mode initially, transitioning to enforcement after Issue #31 completion.

## Technical Context

**Language/Version**: YAML (GitHub Actions), Bash scripting
**Primary Dependencies**: GitHub Actions, ast-grep CLI, cargo toolchain, GitHub Code Scanning
**Storage**: SARIF reports uploaded to GitHub Security tab
**Testing**: Validation through draft PR before merging
**Target Platform**: GitHub Actions runners (Ubuntu, macOS, Windows)
**Project Type**: CI/CD infrastructure
**Performance Goals**: CI completion under 30 minutes
**Constraints**: Must not block PRs during report-only phase
**Scale/Scope**: All pull requests and push events to main

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

**Simplicity**:

- Projects: 1 (single workflow file)
- Using framework directly? ✓ (GitHub Actions native)
- Single data model? ✓ (SARIF format for reports)
- Avoiding patterns? ✓ (direct action usage, no custom abstractions)

**Architecture**:

- EVERY feature as library? ✓ (reusable workflow components)
- Libraries listed: GitHub Actions reusable workflows
- CLI per library: GitHub CLI for testing (`gh workflow run`)
- Library docs: Inline YAML comments and README

**Testing (NON-NEGOTIABLE)**:

- RED-GREEN-Refactor cycle enforced? ✓ (draft PR testing)
- Git commits show tests before implementation? ✓
- Order: Contract→Integration→E2E→Unit strictly followed? ✓
- Real dependencies used? ✓ (actual GitHub runners)
- Integration tests for: workflow validation, SARIF upload
- FORBIDDEN: Implementation before test ✓

**Observability**:

- Structured logging included? ✓ (GitHub Actions annotations)
- Frontend logs → backend? N/A
- Error context sufficient? ✓ (job summaries and artifacts)

**Versioning**:

- Version number assigned? ✓ (workflow uses versioned actions)
- BUILD increments on every change? ✓ (git SHA tracking)
- Breaking changes handled? ✓ (two-stage rollout plan)

## Project Structure

### Documentation (this feature)

```tree
specs/035-ci-add-sdd/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)

```tree
.github/
├── workflows/
│   └── ci.yml           # Main CI workflow (enhanced)
scripts/
├── ci/                  # Existing CI scripts
│   ├── run-local-ci.sh
│   ├── run-sdd-structure-lint.sh
│   ├── check-language-policy.sh
│   └── run-markdown-style.sh
└── sdd/                 # SDD validation scripts
    ├── validate_structure.py
    ├── run_semantic_checks.sh
    └── check-markdown.sh
```

**Structure Decision**: Single workflow file with multiple jobs

## Phase 0: Outline & Research

1. **Extract unknowns from Technical Context**:
   - ast-grep SARIF format generation
   - GitHub Code Scanning API integration
   - Cross-platform matrix strategy
   - Cache optimization for Rust builds

2. **Generate and dispatch research agents**:

   ```bash
   Task: "Research ast-grep SARIF output format"
   Task: "Find GitHub Actions cache best practices for Rust"
   Task: "Research GitHub Code Scanning upload patterns"
   Task: "Find typos-cli integration examples"
   ```

3. **Consolidate findings** in `research.md`:
   - Decision: Use ast-grep's native JSON output, convert to SARIF
   - Rationale: Maximum compatibility with GitHub Security
   - Alternatives considered: Direct SARIF generation (not supported)

**Output**: research.md with implementation decisions

## Phase 1: Design & Contracts

_Prerequisites: research.md complete_

1. **Extract entities from feature spec** → `data-model.md`:
   - CI job definitions and dependencies
   - SARIF report structure
   - Configuration flags for report/enforce modes

2. **Generate API contracts** from functional requirements:
   - Workflow trigger contracts (PR, push)
   - Job output contracts (pass/fail/continue)
   - SARIF upload contract
   - Output to `/contracts/`

3. **Generate contract tests** from contracts:
   - Workflow syntax validation
   - Job dependency validation
   - SARIF format validation

4. **Extract test scenarios** from user stories:
   - Report-only mode validation
   - Enforcement mode validation
   - Transition scenario testing

5. **Update agent file incrementally**:
   - Add CI-specific guidance to CLAUDE.md
   - Document new workflow patterns
   - Keep under 150 lines

**Output**: data-model.md, /contracts/*, quickstart.md, CLAUDE.md updates

## Phase 2: Task Planning Approach

_This section describes what the /tasks command will do - DO NOT execute during /plan_

**Task Generation Strategy**:

- Load plan.md and related documents
- Generate tasks for workflow modification
- Each CI job → configuration task
- Each script integration → validation task
- Report-only implementation → specific task with continue-on-error
- Evidence collection → dedicated tasks

**Ordering Strategy**:

- Workflow structure changes first
- Individual job implementations next
- Integration and testing last
- Enforcement preparation as final task (blocked by Issue #31)

**Estimated Output**: 15-20 numbered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation

_These phases are beyond the scope of the /plan command_

**Phase 3**: Task execution (implementation of CI changes)
**Phase 4**: Validation (draft PR testing)
**Phase 5**: Rollout (merge and monitor)

## Complexity Tracking

_No violations identified - all changes within constitutional limits_

## Progress Tracking

_This checklist is updated during execution flow_

**Phase Status**:

- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:

- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none)

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
