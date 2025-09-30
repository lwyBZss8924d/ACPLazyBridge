---
date: 2025-09-28T23:20:00Z
task: 039-streaming-alignment-session-notifications
status: merged
issue: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45
spec: specs/039-streaming-alignment-session-notifications/spec.md
plan: specs/039-streaming-alignment-session-notifications/plan.md
tasks: specs/039-streaming-alignment-session-notifications/tasks.md
prs:
  - https://github.com/lwyBZss8924d/ACPLazyBridge/pull/48
  - https://github.com/lwyBZss8924d/ACPLazyBridge/pull/50
merged_commit: 81c48b9072f0c5fc1617485fd5336d086b4992e2
evidence:
  - _artifacts/039-streaming-alignment/tests/
  - _artifacts/039-streaming-alignment/logs/
  - _artifacts/039-streaming-alignment/jq/
  - _artifacts/039-streaming-alignment/reports/
---

# 039 – Streaming Alignment: ACP Session Notifications

This changelog documents the merged work for aligning streaming notifications
with official ACP models in the Codex adapter, following the SDD workflow.

## Summary

Replace bespoke streaming types in `crates/codex-cli-acp` with official
`agent_client_protocol` models, route all session notifications through the
canonical path, and remove the simulated fallback branch while preserving
deduplication, notify sink, and idle-timeout behavior.

## Links

- Spec: `specs/039-streaming-alignment-session-notifications/spec.md`
- Plan: `specs/039-streaming-alignment-session-notifications/plan.md`
- Tasks: `specs/039-streaming-alignment-session-notifications/tasks.md`
- Issue: #45
- PRs: #48, #50
- Evidence: `_artifacts/039-streaming-alignment/`

## Changes

### Added

- Adopt official `agent_client_protocol::{SessionNotification, SessionUpdate,
  ContentBlock, ToolCall, ToolCallUpdate}` throughout streaming pipeline.
- Snapshot tests and lifecycle tests to validate ACP JSON schema fidelity.

### Changed

- Replace custom `SessionUpdate`/`ContentBlock`/`ToolCallStatus` with official
  ACP types.
- Map tool-call lifecycle (pending → in_progress → completed/failed) using
  official `ToolCall`/`ToolCallUpdate` with metadata (raw I/O, locations,
  titles, kinds where applicable).
- Preserve last-chunk deduplication semantics on official types.
- Maintain notify sink and idle-timeout behavior.

### Removed

- Simulated fallback branch in `CodexProviderAdapter::spawn_and_stream_codex`.

### Fixed

- Ensure SessionNotifications reach downstream ACP clients (validated by Zed
  smoke test, see T033 notes in `tasks.md`).

## Acceptance Criteria (excerpt)

- SessionNotification/ContentBlock/ToolCall/ToolCallUpdate match ACP schema. ✓
- Notify and idle-timeout flows intact. ✓
- Fallback branch removed and validated by tests/JSONL replay. ✓
- JSONL regression parity maintained. ✓

## Quality Gates

- cargo fmt — pass
- cargo clippy (deny warnings) — pass
- cargo test (workspace, locked) — pass
- SDD docs validation — pass

## Evidence

Artifacts captured under `_artifacts/039-streaming-alignment/`:

- tests: insta snapshots, JSONL diffs, lifecycle logs
- logs: streaming timing and tool-call traces
- jq: JSON validations (where applicable)
- reports: performance metrics (≤150ms baseline maintained)

## Impact & Compatibility

- No public API breaks; adapter behavior conforms to ACP schema.
- Downstream ACP clients (e.g., Zed) receive official notifications.

## Rollback Plan

- Revert to pre-migration commit if required:
  `git revert 81c48b9072f0c5fc1617485fd5336d086b4992e2` (or revert PRs #48/#50).
  Note: reintroduces simulated fallback and custom types.

## References

- Constitution Articles: I (Library-First), III (Test-First), VII (Simplicity),
  VIII (Anti-Abstraction), IX (Integration-First)
- Keep a Changelog: see `sdd-rules/rules/changelog/examples.md`
- Changelog guidelines: `dev-docs/changelogs/README.md`
