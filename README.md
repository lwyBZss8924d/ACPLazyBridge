# ACPLazyBridge

ACP bridge for agents / agent-tools plugin Hub connects IDEs, other types of editors, etc.

## Purpose

Provide a reusable, IDE-agnostic ACP bridge that:

- Adopts Zed’s official ACP integration patterns (agent_servers/agent_ui) as best practice.
- Hosts external CLI agent adapters (Claude, Gemini, Codex, …) with a consistent capability surface.
- Ensures non-interactive approvals by default for IDEs without a UI approval flow.

## Supported Adapters

This repository implements the following adapter:

- **Codex (`codex-cli-acp`)**: A new adapter that aligns with the ACP stream, supports `tool_calls`, and enables non-interactive approvals.

Additionally, the ACPLazyBridge is designed to work with other official adapters, such as:

- **Claude (`@zed-industries/claude-code-acp`)**: Use Zed’s official adapter directly.
- **Gemini (`@google/gemini-cli`)**: Use Zed’s official adapter directly via the `--experimental-acp` flag.

## Non‑interactive approvals (recommended defaults)

To avoid stalling tool_calls in IDEs with no approval UI, map permission modes to:

- default:  approval_policy=never, sandbox_mode=read-only,      network_access=false
- plan:     approval_policy=never, sandbox_mode=read-only,      network_access=false
- acceptEdits:       approval_policy=never, sandbox_mode=workspace-write, network_access=false
- bypassPermissions: approval_policy=never, sandbox_mode=workspace-write, network_access=true

You can expose a YOLO profile (danger-full-access) as an explicit opt-in only.

## Features

- **Codex ACP Adapter (`codex-cli-acp`)**:
  - Robust handling of stdout streaming.
  - Real-time forwarding of `agent_message_chunk`.
  - Mapping for single and batched `tool_calls`.
  - Support for `notify` to complete turns and `idle` fallback.
  - Reports `promptCapabilities` on initialization.
  - Auto-injects a forwarder for immediate turn completion via external signals.
- **Shared Adapter Skeleton (`acp-lazy-core`)**:
  - Provides base utilities for spawning processes, handling handshakes, and managing streams, based on Zed's patterns.
  - Includes helpers for approval and sandbox policies.
- **Comprehensive Testing**:
  - Mocked stdout event sequences for streaming, `tool_calls`, and errors.
  - Guards against duplicate chunks and ensures reliable non-interactive turn completion.

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

## Security & Testing

### Code Quality
The project enforces strict code quality standards:
- Rust formatting with `cargo fmt`
- Linting with `cargo clippy` (all warnings are errors)
- Comprehensive unit and integration tests

### Security Analysis
Automated security scanning with CodeQL:
- Runs on every PR and push to main
- Custom queries enforce WARP protocol rules
- Results available in GitHub Security tab
- See `dev-docs/engineering/codeql.md` for details

### Testing
- Unit tests: `cargo test --workspace`
- Protocol compliance: JSONL scenario replay tests
- Integration tests: Real provider CLI interactions
- See `CONTRIBUTING.md` for complete testing guidelines

## License

MIT
