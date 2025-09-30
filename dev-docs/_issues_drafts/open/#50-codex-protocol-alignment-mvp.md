# Codex Protocol Alignment MVP (Supersedes Issue #46)

created: 2025-09-29T21:24:56Z

`[Status: Draft]`

- **Milestone**: 0.1.0 – Core Runtime & Zed ↔ Codex-CLI MVP
- **Issue URI**: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/50 _(to be created; link placeholder)_
- **SDD Artefacts Drafts**:
    - Spec: `specs/drafts-040-codex-protocol-alignment-mvp/spec.md`
    - Plan: `specs/drafts-040-codex-protocol-alignment-mvp/plan.md`
    - Tasks: `specs/drafts-040-codex-protocol-alignment-mvp/tasks.md`
- **Evidence Root**: `_artifacts/040-codex-protocol-alignment-mvp/`
- **Dependencies**:
    - Task 038 – `specs/038-adopt-acp-runtime/`
    - Task 039 – `specs/039-streaming-alignment-session-notifications/`
    - ACP crate `agent-client-protocol` v0.4.3
- **Supersedes**: `dev-docs/_issues_drafts/open/#46-protocol-cleanup-official-models.md`

## Summary

Complete the Codex adapter migration so every Codex submission, tool lifecycle event, approval flow, and plan update is represented with official ACP models. Retire the narrow scope of Issue #46 and align the milestone documentation with the broader MVP expectation captured in DeepResearch notes.

## Problem

The Codex bridge currently covers only a subset of `EventMsg` variants. Missing mappings for plan updates, apply_patch approvals, MCP calls, slash commands, and experimental JSON streams prevent ACP clients (e.g., Zed) from rendering complete workflows. Documentation still references Issue #46, which no longer reflects the necessary work.

## Why Now

Milestone 0.1.0 promises ACP fidelity for Codex, completing the runtime/streaming migrations started in Issues #44 and #45. Without this alignment the Codex adapter cannot be presented as an MVP deliverable, blocking integration with Zed and future ACP clients.

## Goals

1. Implement end-to-end ACP mappings for submissions, events, plan updates, tool calls, approvals, MCP invocations, slash commands, and notify sequences.
2. Capture submission metadata (cwd, sandbox, approval policy, model, XML annotations, @-mentions) as ACP session state and content annotations.
3. Update milestone documentation to reference Issue #50, marking Issue #46 as superseded.
4. Provide regression evidence (JSONL, schema validation, Zed smoke test logs) under `_artifacts/040-codex-protocol-alignment-mvp/`.

## Non-Goals

- Adding new Codex CLI capabilities.
- Shipping adapters for Claude or Gemini.
- Introducing persistence or background workers.

## Acceptance Criteria

- ✅ All targeted `EventMsg` variants map to ACP `SessionUpdate` payloads with correct status transitions and metadata.
- ✅ Submission context captures cwd, sandbox, approval, model, and reasoning toggles and exposes them to ACP clients.
- ✅ Apply-patch and MCP tool flows emit paired `ToolCall`/`ToolCallUpdate` notifications with raw input/output and location hints.
- ✅ Slash commands and @-mentions are surfaced to clients via `AvailableCommandsUpdate` and resource annotations.
- ✅ Proto and experimental JSON ingestion paths are covered by failing-first tests and JSONL fixtures.
- ✅ Documentation set (`Roadmap.md`, `m1-technical-implementation-plan.md`, `m1-issue-list.md`, `acp-lazybridge-requirements.md`, `acp-lazybridge-project-plan.md`, `migration-blueprint-project-management-plan.md`) updated with Issue #50 references; Issue #46 marked as superseded.
- ✅ Evidence logs stored under `_artifacts/040-codex-protocol-alignment-mvp/` for tests, lint, schema validation, and manual Zed smoke runs.

## Deliverables

- Updated Rust modules: `crates/codex-cli-acp/src/{submission_handler.rs,codex_proto.rs,tool_calls.rs}` + supporting bins
- Regression tests and JSONL fixtures proving ACP fidelity
- Revised issue draft (#50), archived #46, and synchronized requirements documentation
- Quickstart/manual validation steps for Zed ↔ Codex workflows

## Risks & Mitigations

| Risk | Mitigation |
| --- | --- |
| Missing Codex event variants | Complete event mapping inventory during research phase and add failing tests before implementation |
| Tool payload size & secrets | Implement truncation with metadata flags and redact sensitive data per Constitution Article IX |
| Documentation divergence | Update docs early in the branch and rerun `scripts/sdd/check-sdd-consistency.sh` before PR |
| Experimental JSON drift | Keep feature behind config flag; add regression tests for both outputs |

## "ACP" and "Codex-CLI" for "ACPLazyBridge" Dev references

### (Codex-CLI)

<"Codex-CLI" for "ACPLazyBridge" Dev references repo path>

- (/OpenAI/codex/)
- (/OpenAI/codex/docs/)
- (/OpenAI/codex/codex-cli/)
- (/OpenAI/codex/codex-rs/)

</"Codex-CLI" for "ACPLazyBridge" Dev references repo path>

- <https://github.com/openai/codex/tree/main/codex-rs>
- "headless" CLI for use in automation <https://github.com/openai/codex/tree/main/codex-rs/exec/src>
- CLI multitool that provides the aforementioned CLIs via subcommands:<https://github.com/openai/codex/tree/main/codex-rs/cli>
- TODO lists
- @file path for workspace context inptu
- Images
- Tool calls event
- Approval Following event and user input approval actions (Codex CLI's model requires human approval before executing a command)
- Interactive (and background) terminals
- Codex CLI Built-in Slash commands and Custom [Slash commands]
- Client MCP servers

### (ACP)

Cargo.toml

```toml
[dependencies]
agent-client-protocol = { git = "https://github.com/zed-industries/agent-client-protocol" }
```

<"ACP and ACP Rust library" for "ACPLazyBridge" Dev references repo path>

- (/Zed-Industries/agent-client-protocol/)
- (/Zed-Industries/agent-client-protocol/docs/protocol/)
- (/Zed-Industries/agent-client-protocol/schema/schema.json)
- (/Zed-Industries/agent-client-protocol/schema/meta.json)
- (/Zed-Industries/agent-client-protocol/rust/)

</"ACP and ACP Rust library" for "ACPLazyBridge" Dev references repo path>

- <https://agentclientprotocol.com/overview/architecture>
- <https://github.com/zed-industries/agent-client-protocol/blob/main/schema/meta.json>
- <https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json>
Protocol
- [**Overview**](https://agentclientprotocol.com/protocol/overview)
- [Initialization](https://agentclientprotocol.com/protocol/initialization)
- [Session Setup](https://agentclientprotocol.com/protocol/session-setup)
- [Prompt Turn](https://agentclientprotocol.com/protocol/prompt-turn)
- [Content](https://agentclientprotocol.com/protocol/content)
- [Tool Calls](https://agentclientprotocol.com/protocol/tool-calls)
- [File System](https://agentclientprotocol.com/protocol/file-system)
- [Terminals](https://agentclientprotocol.com/protocol/terminals)
- [Agent Plan](https://agentclientprotocol.com/protocol/agent-plan)
- [Session Modes](https://agentclientprotocol.com/protocol/session-modes)
- [Slash Commands](https://agentclientprotocol.com/protocol/slash-commands)
- [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
- [Schema](https://agentclientprotocol.com/protocol/schema)

## Next Actions

1. Create worktree (Issue #50)-(Specs #040): `../acplb-worktrees/040-codex-protocol-alignment-mvp` from `origin/main`.
2. follow Issue #50 DeepResearch first, Run research and design phases to finalise mapping tables and regression scenarios. then Start SDD Tasks (specs/040-codex-protocol-alignment-mvp/) Initialize Phase workflow Create the Specification Documents.
3. Execute Dev Cooking Phase Workflow follow `/specify`, `/plan`, and `/tasks` etc.
4. Implement submission/event/tool updates with TDD discipline; capture evidence.
5. Update documentation and archive Issue #46 references before local pre-PR Validation SDD workflow Phase.

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
