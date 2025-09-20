# Implementation Plan: CI Markdown Style Verification

```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/028-markdown-ci
feature_branch: docs/028-markdown-ci-verification
created: 2025-09-20
last_updated: 2025-09-20
status: in-progress
input: Feature specification from `/specs/036-ci-markdown-style/spec.md`
spec_uri: specs/036-ci-markdown-style/spec.md
plan_uri: specs/036-ci-markdown-style/plan.md
tasks_uri: specs/036-ci-markdown-style/tasks.md
evidence_uris: _artifacts/036-ci-markdown-style/
specs:
    constitution: 1.0.1
    type: plan
    feature_number: 036
```

## Summary

Implement a GitHub Actions workflow that runs markdown style verification on pull requests, following a local-first approach where CI acts as a safety net rather than a fixing mechanism. The implementation will optimize the markdown configuration, add a lightweight CI job, and update documentation.

## Technical Context

**Language/Version**: YAML (GitHub Actions), Bash (scripts)
**Primary Dependencies**: markdownlint-cli2, Node.js (via actions/setup-node)
**Storage**: N/A
**Testing**: Manual PR testing with markdown changes
**Target Platform**: GitHub Actions (ubuntu-latest)
**Project Type**: CI/CD configuration
**Performance Goals**: < 2 minutes execution time
**Constraints**: Must not block PRs initially (report-only mode)
**Scale/Scope**: All markdown files in repository (~139 files)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

**Simplicity**:

- Projects: 1 (CI configuration only)
- Using framework directly? Yes (GitHub Actions, markdownlint)
- Single data model? N/A (no data)
- Avoiding patterns? Yes (no unnecessary abstractions)

**Architecture**:

- EVERY feature as library? Using existing script library (scripts/ci/run-markdown-style.sh)
- Libraries listed: run-markdown-style.sh (validation script)
- CLI per library: Script supports standard CLI patterns
- Library docs: Script is self-documenting with clear output

**Testing (NON-NEGOTIABLE)**:

- RED-GREEN-Refactor cycle enforced? Yes (workflow will fail initially)
- Git commits show tests before implementation? Yes
- Order: Contract→Integration→E2E→Unit strictly followed? N/A for CI config
- Real dependencies used? Yes (actual markdownlint execution)
- Integration tests for: PR workflow validation

**Observability**:

- Structured logging included? Yes (GitHub Actions annotations)
- Frontend logs → backend? N/A
- Error context sufficient? Yes (clear error messages)

**Versioning**:

- Version number assigned? N/A (CI configuration)
- BUILD increments on every change? N/A
- Breaking changes handled? Yes (report-only → enforcement transition)

## Project Structure

### Documentation (this feature)

```tree
specs/036-ci-markdown-style/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)

```tree
.github/
└── workflows/
    └── docs-style.yml   # New CI workflow

.markdownlint.json       # Updated configuration

WARP.md                  # Updated documentation
.github/CLAUDE.md        # Updated documentation
```

**Structure Decision**: Simple configuration files at standard locations

## Phase 0: Outline & Research

1. **Extract unknowns from Technical Context** above:
   - Optimal markdownlint rules for the project
   - Transition strategy from report-only to enforcement
   - Integration with existing CI pipeline

2. **Generate and dispatch research agents**:
   - Research: GitHub Actions path filtering best practices
   - Research: markdownlint-cli2 vs markdownlint-cli performance
   - Research: CI artifact upload strategies

3. **Consolidate findings** in `research.md`:
   - Decision: Use markdownlint-cli2 (faster, better ignore support)
   - Decision: Disable MD013 (line length) per issue comment analysis
   - Decision: Start with continue-on-error: true for gradual adoption

**Output**: research.md with all decisions documented

## Phase 1: Design & Contracts

_Prerequisites: research.md complete_

1. **Configuration Design** (.markdownlint.json):
   - Disable MD013 (line length) - aligns with GitHub standards
   - Keep other rules for consistency
   - Document rationale in comments

2. **Workflow Design** (.github/workflows/docs-style.yml):
   - Trigger on pull_request with path filter
   - Use ubuntu-latest runner
   - Steps: checkout → setup-node → run-markdown-style.sh
   - Initially non-blocking with continue-on-error: true
   - Clear error messages directing to local tools

3. **Documentation Updates**:
   - WARP.md: Add docs-style job to CI checks section
   - .github/CLAUDE.md: Document verification-only approach

4. **Create quickstart.md**:
   - How to run markdown checks locally
   - How to fix common violations
   - Understanding CI failure messages

**Output**: quickstart.md, documentation plan

## Phase 2: Task Planning Approach

_This section describes what the /tasks command will do - DO NOT execute during /plan_

**Task Generation Strategy**:

- Configuration update tasks (markdownlint.json)
- Workflow creation task (docs-style.yml)
- Documentation update tasks (WARP.md, CLAUDE.md)
- Testing tasks (create test PR)
- Transition planning task

**Ordering Strategy**:

- Configuration first (foundation)
- Workflow creation (core implementation)
- Documentation (user guidance)
- Testing (validation)
- Transition plan (future state)

**Estimated Output**: 8-10 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Pre-PR Validation

_Quality gates before submitting pull request_

**SDD Document Validation**:

- [ ] Run `scripts/sdd/validate-sdd-docs.sh` to check structure
- [ ] No unresolved [NEEDS CLARIFICATION] markers
- [ ] No placeholder values

**Workflow Validation**:

- [ ] YAML syntax valid
- [ ] Path filters correct
- [ ] Actions versions specified

## Phase 3+: Future Implementation

_These phases are beyond the scope of the /plan command_

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks following plan)
**Phase 5**: Validation (test with sample PR)
**Phase 6**: Pre-PR validation (run all checks above)

## Complexity Tracking

_No violations - solution follows constitutional principles_

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
- [x] Complexity deviations documented

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Fllow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
