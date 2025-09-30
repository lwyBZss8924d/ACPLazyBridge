# ACPLazyBridge

ACP-LazyBridge Bridge for Agents / Agent-Tools and Extensions, Hub connects IDEs, other types of Editors Workspaces UI, etc.

## About ACP (Agent Client Protocol)

- Agent Client Protocol: [ACP](https://agentclientprotocol.com/overview/introduction)
- ACP JSON Schema: <https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json>

> The Agent Client Protocol (ACP) standardizes communication between code editors (interactive programs for viewing and editing source code) and coding agents (programs that use generative AI to autonomously modify code).

## What is ACP-LazyBridge? (IMAGINE)

```txt

┌──────────────────────┐                                        ┌───────────────────────────┐
│       Clients        │ ◀─────────────── ACP ────────────────▶ │       ACPLB‑Hub           │
│  Agent Panel (Zed/   │              (JSON‑RPC)                │                           │
│  VS Code/Obsidian/   │                                        │ ┌───────────────────────┐ │
│  tldraw/ …)          │                                        │ │ Runtime (ACP v1)      │ │
└──────────────────────┘                                        │ │ - agent_client_       │ │
                                                                │ │   protocol types      │ │
┌──────────────────────┐                                        │ │ - transport: stdio    │ │
│   Multi‑Base Agents  │ ◀────── Main-Agent & Sub-Agents ─────▶ │ │ - session/update      │ │
│  (planner/executor/  │             (Turns / Steps)            │ └───────────────────────┘ │
│   reviewer …)        │                                        │ ┌───────────────────────┐ │
└──────────────────────┘                                        │ │ Adapters & Sandbox    │ │
                                                                │ │ - codex-cli-acp       │ │
┌──────────────────────┐                                        │ │ - claude-code-acp*    │ │
│   Augmented Tooling  │ ◀──────────── Tool Calls ────────────▶ │ │ - gemini-cli-acp*     │ │
│ (search/edit/exec/   │ (FC / Bash / PyScript / MCP-Proxy ...) │ └───────────────────────┘ │
│  fetch/other …)      │                                        │ ┌───────────────────────┐ │
└──────────────────────┘                                        │ │ Extensions            │ │
                                                                │ │ (commands/workflows/  │ │
                                                                │ │  hooks/pipelines)     │ │
                                                                │ └───────────────────────┘ │
                                                                └───────────────────────────┘

Notes: any Workspaces AI Pilot powered by any Coding Base Agent.

                    Figure 1: ACP‑LazyBridge IMAGINE Architecture
```

## Planned Agents Adapters

- **Codex CLI** @lwyBZss8924d/ACPLazyBridge `acp-lazybridge/codex-cli-acp` Agents Adapter 🚧
- **Gemini CLI** Integration: @google-gemini/gemini-cli/tree/main/packages/cli/src/zed-integration (Integration: Zed’s official adapter directly via 'experimental-acp') Agents Adapter
- **Claude Code** Integration: @zed-industries/claude-code-acp ACP adapter Agents Adapter

## Planned Agent Clients

- Zed
- VS Code
- Obsidian
- Tldraw

## Roadmap

For the canonical plan and acceptance criteria, see `dev-docs/_requirements/Roadmap.md`. The highlights below track the active SDD scope.

### Strategic Goals

- Unify all ACPLazyBridge adapters on the upstream `agent-client-protocol` runtime to prevent protocol drift.
- Deliver Codex first, then Claude and Gemini on a shared transport and permission model.
- Grow composer capabilities (subagents, commands, hooks) so IDEs can orchestrate specialised workflows.
- Ship editor integrations (Zed, VS Code, Obsidian, tldraw) that expose a consistent ACP experience.

### Milestone

| Quarter | Release | Focus |
| --- | --- | --- |
| Q3-1 2025 | 0.1.0 | 🚧 (ACPLazyBridge) First Release `acp-lazybridge/codex-cli-acp` Core runtime migration to official ACP libraries + Zed ↔ Codex-CLI MVP |
| Q3-2 2025 | 0.2.0 | Claude & Gemini agent servers on shared runtime + composer plugin foundation |
| Q4-1 2025 | 0.3.0 | Cross-editor ACP clients (VS Code, Obsidian, tldraw) + advanced composer workflows |
| Q4-2 2025 | 0.4.0 | Runtime hardening, multi-agent orchestration, and ecosystem SDK polish |

## Milestone 0.1.0 – Core Runtime & Zed ↔ Codex-CLI MVP `acp-lazybridge/codex-cli-acp`

**Scope**

- ✅ **Completed**: Replace handcrafted JSON-RPC loop with `agent_client_protocol::AgentSideConnection` and Tokio `LocalSet` execution (SDD Task 038, PR #47).
- ✅ **Completed**: Port streaming notifications to official `SessionNotification`, `ContentBlock`, `ToolCall`, and `ToolCallUpdate` types (Issue #45, Phase 4).
- 🔄 **In Progress**: Complete Codex protocol alignment for the MVP, Enhanced Implementation Plan for ACPLazyBridge `acp-lazybridge/codex-cli-acp`, covering submission metadata, tool lifecycle, approvals, and slash commands with official ACP models (Issue #50, supersedes Issue #46).
- ✅ **Completed**: Validate end-to-end with Zed's Custom Agent client connected to the Codex CLI adapter.

---

## License

MIT
