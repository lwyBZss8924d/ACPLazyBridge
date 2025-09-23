# ACPLazyBridge Core Runtime Design

**Status**: Draft – foundation for Milestone 0.1.0 SDD tasks

## Objectives

- Provide a reusable runtime crate that wires the official `agent-client-protocol` crate into ACPLazyBridge.
- Centralise session state, permission mapping, process transport, and notify handling for all provider adapters.
- Offer extension hooks for composer plugins without leaking provider-specific logic into the core.

## Key Components

| Component | Responsibility |
| --- | --- |
| `runtime::Server` | Owns Tokio `LocalSet`, manages `AgentSideConnection` lifecycle, and exposes APIs for prompt handling and cancellation. |
| `sessions::Store` | Tracks active sessions, permission modes, working directories, notify subscriptions, and child process handles. |
| `transport::Process` | Wrapper around spawn/kill/read/write operations for provider CLIs (Codex, Claude, Gemini, etc.). |
| `permissions::Mapper` | Maps ACP `PermissionMode` ↔ CLI overrides (`approval_policy`, `sandbox_mode`, `network_access`). |
| `plugins::Pipeline` | Executes configured composer plugins in deterministic order on inbound/outbound events. |
| `telemetry::Recorder` | Streams tracing spans, JSONL transcripts, and structured logs to `_artifacts/`. |

## Tokio Execution Model

- The runtime owns a dedicated `LocalSet` to host ACP futures (`!Send`).
- Provider process IO runs on spawned tasks (either `spawn_local` or `spawn_blocking` for heavy work).
- Cancellation relies on `AgentSideConnection::subscribe()` to consume client-side notifications promptly.

## Session Lifecycle

1. `initialize` negotiates protocol version, capabilities, and auth methods; persisted in `ServerState`.
2. `new_session` validates absolute `cwd`, records permission mode, and prepares notify sink if configured.
3. `prompt` pipeline:
   - Acquire session entry; tear down any existing child process.
   - Build CLI arguments from permission mapper; spawn process via `transport::Process::spawn`.
   - Register stdout reader task that forwards lines to provider-specific decoder (e.g., Codex stream parser) which returns ACP `SessionNotification` instances.
   - Pass notifications through `plugins::Pipeline` before sending via `AgentSideConnection::session_notification`.
   - Monitor notify sink events and idle timeout to determine stop reason.
4. `cancel` kills the child process, emits cancellation updates, and resolves outstanding `prompt` future with `StopReason::Cancelled`.

## Plugin Pipeline

- Configuration format: `composer.toml` (draft) lists ordered plugin IDs with parameters.
- Hook contract:
    - `on_prompt(request: &mut PromptRequest)` – mutate user prompt before provider receives it.
    - `on_session_update(update: &mut SessionNotification)` – enrich or filter provider updates.
    - `on_tool_call(tool: &mut ToolCallUpdate)` – adjust tool metadata (e.g., add raw stdout snippets).
- Error handling: plugin failures raise structured errors logged to telemetry and optionally surfaced as ACP errors based on severity.

## Provider Adapter Interface

```rust
#[async_trait::async_trait(?Send)]
pub trait ProviderAdapter {
    async fn spawn(&self, session: &SessionContext) -> Result<ProviderHandle>;
    async fn decode_stream(
        &self,
        handle: &mut ProviderHandle,
        dispatcher: &mut dyn NotificationSink,
    ) -> Result<StopReason>;
}
```

- `ProviderHandle` wraps `ProcessTransport` plus provider-specific metadata.
- `NotificationSink` abstracts ACP notifications so adapters remain unaware of plugins or telemetry wiring.

## Observability Plan

- Tracing spans: `session.initialize`, `session.prompt`, `session.tool_call`, `plugin.<name>`.
- Metrics: per-session duration, tool-call counts, plugin latency, CLI restarts.
- Artefacts: JSONL transcripts stored under `_artifacts/logs/<task>/<session>.jsonl` and summarised in review evidence.

## Security Posture

- Default sandbox: read-only filesystem, no network access.
- Escalations recorded in telemetry and optionally require manual approval via future permission prompts.
- Secrets: CLI credentials supplied via environment variables managed outside repository; never logged.

## Testing Strategy

- Unit tests for permission mapper, notify pipeline, and plugin ordering.
- Integration tests using fake provider adapters to simulate stdout sequences, errors, and cancellations.
- Scenario replays for real CLIs recorded in `_artifacts/tests/` and executed via `cargo run --bin playback`.

## Open Questions

- Do we promote the core runtime as a separate crate (`acplb-core`), or keep it internal to the workspace until APIs stabilise?
- Which storage backend (if any) is needed for session persistence beyond process lifetime?
- How will we expose configuration hot-reload for composer plugins in long-running daemon deployments?

## References

- `dev-docs/architecture/acplb-architecture.md`
- `dev-docs/_requirements/Roadmap.md`
- `agent-client-protocol` crate docs (`~/dev-space/agent-client-protocol/rust`)
