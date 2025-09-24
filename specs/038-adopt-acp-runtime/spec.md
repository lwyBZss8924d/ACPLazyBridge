# Feature Specification: Adopt Official ACP Runtime for Core Loop

```yaml
worktree: ../acplb-worktrees/038-adopt-acp-runtime
feature_branch: feature/038-adopt-acp-runtime
created: 2025-09-23T07:23:14Z
last_updated: 2025-09-24T07:33:37Z
status: ready_for_merge
input: User description from GitHub issue #44
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/44
spec_uri: specs/038-adopt-acp-runtime/spec.md
plan_uri: specs/038-adopt-acp-runtime/plan.md
tasks_uri: specs/038-adopt-acp-runtime/tasks.md
evidence_uris: _artifacts/038-adopt-acp-runtime/
specs:
    constitution: 1.0.1
    type: spec
    feature_number: 038
```

## Execution Flow (main)

```text
1. Parse user description from Input
   → GitHub issue #44: Runtime adoption requirements
2. Extract key concepts from description
   → Actors: Runtime Server, Codex Adapter, Agent Protocol
   → Actions: Replace JSON-RPC handling, Implement AgentSideConnection
   → Data: Sessions, Permissions, Notifications
   → Constraints: LocalSet execution, Backward compatibility
3. For each unclear aspect:
   → All critical aspects clarified in issue
4. Fill User Scenarios & Testing section
   → Session lifecycle, notify handling, timeout behavior
5. Generate Functional Requirements
   → Each requirement testable via JSONL scenarios
   → No ambiguous requirements remaining
6. Identify stakeholder outcomes
   → Reliability, extensibility, governance signals
7. Run Review Checklist
   → All checks passing
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines

- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

## User Scenarios & Testing

### Primary User Story

As a developer using ACPLazyBridge, I want the Codex adapter to use the official ACP runtime implementation so that protocol compliance is guaranteed and future ACP features are automatically available without custom parsing logic.

### Acceptance Scenarios

1. **Given** an existing JSONL scenario file, **When** the new runtime processes it, **Then** the output matches the baseline without regression
2. **Given** a session with notify configured, **When** a turn completion signal arrives, **Then** the session ends immediately without waiting for idle timeout
3. **Given** a session without notify, **When** no output occurs for the idle timeout period, **Then** the session ends with stopReason "end_turn"
4. **Given** permission mode settings, **When** creating a new session, **Then** the appropriate CLI overrides are applied to the Codex process
5. **Given** a cancellation request, **When** sent during an active session, **Then** the child process terminates and returns stopReason "cancelled"
6. **Given** `ACPLB_EVIDENCE_PATH` is set, **When** a session initializes, prompts, or cancels, **Then** the runtime appends structured telemetry events to the configured evidence file

### Edge Cases

- What happens when the agent-client-protocol crate is unavailable?
- How does system handle malformed JSONL input with the new runtime?
- What occurs if LocalSet execution fails or deadlocks?
- How does runtime behave under high concurrent session load?

## Requirements

### Functional Requirements (Roadmap Alignment)

- **Roadmap FR-0101**: ACPLazyBridge MUST run each prompt turn through the official ACP runtime so that downstream clients receive protocol-accurate responses without bespoke JSON parsing.
- **Roadmap FR-0104**: JSONL scenario replays MUST remain backward compatible, giving stakeholders confidence that existing workflows operate unchanged after the migration.
- **Roadmap FR-0105**: Notify hooks and idle timeout behavior MUST mirror the legacy bridge so on-call teams do not lose operational safeguards already validated in production.
- **Derived FR-038-01**: Permission decisions MUST continue to translate ACP permission modes into Codex CLI overrides, preserving access control guardrails promised to IDE integrators.
- **Derived FR-038-02**: The runtime MUST provide reusable session and notification primitives so the Claude and Gemini adapters planned for Milestone 0.2.0 can adopt the same foundation without rework.
- **Derived FR-038-03**: Operational telemetry (structured logs, JSONL transcripts, evidence bundles) MUST remain available—including runtime-emitted evidence when `ACPLB_EVIDENCE_PATH` is configured—to satisfy SDD Article III verification requirements.

### Key Outcomes & Stakeholder Impacts

- **Reliability**: The bridge presents the same stop reasons, notifications, and idle timeout behavior developers already scripted against, eliminating retraining costs for client teams.
- **Extensibility**: Future provider adapters plug into shared runtime capabilities, reducing time-to-market for roadmap items FR-0201 and beyond.
- **Governance**: Telemetry and evidence capture keep SDD validation auditable, enabling reviewers to trace behavior back to signed-off artefacts.

---

## Review & Acceptance Checklist

### Content Quality

- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness

- [x] No clarifications pending
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

---

## Execution Status

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked (none found)
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
