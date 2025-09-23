# ACPLazyBridge Roadmap

This roadmap defines the staged delivery plan for ACPLazyBridge as it evolves into a protocol-compliant bridge, composer, and multi-agent hub built on the official `agent-client-protocol` Rust libraries. Each milestone lists the feature scope, acceptance gates, dependencies, and risks that must be addressed before promotion to the next release track.

## Strategic Goals

- Consolidate every ACPLazyBridge agent server on the upstream ACP runtime to eliminate protocol drift.
- Provide a reusable core that supports Codex first, then additional vendors (Claude, Gemini, etc.) with minimal duplication.
- Deliver composer capabilities (subagents, commands, hooks) that let users stitch specialised agents into coordinated workflows.
- Ship ACP clients and UX integrations (Zed, VS Code, Obsidian, tldraw) that surface bridge functionality consistently.

## Timeline Overview (targeting 2025)

| Quarter | Release | Focus |
| --- | --- | --- |
| Q3-1 2025 | 0.1.0 | Core runtime migration to official ACP libraries + Zed ↔ Codex MVP |
| Q3-2 2025 | 0.2.0 | Claude & Gemini agent servers on shared runtime + composer plugin foundation |
| Q4-1 2025 | 0.3.0 | Cross-editor ACP clients (VS Code, Obsidian, tldraw) + advanced composer workflows |
| Q4-2 2025 | 0.4.0 | Runtime hardening, multi-agent orchestration, and ecosystem SDK polish |

> Dates are directional; release readiness is gated by SDD acceptance criteria and CI evidence rather than calendar targets.

## Milestone 0.1.0 – Core Runtime & Zed ↔ Codex MVP

**Scope**

- Replace handcrafted JSON-RPC loop with `agent_client_protocol::AgentSideConnection` and Tokio `LocalSet` execution.
- Port streaming notifications to official `SessionNotification`, `ContentBlock`, `ToolCall`, and `ToolCallUpdate` types.
- Remove `acp-lazy-core::protocol` module and reuse upstream error/response models everywhere.
- Validate end-to-end with Zed’s Custom Agent client connected to the Codex CLI adapter.

**Acceptance**

- CI (`scripts/ci/run-local-ci.sh`) is green and replayed JSONL scenarios match ACP schema snapshots.
- SDD spec/plan/tasks documented for runtime migration, streaming refactor, and protocol cleanup.
- Evidence artefacts stored under `_artifacts/<task>/` (primary) with legacy mirrors retained in `_artifacts/legacy/` only when historical comparisons are required.

**Risks & Mitigations**

- _Tokio `!Send` futures_: confine ACP loop inside `LocalSet`; add integration tests that spawn notify forwarders to catch runtime regressions.
- _CLI parity_: add snapshot tests comparing Codex CLI launch arguments pre/post migration to prevent permission-mode drift.

## Milestone 0.2.0 – Multi-Agent Runtime & Composer Foundations

**Scope**

- Implement Claude and Gemini adapters on the shared runtime; reuse process transport and permission policies.
- Introduce `acplb-subagents`, `acplb-commands`, and `acplb-hooks` plugin primitives with the first shipping plugin: `acplb-subagent-translator`.
- Provide configuration contract (TOML/JSON) for registering subagents, ordering hooks, and mapping permission modes.
- Extend test harnesses to orchestrate parallel sessions and plugin pipelines.

**Acceptance**

- Claude and Gemini binaries pass the same ACP smoke scenarios as Codex, including tool-call lifecycle and cancellation tests.
- Composer plugin configured in CI to translate prompts via a subagent and reinject responses.
- Documentation updates in `dev-docs/core_servers/` and `dev-docs/architecture/` describing runtime extension points and plugin APIs.

**Risks & Mitigations**

- _Vendor CLI updates_: pin tool versions via workspace toolchain manifests; include compatibility matrix in references.
- _Plugin ordering bugs_: create deterministic plugin execution tests with golden outputs.

## Milestone 0.3.0 – Cross-Editor Clients & Advanced Composer Workflows

**Scope**

- Build ACPLazyBridge client integrations for VS Code (fork of Copilot Chat or Cline), Obsidian, and tldraw.
- Expand composer toolkit with workflow templates (e.g., planner → executor → reviewer chains) and richer telemetry.
- Add multi-session orchestration so one master agent can orchestrate subagent task queues.

**Acceptance**

- Each client extension implements ACP configuration, session lifecycle, and tool-permission UI with manual test scripts archived.
- Composer workflows documented with BPMN-like diagrams and validated through automated JSONL playbacks.
- Observability hooks (metrics/logging) emit per-session timelines for debugging.

**Risks & Mitigations**

- _Editor marketplace policies_: maintain forks in a dedicated org, track upstream changes via scheduled dependency audits.
- _User experience fragmentation_: define UX guidelines in `dev-docs/architecture/` to keep agent panels consistent.

## Milestone 0.4.0 – Ecosystem Hardening & SDK Polish

**Scope**

- Publish Rust SDK crates for building third-party ACPLazyBridge plugins and agent servers.
- Provide TypeScript bindings for composer APIs to support browser-based integrations.
- Harden runtime with resource quotas, tracing correlation IDs, and structured error propagation.

**Acceptance**

- SDK examples and tutorials available in `dev-docs/references/` with runnable code samples.
- Load testing artefacts stored under `_artifacts/reports/` showing graceful degradation and recovery strategies.

## Cross-Cutting Initiatives

- **Security**: enforce sandbox policies, redact sensitive data, and document audit trails for CLI interactions.
- **Testing**: continue expanding JSONL scenario library; add chaos tests covering notify timeouts and abrupt CLI exits.
- **Documentation**: every milestone requires refreshed SDD specs/plans/tasks plus roadmap deltas recorded here.

## Dependencies & External Signals

- agent-client-protocol crate updates (watch releases ≥0.4.0 for breaking changes).
- Provider CLI roadmaps (Codex, Claude, Gemini) for feature parity and authentication changes.
- Editor extension APIs (Zed, VS Code, Obsidian, tldraw) to ensure compatibility with UX/permission flows.

## Tracking & Governance

- Roadmap evolution is governed by the SDD Constitution; updates require spec/plan/task artefacts and adherence to the Constitution Update Checklist when touching normative documents.
- Each milestone is managed through GitHub Projects (see `dev-docs/_project_management/`) with links to specs, plans, tasks, and evidence folders.
- Deviations from scope require explicit approval recorded in `sdd-rules` change logs.
