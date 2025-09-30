# [Draft] Implementation Plan: Complete Codex Protocol Alignment for MVP (Issue #50 | Task #040)

```yaml
worktree: ../acplb-worktrees/040-codex-protocol-alignment-mvp
feature_branch: feature/040-codex-protocol-alignment-mvp
created: 2025-09-29T21:24:56Z
last_updated: 2025-09-29T21:24:56Z
status: draft
input: Feature specification from `specs/040-codex-protocol-alignment-mvp/spec.md`
spec_uri: specs/drafts-040-codex-protocol-alignment-mvp/spec.md
plan_uri: specs/drafts-040-codex-protocol-alignment-mvp/plan.md
tasks_uri: specs/drafts-040-codex-protocol-alignment-mvp/tasks.md
evidence_uris:
  - _artifacts/040-codex-protocol-alignment-mvp/tests/
  - _artifacts/040-codex-protocol-alignment-mvp/logs/
  - _artifacts/040-codex-protocol-alignment-mvp/reports/
specs:
  constitution: 1.0.1
  type: plan
  feature_number: 040
```

## Summary

We will extend the Codex adapter inside `crates/codex-cli-acp` to translate the full Codex submission and event surface into official ACP payloads. Work focuses on three pillars:

1. **Submission context** – Introduce a `submission_handler` module that turns Codex `Submission` envelopes into ACP session metadata, capturing cwd, sandbox, approval mode, model, and XML-tagged annotations.
2. **Streaming fidelity** – Expand `codex_proto.rs` to handle every `EventMsg` variant with strongly typed builders and table-driven mappings for tool lifecycle, plan updates, approvals, slash commands, notify events, and experimental JSON stream fallbacks.
3. **Tool classification** – Enhance `tool_calls.rs` (and related helpers) to map Codex tool definitions, apply-patch envelopes, and MCP calls into ACP `ToolKind`, location, and raw payload structures; integrate approval status transitions and output truncation.

Supporting work includes regression fixtures for proto and experimental JSON streams, documentation updates across milestone artefacts, and superseding Issue #46 with the broader Issue #50 scope.

## Technical Context

- **Language/Version**: Rust 1.80 (workspace pinned via `rust-toolchain.toml`)
- **Primary Dependencies**: `agent-client-protocol` v0.4.3, `serde_json`, `tokio`, `tracing`
- **Storage**: In-memory session state only; no persistent storage involved
- **Testing**: `cargo test`, JSONL regression harness (`crates/codex-cli-acp/tests`), integration scripts under `_artifacts/`
- **Target Platform**: macOS/Linux developer environments; CI on GitHub Actions
- **Project Type**: Multi-crate Rust workspace (library + binaries)
- **Performance Goals**: Additional mapping layer must add <50 ms processing overhead per event; maintain existing streaming throughput
- **Constraints**: Must maintain compatibility with both proto and experimental JSON Codex outputs; sandbox/approval mapping limited by Codex CLI options
- **Scale/Scope**: Single-agent MVP scope (Codex adapter) but must accommodate concurrent tool calls and multi-message sessions

## Constitution Check (Pre-Design)

**Simplicity (Article VII)**

- Projects touched: `crates/codex-cli-acp` (library + bins) – remains within 3-project limit
- No new abstraction layers; helper module `submission_handler.rs` just encapsulates parsing for clarity
- Reuse official `agent-client-protocol` structs directly; avoid wrapper types

**Integration-First (Article IX)**

- Contracts defined via ACP schema and Codex event definitions prior to coding
- Plan includes Zed smoke test and JSONL replay validation with real Codex CLI

**Test-First (Article III)**

- Add failing regression cases for each new `EventMsg` mapping before implementation
- Sequence ensures RED → GREEN → REFACTOR across tool lifecycle, plan updates, approvals, and slash commands

No Constitution violations anticipated; Complexity Tracking not required at this stage.

## Architecture & Module Updates

### 1. Submission Context Layer

- Add `crates/codex-cli-acp/src/submission_handler.rs`
    - Functions: `parse_submission`, `extract_content_blocks`, `derive_session_context`
    - Output struct `SessionContext` capturing cwd, sandbox, approval policy, model, reasoning flags, final_output_json_schema, optional notify configuration
    - Translate XML wrappers to structured annotations (user/environment)
- Update `main.rs` and `codex_agent.rs` to call the new handler when enqueuing prompts, storing metadata for downstream updates

### 2. Streaming Pipeline Enhancements

- Refactor `codex_proto.rs`
    - Introduce enums/structs to normalize event mapping (e.g., `PlanUpdateMapper`, `ToolLifecycleMapper`)
    - Cover `EventMsg::{PlanUpdate, TaskStarted, TaskProgress, TaskCompleted, ToolCallBegin, ToolCallUpdate, ToolCallComplete, McpToolCallBegin, McpToolCallEnd, ApplyPatchResult, ExecApprovalRequest, ToolApprovalResponse, SlashCommandList, Notify, TurnCompleted}`
    - Populate ACP `SessionUpdate::AgentMessageChunk`, `AgentThoughtChunk`, `Plan`, `ToolCall`, `ToolCallUpdate`, `AvailableCommandsUpdate`, `CurrentModeUpdate`, `Usage` (token counts) as applicable
    - Ensure deduplication logic remains for agent deltas
- Implement dual ingestion paths:
    - Default: existing proto JSON lines
    - Optional: `ConversationEvent` from `--experimental-json`; share mapping logic via trait or adapter pattern

### 3. Tool Call Classification & Approvals

- Extend `tool_calls.rs`
    - Accept new `ToolEnvelope` enum describing function/local shell/web search/apply_patch/MCP/plan tool
    - Implement location extraction (file paths, ranges) for apply_patch diffs and shell commands
    - Map approvals to ACP status transitions with metadata (reason, user, expiration)
    - Add truncation utility for raw payload previews with explicit `truncated` flags
- Update `notify_source.rs` or related components if additional metadata is required for notify events

### 4. Documentation & Issue Alignment

- Create `dev-docs/_issues_drafts/open/#50-codex-protocol-alignment-mvp.md` with refreshed scope
- Update requirement artefacts listed in the spec to reference Issue #50 and note Issue #46 supersession
- Adjust milestone tables and migration blueprint for traceability

## Phase 0: Research & Unknowns

Deliverables: `research.md`

1. **Codex Event Surface Audit**
   - Inventory `EventMsg` variants from `codex-rs/protocol/src/protocol.rs`
   - Document mapping target for each ACP `SessionUpdate`
2. **ACP Schema Review**
   - Confirm latest `SessionUpdate`, `ToolCall`, `PlanEntry`, `AvailableCommand` definitions in `agent-client-protocol` v0.4.3
   - Note required vs optional fields
3. **Tool Classification Rules**
   - Gather heuristics from `tool_apply_patch.rs`, `openai_tools.rs`, and `claude-code-acp` best practices
4. **Experimental JSON Stream Format**
   - Extract schema for `ConversationEvent` outputs and plan bridging strategy
5. **Documentation Scope**
   - Determine precise sections in each requirement doc that reference Issue #46 to be updated

## Phase 1: Design & Contracts

Deliverables: `data-model.md`, `contracts/`, `quickstart.md`

1. **Session Context Data Model**
   - Define `SessionContext` struct fields and validation rules (cwd absolute, sandbox mapping, etc.)
   - Document transformation of XML annotations into ACP metadata
2. **Event Mapping Contracts**
   - Create mapping table (EventMsg → SessionUpdate) with required fields and error handling expectations
   - Specify tool lifecycle state machine (pending → in_progress → completed/failed) and MCP naming rules
3. **Testing Contracts**
   - Draft JSONL regression scenarios for each major event grouping (plan, tool, approval, MCP, slash command, experimental JSON)
   - Outline schema validation steps using `agent-client-protocol` serialization tests
4. **Quickstart & Evidence Plan**
   - Document manual steps to run Codex CLI in proto and experimental modes with notify forwarder integration
   - Include Zed smoke test procedure referencing `_artifacts/040-codex-protocol-alignment-mvp/tests/`
5. **Doc Update Outline**
   - List sections to edit in each requirements file and the milestone index; prepare snippet templates for Issue #46 supersession notice

Re-run Constitution Check after completing design docs to ensure no violations introduced; if new complexities appear (e.g., additional helper crates), log in Complexity Tracking.

## Phase 2: Task Planning Preview (for /tasks)

- Generate tasks that enforce TDD order: regression fixtures (RED) before implementation
- Separate file-scoped work to maximize parallelism: `submission_handler.rs`, `codex_proto.rs`, `tool_calls.rs`, doc updates
- Include dedicated tasks for documentation consistency scripts and evidence capture

## Testing & Validation Strategy

- **Unit Tests**: Cover submission parsing, content block conversion, tool classification, approval transitions
- **Integration/Regression Tests**: JSONL scenarios for proto pipeline; new tests for experimental JSON ingestion (gated feature flag)
- **Schema Validation**: Use `serde_json::from_value` with ACP types and optional JSON schema validation via upstream crate
- **Manual Smoke Tests**: Zed IDE turn with plan + tool + approval; CLI-run verifying stop reasons and notify behavior
- **Automation**: Add entries to `_artifacts/040-codex-protocol-alignment-mvp/tests/` capturing command output, plus logs for doc linting scripts

## Pre-PR Validation Gates

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features --locked`
- `ast-grep scan -c sgconfig.yml`
- `scripts/sdd/validate-sdd-docs.sh`
- `scripts/sdd/validate-metadata.sh`
- `scripts/sdd/check-sdd-consistency.sh`

## Progress Tracking

- [ ] Phase 0: Research complete
- [ ] Phase 1: Design complete
- [ ] Phase 2: Task planning complete
- [ ] Phase 3: Tasks generated (/tasks output)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

## Complexity Tracking

_No deviations currently identified. Maintain direct use of ACP models and existing crate boundaries._

## Open Questions / Assumptions

- Confirm whether Codex experimental JSON mode will remain behind feature flag for MVP or enabled automatically when available.
- Validate maximum payload sizes for raw tool inputs/outputs to determine truncation thresholds.
- Determine if additional evidence scripts are required for MCP server logs (coordinate with `dev-docs/references/acp.md`).

Resolve these during Phase 0 research.

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
