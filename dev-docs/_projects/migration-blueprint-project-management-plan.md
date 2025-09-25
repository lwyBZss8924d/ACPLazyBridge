# Migration Blueprint Project Management Plan

**Status**: Draft – to be linked with GitHub Projects board for Milestone 0.1.0

## Purpose

Coordinate the migration of ACPLazyBridge to the official `agent-client-protocol` runtime while aligning Specification-Driven Development (SDD) artefacts, GitHub Issues, and evidence workflows.

## Scope

- Milestone 0.1.0 workstreams that replace legacy JSON-RPC handling, adopt typed streaming notifications, and remove internal protocol mirrors.
- Supporting documentation updates (roadmap, architecture, core runtime design).
- Pre-issue analysis feeding the replacement GitHub Issues for the retired #41, #42, and #43 threads.

## Governance

- **Project Owner**: ACPLazyBridge SDD Maintainer (codex AI engineer proxy)
- **Stakeholders**: Architecture WG, Runtime WG, Client Integration WG
- **SDD Artefacts**: `specs/<task>/spec.md`, `plan.md`, `tasks.md` for each issue derived from this blueprint.

## Workstreams & Deliverables

| Workstream | Deliverable | Status | Exit Criteria |
| --- | --- | --- | --- |
| Runtime Adoption | Shared runtime crate built on `AgentSideConnection` | ✅ **Completed** | CI green, JSONL replay diff accepted, telemetry captured (SDD Task 038, PR #47) |
| Streaming Alignment | Notifications emitted via official ACP structs | 🔄 Deferred to Phase 4 | Snapshot tests updated, client compatibility validated (Issue #45) |
| Protocol Cleanup | Legacy protocol module removed | 🔄 Deferred to Phase 5 | No references to `acp-lazy-core::protocol`, docs updated (Issue #46) |
| Documentation | Updated roadmap + architecture set | ✅ **Completed** | Reviewed by architecture WG, linked in GitHub project |

## Project Board Structure

- **Columns**: Backlog → In Spec → In Plan → In Tasks → In Progress → In Review → Done.
- **Cards**: One card per SDD task; each card links to GitHub Issue, spec, plan, tasks, and evidence artefact directory.
- **Labels**: `milestone:0.1.0`, `area:runtime`, `area:streaming`, `area:protocol`, `doc-update`.

## Milestones & Completion Dates

| Milestone | Target Date | Actual Date | Dependencies | Status |
| --- | --- | --- | --- | --- |
| Runtime Adoption Spec Ready | 2025-09-23 | 2025-09-23 | Roadmap approval | ✅ **Completed** |
| Runtime Adoption Code Complete | 2025-09-24 | 2025-09-24 | Spec/plan/tasks executed | ✅ **Completed** (PR #47) |
| Streaming Alignment Complete | 2025-10-06 | TBD | Runtime adoption merged | 🔄 Deferred to Phase 4 (Issue #45) |
| Protocol Cleanup Complete | 2025-10-20 | TBD | Streaming alignment merged | 🔄 Deferred to Phase 5 (Issue #46) |

## Risk Register

| Risk | Impact | Likelihood | Mitigation |
| --- | --- | --- | --- |
| Tokio LocalSet integration delays | Slips runtime milestone | Medium | Prototype in spike branch, pair review with runtime WG |
| Provider CLI changes | Breaks smoke tests | Medium | Pin CLI versions and track upstream release notes |
| Documentation drift | Confuses downstream teams | Low | Weekly project board review; enforce doc check scripts |

## Communication Plan

- Weekly async status update summarised in project board notes.
- Review meetings triggered at column transitions from “In Progress” → “In Review”.
- Incident reports stored under `_artifacts/logs/migration-blueprint/` (legacy mirrors may remain in `_artifacts/legacy/`).

## Success Metrics

### Achieved

- ✅ Zero failing ACP compatibility tests for Codex after migration (JSONL replay validation passing).
- ✅ 100% of completed tasks include traceable SDD artefacts with evidence links (`specs/038-adopt-acp-runtime/`, `_artifacts/038-adopt-acp-runtime/`).
- ✅ Runtime crash rate < 0.5% across recorded JSONL replays (comprehensive test suite passing).

### Outstanding

- 🔄 Zero failing ACP compatibility tests across Claude and Gemini (deferred to Milestone 0.2.0).
- 🔄 Full protocol cleanup completion (deferred to Phase 5, Issue #46).

## Next Steps

### Completed ✅

1. ✅ Runtime adoption completed via SDD Task 038 (PR #47, commit 7ae2628).
2. ✅ All SDD artefacts created and linked with comprehensive evidence.
3. ✅ Architecture WG review completed (runtime design baseline approved).

### Outstanding 🔄

1. 🔄 Publish Issue #45 (streaming alignment) using `_issues_drafts/open/#45-streaming-alignment-session-notifications.md`.
2. 🔄 Publish Issue #46 (protocol cleanup) using `_issues_drafts/open/#46-protocol-cleanup-official-models.md`.
3. 🔄 Initialise GitHub Project board for remaining Milestone 0.1.0 work (Phase 4 & 5).
4. 🔄 Schedule follow-up architecture review for streaming alignment and protocol cleanup phases.
