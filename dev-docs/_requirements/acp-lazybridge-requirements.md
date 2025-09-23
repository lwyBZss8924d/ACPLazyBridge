# ACPLazyBridge Requirements

## Purpose

This document consolidates the product requirements that guide ACPLazyBridge as it transitions to an Agent Client Protocol (ACP) native bridge and composer. It aligns directly with the roadmap in `dev-docs/_requirements/Roadmap.md` and the pre-issue briefs stored under `dev-docs/_issues_drafts/`.

## Strategic Alignment

- Deliver a reusable ACP runtime built on the official `agent-client-protocol` crate.
- Provide first-class adapters for Codex, Claude, Gemini, and future agent CLIs without duplicating protocol logic.
- Enable composer capabilities (subagents, commands, hooks) that let users orchestrate multiple specialised agents.
- Support ACP-compliant IDE integrations (Zed, VS Code, Obsidian, tldraw) with consistent UX expectations.

## Stakeholders & Personas

- **Developers** using IDEs/editors that surface ACP agent panels and expect reliable tool-call execution without manual approvals.
- **Integration engineers** building ACP clients or extending composer plugins who require stable schemas and reference implementations.
- **SDD maintainers** who rely on traceable requirements feeding specs, plans, tasks, and evidence.

## Scope

### In Scope

- Migration of every ACPLazyBridge server to the official ACP runtime and data models.
- Delivery of Codex, Claude, and Gemini adapters that share transport, permission, and telemetry layers.
- Composer plugin pipeline (`acplb-subagents`, `acplb-commands`, `acplb-hooks`) with an initial translator plugin example.
- Client integration requirements for Zed (MVP), followed by VS Code, Obsidian, and tldraw.

### Out of Scope (Current Cycle)

- Non-ACP transport layers (REST/GraphQL bridges) beyond the planned composer hooks.
- Long-lived session persistence across process restarts (to be assessed after milestone 0.3).
- IDE-specific UI customisation beyond ACP-configurable capabilities and permissions.

## Milestones & Requirement Buckets

| Release | Target | Primary Themes | Linked Pre-Issue Notes |
| --- | --- | --- | --- |
| 0.1.0 | Q3-1 2025 | Core runtime migration, Codex MVP, typed notifications | `runtime-adoption-core-loop.md`, `streaming-alignment-session-notifications.md`, `protocol-cleanup-official-models.md` |
| 0.2.0 | Q3-2 2025 | Claude/Gemini adapters, composer foundations | _TBD: composer plugin briefs_ |
| 0.3.0 | Q4-1 2025 | Cross-editor clients, advanced workflows | _TBD: client integration briefs_ |
| 0.4.0 | Q4-2 2025 | Ecosystem SDKs, runtime hardening | _TBD: sdk/hardening briefs_ |

## Functional Requirements

### Milestone 0.1.0 – Core Runtime & Zed ↔ Codex MVP

- **FR-0101**: ACPLazyBridge MUST host ACP traffic via `agent_client_protocol::AgentSideConnection` executed inside a Tokio `LocalSet`, wrapping existing session state and permission mapping. (_Pre-issue:_ `runtime-adoption-core-loop.md`)
- **FR-0102**: Codex streaming MUST emit ACP-native `SessionNotification`, `ContentBlock`, `ToolCall`, and `ToolCallUpdate` structures with chunk de-duplication preserved. (_Pre-issue:_ `streaming-alignment-session-notifications.md`)
- **FR-0103**: Workspace crates MUST remove the bespoke `acp_lazy_core::protocol` module and rely exclusively on official ACP error/response types. (_Pre-issue:_ `protocol-cleanup-official-models.md`)
- **FR-0104**: JSONL playback fixtures MUST reside under `_artifacts/tests/protocol-baseline/` and remain compatible with automated playback tests and Zed’s custom agent client.
- **FR-0105**: Idle timeout, notify-forwarder, and permission-mode behaviour MUST match the legacy implementation with telemetry evidence stored under `_artifacts/logs/runtime-adoption/`.

### Milestone 0.2.0 – Multi-Agent Runtime & Composer Foundations

- **FR-0201**: Claude and Gemini adapters MUST reuse the shared runtime crate, including permission mapping, process transport, and telemetry hooks.
- **FR-0202**: The composer pipeline MUST support ordered plugins with hooks for prompt mutation, session-update inspection, and tool-call augmentation.
- **FR-0203**: ACPLazyBridge MUST ship a reference `acplb-subagent-translator` plugin that delegates translation to a configured subagent session.
- **FR-0204**: Composer configuration MUST be declarative (TOML/JSON) with schema validation and runtime reload procedures documented.

### Milestone 0.3.0 – Cross-Editor Clients & Advanced Workflows

- **FR-0301**: Provide ACP client integrations (or documented forks) for VS Code, Obsidian, and tldraw that exercise the runtime through representative workflows.
- **FR-0302**: Implement multi-session orchestration allowing a primary agent to dispatch tasks to specialised subagents with traceable stop reasons.
- **FR-0303**: Capture workflow telemetry (sequence diagrams, JSONL transcripts) under `_artifacts/reports/workflows/` for each supported client scenario.

### Milestone 0.4.0 – Ecosystem Hardening & SDK Polish

- **FR-0401**: Publish Rust and TypeScript SDKs that expose runtime and composer extension points with examples.
- **FR-0402**: Enforce resilience features (resource quotas, structured error propagation, correlation IDs) validated through stress tests stored under `_artifacts/tests/hardening/`.

## Non-Functional Requirements

- **NFR-01**: Observability – Every prompt turn MUST emit structured tracing, logs, and JSONL artefacts stored in `_artifacts/<task>/`.
- **NFR-02**: Compliance – Documentation and artefacts MUST conform to SDD Constitution Articles III (Test-First), VII (Simplicity), and IX (Integration-First).
- **NFR-03**: Performance – Streaming latency SHOULD remain ≤150 ms in local testing; regressions require mitigation plans.
- **NFR-04**: Security – Permission escalations (sandbox/network) MUST be auditable, with danger modes opt-in only.
- **NFR-05**: Compatibility – The runtime MUST track `agent-client-protocol` release notes and flag breaking changes within two weeks.

## Dependencies & Risks

- Upstream ACP crate version ≥0.4.0 with stable APIs.
- Availability and stability of vendor CLIs (Codex, Claude, Gemini).
- IDE client API updates (Zed, VS Code, Obsidian, tldraw) that may alter ACP capabilities.
- Resource contention when running multiple subagents concurrently (requires benchmarking before milestone 0.3).

## Evidence & Traceability Expectations

- Requirements map to specs (`specs/<NNN>-<slug>/`) with explicit references to the FR/NFR identifiers above.
- Plans and tasks enumerate test strategies and composer scenarios tied to `_artifacts/<task>/` evidence bundles.
- Roadmap updates that affect requirements MUST include cross-links between this document, the relevant pre-issue notes, and GitHub Project cards.

## References

- `dev-docs/_requirements/Roadmap.md`
- `dev-docs/_issues_drafts/runtime-adoption-core-loop.md`
- `dev-docs/_issues_drafts/streaming-alignment-session-notifications.md`
- `dev-docs/_issues_drafts/protocol-cleanup-official-models.md`
- `dev-docs/architecture/acplb-architecture.md`
- `dev-docs/core_servers/acplb-core-runtime.md`
