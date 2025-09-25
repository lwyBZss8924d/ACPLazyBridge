# Milestone 0.1.0 – Issue Index

This index maps the legacy M1 tasks to the current SDD artefacts and clarifies status expectations. Use it as a quick reference when reviewing or creating engineering tasks for the 0.1.0 milestone.

## Active GitHub Issues (Milestone 0.1.0)

| Issue | Title | Status | Dependencies | Completion |
| --- | --- | --- | --- | --- |
| [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) | Adopt Official ACP Runtime for Core Loop | ✅ **Completed** | None | PR #47 (commit 7ae2628, 2025-09-24T11:03:37Z) |
| [#45](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45) | Align Streaming Notifications with ACP Models | 🔄 Planned | #44 | Deferred to Phase 4 follow-up |
| [#46](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/46) | Remove Legacy Protocol Mirror and Adopt Official Models | 🔄 Planned | #44, #45 | Deferred to Phase 5 follow-up |

## Legacy Task Mapping

| Legacy Task | Current Coverage | GitHub Issue | Status | Notes |
| --- | --- | --- | --- | --- |
| core-transport-1 | `specs/038-adopt-acp-runtime/` | [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) | ✅ **Completed** | Transport module merged via SDD Task 038; evidence in `_artifacts/038-adopt-acp-runtime/` |
| core-permissions-1 | `specs/038-adopt-acp-runtime/` | [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) | ✅ **Completed** | Permission mapping verified; env overrides documented in runtime module |
| acp-wire-0 | `#46-protocol-cleanup-official-models.md` | [#46](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/46) | 🔄 Planned | Upstream ACP types adopted; legacy module scheduled for removal in Phase 5 |
| codex-stdio-1 | `specs/038-adopt-acp-runtime/` | [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) | ✅ **Completed** | Handcrafted loop replaced with `AgentSideConnection`; JSONL compatibility maintained |
| codex-proto-1 | `specs/038-adopt-acp-runtime/` & `#45-streaming-alignment-session-notifications.md` | [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) & [#45](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45) | ✅ **Partially Complete** | Core runtime completed; streaming alignment deferred to Phase 4 follow-up |
| codex-notify-1 | `#45-streaming-alignment-session-notifications.md` | [#45](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45) | 🔄 Planned | Notify parity maintained; full streaming alignment deferred to Phase 4 |
| codex-tools-1 | `#45-streaming-alignment-session-notifications.md` | [#45](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45) | 🔄 Planned | Tool-call lifecycle in progress; completion deferred to Phase 4 |
| codex-errors-1 | `#46-protocol-cleanup-official-models.md` | [#46](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/46) | 🔄 Planned | Error handling updated; protocol cleanup deferred to Phase 5 |
| tests-jsonl-1 | `_artifacts/tests/protocol-baseline/` | N/A | ✅ Completed | JSONL fixtures migrated; SDD Task 038 evidence in `_artifacts/038-adopt-acp-runtime/` |

## How to Use This Index

1. **Before opening an issue** – start from the corresponding draft in `_issues_drafts/` and ensure the status above aligns with reality.
2. **When updating status** – edit both the issue draft and this table (or retire the row once the milestone closes).
3. **For evidence** – collect logs/tests under `_artifacts/<task>/...`; legacy references (`_artifacts/tests/legacy/`) are read-only.

## Related Artefacts

- Requirements: `dev-docs/_requirements/acp-lazybridge-requirements.md`
- Technical Playbook: `dev-docs/_requirements/m1-technical-implementation-plan.md`
- Migration Blueprint: `dev-docs/_projects/migration-blueprint-project-management-plan.md`
- Issue Drafts: `_issues_drafts/closed/#44-runtime-adoption-core-loop.md`, `_issues_drafts/open/#45-streaming-alignment-session-notifications.md`, `_issues_drafts/open/#46-protocol-cleanup-official-models.md`
- SDD Task 038: `specs/038-adopt-acp-runtime/` (completed with PR #47, commit 7ae2628)
