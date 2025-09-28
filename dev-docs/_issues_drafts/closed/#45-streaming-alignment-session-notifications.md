# Align Streaming Notifications with ACP Models

_issues for SDD Task (closed)_

**GitHub Issue**: [#45](dev-docs/_issues_drafts/closed/#45-streaming-alignment-session-notifications.md) | <https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45>
**Status**: closed (merged to main)

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

## Acceptance Criteria

- Streaming output validated via snapshot tests that assert JSON matches ACP schemas.
- Notify and idle-timeout flows remain intact; Zed client smoke tests pass.
- SDD artefacts (spec/plan/tasks) capture the refactor details and test evidence.

## References

- `dev-docs/_requirements/Roadmap.md`
- `dev-docs/core_servers/acplb-core-runtime.md`
- `dev-docs/_project_management/migration-blueprint-project-management-plan.md`

---

## Resolution (2025-09-28T18:05:00Z)

- Status: merged to `main`
- SDD Task: 039
- PRs: [#48](https://github.com/lwyBZss8924d/ACPLazyBridge/pull/48), [#50](https://github.com/lwyBZss8924d/ACPLazyBridge/pull/50)
- Merged commit: `81c48b9072f0c5fc1617485fd5336d086b4992e2`
- Specs updated (status=merged):
    - `specs/039-streaming-alignment-session-notifications/spec.md`
    - `specs/039-streaming-alignment-session-notifications/plan.md`
    - `specs/039-streaming-alignment-session-notifications/tasks.md`
- Protocol version aligned to `agent-client-protocol v0.4.3`.
