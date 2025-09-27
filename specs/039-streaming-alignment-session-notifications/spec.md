# Feature Specification: Align Streaming Notifications with ACP Models

```yaml
worktree: ../acplb-worktrees/039-streaming-alignment
feature_branch: feature/039-streaming-alignment-v2
created: 2025-09-25T07:40:16Z
last_updated: 2025-09-27T21:00:00Z
status: ready_for_pr
input: User description from GitHub issue #45
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45
spec_uri: specs/039-streaming-alignment-session-notifications/spec.md
plan_uri: specs/039-streaming-alignment-session-notifications/plan.md
tasks_uri: specs/039-streaming-alignment-session-notifications/tasks.md
evidence_uris:
  - _artifacts/039-streaming-alignment/tests/
  - _artifacts/039-streaming-alignment/logs/
  - _artifacts/039-streaming-alignment/jq/
  - _artifacts/039-streaming-alignment/reports/
specs:
  constitution: 1.0.1
  type: spec
  feature_number: 039
dependencies:
  - specs/038-adopt-acp-runtime/spec.md
  - dev-docs/architecture/acplb-architecture.md
  - dev-docs/core_servers/acplb-core-runtime.md
  - agent-client-protocol v0.4.2
```

## Summary

Replace bespoke `SessionUpdate`, `ContentBlock`, and tool-call structs in `crates/codex-cli-acp` with the official `agent_client_protocol` models. Route all session notifications through `AgentSideConnection::session_notification`, remove the temporary simulation fallback path in `CodexProviderAdapter::spawn_and_stream_codex`, and ensure compatibility with downstream ACP clients while maintaining the behavior parity established in Task 038.

## Problem Statement

Following the runtime adoption in Task 038 (PR #47), the Codex adapter still uses custom streaming types (`SessionUpdate`, `ContentBlock`, `ToolCallStatus`) that duplicate the official ACP protocol models, and it retains a simulated fallback branch inside `CodexProviderAdapter::spawn_and_stream_codex`. This creates:

- **Schema drift risk**: Custom types may diverge from official protocol updates
- **Maintenance overhead**: Duplicate serde definitions and mapping logic
- **Integration friction**: Downstream clients expect exact ACP schema compliance and direct Codex streaming without fallback simulation
- **Testing complexity**: Need to verify both custom and official type serialization

## Motivation

- **Protocol Fidelity**: Guarantees schema compliance for agent messages, tool calls, plans, and permission prompts
- **Reduced Maintenance**: Reuses upstream serde definitions and validation
- **Future Readiness**: Prepares runtime for composer plugins and richer client UI integration
- **Consistency**: Aligns with the architectural principle of "Protocol Fidelity" established in Task 038

## User Scenarios & Testing

### Primary User Story

As a developer using ACPLazyBridge with an ACP-compliant client, I want streaming notifications to use official protocol types so that my client receives exactly the schema it expects without custom translation layers.

### Acceptance Scenarios

1. **Given** a Codex streaming response with agent messages, **When** converted to ACP notifications, **Then** the output matches `agent_client_protocol::SessionNotification` schema exactly
2. **Given** a tool call with status transitions, **When** streamed through the adapter, **Then** `ToolCall` and `ToolCallUpdate` types follow official schema with correct status progression
3. **Given** duplicate streaming chunks, **When** processed by deduplication logic, **Then** official types are correctly compared and filtered
4. **Given** a notify event during streaming, **When** received by the adapter, **Then** turn completion occurs immediately with official stop reason types
5. **Given** an idle timeout during streaming, **When** no output occurs, **Then** session ends with official `StopReason` enum value
6. **Given** the Codex CLI sandbox exposes the smoke-test binary, **When** tests run with `ACPLB_CODEX_SMOKE_BIN` set, **Then** the non-interactive CLI smoke test succeeds with `codex --version` and `codex exec --help`

## Requirements

### Functional Requirements

- **FR-039-01**: Replace custom `SessionUpdate` wrappers with the official `agent_client_protocol::{SessionNotification, SessionUpdate}` types and variants
- **FR-039-02**: Replace custom `ContentBlock` enum with official `agent_client_protocol::ContentBlock` type
- **FR-039-03**: Replace custom `ToolCallStatus` with official status types from the protocol
- **FR-039-04**: Map all tool calls through official `ToolCall` / `ToolCallUpdate` (with `ToolCallUpdateFields`) structures with proper metadata
- **FR-039-05**: Preserve deduplication logic while operating on official types, maintaining the existing single last-chunk suppression semantics unless extended by design
- **FR-039-06**: Maintain notify sink and idle-timeout behavior established in Task 038
- **FR-039-07**: Ensure backward compatibility with existing JSONL scenarios
- **FR-039-08**: Remove the simulated fallback branch in `CodexProviderAdapter::spawn_and_stream_codex` so that Codex output streams solely through official ACP models
- **FR-039-09**: Provide an optional Codex CLI smoke test gated by `ACPLB_CODEX_SMOKE_BIN` to verify non-interactive commands in the sandbox

### Non-Functional Requirements

- **NFR-039-01**: Streaming latency must not exceed the 150ms baseline established in Task 038
- **NFR-039-02**: Memory usage for session state must remain comparable to current implementation
- **NFR-039-03**: All changes must maintain 100% JSONL protocol compliance

## Scope

### In Scope

- Swap custom enums/structs with upstream types in Codex streaming pipeline
- Update tool-call mapper to emit `ToolCall` and `ToolCallUpdate` with:
    - Correct status transitions (pending → in_progress → completed/failed)
    - Raw IO payloads and location metadata
    - Error categorization using official types
- Ensure chunk de-duplication logic operates on official types while preserving the existing last-chunk suppression semantics (and documenting any future extension hooks)
- Remove the simulated fallback branch in `CodexProviderAdapter::spawn_and_stream_codex` so Codex output is streamed directly through official models
- Extend tests to cover tool-call lifecycle, malformed JSON handling, fallback-removal regressions, and notify-triggered turn completion
- Update integration with `AgentSideConnection::session_notification`
- Document mapping rules and limitations, including deduplication rationale

### Out of Scope

- Introducing composer plugins (future milestone)
- Removing legacy protocol module (tracked by Issue #46)
- Changes to permission mapping or session store
- Modifications to the runtime server itself
- CLI argument changes or environment variable additions

## Acceptance Criteria

- [x] Snapshot tests assert JSON output matches ACP schema for:
    - SessionNotification
    - ContentBlock variants
    - ToolCall and ToolCallUpdate
    - StopReason values
- [x] Notify and idle-timeout flows remain intact with same timing
- [x] Simulated fallback branch in `CodexProviderAdapter::spawn_and_stream_codex` removed and validated via tests or JSONL replay
- [ ] Zed client smoke tests pass with updated adapter (T033 - requires manual verification)
- [x] JSONL regression tests show no behavioral changes
- [x] Evidence collected under `_artifacts/039-streaming-alignment/`
- [ ] Documentation updated in `dev-docs/core_servers/acplb-core-runtime.md`

## Dependencies

- **Task 038 Completion**: PR #47 merged (commit 7ae2628) establishing shared runtime
- **Architecture Baseline**: `dev-docs/architecture/acplb-architecture.md` defines streaming contracts
- **Runtime Design**: `dev-docs/core_servers/acplb-core-runtime.md` specifies notification flow
- **Protocol Version**: `agent-client-protocol` crate v0.4.2 or compatible

## Evidence Expectations

- Snapshot tests comparing official type serialization: `_artifacts/039-streaming-alignment/tests/snapshots/`
- Streaming behavior logs with timing analysis: `_artifacts/039-streaming-alignment/logs/streaming_*.log`
- JSONL regression outputs: `_artifacts/039-streaming-alignment/tests/jsonl_*.jsonl`
- Performance metrics showing latency parity: `_artifacts/039-streaming-alignment/reports/perf.md`
- Tool call lifecycle traces: `_artifacts/039-streaming-alignment/logs/tool_calls_*.log`

## Implementation Notes

### Successfully Implemented (T001-T032, T034a/T034b)

- **Type Migration Complete**: All custom types replaced with official `agent_client_protocol` v0.4.2 types
- **Direct ACP Integration**: CodexStreamManager now emits official SessionNotification/SessionUpdate structs end-to-end
- **Tool Call Fidelity**: Full metadata preservation including titles, status transitions, working directories, and raw I/O
- **Error Handling**: ACP-compliant ToolCallUpdate payloads for failure states with structured diagnostics
- **Fallback Removal**: Simulated branch in CodexProviderAdapter eliminated; all streaming uses official models
- **Test Coverage**: Comprehensive snapshot and lifecycle tests validate ACP JSON layout and behavior
- **Stop Reason Support**: Notify and idle timeout handling with official StopReason enum values

### Known Limitations and Gaps

Per the implementation review in tasks.md (lines 139-175), the following gaps remain vs. the Deep-Research Checklist:

- **Limited SessionUpdate Variants**: Only AgentMessageChunk is implemented; missing UserMessageChunk, AgentThoughtChunk, Plan, AvailableCommandsUpdate, and CurrentModeUpdate support
- **Text-Only ContentBlocks**: No handling for Image, Audio, or Resource block types from future Codex or MCP bridges
- **Incomplete ToolCallUpdate Propagation**: Updates only set status, content, and raw_output; titles, kinds, locations, and raw inputs are not propagated in update fields
- **Narrow Test Scenarios**: Lifecycle tests cover happy-path tool calls but lack assertions for the richer protocol variants

These gaps are tracked for future remediation in Phase 3.6 (see tasks.md). The current implementation fully satisfies the core requirements while maintaining protocol compliance and backward compatibility.

### Immediate Next Steps (Phase 3.5 Focus)

- **T033**: Run the manual Zed client compatibility smoke test to confirm downstream editor behavior before PR handoff.
- **T035–T036**: Refresh module-level documentation in `crates/codex-cli-acp/src/{codex_proto.rs,tool_calls.rs}` to describe the adopted ACP models and stop-reason handling.
- **T037**: Capture the migration summary in `dev-docs/core_servers/acplb-core-runtime.md`, noting the removal of the simulated fallback.
- **T038–T040**: Regenerate regression evidence (JSONL replays, performance notes, insta baselines) under `_artifacts/039-streaming-alignment/` ahead of validation.

## Constitutional Alignment

### Article III - Test-First Development

All type replacements will follow RED→GREEN→REFACTOR:

- Write snapshot tests expecting official types (must fail)
- Replace custom types with official ones
- Refactor for clarity while maintaining tests

### Article VII - Simplicity

- Direct use of `agent_client_protocol` types without wrappers
- No additional abstraction layers
- Preserve existing deduplication logic without over-engineering

### Article IX - Integration-First

- Define type mapping contracts before implementation
- Test against real ACP client expectations
- Validate with actual Codex streaming scenarios

## References

- Issue draft: `dev-docs/_issues_drafts/open/#45-streaming-alignment-session-notifications.md`
- Baseline implementation: `specs/038-adopt-acp-runtime/` (completed Task 038)
- Roadmap: `dev-docs/_requirements/Roadmap.md`
- Migration plan: `dev-docs/_projects/migration-blueprint-project-management-plan.md`
- Milestone tracking: `dev-docs/_requirements/m1-technical-implementation-plan.md`

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
