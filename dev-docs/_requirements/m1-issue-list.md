# Milestone 0.1.0 – Issue Index

This index maps the legacy M1 tasks to the current SDD artefacts and clarifies status expectations. Use it as a quick reference when reviewing or creating engineering tasks for the 0.1.0 milestone.

## Active GitHub Issues (Milestone 0.1.0)

| Issue | Title | Status | Dependencies |
| --- | --- | --- | --- |
| [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) | Adopt Official ACP Runtime for Core Loop | 🚧 In Progress | None |
| [#45](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45) | Align Streaming Notifications with ACP Models | 🔄 Planned | #44 |
| [#46](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/46) | Remove Legacy Protocol Mirror and Adopt Official Models | 🔄 Planned | #44, #45 |

## Legacy Task Mapping

| Legacy Task | Current Coverage | GitHub Issue | Status | Notes |
| --- | --- | --- | --- | --- |
| core-transport-1 | `#44-runtime-adoption-core-loop.md` | [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) | 🚧 In progress | Transport module merged; evidence in `_artifacts/tests/runtime-adoption/` |
| core-permissions-1 | `#44-runtime-adoption-core-loop.md` | [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) | 🚧 In progress | Permission mapping verified; env overrides documented |
| acp-wire-0 | `#46-protocol-cleanup-official-models.md` | [#46](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/46) | 🔄 Planned | Upstream ACP types adopted; legacy module scheduled for removal |
| codex-stdio-1 | `#44-runtime-adoption-core-loop.md` | [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) | 🚧 In progress | Handcrafted loop replaced; playback validation ongoing |
| codex-proto-1 | `#44-runtime-adoption-core-loop.md` & `#45-streaming-alignment-session-notifications.md` | [#44](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44) & [#45](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45) | 🚧 In progress | Streaming migration + notify parity tracked together |
| codex-notify-1 | `#45-streaming-alignment-session-notifications.md` | [#45](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45) | 🔄 Planned | Idle timeout/notify tests needed before closure |
| codex-tools-1 | `#45-streaming-alignment-session-notifications.md` | [#45](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45) | 🔄 Planned | Will finalize after ACP tool-call migration |
| codex-errors-1 | `#46-protocol-cleanup-official-models.md` | [#46](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/46) | 🔄 Planned | Invalid params + path validation to be revisited post-cleanup |
| tests-jsonl-1 | `_artifacts/tests/protocol-baseline/` | N/A | ✅ Completed | JSONL fixtures migrated; future additions go through evidence directory |

## How to Use This Index

1. **Before opening an issue** – start from the corresponding draft in `_issues_drafts/` and ensure the status above aligns with reality.
2. **When updating status** – edit both the issue draft and this table (or retire the row once the milestone closes).
3. **For evidence** – collect logs/tests under `_artifacts/<task>/...`; legacy references (`_artifacts/tests/legacy/`) are read-only.

## Related Artefacts

- Requirements: `dev-docs/_requirements/acp-lazybridge-requirements.md`
- Technical Playbook: `dev-docs/_requirements/m1-technical-implementation-plan.md`
- Migration Blueprint: `dev-docs/_projects/migration-blueprint-project-management-plan.md`
- Issue Drafts: `_issues_drafts/open/#44-runtime-adoption-core-loop.md`, `_issues_drafts/open/#45-streaming-alignment-session-notifications.md`, `_issues_drafts/open/#46-protocol-cleanup-official-models.md`
