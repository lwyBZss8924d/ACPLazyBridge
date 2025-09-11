# CLAUDE.md (AGENTS.md)

Team Development Workflow for Claude Code
- Source of tasks: dev-docs/plan/issues/* (issue-list with design, references, acceptance criteria)
- Start point: always from origin/main
- Worktree-first: create a new worktree and feature branch per task
  - Container path (required): /Users/arthur/dev-space/acplb-worktrees/<task-dir>
  - Command:
    - git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/<task-dir> origin/main -b feature/<slug>
  - Optional symlink for IDE navigation:
    - ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>
- Quality gates: cargo fmt --all -- --check; cargo clippy --workspace --all-targets --all-features -- -D warnings; cargo test --workspace --all-features --locked
- Protocol discipline: logs to stderr only; stdout must be strict JSONL
- Evidence: commit/run JSONL scenarios under dev-docs/review/_artifacts/tests and provide outputs/logs for review
- PR: link to the issue, explain design, include evidence, and use squash merge after approval

This Development guide file offers instructions for any AI Developer coding agent. Such as *Claude Code (claude.ai/code)* working on this repository project High-Level Concept Overview & Software Development Team AI Developer Member Collaboration Global Standards. AI Developer coding agents team members include:

- **CLAUDE** "Claude Code Agent" (anthropic Claude Code CLI client link - `CLAUDE.md` )
- **WARP** "WARP Agent" (WARP.dev Terminal link - `WARP.md` )
- **GEMINI** "GEMINI CLI Agent" (google gemini-cli link - `GEMINI.md` )
- **CURSOR** "Cursor Agent" (Cursor IDE cursor-agent link - cursor rules file `.cursorrules` )
- **CODEX** "CODEX Agent" (OpenAI codex-cli link - `AGENTS.md` )

All AI Developers coding agent's client rules in this codebase ~/ root file are uniformly linked to /AGENTS.md

## Overview

ACPLazyBridge is an IDE-agnostic Agent Client Protocol (ACP) bridge that provides unified adapter implementations for various AI agents (Claude, Gemini, Codex). The project uses Rust and implements ACP over stdio with line-separated JSON (JSONL), featuring streaming support, tool calls, permission mapping, and a plugin system.

⚠️ ACPLazyBridge related interface design & implementation must strictly follow ACP specifications & check Codex CLI parameters!
**ACP-DocsAndSourceCodeReference**: [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)

## Prerequisites

- Rust toolchain (stable) via rustup
- cargo, rustfmt, clippy
- Optional: Codex CLI, Claude Code ACP, or Gemini CLI for testing actual agent integrations

## Common Development Commands

### Build
```bash
# Build entire workspace
cargo build --workspace

# Build with all features
cargo build --workspace --all-features

# Build specific crate
cargo build -p codex-cli-acp
cargo build -p acp-lazy-core
```

### Test
```bash
# Run all tests in workspace
cargo test --workspace --all-targets

# Run tests with all features
cargo test --workspace --all-targets --all-features

# Run tests for specific crate
cargo test -p acp-lazy-core
```

### Code Quality
```bash
# Format all code
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check

# Run clippy linter
cargo clippy --workspace --all-targets -- -D warnings

# Quick check without building
cargo check --workspace

# Run CodeQL security analysis locally (optional, runs in CI)
# See dev-docs/engineering/codeql.md for setup and custom queries
```

### Documentation
```bash
# Build documentation
cargo doc --workspace --no-deps

# Build and open docs in browser
cargo doc --workspace --no-deps --open
```

### Running the Codex Adapter
```bash
# Run with default settings (skeleton currently)
cargo run -p codex-cli-acp

# Run with verbose logging
RUST_LOG=info cargo run -p codex-cli-acp

# Debug mode with backtrace
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -p codex-cli-acp
```

### Testing ACP Protocol Compliance
```bash
# Test basic ACP handshake with Codex proto
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | codex proto

# Test with JSONL file containing multiple messages
cat test/acp_messages.jsonl | codex proto -c approval_policy="never"

# Test with specific permission modes
codex proto -c sandbox_mode="read-only" < test/readonly_session.jsonl
codex proto -c sandbox_mode="workspace-write" < test/edit_session.jsonl

# Debug protocol messages
RUST_LOG=trace codex proto 2>&1 | tee proto_debug.log

# Validate JSON-RPC responses
codex proto < test/requests.jsonl | jq -c 'select(.jsonrpc == "2.0")'
```

### Spawning Codex for Integration Testing
```bash
# Spawn Codex proto for manual testing
mkfifo request.pipe response.pipe
codex proto < request.pipe > response.pipe &
CODEX_PID=$!

# Send test messages
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' > request.pipe
cat response.pipe

# Clean up
kill $CODEX_PID
rm request.pipe response.pipe
```

### Benchmarking and Performance Testing
```bash
# Measure initialization time
time echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | codex proto

# Test streaming performance
cargo test --package codex-cli-acp --test streaming_benchmark --release

# Profile memory usage
valgrind --tool=massif cargo run -p codex-cli-acp
```

## ACP Protocol Specification

### Protocol Overview
The Agent Client Protocol (ACP) follows the JSON-RPC 2.0 specification with two message types:
- **Methods**: Request-response pairs expecting a result or error
- **Notifications**: One-way messages without expected responses

### Message Flow Sequence
1. **Initialization Phase**
   - Client → Agent: `initialize` request with capabilities
   - Agent → Client: `initialize` response with server capabilities
   - Client → Agent: `authenticate` (if required by agent)

2. **Session Setup**
   - Client → Agent: `session/new` to create new session
   - OR Client → Agent: `session/load` to resume existing session (optional)
   - Agent → Client: Response with session_id

3. **Prompt Turn**
   - Client → Agent: `session/prompt` with user message
   - Agent → Client: `session/update` notifications for progress
   - Agent → Client: Tool call requests as needed
   - Client → Agent: `session/cancel` to interrupt (optional)
   - Agent → Client: `session/prompt` response with stop reason

### Agent Methods (Server-side)

#### Baseline Methods (Required)
- `initialize`: Negotiate versions and exchange capabilities
- `authenticate`: Authenticate with the agent (if required)
- `session/new`: Create a new conversation session
- `session/prompt`: Send user prompts to the agent

#### Optional Methods
- `session/load`: Load an existing session (requires `loadSession` capability)

#### Notifications
- `session/cancel`: Cancel ongoing operations (no response expected)

### Client Methods (IDE-side)

#### Baseline Methods (Required)
- `session/request_permission`: Request user authorization for tool calls

#### Optional Methods
- `fs/read_text_file`: Read file contents (requires `fs.readTextFile` capability)
- `fs/write_text_file`: Write file contents (requires `fs.writeTextFile` capability)

#### Notifications
- `session/update`: Send progress updates during prompt processing

### Protocol Requirements
- All file paths MUST be absolute
- Line numbers are 1-based
- Messages use JSON-RPC 2.0 format
- Communication via stdio with line-separated JSON (JSONL)

## Codex CLI Integration Parameters

### Running Codex in Proto Mode
The Codex CLI supports ACP-compatible communication via the `proto` command:

```bash
# Basic proto mode for stdio communication
codex proto

# With configuration overrides
codex proto -c model="openai/gpt-5" -c sandbox_mode="workspace-write"

# With full parameter control
codex proto \
  -c approval_policy="never" \
  -c sandbox_mode="workspace-write" \
  -c 'sandbox_permissions=["disk-full-read-access"]'
```

### Codex CLI Parameters for ACP Integration

#### Sandbox Modes (`--sandbox` or `-s`)
- `read-only`: Read-only access to files, no execution
- `workspace-write`: Read/write in workspace, command execution allowed
- `danger-full-access`: Unrestricted access (use with extreme caution)

#### Approval Policies (`--ask-for-approval` or `-a`)
- `untrusted`: Only run trusted commands without approval
- `on-failure`: Run all commands, ask only on failure
- `on-request`: Model decides when to ask for approval
- `never`: Never ask for approval, return failures immediately

#### Convenience Flags
- `--full-auto`: Equivalent to `-a on-failure --sandbox workspace-write`
- `--dangerously-bypass-approvals-and-sandbox` (alias: `--yolo`): No sandbox, no prompts

#### Configuration Override (`-c`)
Override config.toml values using dotted paths:
```bash
# Model selection
-c model="openai/gpt-5"

# Sandbox permissions array
-c 'sandbox_permissions=["disk-full-read-access"]'

# Nested configuration
-c shell_environment_policy.inherit=all

# Network access in workspace-write mode
-c sandbox_workspace_write.network_access=true
```

### Platform-Specific Sandbox Implementation
- **macOS 12+**: Uses Apple Seatbelt with `sandbox-exec`
- **Linux**: Uses Landlock/seccomp APIs
- **Docker/Containers**: May require `--dangerously-bypass-approvals-and-sandbox` if Landlock unavailable

## Architecture

### Workspace Layout

- **`crates/acp-lazy-core/`**: Core library with shared utilities
  - `permissions`: Maps ACP permission modes to non-interactive Codex overrides
  - `transport`: Provides spawn/stdio utilities, line-based JSON reading, and writing
  - `logging`: Tracing initialization and configuration

- **`crates/codex-cli-acp/`**: Codex native adapter implementation (binary)
  - Implements ACP server over stdio
  - Handles initialize/new_session/prompt/cancel requests
  - Streams agent responses and tool calls

- **`local_refs/`**: Vendored references from Zed
  - `zed-acp-examples/`: Reference implementations
  - `codex/`: Codex documentation
  - `agent-client-protocol/`: ACP specification

- **`dev-docs/`**: Development documentation (Chinese)
  - `requirements/`: Project requirements
  - `design/`: Architecture and design decisions
  - `plan/`: Implementation roadmap

### Core Concepts

#### ACP Protocol Flow
1. **Initialize**: Client sends initialization request, server responds with capabilities
2. **New Session**: Client requests new session with working directory and MCP servers
3. **Prompt**: Client sends prompt, server streams responses via agent_message_chunk events
4. **Tool Calls**: Server emits tool_call events (pending → completed) with results
5. **Turn Completion**: Server sends notify("agent-turn-complete") or uses idle fallback

#### Permission Mapping (Non-Interactive by Default)
The system maps ACP permission modes to Codex CLI parameters to avoid UI approval prompts:

##### ACP Mode → Codex Parameters Mapping
| ACP Permission Mode | Codex Approval Policy | Codex Sandbox Mode | Network Access | Codex CLI Flags |
|-------------------|---------------------|-------------------|---------------|-----------------|
| `default` / `plan` | `never` | `read-only` | `false` | `codex proto -c approval_policy="never" -c sandbox_mode="read-only"` |
| `acceptEdits` | `never` | `workspace-write` | `false` | `codex proto -c approval_policy="never" -c sandbox_mode="workspace-write"` |
| `bypassPermissions` | `never` | `workspace-write` | `true` | `codex proto -c approval_policy="never" -c sandbox_mode="workspace-write" -c sandbox_workspace_write.network_access=true` |
| `YOLO` / `danger-full-access` | `never` | `danger-full-access` | `true` | `codex proto --dangerously-bypass-approvals-and-sandbox` |

##### Codex-Specific Permission Details
- **Trusted Commands**: Commands like `ls`, `cat`, `sed` run without approval in `untrusted` mode
- **Workspace Scope**: Includes current directory and `/tmp` by default
- **Network Control**: Disabled by default in `workspace-write`, requires explicit config
- **Escalation**: In `on-failure` mode, failures trigger approval requests for unsandboxed retry

#### Streaming Implementation
- Line-based JSON reading with queue processing
- De-duplication guards prevent repeated final chunks
- Idle fallback (1.2s) ensures turn completion even without explicit notification

#### Tool Calls
- Supports single and batched tool_calls
- Maps tool kinds: read/edit/delete/move/search/execute/fetch/think/other
- Local shell commands include stdout preview (2KB cap) in completed content

### Native vs Proxy Approaches

- **Native** (e.g., `codex-cli-acp`): Direct implementation of ACP protocol, spawning provider CLI as subprocess
- **Proxy** (planned `acp-proxy`): Wraps existing ACP servers (claude-code-acp, gemini --experimental-acp) with unified policies

### Plugin System (Planned)
The architecture includes plans for an extensible plugin system:
- Inbound/outbound processing chains
- Sub-agent invocation capabilities
- TOML/YAML configuration
- Example plugins: translation, prompt optimization

## Implementation Status

### Completed (M0)
- Rust workspace setup with two crates
- Basic project structure and dependencies
- Reference materials vendored

### In Progress (M1 - Codex Native Adapter)
- ACP stdio loop implementation
- Streaming and tool call support
- Permission mapping
- Smoke testing

### Planned
- M2: Proxy adapter for Claude/Gemini
- M3: Plugin system v0
- M4: Native Claude/Gemini adapters
- M5: HTTP/SSE bridge for non-ACP editors

## Protocol Implementation Guidelines

### JSON-RPC 2.0 Message Structure
All ACP messages must follow JSON-RPC 2.0 format:

#### Request Message
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {}
  }
}
```

#### Response Message
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "serverInfo": {
      "name": "codex-cli-acp",
      "version": "0.1.0"
    }
  }
}
```

#### Notification Message (No ID)
```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "session_123",
    "content": "Processing request..."
  }
}
```

#### Error Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32600,
    "message": "Invalid Request",
    "data": "Additional error details"
  }
}
```

### Event Streaming Specifications

#### Agent Message Chunks
Stream agent responses as they're generated:
```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "session_123",
    "type": "agent_message_chunk",
    "content": "Here's the analysis of your code..."
  }
}
```

#### Tool Call Events
##### Pending State
```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "session_123",
    "type": "tool_call",
    "toolCallId": "tool_456",
    "name": "fs/read_text_file",
    "arguments": {
      "path": "/absolute/path/to/file.rs"
    },
    "status": "pending"
  }
}
```

##### Completed State
```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "session_123",
    "type": "tool_call",
    "toolCallId": "tool_456",
    "status": "completed",
    "result": {
      "content": "file contents here..."
    }
  }
}
```

### JSONL Communication Format
- Each JSON message is on a single line
- Lines are terminated with `\n`
- No pretty-printing or multi-line JSON
- Example stdio stream:
```
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"capabilities":{}}}
{"jsonrpc":"2.0","id":2,"method":"session/new","params":{"workingDirectory":"/path/to/project"}}
{"jsonrpc":"2.0","id":2,"result":{"sessionId":"session_123"}}
```

### Error Handling Requirements
- Use standard JSON-RPC 2.0 error codes:
  - `-32700`: Parse error
  - `-32600`: Invalid Request
  - `-32601`: Method not found
  - `-32602`: Invalid params
  - `-32603`: Internal error
- Include descriptive error messages
- Add optional `data` field for debugging details

## Development Guidelines

### Adding a New Adapter
1. Create new crate under `crates/` (e.g., `crates/gemini-acp/`)
2. Add to workspace members in root `Cargo.toml`
3. Implement ACP server protocol using `acp-lazy-core` utilities
4. Add binary entry point with stdio handling
5. Map provider-specific events to ACP events
6. Add tests and documentation

### Debugging Tips
- Enable debug logging: `RUST_LOG=debug`
- Capture full backtraces: `RUST_BACKTRACE=full`
- Monitor stderr for provider subprocess output
- Use `tracing` macros for structured logging
- Check `dev-docs/` for Chinese language design documentation

### Testing Strategy
- Unit tests: Protocol parsing, permission mapping, event de-duplication
- Integration tests: Mock stdout event sequences, tool call flows
- Smoke tests: Real provider integration with actual API calls

## Practical Examples

### Spawning Codex in Proto Mode
```rust
use std::process::{Command, Stdio};
use std::io::{BufReader, BufWriter, BufRead, Write};

// Spawn Codex CLI in proto mode with appropriate permissions
let mut child = Command::new("codex")
    .arg("proto")
    .arg("-c")
    .arg("approval_policy=never")
    .arg("-c")
    .arg("sandbox_mode=workspace-write")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

let stdin = BufWriter::new(child.stdin.take().unwrap());
let stdout = BufReader::new(child.stdout.take().unwrap());
```

### Sample ACP Session Flow

#### 1. Initialize Connection
```rust
// Send initialize request
let init_request = json!({
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
        "protocolVersion": 1,
        "capabilities": {
            "fs": {
                "readTextFile": true,
                "writeTextFile": true
            }
        }
    }
});
writeln!(stdin, "{}", init_request)?;

// Read initialize response
let response: String = stdout.read_line()?;
// Parse response and extract server capabilities
```

#### 2. Create New Session
```rust
let new_session = json!({
    "jsonrpc": "2.0",
    "id": 2,
    "method": "session/new",
    "params": {
        "workingDirectory": "/absolute/path/to/project",
        "mcpServers": []  // Optional MCP server configurations
    }
});
writeln!(stdin, "{}", new_session)?;
```

#### 3. Send Prompt and Handle Streaming
```rust
let prompt = json!({
    "jsonrpc": "2.0",
    "id": 3,
    "method": "session/prompt",
    "params": {
        "sessionId": "session_123",
        "prompt": "Analyze the main.rs file and suggest improvements",
        "includeWorkspaceContext": true
    }
});
writeln!(stdin, "{}", prompt)?;

// Process streaming updates
loop {
    let line = stdout.read_line()?;
    let msg: Value = serde_json::from_str(&line)?;
    
    if msg["method"] == "session/update" {
        match msg["params"]["type"].as_str() {
            Some("agent_message_chunk") => {
                // Display agent response chunk
                print!("{}", msg["params"]["content"]);
            }
            Some("tool_call") => {
                // Handle tool call
                let status = msg["params"]["status"].as_str();
                if status == Some("pending") {
                    // Tool call started
                } else if status == Some("completed") {
                    // Tool call finished
                }
            }
            _ => {}
        }
    } else if msg["id"] == 3 {
        // Final response received
        break;
    }
}
```

### Environment Variable Overrides

ACPLazyBridge supports environment variable overrides for permissions and behavior using the `ACPLB` prefix:

#### Permission Overrides
```bash
# Override approval policy (never|on-request|on-failure|untrusted)
export ACPLB_APPROVAL_POLICY=on-request

# Override sandbox mode (read-only|workspace-write|danger-full-access)
export ACPLB_SANDBOX_MODE=workspace-write

# Override network access (true|false)
export ACPLB_NETWORK_ACCESS=true
```

#### Notify Integration
```bash
# Enable notify sink monitoring for immediate turn completion
export ACPLB_NOTIFY_PATH=/tmp/codex-notify.jsonl
export ACPLB_NOTIFY_KIND=file  # or "fifo"

# Control forwarder injection (auto|never|force)
export ACPLB_NOTIFY_INJECT=auto

# Use custom notify command (JSON array format)
export ACPLB_NOTIFY_CMD='["python", "/path/to/custom-notify.py"]'

# Timing configuration
export ACPLB_IDLE_TIMEOUT_MS=1200  # Default idle timeout
export ACPLB_POLLING_INTERVAL_MS=100  # Polling interval
```

#### Run with environment overrides
```bash
# With notify integration
ACPLB_NOTIFY_PATH=/tmp/notify.jsonl cargo run -p codex-cli-acp

# With custom settings
ACPLB_IDLE_TIMEOUT_MS=2000 ACPLB_NOTIFY_KIND=fifo cargo run -p codex-cli-acp
```

### Testing ACP Compliance
```bash
# Test initialize handshake
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | codex proto

# Test with custom model and permissions
codex proto \
  -c model="openai/gpt-5" \
  -c approval_policy="never" \
  -c sandbox_mode="read-only" \
  < test_messages.jsonl

# Debug with verbose logging
RUST_LOG=debug codex proto 2>debug.log
```

### Integration Test Example
```rust
#[tokio::test]
async fn test_acp_protocol_compliance() {
    // Spawn codex-cli-acp adapter
    let adapter = spawn_adapter()?;
    
    // Test initialize
let init_response = adapter.initialize(1).await?;
    assert!(init_response.capabilities.contains_key("fs"));
    
    // Test session creation
    let session = adapter.new_session("/test/project").await?;
    assert!(!session.id.is_empty());
    
    // Test prompt with streaming
    let mut stream = adapter.prompt(&session.id, "test prompt").await?;
    
    let mut chunks = vec![];
    while let Some(update) = stream.next().await {
        match update.type {
            UpdateType::AgentMessageChunk(text) => chunks.push(text),
            UpdateType::ToolCall(call) => {
                // Verify tool call format
                assert!(call.id.starts_with("tool_"));
            }
            _ => {}
        }
    }
    
    assert!(!chunks.is_empty());
}
```

## References

- ACP Specification: See `local_refs/agent-client-protocol/`
- Zed Examples: See `local_refs/zed-acp-examples/`
- Codex Documentation: See `local_refs/codex/`
- Design Docs: See `dev-docs/design/acp-lazybridge-architecture.md` (Chinese)
- Project Plan: See `dev-docs/plan/acp-lazybridge-project-plan.md` (Chinese)
- Requirements: See `dev-docs/requirements/acp-lazybridge-requirements.md` (Chinese)

## Important Notes

- The project strictly follows ACP specifications - check `local_refs/` for reference
- Default configuration uses non-interactive approvals to prevent IDE stalls
- YOLO/danger-full-access mode must be explicitly opted into
- Chinese documentation in `dev-docs/` contains detailed implementation guidance

## Troubleshooting

- **Build failures**: Run `cargo clean` and verify Rust stable toolchain
- **Protocol issues**: Enable `RUST_LOG=debug` and examine stdout/stderr
- **Permission errors**: Check permission mapping configuration
- **Streaming issues**: Verify line-based JSON format and de-duplication logic

---

## Engineering Workflow Policy (Worktree-first, Submission & Evidence)

This section defines mandatory workflow rules for Claude Code (and any AI coding agent) when contributing to ACPLazyBridge. Follow these rules strictly to keep CI/CD deterministic and reviews traceable.

### 1) Worktree-first Rule
- Before any task:
  - List worktrees: `git worktree list`
  - Ensure current path is a valid worktree and the intended branch is checked out
- Creating a worktree for a new task branch:
  - Branch naming: `feature/<module>-<id>`, `fix/<module>-<id>`, `docs/<module>-<id>`, `chore/<module>-<id>`
  - Create: `git worktree add ../<module>-<id> <branch>`
- Multi-worktree runtime isolation (if running multiple apps/tools):
  - Use unique ports or sockets per worktree (if applicable to your task)
  - Store per-worktree env in `.env.worktree` if necessary

### 2) Task Source of Truth
- Use the M1 task list as the task index:
  - `dev-docs/plan/issues/m1-issue-list.md`
- For each ISSUE, use this template:
  - `dev-docs/plan/issues/TEMPLATE.md`
- Implementation plan and specs reference:
  - `dev-docs/plan/m1-technical-implementation-plan.md`
  - `dev-docs/requirements/acp-lazybridge-requirements.md`
  - `dev-docs/design/acp-lazybridge-architecture.md`
  - `local_refs/agent-client-protocol/`, `local_refs/codex/`, `local_refs/zed-acp-examples/`

### 3) Submission Requirements
- JSON-RPC / ACP compliance:
  - Use proper error codes: -32700/-32600/-32601/-32602/-32603
  - Enforce constraints: absolute paths, 1-based line numbers, JSONL one message per line
- Non-interactive permissions mapping:
  - Default to `approval_policy=never`; sandbox per mode; network access only when required by the mode
  - YOLO/danger modes must be explicitly opted-in and produce conspicuous warnings
- Streaming & turn completion:
  - Forward agent_message_delta → agent_message_chunk
  - Prefer notify("agent-turn-complete"); fallback to idle timer (default ~1200ms)
  - De-duplicate final chunks

### 4) Evidence & Logs (Mandatory)
- Tests and logs directory:
  - Tests: `dev-docs/review/_artifacts/tests/*.jsonl`
  - Logs: `dev-docs/review/_artifacts/logs/`
- Run with persistent logs:
  - `... | tee dev-docs/review/_artifacts/logs/run_$(date +%Y%m%d_%H%M%S).log`
- Use jq filters for snapshotting:
  - See `dev-docs/review/_artifacts/jq/filters.md`
- Privacy & safety:
  - Do not log secrets; redact sensitive values
  - YOLO usage must be explicitly justified in PR description

### 5) Traceability Updates (No Orphans)
- Update mapping files before requesting review:
  - `dev-docs/review/_artifacts/IMPL.csv` — symbol → file:line → mapped IDs
  - `dev-docs/review/_artifacts/traceability.csv` — set each touched REQ/ARC → SPEC/CODEX/ZED to `Verified` or `Partial`
- PR description must include:
  - Issue ID, branch/worktree, test JSONL file names, log file name, jq snapshots
  - SPEC/REQ/ARC/CODEX/ZED lines referenced

### 6) Example Branch Flow
- Create branch/worktree:
  - `git worktree add ../codex-proto-1 feature/codex-proto-1`
- Implement task per ISSUE and plan
- Run JSONL tests and capture logs with `tee`
- Update `traceability.csv` and `IMPL.csv`
- Open PR with links to evidence files and relevant spec lines

---

## Non-mock Testing Plan (Claude Code)

Status
- The current repository does not yet provide the `claude-code-acplb` binary; the process defined in this section will be enabled once that binary is delivered.

Goals
- Provide scripted non-mock testing and Zed manual smoke configuration for Claude Code, with unified evidence retention and acceptance standards.

Prerequisites
- Install Claude Code; global/project settings: `~/.claude/settings.json` / `.claude/settings.json`
- Set API key (via environment variable, do not print): `ANTHROPIC_API_KEY`
- Build bridge binary (after delivery): `cargo build --release -p claude-code-acplb`

Scripted runs
- Scenarios: `dev-docs/review/_artifacts/tests/`
- Example (after delivery):
  - `target/release/claude-code-acplb < dev-docs/review/_artifacts/tests/handshake.jsonl | tee dev-docs/review/_artifacts/logs/run_$(date +%Y%m%d_%H%M%S).log`
- Optional snapshots: refer to `dev-docs/review/_artifacts/jq/filters.md`

Zed manual smoke testing
- Add ACPLazyBridge (Claude) entry in `~/.config/zed/settings.json` pointing to `target/release/claude-code-acplb` (enable after delivery)
- Keep stdout as pure JSONL, write logs to stderr and archive to `dev-docs/review/_artifacts/logs/`

Acceptance criteria
- initialize negotiation succeeds; prompts `promptCapabilities.image=false`
- session/new returns valid `sessionId`
- session/prompt streams continuous `session/update(type=agent_message_chunk)`, finally `result.stopReason` exists
- session/cancel → `stopReason=Cancelled`

Security / Secrets
- Pass API key only via environment variables, do not expose in logs and PRs; example commands use placeholder `{{ANTHROPIC_API_KEY}}`

References
- Repository-level policy: `CONTRIBUTING.md`
- WARP-Agent process: `WARP.md`
