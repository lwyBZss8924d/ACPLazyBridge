# AGENTS.md (crates/codex-cli-acp/)

Adapter implementation guidance for AI Engineers working with the Codex CLI ACP bridge.

## Authority

- Constitution: ../../.specify/memory/constitution.md (Articles I, III, VII, VIII, IX)
- Parent: ../AGENTS.md (workspace-level guidance)
- Core Library: ../acp-lazy-core/AGENTS.md (protocol implementation)

## Crate Purpose

`codex-cli-acp` implements the ACP server adapter for Codex CLI:

- ACP server protocol handling
- Tool call mapping and execution
- Streaming support for real-time updates
- Notification system for turn completion

## Library + CLI Architecture (Article I)

```rust
// src/lib.rs - Library interface
pub mod codex_agent;
pub mod codex_proto;
pub mod tool_calls;
pub mod validation;
pub mod notify_source;

// src/main.rs - Thin CLI wrapper
fn main() -> Result<()> {
    // Minimal setup, delegate to library
    codex_cli_acp::run()
}

// src/bin/ - Additional utilities
// acplb_notify_forwarder.rs
// playback.rs
```

## Test-First Development (Article III)

Follow strict TDD for all features:

```bash
# 1. Write failing integration test
cargo test -p codex-cli-acp test_session_prompt
# ✗ Fails first

# 2. Implement handler
# 3. Verify test passes
cargo test -p codex-cli-acp test_session_prompt
# ✓ Passes

# 4. Run all tests
cargo test -p codex-cli-acp --all-features
```

## Core Components

### Main Server (`main.rs`)

Implements ACP methods:

- `initialize` - Protocol handshake
- `session/new` - Create session
- `session/prompt` - Execute prompts
- `session/cancel` - Cancel execution

### Codex Protocol (`codex_proto.rs`)

Maps Codex events to ACP:

- AgentMessage → AgentMessageChunk
- ToolCall → Tool execution events
- TaskComplete → Session completion
- Error → Error responses

### Tool Calls (`tool_calls.rs`)

Categorizes and maps tools:

- File operations (read/edit/delete)
- Shell execution
- Search operations
- Think/fetch tools

### Validation (`validation.rs`)

Input validation and error handling:

- Path validation (absolute paths)
- Parameter checking
- Error classification
- Response formatting

### Notification System (`notify_source.rs`)

Turn completion signaling:

- File-based notifications
- FIFO support
- Polling mechanisms
- Event forwarding

## Protocol Compliance

### JSON-RPC Requirements

```rust
// Correct protocol version (integer)
const PROTOCOL_VERSION: u32 = 1;

// Error codes
const PARSE_ERROR: i32 = -32700;
const INVALID_REQUEST: i32 = -32600;
const METHOD_NOT_FOUND: i32 = -32601;
const INVALID_PARAMS: i32 = -32602;
const INTERNAL_ERROR: i32 = -32603;
```

### I/O Separation

```rust
// stdout: Protocol only
println!("{}", serde_json::to_string(&response)?);

// stderr: Logs only
eprintln!("[DEBUG] Processing request");
```

## Testing Strategies

### Unit Tests

```rust
// In src/tool_calls.rs
#[cfg(test)]
mod tests {
    #[test]
    fn test_categorize_tool() {
        assert_eq!(categorize("fs/read"), ToolKind::Read);
    }
}
```

### Integration Tests

```rust
// tests/acp_integration_test.rs
#[tokio::test]
async fn test_full_session() {
    // Test complete session flow
}
```

### JSONL Regression Tests

```bash
# Test with baseline scenarios
cat ../../_artifacts/tests/protocol-baseline/handshake.jsonl | \
  cargo run -p codex-cli-acp
```

## Common Implementation Tasks

### Adding New Tool Mapping

1. Define in `src/tool_calls.rs`
2. Add categorization logic
3. Implement handler
4. Write tests
5. Update documentation

### Implementing Session Features

1. Extend `src/codex_proto.rs`
2. Map to ACP events
3. Handle streaming
4. Test with scenarios

### Debugging Protocol Issues

```bash
# Enable trace logging
RUST_LOG=trace cargo run -p codex-cli-acp 2>trace.log

# Test specific method
echo '{"jsonrpc":"2.0","id":1,"method":"session/new","params":{}}' | \
  cargo run -p codex-cli-acp

# Use playback utility
cargo run --bin playback < test.jsonl
```

## Performance Optimization

- Stream large responses
- Use line-buffered I/O
- Minimize allocations
- Profile with `perf` or `valgrind`

## Security Considerations

- Validate all file paths
- Check permission modes
- Sanitize shell commands
- Never log sensitive data
- Use timeouts for operations

## Binary Utilities

### acplb_notify_forwarder

Forwards Codex notifications:

```bash
cargo run --bin acplb_notify_forwarder
```

### playback

Test utility for JSONL replay:

```bash
cargo run --bin playback < scenario.jsonl
```

## Evidence Collection

```bash
# Capture test evidence
cargo test -p codex-cli-acp 2>&1 | \
  tee ../../_artifacts/tests/<task>/codex_$(date +%Y%m%d_%H%M%S).log

# Profile performance
cargo build --release -p codex-cli-acp
time cargo run --release -p codex-cli-acp < benchmark.jsonl
```

## Quick Reference

```bash
# Build adapter
cargo build -p codex-cli-acp --release

# Run with logging
RUST_LOG=info cargo run -p codex-cli-acp

# Test all features
cargo test -p codex-cli-acp --all-features

# Check code quality
cargo clippy -p codex-cli-acp -- -D warnings
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-27T11:37:10Z"
document:
    type: "agent-memory"
    path: "crates/codex-cli-acp/AGENTS.md"
    version: "1.0.0"
    last_updated: "2025-09-27T11:37:10Z"
    dependencies:
        - "../AGENTS.md"
        - "../acp-lazy-core/AGENTS.md"
        - "../../AGENTS.md"
        - "../../.specify/memory/constitution.md"
```
