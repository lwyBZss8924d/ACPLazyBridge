# [Draft] Feature Specification: Complete Codex Protocol Alignment for MVP (Issue #50 | Task #040)

```yaml
worktree: ../acplb-worktrees/040-codex-protocol-alignment-mvp
feature_branch: feature/040-codex-protocol-alignment-mvp
created: 2025-09-29T21:24:56Z
last_updated: 2025-09-29T21:24:56Z
status: draft
input: Milestone 0.1.0 research for Codex ↔ ACP bridge (Issue draft #50)
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/50
spec_uri: specs/drafts-040-codex-protocol-alignment-mvp/spec.md
plan_uri: specs/drafts-040-codex-protocol-alignment-mvp/plan.md
tasks_uri: specs/drafts-040-codex-protocol-alignment-mvp/tasks.md
evidence_uris:
  - _artifacts/040-codex-protocol-alignment-mvp/tests/
  - _artifacts/040-codex-protocol-alignment-mvp/logs/
  - _artifacts/040-codex-protocol-alignment-mvp/reports/
specs:
  constitution: 1.0.1
  type: spec
  feature_number: 040
dependencies:
  - specs/038-adopt-acp-runtime/spec.md
  - specs/039-streaming-alignment-session-notifications/spec.md
  - dev-docs/_requirements/Roadmap.md
  - dev-docs/_requirements/m1-technical-implementation-plan.md
  - dev-docs/_projects/migration-blueprint-project-management-plan.md
```

## Summary

Deliver the remaining protocol alignment required for the Milestone 0.1.0 Codex ↔ ACP MVP. This feature replaces the ad hoc submission parsing, content conversion, plan/tool bridging, and approval flows inside `crates/codex-cli-acp` with comprehensive mappings to the official `agent-client-protocol` models. The outcome is a bridge that emits fully populated ACP notifications for every Codex event and surfaces Codex permission, sandbox, and slash-command affordances to ACP clients without custom shims.

## Problem Statement

Following Tasks 038 and 039, ACPLazyBridge can launch Codex and stream base `SessionUpdate::agent_message_chunk` events, but large portions of the Codex protocol are still translated partially or not at all:

- `codex_proto.rs` only maps a subset of `EventMsg` variants, omitting plan updates, approval requests, tool lifecycle metadata, and rich stop reasons.
- `tool_calls.rs` cannot classify newer Codex tool envelopes (apply_patch, MCP, plan tool) into the granular ACP `ToolKind` families or emit accurate location data.
- Submission payloads lose important context (`Op::UserTurn` cwd, sandbox flags, and XML-wrapped metadata), leading to inconsistent session state and content annotations.
- Slash commands, experimental JSON stream events, and @-mention resource hints are not exposed to ACP clients, limiting Zed and future IDE integrations.
- GitHub Issue #46 is written for a narrow protocol cleanup and no longer represents the scope required to finish the MVP.

Without closing these gaps, the Codex adapter fails to meet the integration contract captured in the roadmap and the migration blueprint. ACP clients cannot rely on the bridge for advanced tooling, real-time approvals, or accurate plan state, blocking the milestone release.

## Motivation

- **Protocol fidelity**: Guarantee that every Codex event is represented with the official ACP schema so downstream clients do not maintain custom adapters.
- **IDE readiness**: Provide Zed and other ACP clients with plan, tool, and approval signals they need to drive UI affordances.
- **Operational transparency**: Preserve sandbox, model, and permission metadata for auditability and to support Constitution Article IX (Integration-First).
- **Documentation currency**: Retire the outdated Issue #46 brief and align requirements documents with the broader MVP target captured in Issue #50.

## Goals

1. Map the full Codex submission/event surface to ACP `SessionUpdate` variants, including plan, tool lifecycle, approvals, slash commands, and notify signals.
2. Preserve contextual metadata (cwd, sandbox, model, reasoning toggles, XML wrappers, @-mentions) as ACP content annotations or resource links so clients can display provenance.
3. Update the Milestone 0.1.0 requirements artefacts and issue drafts to reflect the expanded scope (Issue #50 superceding Issue #46).
4. Establish traceable evidence (tests, JSONL scenarios, documentation updates) proving ACP fidelity for the MVP deliverable.

## Non-Goals

- Building new Codex features beyond the existing CLI surface.
- Shipping adapters for non-Codex agents (Claude, Gemini) within this task.
- Introducing persistence or queueing beyond in-memory session state.
- Refactoring unrelated runtime modules (transport, permissions) already completed in Task 038.

## Scope

### In Scope

- Enhancing `codex_proto.rs`, `tool_calls.rs`, and new helper modules to cover all targeted Codex events.
- Formalising submission parsing and session context handling for ACP session lifecycle methods.
- Extending tool classification to support apply_patch and MCP variants with accurate `ToolKind`, status transitions, and locations.
- Surfacing slash-command availability and @-mention resource suggestions to ACP clients.
- Preparing experimental JSON stream ingestion pathways for future task phases (flagged but not enabled by default).
- Updating `dev-docs/_issues_drafts/` (Issue #50 draft) and the requirement/artifact references listed in the Dependencies section.

### Out of Scope

- Runtime telemetry integrations beyond the evidence logging mandated in tasks.
- Non-ACP protocol bridges or service orchestrations.
- UI changes inside client applications (Zed, VS Code).

## User Scenarios & Testing

### Primary User Story

As an IDE engineer using ACPLazyBridge with the Codex CLI, I want all Codex events—messages, tool calls, plan updates, approvals, and resource hints—to arrive through ACP notifications so the IDE can render the full workflow without custom translation code.

### Acceptance Scenarios

1. **Given** a Codex session started via `session/new` with sandbox overrides, **when** Codex emits a plan update and subsequent tool calls, **then** the ACP client receives `SessionUpdate::Plan` entries with matching titles/statuses and tool lifecycle updates with accurate `ToolKind`, raw payloads, and stop reasons.
2. **Given** a Codex apply-patch approval request, **when** the user grants approval, **then** the bridge emits corresponding `ToolCallUpdate` messages with approval metadata and location hints, and the resulting patch outcome is streamed as ACP agent messages.
3. **Given** a user prompt containing XML `<environment_context>` and @-mentions, **when** the bridge processes the submission, **then** session state captures cwd/sandbox/model metadata and ACP content blocks annotate resource links for the mentioned files.
4. **Given** the Codex CLI running with `--experimental-json`, **when** the bridge is configured for that mode, **then** it emits equivalent ACP updates for the conversation events without regressing the default proto pipeline.

### Edge Cases

- Codex emits partial tool results followed by failure → ensure ACP updates reflect status transition to `failed` with diagnostic data.
- Multiple parallel tool calls → maintain unique `toolCallId` handles and independent status tracking.
- MCP tool invocation referencing external servers → propagate raw input/output and duration while redacting secrets per Constitution Article IX.
- Idle timeout triggers before notify → emit `stopReason` with timeout context and ensure session termination flows without dangling tasks.

## Functional Requirements

- **FR-040-001**: The bridge MUST parse `Submission { id, op }` messages, handling `Op::UserTurn`, `Op::UserInput`, and approval overrides while capturing cwd, sandbox, approval policy, model, and optional JSON schema metadata.
- **FR-040-002**: The bridge MUST convert `InputItem::{Text,Image,LocalImage}` plus XML wrappers into ACP `ContentBlock` variants, preserving annotations for user instructions, environment context, and resource links.
- **FR-040-003**: The bridge MUST map all `EventMsg` variants (plan updates, tool lifecycle, MCP events, approvals, task summaries, turn completion) to ACP `SessionUpdate` payloads with accurate status transitions and raw payload fields.
- **FR-040-004**: `tool_calls.rs` MUST classify Codex tool kinds (function, local shell, web search, apply_patch, MCP) into ACP `ToolKind` values, attach location metadata, and surface raw inputs/outputs within size limits.
- **FR-040-005**: MCP tool events MUST emit paired `ToolCall` and `ToolCallUpdate` notifications with timing, success flags, and namespaced tool identifiers.
- **FR-040-006**: Approval prompts (execute, apply_patch) MUST appear as ACP tool-call updates with `pending` → `in_progress` → terminal status and include the approval rationale and granted scope.
- **FR-040-007**: Slash command listings and @-mention suggestions MUST be exposed via ACP `AvailableCommandsUpdate` or resource link annotations so clients can mirror Codex affordances.
- **FR-040-008**: The bridge MUST support both proto and experimental JSON stream modes, gated by configuration, with parity test coverage.
- **FR-040-009**: Requirements documents (`Roadmap.md`, `m1-technical-implementation-plan.md`, `m1-issue-list.md`, `acp-lazybridge-requirements.md`, `acp-lazybridge-project-plan.md`, `migration-blueprint-project-management-plan.md`) MUST be updated to reference Issue #50 and the expanded MVP scope.
- **FR-040-010**: Issue #46 MUST be archived or marked as superseded by Issue #50, with traceability captured in `_issues_drafts/` and the milestone index.

## Non-Functional Requirements

- **NFR-040-001**: Maintain streaming latency comparable to current proto pipeline (<50 ms overhead per event on macOS dev hardware).
- **NFR-040-002**: Ensure ACP payloads remain valid against `agent-client-protocol` v0.4.3 schema using serialization tests and JSON schema validation.
- **NFR-040-003**: Preserve backward compatibility for existing JSONL regression tests; add new fixtures without breaking historical evidence.
- **NFR-040-004**: All documentation changes MUST pass `scripts/sdd/check-sdd-consistency.sh` and related linting tools.
- **NFR-040-005**: Evidence artifacts MUST be stored under `_artifacts/040-codex-protocol-alignment-mvp/` with timestamps and referenced in the plan/tasks.

## Dependencies & Constraints

- Relies on runtime groundwork from Tasks 038 and 039; code changes must align with their architecture decisions.
- Depends on official ACP crate `agent-client-protocol` >= v0.4.3; any upstream changes must be tracked.
- Must coordinate with documentation updates to avoid conflicting edits with other Milestone 0.1.0 tasks.
- Sandbox and approval mapping must respect Codex CLI limitations (no dynamic approvals when `approval_policy=never`).

## Risks & Mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Incomplete event coverage leading to client regressions | High | Build exhaustive scenario matrix covering all EventMsg variants; add failing tests before implementation per Article III |
| Tool output size exceeding ACP limits | Medium | Implement truncation with clear metadata and evidence logs |
| Documentation drift during long-running branch | Medium | Schedule doc updates early and rerun consistency scripts before PR |
| Experimental JSON path diverges from proto | Medium | Gate behind feature flag and add regression tests for both modes |

## Evidence Expectations

- JSONL regression runs covering plan, tool, approval, MCP, and slash command scenarios.
- Schema validation logs demonstrating ACP payload compliance.
- Updated documentation diffs with lint/consistency logs stored under `_artifacts/040-codex-protocol-alignment-mvp/reports/`.
- Manual Zed integration notes captured in quickstart/research artefacts per plan.

## Constitutional Alignment

- **Article III – Test-First**: Write failing lifecycle tests for each new `EventMsg` mapping before implementing; capture approval/tool scenarios in regression suites.
- **Article VII – Simplicity**: Replace bespoke mapping logic with direct usage of official ACP models and avoid additional abstraction layers beyond necessary helpers.
- **Article IX – Integration-First**: Validate changes against real Codex CLI runs and Zed smoke tests, ensuring contracts are defined before implementation.

## References

- `dev-docs/_issues_drafts/open/#50-codex-protocol-alignment-mvp.md`
- `dev-docs/_requirements/Roadmap.md`
- `dev-docs/_requirements/m1-technical-implementation-plan.md`
- `dev-docs/_requirements/m1-issue-list.md`
- `dev-docs/_requirements/acp-lazybridge-requirements.md`
- `dev-docs/_projects/migration-blueprint-project-management-plan.md`
- `specs/038-adopt-acp-runtime/`
- `specs/039-streaming-alignment-session-notifications/`
- `agent-client-protocol` repository (`~/dev-space/agent-client-protocol`)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
