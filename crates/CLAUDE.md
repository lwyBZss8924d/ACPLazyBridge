# CLAUDE.md (crates/)

## Authority

- Constitution: ../.specify/memory/constitution.md (Articles I, III, VII, VIII, IX)
- SDD Integration: ../.specify/CLAUDE.md (operational context)
- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- ACP Protocol Spec: ../dev-docs/references/acp.md

## Purpose

Rust workspace containing the ACP protocol implementation. This workspace manages multiple crates that together form the ACPLazyBridge system.

## SDD Integration

For comprehensive SDD workflow details, see **[../.specify/CLAUDE.md](../.specify/CLAUDE.md)**

Key SDD principles for Rust crates:

- **Library-First (Article I)**: All crates are libraries with optional CLI interfaces
- **Test-First (Article III)**: Write failing tests before implementation (RED→GREEN→REFACTOR)
- **Simplicity (Article VII)**: No unnecessary abstractions or future-proofing
- **Anti-Abstraction (Article VIII)**: Use framework features directly
- **Integration-First (Article IX)**: Define contracts before implementation

### Constitutional Gates for Crates

Every crate must:

1. Be a library first (`lib.rs` required, `main.rs` optional)
2. Have tests that fail before implementation exists
3. Use direct framework features (no wrapper traits unless justified)
4. Maintain evidence in `_artifacts/<task>/` or `dev-docs/review/_artifacts/<task>/`
5. Follow the SDD workflow: spec → plan → tasks → implementation

### TDD Workflow

```bash
# 1. RED: Write failing test
cargo test --lib test_new_feature
# ✗ test fails (expected)

# 2. GREEN: Implement minimal code to pass
# ... write implementation ...
cargo test --lib test_new_feature
# ✓ test passes

# 3. REFACTOR: Improve without breaking tests
cargo test --workspace
# ✓ all tests pass
```

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

### Test-First Development (Article III - NON-NEGOTIABLE)

**CRITICAL**: Tests MUST be written and MUST fail before ANY implementation.

```rust
// 1. RED: Write test that fails
#[test]
fn test_new_protocol_method() {
    let result = handle_new_method(params);
    assert_eq!(result, expected); // This MUST fail first
}

// 2. GREEN: Implement minimal code to pass
// 3. REFACTOR: Improve without breaking tests
```

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
# Run tests with evidence capture (primary location)
cargo test --workspace 2>&1 | \
  tee ../_artifacts/tests/<task>/test_$(date +%Y%m%d_%H%M%S).log

# Alternative: legacy location
cargo test --workspace 2>&1 | \
  tee ../dev-docs/review/_artifacts/tests/<task>/test_$(date +%Y%m%d_%H%M%S).log

# Generate test coverage
cargo tarpaulin --workspace --out Html \
  --output-dir ../_artifacts/reports/<task>/
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

1. Write failing test first (TDD - Article III)
2. Define in `acp-lazy-core/src/protocol.rs`
3. Implement handler in `codex-cli-acp/src/codex_proto.rs`
4. Verify tests pass (RED→GREEN→REFACTOR)
5. Update evidence in `_artifacts/<task>/` or `dev-docs/review/_artifacts/<task>/`

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

## Constitutional Compliance

### Required for Every Change

- [ ] **Article I (Library-First)**: Changes in `lib.rs`, not just `main.rs`
- [ ] **Article III (Test-First)**: Tests written and failed before implementation
- [ ] **Article VII (Simplicity)**: No unnecessary abstractions or patterns
- [ ] **Article VIII (Anti-Abstraction)**: Using framework features directly
- [ ] **Article IX (Integration-First)**: Contracts defined before implementation

### Anti-patterns to Avoid

❌ **Wrapper traits** around framework types (unless absolutely necessary)
❌ **Future-proofing** for unspecified requirements
❌ **Implementation before tests** (violates Article III)
❌ **More than 3 sub-crates** without justification (violates Article VII)

### Good Patterns

✅ Direct use of `serde`, `tokio`, etc. without wrappers
✅ Tests that demonstrate the bug/feature before fixing/implementing
✅ Simple, concrete implementations over abstract ones
✅ Evidence collected for every change

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

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "claude-memory"
    path: "./crates/CLAUDE.md"
    version: "1.0.1"
    last_updated: "2025-09-17T08:26:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - "./CLAUDE.md"
```
