# ACPLazyBridge

ACP bridge for agents / agent-tools plugin Hub connects IDEs, other types of editors, etc.

## ACP (Agent Client Protocol) Protocol

- Agent Client Protocol: <https://github.com/zed-industries/agent-client-protocol>
- ACP JSON Schema: <https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json>

> The Agent Client Protocol (ACP) standardizes communication between code editors (interactive programs for viewing and editing source code) and coding agents (programs that use generative AI to autonomously modify code).

## Dev Purpose

Governance: .specify/memory/constitution.md (normative; English-only)

Provide a reusable, IDE-agnostic ACP bridge that:

- Adopts Zed’s official ACP integration patterns (agent_servers/agent_ui) as best practice.
- Hosts external CLI agent adapters (Claude, Gemini, Codex, …) with a consistent capability surface.
- Ensures non-interactive approvals by default for IDEs without a UI approval flow.

## Planned adapters

- @zed-industries/claude-code-acp (use Zed’s official adapter directly)
- @google/gemini-cli (use Zed’s official adapter directly via --experimental-acp)
- @zed-industries/codex-cli-acp (new adapter implemented here; aligns ACP stream, tool_calls, and non-interactive approvals)

## Planned Agent Clients

- Zed
- VS Code
- Obsidian
- Tldraw

## Non‑interactive approvals (recommended defaults)

To avoid stalling tool_calls in IDEs with no approval UI, map permission modes to:

- default:  approval_policy=never, sandbox_mode=read-only,      network_access=false
- plan:     approval_policy=never, sandbox_mode=read-only,      network_access=false
- acceptEdits:       approval_policy=never, sandbox_mode=workspace-write, network_access=false
- bypassPermissions: approval_policy=never, sandbox_mode=workspace-write, network_access=true

You can expose a YOLO profile (danger-full-access) as an explicit opt-in only.

## Roadmap (2025 focus)

For the canonical plan and acceptance criteria, see `dev-docs/_requirements/Roadmap.md`. The highlights below track the active SDD scope.

### Strategic Goals

- Unify all ACPLazyBridge adapters on the upstream `agent-client-protocol` runtime to prevent protocol drift.
- Deliver Codex first, then Claude and Gemini on a shared transport and permission model.
- Grow composer capabilities (subagents, commands, hooks) so IDEs can orchestrate specialised workflows.
- Ship editor integrations (Zed, VS Code, Obsidian, tldraw) that expose a consistent ACP experience.

### Timeline Targets (directional)

| Quarter | Release | Focus |
| --- | --- | --- |
| Q3-1 2025 | 0.1.0 | Core runtime migration to official ACP libraries + Zed ↔ Codex MVP |
| Q3-2 2025 | 0.2.0 | Claude & Gemini adapters on shared runtime + composer plugin foundation |
| Q4-1 2025 | 0.3.0 | Cross-editor ACP clients (VS Code, Obsidian, tldraw) + advanced composer workflows |
| Q4-2 2025 | 0.4.0 | Runtime hardening, multi-agent orchestration, and SDK polish |

> Release readiness is gated by SDD evidence, not calendar dates.

### Milestone Snapshots

- **0.1.0 – Core Runtime & Zed ↔ Codex MVP (current)**
    - ✅ Migrated runtime to `agent_client_protocol::AgentSideConnection` (SDD Task 038 / PR #47).
    - ✅ E2E validated with Zed Custom Agent + Codex adapter; CI + JSONL baselines are green.
    - 🔄 Remaining follow-up: port streaming notifications to upstream types and deprecate local protocol models.
- **0.2.0 – Multi-Agent Runtime & Composer Foundations**
    - Ship Claude and Gemini binaries on the shared runtime.
    - Introduce `acplb-subagents`, `acplb-commands`, and composer configuration contracts.
    - Extend harnesses for parallel sessions and plugin pipelines.
- **0.3.0 – Cross-Editor Clients & Advanced Composer Workflows**
    - Build ACP clients for VS Code, Obsidian, and tldraw.
    - Deliver reusable workflow templates (planner → executor → reviewer) with telemetry.
    - Add multi-session orchestration with evidence captured via JSONL playbacks.
- **0.4.0 – Ecosystem Hardening & SDK Polish**
    - Publish Rust/TypeScript SDKs for bridge plugins and third-party agent servers.
    - Harden runtime (quotas, tracing correlation, structured errors) and document load-testing artefacts.

---

## License

MIT
