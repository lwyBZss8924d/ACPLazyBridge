# CLAUDE.md (crates/)

## Authority

- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- ACP Protocol Spec: ../dev-docs/references/acp.md

## Purpose

Rust workspace containing the ACP protocol implementation. This workspace manages multiple crates that together form the ACPLazyBridge system.

## Workspace Structure

```tree
crates/
├── acp-lazy-core/      # Core protocol library
│   ├── src/
│   │   ├── lib.rs      # Public API
│   │   ├── protocol.rs # ACP protocol types
│   │   ├── transport.rs # Transport layer
│   │   └── permissions.rs # Permission system
│   └── CLAUDE.md       # Crate-specific guidance
└── codex-cli-acp/      # Codex CLI adapter
    ├── src/
    │   ├── main.rs     # CLI entry point
    │   ├── codex_proto.rs # Protocol handler
    │   ├── tool_calls.rs # Tool mapping
    │   └── validation.rs # Request validation
    └── CLAUDE.md       # Crate-specific guidance
```

## Workspace-Level Commands

### Build All Crates

```bash
# Build entire workspace
cargo build --workspace

# Build with all features
cargo build --workspace --all-features

# Release build
cargo build --workspace --release
```

### Test All Crates

```bash
# Run all tests
cargo test --workspace --all-features --locked

# Run with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --workspace test_name
```

### Quality Gates

```bash
# Format check (must pass)
cargo fmt --all -- --check

# Clippy linting (must pass)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Documentation
cargo doc --workspace --no-deps
```

## ACP Protocol Implementation

### Version Handling

**Current**: ACP v1 (protocolVersion: 1 as integer)

```rust
// Always use integer protocol version
const PROTOCOL_VERSION: u32 = 1;

// In JSON
{"protocolVersion": 1}  // ✅ Correct
{"protocolVersion": "1"} // ❌ Wrong - must be integer
```

### Message Format

```rust
// stdout: Protocol messages only
println!("{}", serde_json::to_string(&message)?);

// stderr: All logs and diagnostics
eprintln!("[DEBUG] Processing request");
```

### Error Handling

```rust
// Use standard JSON-RPC error codes
const PARSE_ERROR: i32 = -32700;
const INVALID_REQUEST: i32 = -32600;
const METHOD_NOT_FOUND: i32 = -32601;
const INVALID_PARAMS: i32 = -32602;
const INTERNAL_ERROR: i32 = -32603;
```

## Testing Patterns

### Protocol Testing

```bash
# Test basic handshake
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp

# Test with JSONL file
cat test/scenarios.jsonl | cargo run -p codex-cli-acp

# Test with debugging
RUST_LOG=debug cargo run -p codex-cli-acp < test/input.jsonl 2>debug.log
```

### Evidence Collection

```bash
# Run tests with evidence capture
cargo test --workspace 2>&1 | \
  tee ../dev-docs/review/_artifacts/tests/<task>/test_$(date +%Y%m%d_%H%M%S).log

# Generate test coverage
cargo tarpaulin --workspace --out Html \
  --output-dir ../dev-docs/review/_artifacts/reports/<task>/
```

## Crate-Specific Guidance

### acp-lazy-core

Focus areas:

- Protocol type definitions
- Transport abstraction
- Permission models
- Keep stdout clean for protocol

### codex-cli-acp

Focus areas:

- ACP server implementation
- Tool call mapping
- Streaming support
- JSONL compliance

## Common Patterns

### Adding New Protocol Methods

1. Define in `acp-lazy-core/src/protocol.rs`
2. Implement handler in `codex-cli-acp/src/codex_proto.rs`
3. Add tests in `codex-cli-acp/tests/`
4. Update evidence in `dev-docs/review/_artifacts/`

### Tool Call Implementation

```rust
// Map ACP tool calls to Codex format
match tool_call.name.as_str() {
    "fs/read_text_file" => handle_read_file(params),
    "fs/write_text_file" => handle_write_file(params),
    _ => Err(method_not_found()),
}
```

## Dependencies

### Workspace Dependencies

```toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

### Crate Dependencies

Reference workspace versions:

```toml
[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
```

## Publishing

### Pre-publish Checklist

- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG updated
- [ ] All tests pass
- [ ] Documentation complete
- [ ] Evidence collected

### Publish Commands

```bash
# Dry run
cargo publish --dry-run -p acp-lazy-core

# Actual publish (requires token)
cargo publish -p acp-lazy-core
```

## Performance Considerations

### Streaming

- Use line-buffered output for JSONL
- Flush stdout after each message
- Keep message size reasonable

### Memory

- Stream large file contents
- Avoid loading entire responses in memory
- Use iterators where possible

## Security

### Critical Rules

- Never log secrets or tokens
- Validate all input
- Sanitize file paths
- Check permissions before operations

### Input Validation

```rust
// Validate protocol version
if params.protocol_version != PROTOCOL_VERSION {
    return Err(invalid_params("Unsupported protocol version"));
}

// Validate file paths
let path = canonicalize(requested_path)?;
if !path.starts_with(workspace_root) {
    return Err(invalid_params("Path outside workspace"));
}
```

## Quick Reference

### Run Adapter

```bash
# Development
cargo run -p codex-cli-acp

# With logging
RUST_LOG=info cargo run -p codex-cli-acp

# Production
cargo run --release -p codex-cli-acp
```

### Debug Issues

```bash
# Verbose logging
RUST_LOG=trace cargo run -p codex-cli-acp 2>trace.log

# With backtrace
RUST_BACKTRACE=1 cargo run -p codex-cli-acp

# Test specific scenario
echo '{"jsonrpc":"2.0","id":1,"method":"test"}' | cargo run -p codex-cli-acp
```

---

Specification Version: 1.0.3 | crates/CLAUDE.md Format: 1.0 | Last Updated: 2025-09-11
