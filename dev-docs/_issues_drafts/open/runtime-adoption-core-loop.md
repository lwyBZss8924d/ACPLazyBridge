# Adopt Official ACP Runtime for Core Loop

**GitHub Issue**: [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44)
**Status**: 🚧 In Progress (Milestone 0.1.0)

## Summary

Implement a shared ACPLazyBridge runtime built on `agent-client-protocol::AgentSideConnection`, replacing the handcrafted JSON-RPC handling in `crates/codex-cli-acp`. The runtime must run inside Tokio `LocalSet`, preserve notify/timeout semantics, and expose reusable abstractions for future agent servers.

## Motivation

- Eliminates protocol drift and duplicated error handling.
- Establishes a reusable runtime crate that other adapters (Claude, Gemini) can consume.
- Unlocks upstream features (plan updates, permission prompts) without bespoke parsing.

## Scope

- Introduce core runtime module/crate wrapping `AgentSideConnection`, session store, permission mapper, and notify handling.
- Refactor Codex adapter to use the new runtime while maintaining existing permission mode overrides and CLI spawning logic.
- Add regression tests covering initialise → new_session → prompt flows, notify events, idle timeout, and cancellation.
- Capture telemetry artefacts (JSONL transcripts, logs) demonstrating parity with the legacy loop.

## Out of Scope

- Streaming data model refactor (handled by separate issue).
- Removal of legacy protocol module (handled by separate issue).
- Composer plugin integration (future milestone).

## Acceptance Criteria

- Codex adapter compiles and runs solely via the new runtime abstraction.
- JSONL scenario replays show no regressions compared to baseline snapshots.
- CI remains green; docs/specs/plans/tasks created per SDD workflow.

## Dependencies

- `agent-client-protocol` crate ≥ 0.4.0 available in workspace.
- Architecture baseline: `dev-docs/core_servers/acplb-core-runtime.md`.

## Evidence Expectations

- Test artefacts stored under `_artifacts/tests/runtime-adoption/` (legacy mirrors optional under `_artifacts/legacy/`).
- Logs capturing notify forwarder behaviour stored under `_artifacts/logs/runtime-adoption/`.

## References

- `dev-docs/_requirements/Roadmap.md`
- `dev-docs/architecture/acplb-architecture.md`
- `dev-docs/core_servers/acplb-core-runtime.md`
- `dev-docs/_project_management/migration-blueprint-project-management-plan.md`
