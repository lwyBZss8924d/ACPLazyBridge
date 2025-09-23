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

| Workstream | Deliverable | Exit Criteria |
| --- | --- | --- |
| Runtime Adoption | Shared runtime crate built on `AgentSideConnection` | CI green, JSONL replay diff accepted, telemetry captured |
| Streaming Alignment | Notifications emitted via official ACP structs | Snapshot tests updated, client compatibility validated |
| Protocol Cleanup | Legacy protocol module removed | No references to `acp-lazy-core::protocol`, docs updated |
| Documentation | Updated roadmap + architecture set | Reviewed by architecture WG, linked in GitHub project |

## Project Board Structure

- **Columns**: Backlog → In Spec → In Plan → In Tasks → In Progress → In Review → Done.
- **Cards**: One card per SDD task; each card links to GitHub Issue, spec, plan, tasks, and evidence artefact directory.
- **Labels**: `milestone:0.1.0`, `area:runtime`, `area:streaming`, `area:protocol`, `doc-update`.

## Milestones & Target Dates

| Milestone | Target Date | Dependencies |
| --- | --- | --- |
| Runtime Adoption Spec Ready | 2025-10-06 | Roadmap approval |
| Runtime Adoption Code Complete | 2025-11-10 | Spec/plan/tasks executed |
| Streaming Alignment Complete | 2025-11-24 | Runtime adoption merged |
| Protocol Cleanup Complete | 2025-12-08 | Streaming alignment merged |

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

- Zero failing ACP compatibility tests across Codex, Claude, and Gemini after migration.
- 100% of tasks include traceable SDD artefacts with evidence links.
- Runtime crash rate < 0.5% across recorded JSONL replays.

## Next Steps

1. Publish replacement GitHub Issues using the pre-issue notes in `dev-docs/requirements/pre-issues-notes/`.
2. Initialise GitHub Project board with columns and labels defined above.
3. Schedule architecture WG review to sign off the runtime design baseline.
