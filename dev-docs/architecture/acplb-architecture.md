# ACPLazyBridge Architecture

**Status**: Draft – pending validation via Milestone 0.1.0 specs

## Purpose

This document defines the target architecture for ACPLazyBridge after migrating to the official `agent-client-protocol` (ACP) Rust libraries. It should be referenced whenever we author new specs, plans, or tasks touching the bridge runtime, composer plugins, or client integrations.

## Architectural Principles

1. **Protocol Fidelity** – The ACP crate is the single source of truth for JSON-RPC messaging, error handling, and content schemas. ACPLazyBridge does not fork or duplicate protocol structures.
2. **Composable Runtime** – All agent servers (Codex, Claude, Gemini, etc.) share the same runtime scaffolding: Tokio `LocalSet`, `AgentSideConnection`, permission translators, and notify handling.
3. **Integration-First** – Composer plugins, subagents, and clients consume the same ACP contracts as the core runtime. No internal-only message formats are introduced.
4. **Observability by Default** – Structured tracing, metrics, and artefact capture (JSONL transcripts, logs) are mandatory for every prompt turn.

## Logical View

```text
+-----------------+       +-----------------+       +------------------+
| ACP Clients     | <---> | ACPLazyBridge   | <---> | ACP Agent Servers |
| (Zed, VS Code,  |       | Core Runtime    |       | (Codex, Claude,   |
|  Obsidian, ...) |       |                 |       |  Gemini, custom)  |
+-----------------+       +-----------------+       +------------------+
                                 |
                                 v
                        +------------------+
                        | Composer Plugins |
                        | (subagents,      |
                        |  commands, hooks)|
                        +------------------+
```

- **Client Layer** – Implements ACP client semantics, including permission prompts, session lifecycle, and UI rendering.
- **Core Runtime** – Hosts `AgentSideConnection`, manages sessions, permission mapping, processing pipelines, idle timeouts, and notify sinks.
- **Agent Server Adapters** – Encapsulate provider-specific process management and translate provider events into ACP notifications.
- **Composer Plugins** – Optional pipelines that intercept inbound/outbound session traffic to add translation, planning, or delegation features.

## Deployment View

- Each agent server binary runs as a standalone process (e.g., `codex-cli-acp`, `claude-cli-acp`).
- Composer plugins run inside the core runtime process and may spawn sub-sessions to other agent servers through ACP.
- Clients connect over stdio (CLI) or TCP/IPC (future enhancement) using ACP JSONL streams.

## Runtime View (Prompt Turn)

1. Client sends `session/prompt` via ACP.
2. Core runtime invokes provider adapter through shared `Agent` implementation.
3. Provider emits events (messages, tool calls) captured via `AgentSideConnection::session_notification`.
4. Composer plugins may transform or branch the stream (e.g., translator subagent, command triggers).
5. Core runtime forwards structured notifications to client and records artefacts.
6. Stop reason determined by notify event, idle timeout, or provider completion.

## Extension Points

- **Agent Abstractions**: Implement `acplb_core::agents::ProviderAgent` trait to plug new providers into runtime.
- **Composer Plugins**: Implement `acplb_core::plugins::Plugin` trait with `on_user_prompt`, `on_agent_update`, and `on_tool_invocation` hooks.
- **Permission Policies**: Policy table mapping ACP permission modes to CLI overrides is centralised in `acplb_core::permissions`.

## Cross-Cutting Concerns

- **Security**: sandbox defaults are read-only; network disabled unless permission mode escalates.
- **Testing**: JSONL scenario catalogue replicated per provider; plugin pipelines validated through deterministic fixtures.
- **Observability**: tracing spans per session and per plugin; persisted logs under `_artifacts/logs/<task>/`.

## Roadmap Alignment

- **Milestone 0.1.0** – Core runtime + Codex adapter.
- **Milestone 0.2.0** – Additional provider adapters + composer plugin scaffolding.
- **Milestone 0.3.0** – Multi-client support + advanced workflow orchestration.
- **Milestone 0.4.0** – SDK distribution and hardened orchestration.

## Related Documents

- `dev-docs/core_servers/acplb-core-runtime.md`
- `dev-docs/_requirements/Roadmap.md`
