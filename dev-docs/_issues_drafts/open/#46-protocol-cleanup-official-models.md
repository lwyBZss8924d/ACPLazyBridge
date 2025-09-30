# Remove Legacy Protocol Mirror and Adopt Official Models

> **Superseded**: This draft is retained for historical context. Active planning continues in `_issues_drafts/open/#50-codex-protocol-alignment-mvp.md` (Issue #50).

_issues for SDD Task (pending)_
`[TODO]: ("to be updated")` Research in progress 🔄 (_To be supplemented and updated before releasing new ISSUES# details and comments to replace GitHub ISSUES-46, and use the new ISSUES #50 draft to replace ISSUES-46_)

**GitHub Issue**: [#46](dev-docs/_issues_drafts/open/#46-protocol-cleanup-official-models.md) | <https://github.com/lwyBZss8924d/ACPLazyBridge/issues/46>
**Status**: issue is open (Milestone 0.1.0)
**Dependencies**:

- [#44](dev-docs/_issues_drafts/open/#44-runtime-adoption-core-loop.md) | <https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44>

- [#45](dev-docs/_issues_drafts/open/#45-streaming-alignment-session-notifications.md) | <https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45>

## Summary

Delete the bespoke JSON-RPC/ACP model in `crates/acp-lazy-core/src/protocol.rs` and migrate all call sites to the official `agent_client_protocol` types. Ensure tests, docs, and tooling reference the upstream crate exclusively.

## Motivation

- Eliminates duplicated error definitions and reduces risk of schema drift.
- Simplifies workspace dependencies by relying on a single protocol source.
- Completes the migration begun in the runtime and streaming refactors.

## Scope

- Replace all imports from `acp_lazy_core::protocol` with `agent_client_protocol` equivalents.
- Remove protocol module and adjust `lib.rs` exports accordingly.
- Update unit/integration tests, fixtures, and documentation impacted by the type change.
- Run structure, language, markdown, and semantic checks to confirm documentation accuracy.

## Out of Scope

- Additional feature work beyond the type migration.
- Composer plugin development.

## Acceptance Criteria

- No remaining references to `acp_lazy_core::protocol` in the workspace.
- Test suites (`cargo test --workspace --all-features --locked`) and CI scripts succeed.
- Documentation references updated (roadmap, design docs, READMEs where applicable).
- Evidence archived under `_artifacts/tests/protocol-cleanup/` and `_artifacts/logs/protocol-cleanup/` (legacy mirrors optional under `_artifacts/legacy/`).

## Dependencies

- Runtime adoption and streaming alignment issues completed.
- Latest architecture and core runtime docs reviewed.

## Evidence Expectations

- Test logs demonstrating full workspace test pass.
- Markdown lint logs stored with documentation updates.

## References

- `dev-docs/_requirements/Roadmap.md`
- `dev-docs/architecture/acplb-architecture.md`
- `dev-docs/core_servers/acplb-core-runtime.md`
- `dev-docs/_project_management/migration-blueprint-project-management-plan.md`

## Deferred follow-up from SDD Task 038 (Phase 5)

Streaming alignment (Issue #45) sets the stage, but SDD Task 038 also deferred the full protocol cleanup. Capture the remaining work here:

- **Retire `acp-lazy-core/src/protocol.rs`.** Replace every consumer with `agent_client_protocol` equivalents, update exports in `crates/acp-lazy-core/src/lib.rs`, and delete the legacy module/tests.
- **Refresh test fixtures and JSONL baselines.** Record new `_artifacts/038-adopt-acp-runtime/tests/protocol-cleanup_*.log` runs (workspace tests, JSONL regressions, markdown/semantic checks) to demonstrate parity after the removal.
- **Documentation sweep.** Update `dev-docs/core_servers/acplb-core-runtime.md`, roadmap entries, and the streaming issue (#45) to point at the official models only; run `scripts/sdd/validate-sdd-docs.sh` to capture the final logs.
- **SDD artefact follow-up.** Author a dedicated spec/plan/tasks set (or append to this draft) so Constitution gates for the cleanup are auditable once Issue #45 lands.

