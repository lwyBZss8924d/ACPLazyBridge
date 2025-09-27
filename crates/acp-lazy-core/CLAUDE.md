# CLAUDE.md (crates/acp-lazy-core)

## Authority

- Constitution: ../../.specify/memory/constitution.md (Articles I, III, VII, VIII, IX)
- SDD Integration: ../../.specify/CLAUDE.md (operational context)
- Workspace guidance: ../CLAUDE.md
- See ../../sdd-rules/CLAUDE.md and ../../sdd-rules/AGENTS.md

## Purpose

Core ACP (Agent Client Protocol) library providing:

- JSON-RPC 2.0 protocol types and handling
- Transport layer abstractions (stdio, process)
- Permission mapping for Codex integration
- Shared utilities for all ACP adapters

This is a **library-first** crate (Article I) with no CLI interface.

## SDD Integration

For comprehensive SDD workflow details, see **[../../.specify/CLAUDE.md](../../.specify/CLAUDE.md)**

### Constitutional Requirements for This Crate

- **Article I (Library-First)**: ✅ Pure library crate with `lib.rs`
- **Article III (Test-First)**: All protocol changes require failing tests first
- **Article VII (Simplicity)**: Direct trait implementations, no unnecessary wrappers
- **Article VIII (Anti-Abstraction)**: Use `serde`, `tokio` directly
- **Article IX (Integration-First)**: Protocol contracts defined before implementation

### TDD Workflow for Protocol Changes

```rust
// 1. RED: Write failing test for new protocol type
#[test]
fn test_new_protocol_message() {
    let msg = IncomingMessage::parse(json_input).unwrap();
    assert_eq!(msg.method, "new_method"); // Fails until implemented
}

// 2. GREEN: Add minimal protocol definition
#[derive(Serialize, Deserialize)]
pub struct NewMethod { /* fields */ }

// 3. REFACTOR: Improve with validation, docs, etc.
```

## What to do here

### Core Development Tasks

1. **Protocol Extensions**

   ```bash
   # Write test first (must fail)
   cargo test test_new_protocol_type
   # Implement in src/protocol.rs
   # Verify test passes
   cargo test --lib
   ```

2. **Permission Mapping**

   ```bash
   # Test permission conversion
   cargo test test_acp_to_codex_mapping
   # Update src/permissions.rs
   # Verify all permission tests
   cargo test permissions
   ```

3. **Transport Layer**

   ```bash
   # Test transport behavior
   cargo test transport::tests
   # Ensure stdout/stderr separation
   RUST_LOG=debug cargo test -- --nocapture 2>debug.log
   ```

### Quality Gates

```bash
# Format check (must pass)
cargo fmt -- --check

# Clippy linting (must pass)
cargo clippy --all-targets --all-features -- -D warnings

# All tests (must pass)
cargo test --all-features --locked

# Documentation
cargo doc --no-deps --open
```

### Evidence Collection

```bash
# Primary location
cargo test 2>&1 | tee ../../_artifacts/tests/acp-lazy-core/test_$(date +%Y%m%d_%H%M%S).log

# Legacy location (if needed)
cargo test 2>&1 | tee ../../_artifacts/tests/legacy/acp-lazy-core/test_$(date +%Y%m%d_%H%M%S).log
```

## Testing Guidelines

### Unit Tests (Required)

Test each protocol type independently:

```rust
// src/protocol.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_serialization() {
        // Test-first: write this before implementing Error
        let error = Error::new(ErrorCode::InvalidParams, "test");
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("\"code\":-32602"));
    }
}
```

### Integration Tests

Test transport and protocol together:

```rust
// tests/integration.rs
#[tokio::test]
async fn test_message_flow() {
    // Test complete message lifecycle
    let transport = ProcessTransport::new();
    // ... test implementation
}
```

### Protocol Compliance Tests

Verify JSON-RPC 2.0 compliance:

```rust
#[test]
fn test_jsonrpc_compliance() {
    // Ensure protocol version is integer, not string
    let init = r#"{"protocolVersion": 1}"#; // ✅
    let bad = r#"{"protocolVersion": "1"}"#;  // ❌
}
```

## Common Tasks

### Adding New Protocol Types

1. **Write failing test first** (Article III)

   ```rust
   // tests/new_type.rs
   #[test]
   fn test_new_message_type() {
       let result = parse_new_type(input);
       assert!(result.is_ok()); // This must fail initially
   }
   ```

2. **Define in protocol.rs**

   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   #[serde(tag = "method", rename_all = "camelCase")]
   pub struct NewMethod {
       pub params: NewParams,
   }
   ```

3. **Add to MessageType enum**

   ```rust
   pub enum MessageType {
       // ... existing variants
       NewMethod(NewMethod),
   }
   ```

4. **Verify tests pass** (GREEN phase)

### Extending Permissions

1. **Test permission mapping first**

   ```rust
   #[test]
   fn test_new_permission() {
       let acp_perm = AcpPermission::NewType;
       let codex = map_acp_to_codex(&acp_perm);
       assert_eq!(codex, expected_mapping);
   }
   ```

2. **Update permissions.rs**

   ```rust
   pub fn map_acp_to_codex(acp: &AcpPermission) -> CodexPermission {
       match acp {
           // ... existing mappings
           AcpPermission::NewType => CodexPermission::Corresponding,
       }
   }
   ```

### Transport Debugging

```bash
# Test with protocol messages
echo '{"jsonrpc":"2.0","id":1,"method":"test"}' | cargo run --example test_transport

# Verify stdout/stderr separation
cargo run --example test_transport 2>stderr.log 1>stdout.log
# stdout.log should contain only JSON-RPC
# stderr.log should contain only logs
```

## Notes

### Critical Rules

- **stdout is sacred**: Only JSON-RPC protocol messages
- **stderr for logs**: All debug, info, error logs go to stderr
- **Test-first always**: No protocol changes without failing tests
- **No wrapper traits**: Use framework types directly (Article VIII)

### Performance Considerations

- Stream large messages instead of loading in memory
- Use `tokio::io::BufReader` for line-based protocol
- Flush stdout after each message for real-time communication

### Security

- Validate all incoming JSON
- Sanitize error messages (no internal paths)
- Never log sensitive data
- Verify protocol version strictly (integer, not string)

## Quick Reference

### Run Tests

```bash
# All tests
cargo test

# Specific module
cargo test protocol::
cargo test permissions::
cargo test transport::

# With output
cargo test -- --nocapture

# With logging
RUST_LOG=debug cargo test
```

### Check Protocol Message

```bash
# Validate JSON-RPC format
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run --example validate_message
```

### Generate Docs

```bash
# Build and open documentation
cargo doc --no-deps --open
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-27T11:37:10Z"
document:
    type: "claude-memory"
    path: "./crates/acp-lazy-core/CLAUDE.md"
    version: "1.0.1"
    last_updated: "2025-09-27T11:37:10Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - "./CLAUDE.md"
```
