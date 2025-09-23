# CLAUDE.md (crates/codex-cli-acp)

## Authority

- Constitution: ../../.specify/memory/constitution.md (Articles I, II, III, VII, VIII, IX)
- SDD Integration: ../../.specify/CLAUDE.md (operational context)
- Workspace guidance: ../CLAUDE.md
- Core library: ../acp-lazy-core/CLAUDE.md
- See ../../sdd-rules/CLAUDE.md and ../../sdd-rules/AGENTS.md

## Purpose

ACP server implementation for Codex CLI providing:

- JSON-RPC 2.0 server with JSONL streaming
- Tool call mapping between ACP and Codex formats
- Session management and turn completion
- Notification system for external control
- Protocol validation and error handling

This crate is **BOTH**:

- **Library** (`lib.rs`) - Reusable components (Article I)
- **Binary** (`main.rs`) - ACP server CLI (Article II)

## SDD Integration

For comprehensive SDD workflow details, see **[../../.specify/CLAUDE.md](../../.specify/CLAUDE.md)**

### Constitutional Requirements for This Crate

- **Article I (Library-First)**: ✅ Has `lib.rs` with reusable components
- **Article II (CLI Interface)**: ✅ Main binary provides ACP server
- **Article III (Test-First)**: All features require failing tests first
- **Article VII (Simplicity)**: Direct integration with acp-lazy-core
- **Article VIII (Anti-Abstraction)**: Uses frameworks directly
- **Article IX (Integration-First)**: JSONL contracts tested first

### TDD Workflow for Server Features

```rust
// 1. RED: Write failing test for new tool call
#[test]
fn test_new_tool_mapping() {
    let acp_call = parse_acp_tool_call(input);
    let codex_call = map_to_codex(acp_call);
    assert_eq!(codex_call.name, "expected"); // Fails first
}

// 2. GREEN: Implement mapping in tool_calls.rs
// 3. REFACTOR: Improve error handling, add validation
```

## What to do here

### Core Development Tasks

1. **Protocol Handler Development**

   ```bash
   # Test protocol handling
   cargo test codex_proto::tests

   # Test with real JSONL
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
     cargo run
   ```

2. **Tool Call Mapping**

   ```bash
   # Test tool call conversion
   cargo test tool_calls::tests

   # Verify mapping correctness
   cargo test test_fs_operations
   cargo test test_browser_operations
   ```

3. **Streaming Implementation**

   ```bash
   # Test streaming with chunks
   cargo test test_streaming_response

   # Test with real Codex
   cargo run < test/streaming.jsonl
   ```

4. **Notification System**

   ```bash
   # Test notification sources
   cargo test notify_source::tests

   # Test with external notifier
   ACPLB_NOTIFY_PATH=/tmp/notify.pipe \
   ACPLB_NOTIFY_KIND=fifo \
     cargo run
   ```

### Quality Gates

```bash
# Format check (must pass)
cargo fmt -- --check

# Clippy linting (must pass)
cargo clippy --all-targets --all-features -- -D warnings

# All tests (must pass)
cargo test --all-features --locked

# Test with JSONL scenarios
cat ../../_artifacts/tests/legacy/*.jsonl | cargo run
```

### Evidence Collection

```bash
# Primary location
cargo test 2>&1 | tee ../../_artifacts/tests/codex-cli-acp/test_$(date +%Y%m%d_%H%M%S).log

# JSONL scenario testing
cat test/scenarios.jsonl | cargo run 2>&1 | \
  tee ../../_artifacts/logs/codex-cli-acp/scenario_$(date +%Y%m%d_%H%M%S).log

# Legacy location (if needed)
cargo test 2>&1 | tee ../../_artifacts/tests/legacy/codex-cli-acp/test_$(date +%Y%m%d_%H%M%S).log
```

## Testing Guidelines

### Unit Tests (Required)

Test each component independently:

```rust
// src/tool_calls.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_read_mapping() {
        // Test-first: write before implementing
        let acp = AcpToolCall::FsRead { path: "test.txt" };
        let codex = map_to_codex(acp);
        assert_eq!(codex.tool_name, "fs/read_text_file");
    }
}
```

### Integration Tests

Test complete message flows:

```rust
// tests/session_test.rs
#[tokio::test]
async fn test_session_lifecycle() {
    // Test initialize -> session -> prompt -> complete
    let server = start_test_server();
    // ... full protocol flow
}
```

### JSONL Scenario Tests

Create test scenarios in JSONL:

```jsonl
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}
{"jsonrpc":"2.0","id":2,"method":"sessions/create","params":{"config":{}}}
{"jsonrpc":"2.0","id":3,"method":"sessions/update","params":{"sessionId":"test","turns":[...]}}
```

Test with:

```bash
cargo run < tests/scenario.jsonl > output.jsonl
# Validate output is valid JSONL with expected responses
```

### Playback Testing

Use the playback binary for regression testing:

```bash
# Record a session
cargo run > session.jsonl 2>session.log

# Playback and compare
cargo run --bin playback < session.jsonl > replay.jsonl
diff session.jsonl replay.jsonl
```

## Common Tasks

### Adding New Tool Calls

1. **Write failing test first** (Article III)

   ```rust
   // tests/tool_calls_test.rs
   #[test]
   fn test_new_tool() {
       let result = map_new_tool(input);
       assert_eq!(result, expected); // Must fail first
   }
   ```

2. **Add ACP tool type**

   ```rust
   // src/tool_calls.rs
   pub enum AcpToolCall {
       // ... existing variants
       NewTool { params: NewParams },
   }
   ```

3. **Implement Codex mapping**

   ```rust
   pub fn map_to_codex(acp: AcpToolCall) -> CodexToolCall {
       match acp {
           AcpToolCall::NewTool { params } => {
               CodexToolCall {
                   tool_name: "category/new_tool",
                   arguments: json!(params),
               }
           }
           // ... other mappings
       }
   }
   ```

4. **Verify tests pass** and collect evidence

### Extending Protocol Handlers

1. **Test protocol message first**

   ```rust
   #[test]
   fn test_new_method() {
       let msg = parse_jsonrpc(r#"{"method":"new/method"}"#);
       let response = handle_new_method(msg);
       assert!(response.is_ok());
   }
   ```

2. **Add handler in codex_proto.rs**

   ```rust
   pub async fn handle_method(params: Value) -> Result<Value> {
       // Implementation following test requirements
   }
   ```

3. **Wire into main dispatch**

   ```rust
   match method {
       "new/method" => handle_new_method(params).await,
       // ... other methods
   }
   ```

### Debugging Streaming

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Capture stderr separately
cargo run 2>debug.log 1>protocol.jsonl

# Test chunk timing
cargo run --example streaming_test
```

## Binary Documentation

### Main Server (`codex-cli-acp`)

The default binary that implements the ACP server:

```bash
# Run server
cargo run

# With environment configuration
ACPLB_IDLE_TIMEOUT_MS=2000 \
ACPLB_POLLING_INTERVAL_MS=200 \
  cargo run
```

### Playback Utility (`playback`)

For testing and debugging:

```bash
# Run playback
cargo run --bin playback < recorded.jsonl

# Compare outputs
diff -u original.jsonl playback.jsonl
```

### Notify Forwarder (`acplb-notify-forwarder`)

External notification helper:

```bash
# Forward notifications
cargo run --bin acplb-notify-forwarder \
  --input /tmp/source \
  --output /tmp/sink
```

## Protocol Compliance

### JSONL Requirements

- **stdout**: ONLY valid JSONL messages
- **stderr**: ALL logs, debug output, errors
- **No mixing**: Never write non-JSON to stdout

```rust
// ✅ CORRECT
println!("{}", serde_json::to_string(&response)?);
eprintln!("[DEBUG] Processing request");

// ❌ WRONG
println!("Processing..."); // Breaks JSONL
```

### Protocol Version

```rust
// Always integer, never string
const PROTOCOL_VERSION: u32 = 1;

// In JSON
{"protocolVersion": 1}  // ✅
{"protocolVersion": "1"} // ❌
```

## Notes

### Critical Rules

- **stdout purity**: Only JSONL, no exceptions
- **Test-first always**: No features without failing tests
- **Direct dependencies**: Use acp-lazy-core directly
- **Evidence for everything**: Log all test runs

### Performance Considerations

- Stream large responses with chunks
- Use idle timeout for turn completion
- Flush stdout after each message
- Consider notification vs timeout trade-offs

### Security

- Validate all tool call parameters
- Sanitize file paths
- Never expose internal errors
- Check permissions via ACP mode

## Quick Reference

### Run Server

```bash
# Development
cargo run

# Production
cargo run --release

# With debugging
RUST_LOG=debug cargo run 2>debug.log
```

### Test Commands

```bash
# All tests
cargo test

# Specific test
cargo test test_tool_mapping

# With output
cargo test -- --nocapture

# Integration tests
cargo test --test '*'
```

### JSONL Testing

```bash
# Single message
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | cargo run

# Scenario file
cat scenario.jsonl | cargo run

# With validation
cat scenario.jsonl | cargo run | jq -c . > /dev/null && echo "Valid JSONL"
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "claude-memory"
    path: "./crates/codex-cli-acp/CLAUDE.md"
    version: "1.0.1"
    last_updated: "2025-09-17T08:26:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - "./CLAUDE.md"
```
