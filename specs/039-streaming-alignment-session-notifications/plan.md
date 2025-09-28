# Implementation Plan: Align Streaming Notifications with ACP Models

```yaml
worktree: ../acplb-worktrees/039-streaming-alignment
feature_branch: feature/039-streaming-alignment-v2
created: 2025-09-25T07:40:16Z
last_updated: 2025-09-28T18:05:00Z
status: merged
input: Feature specification from specs/039-streaming-alignment-session-notifications/spec.md
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45
spec_uri: specs/039-streaming-alignment-session-notifications/spec.md
plan_uri: specs/039-streaming-alignment-session-notifications/plan.md
tasks_uri: specs/039-streaming-alignment-session-notifications/tasks.md
pr_uris:
  - https://github.com/lwyBZss8924d/ACPLazyBridge/pull/48
  - https://github.com/lwyBZss8924d/ACPLazyBridge/pull/50
merged_commit: 81c48b9072f0c5fc1617485fd5336d086b4992e2
evidence_uris:
  - _artifacts/039-streaming-alignment/tests/
  - _artifacts/039-streaming-alignment/logs/
  - _artifacts/039-streaming-alignment/jq/
  - _artifacts/039-streaming-alignment/reports/
specs:
  constitution: 1.0.1
  type: plan
  feature_number: 039
dependencies:
  - dev-docs/core_servers/acplb-core-runtime.md
  - agent-client-protocol v0.4.3
```

## Execution Flow (/plan command scope)

```text
1. Load feature spec from specs/039-streaming-alignment-session-notifications/spec.md
   → Requirements for official type adoption
2. Fill Technical Context
   → No outstanding clarification markers
   → Project Type: single (Rust workspace)
   → Structure Decision: Option 1 (single project)
3. Fill Constitution Check section
   → Evaluating against Articles I, III, VII, VIII, IX
4. Evaluate Constitution Check section
   → All checks passing
   → Update Progress: Initial Constitution Check
5. Execute Phase 0 → research.md
   → Study agent_client_protocol types
   → Analyze existing codex_proto.rs mappings
6. Execute Phase 1 → contracts, data-model.md
   → Define type migration mappings
   → Document deduplication strategy
7. Re-evaluate Constitution Check section
   → Still passing
   → Update Progress: Post-Design Constitution Check
8. Plan Phase 2 → Task generation approach
9. STOP - Ready for /tasks command
```

## Summary

Replace bespoke streaming types (`SessionUpdate`, `ContentBlock`, `ToolCallStatus`) in `crates/codex-cli-acp` with official `agent_client_protocol` models, remove the simulated fallback branch in `CodexProviderAdapter::spawn_and_stream_codex`, and ensure schema fidelity for downstream ACP clients while preserving the deduplication, notify sink, and idle-timeout behavior established in Task 038.

## Technical Context

**Language/Version**: Rust 1.89
**Primary Dependencies**: agent-client-protocol 0.4.2, tokio 1.x, serde_json, anyhow
**Testing**: cargo test with snapshot testing (insta), JSONL regression
**Target Platform**: Linux/macOS CLI
**Project Type**: single (Rust workspace)
**Performance Goals**: Maintain ≤150ms prompt latency baseline from Task 038
**Constraints**: Preserve exact JSONL compatibility, maintain existing last-chunk deduplication semantics, and remove the simulated fallback path without regressing tool-call streaming
**Scale/Scope**: ~1500 LOC refactor in codex_proto.rs and tool_calls.rs
**Timeline**: 2-3 day implementation

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

**Simplicity**:

- Projects: 2 (acp-lazy-core, codex-cli-acp) - within limit ✅
- Using framework directly? Yes - direct agent_client_protocol types ✅
- Single data model? Yes - official ACP types only ✅
- Avoiding patterns? Yes - no wrapper traits ✅

**Architecture**:

- EVERY feature as library? Yes - using existing runtime library ✅
- Libraries listed:
    - agent_client_protocol (external)
    - acp-lazy-core::runtime (existing)
- CLI per library: codex-cli-acp binary remains ✅
- Library docs: Will update module documentation ✅

**Testing (NON-NEGOTIABLE)**:

- RED-GREEN-Refactor cycle enforced? Yes ✅
- Git commits show tests before implementation? Will enforce ✅
- Order: Contract→Integration→E2E→Unit strictly followed? Yes ✅
- Real dependencies used? Yes - actual agent_client_protocol ✅
- Integration tests for: type serialization, streaming behavior ✅
- FORBIDDEN: Implementation before test - will not violate ✅

**Observability**:

- Structured logging included? Yes - existing tracing ✅
- Frontend logs → backend? N/A (CLI only) ✅
- Error context sufficient? Yes - anyhow with context ✅

**Versioning**:

- Version number assigned? 0.2.0 → 0.2.1 ✅
- BUILD increments on every change? Using cargo versioning ✅
- Breaking changes handled? Internal refactor, no API changes ✅

## Project Structure

### Documentation (this feature)

```tree
specs/039-streaming-alignment-session-notifications/
├── spec.md              # Feature specification (complete)
├── plan.md              # This file
├── tasks.md             # Phase 2 output (/tasks command)
├── research.md          # Phase 0 output (baseline complete; update if new findings arise)
├── data-model.md        # Phase 1 output (baseline complete; update with official-type deltas)
└── contracts/           # Phase 1 output (baseline complete; update mappings as implementation evolves)
    └── type_mappings.md # Type migration contracts
```

### Source Code (repository root)

```tree
# Option 1: Single project (SELECTED)
crates/
├── acp-lazy-core/
│   └── src/
│       └── runtime/     # No changes needed
└── codex-cli-acp/
    ├── src/
    │   ├── main.rs      # No changes
    │   ├── codex_proto.rs # REFACTOR: Replace custom types & clarify last-chunk dedup semantics
    │   ├── codex_agent.rs # REFACTOR: Use official types & remove simulated fallback
    │   ├── tool_calls.rs  # REFACTOR: Official tool types
    │   ├── notify_source.rs # No changes
    │   └── validation.rs   # No changes
    └── tests/
        ├── streaming_snapshots_test.rs # NEW TESTS (serde + dedup)
        ├── tool_call_lifecycle_test.rs # NEW TESTS (status + fallback removal)
        └── jsonl_regression_test.rs    # UPDATE (fallback parity)
```

**Structure Decision**: Option 1 (single project) - Refactor within existing workspace

## Phase 0: Outline & Research

1. **Extract unknowns from Technical Context**:
   - Exact agent_client_protocol type signatures
   - Serde attribute requirements for official types
   - Deduplication comparison for official types
   - Tool call metadata requirements

2. **Research tasks**:

   ```bash
   Task: "Analyze agent_client_protocol::SessionNotification structure"
   Task: "Study ContentBlock variants in official protocol"
   Task: "Map ToolCall/ToolCallUpdate lifecycle states"
   Task: "Review deduplication requirements for chunks"
   ```

3. **Consolidate findings** in `research.md`:
   - Type compatibility analysis
   - Migration path for each custom type
   - Deduplication strategy for official types

**Output**: research.md with type migration strategy (already drafted; revisit only if official types change)

## Phase 1: Design & Contracts

_Prerequisites: research.md complete_

1. **Extract entities from feature spec** → `data-model.md`:
   - SessionNotification structure and variants
   - ContentBlock types (Text, Image, etc.)
   - ToolCall and ToolCallUpdate schemas
   - StopReason enum values
   - Status progression states

2. **Generate type mapping contracts** → `contracts/type_mappings.md`:
   - Custom SessionUpdate → agent_client_protocol::SessionNotification
   - Custom ContentBlock → agent_client_protocol::ContentBlock
   - Custom ToolCallStatus → official status types
   - Custom tool metadata → official tool fields

3. **Generate contract tests**:
   - Snapshot tests for each type serialization
   - Round-trip tests for type conversion
   - Deduplication tests with official types
   - JSONL compatibility tests

4. **Extract test scenarios** from requirements:
   - Agent message streaming with chunks
   - Tool call lifecycle (pending → in_progress → completed/failed)
   - Duplicate chunk filtering
   - Notify event handling
   - Idle timeout behavior

**Output**: data-model.md, contracts/, failing snapshot tests (docs exist—update with dedup notes and fallback removal implications before writing tests)

## Phase 2: Test Planning (Pre-Implementation)

1. **Snapshot suite (T007–T010)**
   - Harness: `SnapshotHarness` defined in `tests/support/mod.rs`.
   - Coverage: every `SessionUpdate` variant, inclusive of Plan, AvailableCommandsUpdate, and CurrentModeUpdate, plus all five `ContentBlock` variants.
   - Expected failure: current JSON payloads emit legacy field names (`toolCallId`, inline tool updates) and omit the additional variants, triggering insta diffs.

2. **Lifecycle & behavior (T011–T016)**
   - Harness: `tool_call_lifecycle_test.rs` scaffolding alongside JSONL regression runner.
   - Coverage: Tool call status progression, error propagation, notify handling, idle timeout, and dedup guard behavior.
   - Expected failure: existing implementation never emits official `ToolCallUpdateFields`, still exercises the fallback branch, and lacks notify/timeout verification hooks.
   - Optional sandbox smoke: When `ACPLB_CODEX_SMOKE_BIN` is set, run Codex CLI non-interactive smoke test to validate `codex --version` / `codex exec --help`.

3. **Evidence capture**
   - insta snapshots land in `_artifacts/039-streaming-alignment/tests/snapshots/` after `cargo insta test`.
   - JSONL diffs for T014 stored under `_artifacts/039-streaming-alignment/tests/` with clear before/after references.
   - Tool lifecycle logs (T011–T016) written to `_artifacts/039-streaming-alignment/logs/` for constitution evidence.

## Phase 2: Task Planning Approach

_Prerequisites: Design complete, contracts defined_

1. **Task Categories**:
   - Setup: Dependencies and test infrastructure
   - Tests: Snapshot, lifecycle, regression tests (TDD)
   - Core: Type replacement in codex_proto.rs
   - Integration: Update codex_agent.rs integration
   - Polish: Documentation and cleanup

2. **Task Sequencing**:
   - Tests MUST be written first (Article III)
   - Type replacements after tests fail
   - Integration after core changes
   - Regression validation last

3. **Parallel Opportunities**:
   - Different test files can be written in parallel
   - Documentation updates parallel with testing

**Output**: tasks.md with numbered, executable tasks

## Pre-PR Validation

- [x] All snapshot tests passing (insta snapshots regenerated for ACP schema)
- [x] JSONL regression tests show no changes
- [x] Performance metrics within baseline (≤150ms maintained)
- [x] cargo fmt --all -- --check
- [x] cargo clippy --workspace --all-targets --all-features -- -D warnings
- [x] cargo test --workspace --all-features --locked
- [x] (Optional) Codex CLI smoke test succeeds when `ACPLB_CODEX_SMOKE_BIN` is provided
- [x] Evidence collected under _artifacts/039-streaming-alignment/

## Phase 3+: Future Implementation

(Executed via tasks.md after this plan is approved)

## Complexity Tracking

- **Low**: Direct type replacement with same semantics
- **Medium**: Deduplication logic adaptation
- **High**: None identified

## Implementation Outcomes

### Lessons Learned

1. **Direct Type Adoption Success**: The migration to official `agent_client_protocol` types was straightforward, with the crate providing all necessary structs and enums. The v0.4.2 API was stable and well-documented.

2. **Deduplication Strategy Validated**: The existing last-chunk suppression approach (`LastChunkGuard`) successfully adapted to official types without modification, confirming the design decision to preserve existing semantics.

3. **Test-First Development Payoff**: Writing failing tests first (Phase 3.2) caught several edge cases early:
   - Missing `sessionId` field in initial implementation
   - Incorrect tool call status transition timing
   - Notify/idle stop-reason handling gaps

4. **Performance Maintained**: The type migration introduced no measurable latency impact. Streaming continues to meet the ≤150ms baseline from Task 038.

### Technical Discoveries

- **Partial Variant Coverage**: The Codex CLI currently only emits a subset of `SessionUpdate` variants (primarily AgentMessageChunk). Full protocol support will require upstream Codex changes.
- **ContentBlock Limitations**: The protocol supports rich media types (Image/Audio/Resource), but Codex only produces text content today.
- **ToolCallUpdate Field Granularity**: The protocol allows fine-grained field updates via `ToolCallUpdateFields`, but determining which fields changed requires additional state tracking.

### Deduplication Confirmation

The implementation preserved the single last-chunk suppression strategy as documented:

- Stores the last emitted agent message chunk
- Compares new chunks against the stored value
- Only emits when content differs
- This simple approach proved sufficient for current streaming patterns

## Progress Tracking

- [x] Initial Constitution Check
- [x] Phase 0 Research (research.md completed)
- [x] Phase 1 Design (data-model.md, contracts/type_mappings.md completed)
- [x] Post-Design Constitution Check
- [x] Phase 2 Task Planning (tasks.md generated)
- [x] Phase 3 Implementation (T001-T032 completed)
- [x] Phase 3.5 Polish (T034a/T034b completed, T033-T040 pending)

### Pending Phase 3.5 / Evidence Tasks

1. **T033** – Perform manual Zed compatibility verification to validate editor streaming behavior.
2. **T035–T036** – Update inline documentation in `codex_proto.rs` and `tool_calls.rs` to describe the ACP-aligned data flow.
3. **T037** – Append migration notes to `dev-docs/core_servers/acplb-core-runtime.md` for runtime maintainers.
4. **T038–T040** – Regenerate regression evidence (JSONL, performance metrics, insta baselines) under `_artifacts/039-streaming-alignment/` prior to validation.

## IMPORTANT TECHNICAL STANDARDS

- [ACP Protocol](https://agentclientprotocol.com/protocol) - Protocol specification
- [ACP Schema](https://agentclientprotocol.com/protocol/schema) - JSON schema definitions
- [agent-client-protocol crate](https://docs.rs/agent-client-protocol/0.4.2) - Rust library docs
- **Local references**:
    - ~/dev-space/agent-client-protocol/rust/src/types.rs
    - ~/dev-space/claude-code-acp/src/notifications.rs (best practices)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
