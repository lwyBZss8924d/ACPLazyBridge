# Implementation Plan: Adopt Official ACP Runtime

```yaml
worktree: ../acplb-worktrees/038-adopt-acp-runtime
feature_branch: feature/038-adopt-acp-runtime
created: 2025-09-23T07:23:14Z
last_updated: 2025-09-24T07:33:37Z
status: ready_for_review
input: Feature specification from specs/038-adopt-acp-runtime/spec.md
spec_uri: specs/038-adopt-acp-runtime/spec.md
plan_uri: specs/038-adopt-acp-runtime/plan.md
tasks_uri: specs/038-adopt-acp-runtime/tasks.md
evidence_uris: _artifacts/038-adopt-acp-runtime/
specs:
    constitution: 1.0.1
    type: plan
    feature_number: 038
```

## Execution Flow (/plan command scope)

```text
1. Load feature spec from Input path
   → Found at specs/038-adopt-acp-runtime/spec.md
2. Fill Technical Context (scan for outstanding clarifications)
   → No outstanding clarification markers found
   → Detect Project Type: single (Rust workspace)
   → Set Structure Decision: Option 1 (single project)
3. Fill the Constitution Check section
   → Evaluating against Articles I, III, VII, VIII, IX
4. Evaluate Constitution Check section
   → All checks passing
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → Research AgentSideConnection API
   → Study LocalSet execution model
6. Execute Phase 1 → contracts, data-model.md
   → Define runtime contracts
   → Model session entities
7. Re-evaluate Constitution Check section
   → Still passing
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach
9. STOP - Ready for /tasks command
```

## Summary

Deliver Roadmap FR-0101 by replacing the handcrafted JSON-RPC loop with the official ACP runtime, while satisfying FR-0104/FR-0105 through unchanged JSONL transcripts, notify semantics, and documented evidence. The outcome provides a reusable foundation so future adapters (FR-0201) share the same runtime surface area.

## Technical Context

**Language/Version**: Rust 1.89
**Primary Dependencies**: agent-client-protocol 0.4.2, tokio 1.x with LocalSet, anyhow, serde_json
**Storage**: In-memory session store (HashMap)
**Testing**: cargo test with JSONL replay validation
**Target Platform**: Linux/macOS CLI
**Project Type**: single (Rust workspace)
**Performance Goals**: ≤150 ms prompt latency in local testing (per Roadmap FR-0105) with safe handling for single-digit concurrent sessions typical of CLI workflows
**Constraints**: !Send futures require LocalSet, maintain JSONL compatibility
**Telemetry**: Runtime emits structured evidence when `ACPLB_EVIDENCE_PATH` is configured (consumed by SDD artefacts)
**Scale/Scope**: ~2000 LOC refactor, 3-5 day implementation

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

**Simplicity**:

- Projects: 2 (acp-lazy-core, codex-cli-acp) - within limit ✅
- Using framework directly? Yes - AgentSideConnection without wrappers ✅
- Single data model? Yes - direct ACP types ✅
- Avoiding patterns? Yes - no Repository/UoW ✅

**Architecture**:

- EVERY feature as library? Yes - runtime module in acp-lazy-core ✅
- Libraries listed:
    - acp-lazy-core::runtime (new module)
    - agent-client-protocol (external dependency)
- CLI per library: codex-cli-acp binary remains ✅
- Library docs: Will add module documentation ✅

**Testing (NON-NEGOTIABLE)**:

- RED-GREEN-Refactor cycle enforced? Yes ✅
- Git commits show tests before implementation? Will enforce ✅
- Order: Contract→Integration→E2E→Unit strictly followed? Yes ✅
- Real dependencies used? Yes - actual agent-client-protocol ✅
- Integration tests for: new runtime module, Agent trait impl ✅
- FORBIDDEN: Implementation before test - will not violate ✅

**Observability**:

- Structured logging included? Yes - tracing to stderr ✅
- Frontend logs → backend? N/A (CLI only) ✅
- Error context sufficient? Yes - anyhow with context ✅

**Versioning**:

- Version number assigned? 0.1.0 → 0.2.0 ✅
- BUILD increments on every change? Using cargo versioning ✅
- Breaking changes handled? Internal refactor, no public API changes ✅

## Project Structure

### Documentation (this feature)

```tree
specs/038-adopt-acp-runtime/
├── plan.md              # This file
├── spec.md              # Feature specification (complete)
├── research.md          # Phase 0 output (to create)
├── data-model.md        # Phase 1 output (to create)
├── contracts/           # Phase 1 output (to create)
└── tasks.md             # Phase 2 output (/tasks command)
```

### Source Code (repository root)

```tree
# Option 1: Single project (SELECTED)
crates/
├── acp-lazy-core/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── runtime/        # NEW MODULE
│   │   │   ├── mod.rs
│   │   │   ├── server.rs
│   │   │   ├── session.rs
│   │   │   └── adapter.rs
│   │   ├── permissions.rs
│   │   ├── protocol.rs
│   │   └── transport.rs
│   └── tests/
│       └── runtime_test.rs # NEW TEST
└── codex-cli-acp/
    ├── src/
    │   ├── main.rs         # REFACTOR
    │   └── codex_agent.rs  # NEW FILE
    └── tests/
        └── acp_integration_test.rs # NEW TEST
```

**Structure Decision**: Option 1 (single project) - Rust workspace structure

## Phase 0: Outline & Research

1. **Extract unknowns from Technical Context**:
   - AgentSideConnection API patterns
   - LocalSet execution requirements
   - Session lifecycle management
   - Notify integration patterns

2. **Generate and dispatch research agents**:

   ```bash
   Task: "Research AgentSideConnection usage in examples/agent.rs"
   Task: "Study LocalSet and !Send future handling"
   Task: "Analyze notify sink integration patterns"
   ```

3. **Consolidate findings** in `research.md`:
   - Decision: Use AgentSideConnection with Agent trait
   - Rationale: Official implementation, maintained upstream
   - Alternatives considered: Custom protocol layer (rejected)

**Output**: research.md with implementation patterns

## Phase 1: Design & Contracts

_Prerequisites: research.md complete_

1. **Extract entities from feature spec** → `data-model.md`:
   - Runtime lifecycle responsibilities (session negotiation, notifications)
   - Session state persistence and permission metadata
   - Agent trait coverage (initialize, authenticate, new_session, load_session, prompt, set_session_mode, cancel, ext_method, ext_notification)
   - External signals (notify sources, evidence capture)

2. **Generate API contracts** from functional requirements:
   - Document behavior for every Agent trait method (including unsupported flows)
   - Capture runtime lifecycle expectations: start, stop, cancellation, evidence hooks
   - Output to `contracts/runtime_api.md` with explicit links to Roadmap FR-0101/FR-0104/FR-0105

3. **Generate contract tests** from contracts:
   - Write failing tests for each Agent trait method, including default error paths
   - Cover session lifecycle and notification edge cases
   - Ensure JSONL regression replay harness asserts equivalence with baseline outputs

4. **Extract test scenarios** from user stories and Roadmap FR-0104/FR-0105:
   - JSONL replay scenarios
   - Notify event handling
   - Timeout behavior
   - Cancellation flow

**Output**: data-model.md, contracts/, failing tests

## Phase 2: Task Planning Approach

_This section describes what the /tasks command will do - DO NOT execute during /plan_

**Task Generation Strategy**:

- Load templates/tasks-template.md as base
- Generate tasks from Phase 1 design docs
- Each Agent method → test + implementation task
- Session lifecycle → integration test tasks
- JSONL scenarios → regression test tasks

**Ordering Strategy**:

- TDD order: Tests before implementation
- Dependency order: Core runtime before adapter
- Mark [P] for parallel execution where possible

**Estimated Output**: 20-25 numbered, ordered tasks in tasks.md

## Pre-PR Validation

_Quality gates before submitting pull request_

**SDD Document Validation**:

- [x] Run `scripts/sdd/validate-sdd-docs.sh` (latest log: `_artifacts/038-adopt-acp-runtime/tests/sdd_validate_20250924T063333Z.log`)
- [x] No unresolved clarifications remain
- [x] No placeholder values

**Code Quality Gates**:

- [x] All tests pass (`cargo test --workspace`)
- [x] Format check (`cargo fmt --all -- --check`)
- [x] Lint check (`cargo clippy --workspace --all-targets`)
- [x] JSONL regression tests pass

## Phase 3+: Future Implementation

_These phases are beyond the scope of the /plan command_

**Phase 3**: Task execution (/tasks creates tasks.md)
**Phase 4**: Implementation (execute tasks.md)
**Phase 5**: Validation (regression tests, performance)
**Phase 6**: Pre-PR validation

## Complexity Tracking

_No violations - all Constitution checks passing_

## Progress Tracking

**Phase Status**:

- [x] Phase 0: Research complete
- [x] Phase 1: Design complete
- [x] Phase 2: Task planning complete
- [x] Phase 3: Implementation complete (runtime integration + telemetry)
- [ ] Phase 4: Implementation complete _(deferred; follow-up milestone once Phase 4 scope is scheduled)_
- [ ] Phase 5: Validation passed _(deferred until Phase 4 deliverables are merged)_

**Note:** Deferred work is tracked under Issue #45 (streaming alignment) and Issue #46 (protocol cleanup); see the corresponding drafts in dev-docs/_issues_drafts/open/.

**Gate Status**:

- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All outstanding clarifications resolved
- [x] Complexity deviations documented (none)

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
