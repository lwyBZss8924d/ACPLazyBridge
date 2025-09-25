# Align Streaming Notifications with ACP Models

_issues for SDD Task (pending)_

**GitHub Issue**: [#45](dev-docs/_issues_drafts/open/#45-streaming-alignment-session-notifications.md) | <https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45>
**Status**: issue is open (Milestone 0.1.0)
**Dependencies**:

- [#44](dev-docs/_issues_drafts/open/#44-runtime-adoption-core-loop.md) | <https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44>


## Summary

Replace bespoke `SessionUpdate`, `ContentBlock`, and tool-call structs in `crates/codex-cli-acp` with the official `agent_client_protocol` models. Route all session notifications through `AgentSideConnection::session_notification`, ensuring compatibility with downstream ACP clients.

## Motivation

- Guarantees schema fidelity for agent messages, tool calls, plans, and permission prompts.
- Reduces maintenance overhead by reusing upstream serde definitions.
- Prepares the runtime for composer plugins and richer client UI integration.

## Scope

- Swap custom enums/structs with upstream types in Codex streaming pipeline.
- Update tool-call mapper to emit `ToolCall` and `ToolCallUpdate` with correct status transitions, raw IO payloads, and location metadata.
- Ensure chunk de-duplication logic operates on official types without spamming clients.
- Extend tests to cover tool-call lifecycle, malformed JSON handling, and notify-triggered turn completion.
- Document mapping rules and limitations in `dev-docs/core_servers/acplb-core-runtime.md` or related references.

## Out of Scope

- Introducing composer plugins (future milestone).
- Removing legacy protocol module (handled separately).

## Acceptance Criteria

- Streaming output validated via snapshot tests that assert JSON matches ACP schemas.
- Notify and idle-timeout flows remain intact; Zed client smoke tests pass.
- SDD artefacts (spec/plan/tasks) capture the refactor details and test evidence.

## Dependencies

- Runtime adoption issue merged (shared runtime available).
- Architecture baseline: `dev-docs/architecture/acplb-architecture.md`.

## Evidence Expectations

- Snapshot tests stored under `_artifacts/tests/streaming-alignment/` with diffs vs baseline (legacy mirrors optional under `_artifacts/legacy/`).
- Logs verifying dedupe behaviour under `_artifacts/logs/streaming-alignment/`.

## References

- `dev-docs/_requirements/Roadmap.md`
- `dev-docs/core_servers/acplb-core-runtime.md`
- `dev-docs/_project_management/migration-blueprint-project-management-plan.md`


## Deferred follow-up from SDD Task 038 (Phase 4)

The initial runtime adoption (Issue #44 / SDD Task 038) landed the shared runtime but explicitly deferred several streaming/alignment items. Fold the following deliverables into this issue:

- **Adopt official ACP session/update models end-to-end.** Replace the interim `CodexSessionUpdate` structures in `crates/codex-cli-acp/src/codex_proto.rs` / `codex_agent.rs` with `agent_client_protocol::SessionUpdate` and `SessionNotification` so the runtime no longer double-converts streaming payloads.
- **Reconnect real Codex process streaming.** Remove the simulated fallback branch in `CodexProviderAdapter::spawn_and_stream_codex` once the official models are in place, ensuring tool-call deltas, errors, and completion reasons propagate exactly as Codex emits them.
- **Notify/timeout parity hardening.** Exercise idle-timeout and external notify paths using the shared runtime; store evidence under `_artifacts/038-adopt-acp-runtime/tests/streaming-alignment_*.log` to replace the interim telemetry collected during Task 038.
- **Documentation + SDD updates.** Produce a dedicated spec/plan/tasks set for the follow-up work (or extend this draft) and link the new evidence to `dev-docs/core_servers/acplb-core-runtime.md` so downstream adapters can rely on the behavior contract.

