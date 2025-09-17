# ACPLazyBridge

ACP bridge for agents / agent-tools plugin Hub connects IDEs, other types of editors, etc.

## ACP (Agent Client Protocol) Protocol

- Agent Client Protocol: <https://github.com/zed-industries/agent-client-protocol>
- ACP JSON Schema: <https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json>

> The Agent Client Protocol (ACP) standardizes communication between code editors (interactive programs for viewing and editing source code) and coding agents (programs that use generative AI to autonomously modify code).
> The protocol is still under heavy development, and we aim to mature it as we get confidence in the design by implementing it in various settings.

## Purpose

Governance: .specify/memory/constitution.md (normative; English-only)

Provide a reusable, IDE-agnostic ACP bridge that:

- Adopts Zed’s official ACP integration patterns (agent_servers/agent_ui) as best practice.
- Hosts external CLI agent adapters (Claude, Gemini, Codex, …) with a consistent capability surface.
- Ensures non-interactive approvals by default for IDEs without a UI approval flow.

## Planned adapters

- @zed-industries/claude-code-acp (use Zed’s official adapter directly)
- @google/gemini-cli (use Zed’s official adapter directly via --experimental-acp)
- @zed-industries/codex-cli-acp (new adapter implemented here; aligns ACP stream, tool_calls, and non-interactive approvals)

## Non‑interactive approvals (recommended defaults)

To avoid stalling tool_calls in IDEs with no approval UI, map permission modes to:

- default:  approval_policy=never, sandbox_mode=read-only,      network_access=false
- plan:     approval_policy=never, sandbox_mode=read-only,      network_access=false
- acceptEdits:       approval_policy=never, sandbox_mode=workspace-write, network_access=false
- bypassPermissions: approval_policy=never, sandbox_mode=workspace-write, network_access=true

You can expose a YOLO profile (danger-full-access) as an explicit opt-in only.

## Roadmap (high level)

1. Codex ACP adapter (codex-cli-acp)

- Stream: robust stdout line queue; forward agent_message_delta as agent_message_chunk in real time
- Tooling: map single + batched tool_calls; improve titles/kinds; show stdout preview in tool_call_update
- Turn completion: prefer notify agent-turn-complete; idle fallback only
- Capabilities: return promptCapabilities in initialize
- Notify integration: Auto-inject forwarder for immediate turn completion via external signals

1. Shared adapter skeleton

- Provide base spawn/handshake/stream utilities (based on Zed’s agent_servers patterns)
- Add approval policy + sandbox policy bridge helpers

1. Tests & smoke tools

- Mocked stdout event sequences (streaming + tool_calls + errors)
- Non-interactive turn completion & duplicate-chunk guards

## Configuration

### Notify Integration

The adapter supports external notification signals for immediate turn completion:

#### Environment Variables

- `ACPLB_NOTIFY_PATH`: Path to notify sink file/FIFO (enables notify integration)
- `ACPLB_NOTIFY_KIND`: Type of sink - `file` or `fifo` (default: `file`)
- `ACPLB_NOTIFY_INJECT`: Auto-injection policy - `auto`, `never`, or `force` (default: `auto`)
    - `auto`: Inject forwarder if no custom command is provided
    - `never`: Never inject forwarder, respect Codex config
    - `force`: Always inject forwarder, override Codex config
- `ACPLB_NOTIFY_CMD`: Custom notify command array in JSON format (overrides forwarder)
- `ACPLB_IDLE_TIMEOUT_MS`: Idle timeout in milliseconds (default: 1200)
- `ACPLB_POLLING_INTERVAL_MS`: Polling interval for timeout checks (default: 100)

#### How It Works

1. When `ACPLB_NOTIFY_PATH` is set, the adapter monitors the specified sink for notifications
2. In `auto` mode, it injects `acplb-notify-forwarder` to forward Codex notifications to the sink
3. On receiving `agent-turn-complete` notification, the turn ends immediately
4. Idle timeout serves as a fallback if no completion signal is received

#### Example Usage

```bash
# Use file-based notify sink
export ACPLB_NOTIFY_PATH=/tmp/codex-notify.jsonl
export ACPLB_NOTIFY_KIND=file
cargo run -p codex-cli-acp

# Use FIFO for real-time notifications
mkfifo /tmp/codex-notify.fifo
export ACPLB_NOTIFY_PATH=/tmp/codex-notify.fifo
export ACPLB_NOTIFY_KIND=fifo
cargo run -p codex-cli-acp

# Disable auto-injection if using custom Codex notify
export ACPLB_NOTIFY_INJECT=never
```

---

## License

MIT
