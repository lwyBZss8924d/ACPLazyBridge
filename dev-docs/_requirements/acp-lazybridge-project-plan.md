# ACPLazyBridge Implementation Plan

```yaml
Issue-URI: <https://github.com/lwyBZss8924d/ACPLazyBridge/issues/NNN>
Plan-URI: dev-docs/_requirements/acp-lazybridge-project-plan.md
Related:
  - Roadmap: dev-docs/_requirements/Roadmap.md
  - Requirements: dev-docs/_requirements/acp-lazybridge-requirements.md
  - Architecture: dev-docs/architecture/acplb-architecture.md
  - Core Runtime Design: dev-docs/core_servers/acplb-core-runtime.md
  - Migration Blueprint: dev-docs/_projects/migration-blueprint-project-management-plan.md
```

## 1. Purpose & Scope

- Define the staged execution plan that delivers ACPLazyBridge from the core runtime migration (Milestone 0.1.0) through ecosystem expansion (Milestone 0.4.0).
- Align engineering workstreams with Specification-Driven Development (SDD) artefacts, ensuring every milestone has traceable specs, plans, tasks, and evidence.
- Establish the governance hooks (quality gates, constitutional checks, evidence capture) required for each milestone before PR approval.

## 2. Milestone Summary

| Milestone | Target | Primary Outcomes | Status | Key References |
| --- | --- | --- | --- | --- |
| 0.1.0 – Core Runtime & Zed ↔ Codex MVP | Q3-1 2025 | Adopt `agent-client-protocol` runtime, replace handcrafted streaming, deprecate internal protocol models | 🚧 **In Progress** | FR-0101…FR-0105 in `acp-lazybridge-requirements.md`; `specs/038-adopt-acp-runtime/` (PR #47); follow-ups: `_issues_drafts/open/#45-streaming-alignment-session-notifications.md`, `_issues_drafts/open/#50-codex-protocol-alignment-mvp.md` |
| 0.2.0 – Multi-Agent Runtime & Composer Foundations | Q3-2 2025 | Claude & Gemini adapters on shared runtime; composer pipeline (`subagents`, `commands`, `hooks`) | 🔄 Planned | FR-0201…FR-0204; Issue drafts pending |
| 0.3.0 – Cross-Editor Clients & Advanced Workflows | Q4-1 2025 | VS Code / Obsidian / tldraw integrations; multi-session orchestration; workflow telemetry | 🔄 Planned | FR-0301…FR-0303; Issue drafts pending |
| 0.4.0 – Ecosystem Hardening & SDK Polish | Q4-2 2025 | Rust & TypeScript SDKs, runtime hardening, resilience testing | 🔄 Planned | FR-0401…FR-0402; Issue drafts pending |

## 3. Workstreams & Responsibilities

1. **Runtime Adoption** (Milestone 0.1.0) ✅ **Completed**
   - Owner: Runtime WG
   - Deliverables: Shared runtime crate, LocalSet orchestration, Codex adapter migration, regression evidence.
   - Issue Drafts: `specs/038-adopt-acp-runtime/` (Issue #44 completed via PR #47).
2. **Streaming Alignment** (Milestone 0.1.0, Phase 4) **✅ **Completed**
   - Owner: Runtime WG
   - Deliverables: Official ACP streaming models, deduplication safeguards, notify/timeout parity.
   - Issue Drafts: `_issues_drafts/open/#45-streaming-alignment-session-notifications.md`.
3. **Codex Protocol Alignment** (Milestone 0.1.0, Phase 5)
   - Owner: Runtime WG
   - Deliverables: Legacy protocol module removal, complete ACP submission/event coverage, documentation updates.
   - Issue Drafts: `_issues_drafts/open/#50-codex-protocol-alignment-mvp.md` (supersedes `_issues_drafts/open/#46-protocol-cleanup-official-models.md`).
4. **Composer Pipeline** (Milestones 0.2.0–0.3.0)
   - Owner: Composer WG
   - Deliverables: Plugin traits, translator subagent, configuration schema, orchestration telemetry.
5. **Client Integrations** (Milestones 0.3.0+)
   - Owner: Client WG
   - Deliverables: VS Code extension fork, Obsidian integration, tldraw agent UI, end-to-end workflow scripts.
6. **Ecosystem & Tooling** (Milestones 0.4.0)
   - Owner: Ecosystem WG
   - Deliverables: SDK documentation, resilience tests, long-term support plan.

## 4. SDD Artefact Expectations

- Every milestone must have corresponding `specs/<NNN>-<slug>/` entries with metadata blocks referencing Issue/Plan/Tasks/Evidence URIs.
- Pre-issue notes in `_issues_drafts/` serve as the mandatory analysis stage before raising GitHub issues.
- `_artifacts/<task>/` is the sole destination for evidence; legacy archives (`_artifacts/<type>/legacy/`) are read-only and retained for history.
- Changelog entries recorded in `dev-docs/changelogs/` after each milestone release.

## 5. Quality Gates & Constitutional Compliance

- Quality gates (fmt, clippy, test, `scripts/ci/run-local-ci.sh`, JSONL replay) must pass before PR merge.
- Constitutional checks per milestone:
    - Article III (Test-First): Failing tests or JSONL scenarios exist prior to implementation.
    - Article VII (Simplicity): Keep touched projects ≤3 per milestone change-set.
    - Article IX (Integration-First): Define contracts and playback scenarios before runtime changes.

## 6. Risk Register

| Risk | Impact | Mitigation |
| --- | --- | --- |
| ACP crate breaking changes | Runtime regressions | Monitor releases; add compatibility tests in `_artifacts/tests/protocol-baseline/` |
| Vendor CLI updates (Codex/Claude/Gemini) | Adapter downtime | Pin versions; maintain compatibility matrix in `references/cli_agents/` |
| Composer plugin complexity | Tooling instability | Incremental rollout; feature flags; extensive JSONL scenarios |
| Multi-editor UI variance | UX fragmentation | Define unified UX guidelines; snapshot tests for ACP payloads |

## 7. Schedule & Checkpoints

- **Quarterly Reviews:** Reconcile roadmap vs. actual progress; adjust milestones in `Roadmap.md`.
- **Monthly Sync:** Runtime, Composer, Client, Ecosystem WGs review open specs and issue drafts.
- **Evidence Review:** Migration blueprint board (`dev-docs/_projects/migration-blueprint-project-management-plan.md`) stores status updates and incident reports.

## 8. Deliverables Checklist by Milestone

- SDD artefacts (spec/plan/tasks) approved and linked.
- `_artifacts/<task>/` populated with tests/logs/jq/reports.
- Changelog entry created in `dev-docs/changelogs/`.
- References updated (`architecture/`, `core_servers/`, `requirements/`).

## 9. Follow-up Actions

- Complete conversion of remaining legacy documents (`m1-technical-implementation-plan.md`, `m1-issue-list.md`) into English SDD format.
- Publish GitHub Issue #50 referencing `_issues_drafts/open/#50-codex-protocol-alignment-mvp.md` and mark Issue #46 as superseded.
- Open GitHub Project cards aligned with this plan for each active issue draft.
- Schedule architecture review ahead of Milestone 0.2.0 to validate composer design extensions.
