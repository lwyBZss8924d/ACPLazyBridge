# Milestone 0.1.0 – Issue Index

This index maps the legacy M1 tasks to the current SDD artefacts and clarifies status expectations. Use it as a quick reference when reviewing or creating engineering tasks for the 0.1.0 milestone.

| Legacy Task | Current Coverage | Status | Notes |
| --- | --- | --- | --- |
| core-transport-1 | `runtime-adoption-core-loop.md` (Runtime Adoption) | ✅ Completed | Transport module merged; evidence in `_artifacts/tests/runtime-adoption/` |
| core-permissions-1 | `runtime-adoption-core-loop.md` | ✅ Completed | Permission mapping verified; env overrides documented |
| acp-wire-0 | `protocol-cleanup-official-models.md` | ✅ Completed | Upstream ACP types adopted; legacy module scheduled for removal |
| codex-stdio-1 | `runtime-adoption-core-loop.md` | 🚧 In progress | Handcrafted loop replaced; playback validation ongoing |
| codex-proto-1 | `runtime-adoption-core-loop.md` & `streaming-alignment-session-notifications.md` | 🚧 In progress | Streaming migration + notify parity tracked together |
| codex-notify-1 | `streaming-alignment-session-notifications.md` | 🚧 In progress | Idle timeout/notify tests needed before closure |
| codex-tools-1 | `streaming-alignment-session-notifications.md` | 🔄 Planned | Will finalize after ACP tool-call migration |
| codex-errors-1 | `protocol-cleanup-official-models.md` | 🔄 Planned | Invalid params + path validation to be revisited post-cleanup |
| tests-jsonl-1 | `_artifacts/tests/protocol-baseline/` | ✅ Completed | JSONL fixtures migrated; future additions go through evidence directory |

## How to Use This Index

1. **Before opening an issue** – start from the corresponding draft in `_issues_drafts/` and ensure the status above aligns with reality.
2. **When updating status** – edit both the issue draft and this table (or retire the row once the milestone closes).
3. **For evidence** – collect logs/tests under `_artifacts/<task>/...`; legacy references (`_artifacts/tests/legacy/`) are read-only.

## Related Artefacts

- Requirements: `dev-docs/_requirements/acp-lazybridge-requirements.md`
- Technical Playbook: `dev-docs/_requirements/m1-technical-implementation-plan.md`
- Migration Blueprint: `dev-docs/_projects/migration-blueprint-project-management-plan.md`
- Issue Drafts: `_issues_drafts/runtime-adoption-core-loop.md`, `streaming-alignment-session-notifications.md`, `protocol-cleanup-official-models.md`
