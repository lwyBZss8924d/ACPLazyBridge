# AGENTS.md (crates/acp-lazy-core/)

Core library guidance for AI Engineers working with the ACPLazyBridge protocol implementation.

## Authority

- Constitution: ../../.specify/memory/constitution.md (Articles I, III, VII, VIII, IX)
- Parent: ../AGENTS.md (workspace-level guidance)
- Workspace: ../CLAUDE.md (workspace conventions)

## Crate Purpose

`acp-lazy-core` is the foundational library providing:

- ACP protocol type definitions
- Transport layer abstractions
- Permission system implementations
- Shared utilities for ACPLazyBridge

## Library-First Architecture (Article I)

This crate is a pure library with no CLI components:

```rust
// src/lib.rs - Public API
pub mod protocol;   // ACP types
pub mod transport;  // I/O abstractions
pub mod permissions; // Permission models
pub mod runtime;    // Runtime components
```

## Test-First Development (Article III)

Every feature MUST follow RED→GREEN→REFACTOR:

```bash
# 1. Write failing test
cargo test -p acp-lazy-core test_new_feature
# ✗ Fails (expected)

# 2. Implement minimal code
# 3. Verify test passes
cargo test -p acp-lazy-core test_new_feature
# ✓ Passes

# 4. Refactor and verify all tests
cargo test -p acp-lazy-core --all-features
```

## Core Components

### Protocol Module

Defines ACP protocol types:

- Request/Response structures
- Error codes and types
- Protocol version handling
- JSON-RPC message types

### Transport Module

Handles I/O operations:

- Process spawning and management
- JSONL reading/writing
- Stream management
- Message queuing

### Permissions Module

Maps ACP permissions to implementations:

- Permission modes
- Capability flags
- Security policies
- Access control

### Runtime Module

Orchestrates protocol execution:

- Session management
- Adapter coordination
- Server lifecycle
- State management

## Anti-Abstraction Guidelines (Article VIII)

✅ Good patterns:

```rust
// Direct use of serde
#[derive(Serialize, Deserialize)]
pub struct Request { ... }

// Direct tokio usage
pub async fn handle_message(msg: Message) -> Result<Response>
```

❌ Avoid:

```rust
// Unnecessary wrapper traits
trait SerializableWrapper { ... }

// Over-abstraction
trait MessageHandler<T, U, V> { ... }
```

## Testing Requirements

### Unit Tests

Located in `src/*.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_parsing() {
        // Test implementation
    }
}
```

### Integration Tests

Located in `tests/`:

```rust
// tests/runtime_test.rs
use acp_lazy_core::runtime::*;

#[tokio::test]
async fn test_full_session() {
    // Integration test
}
```

## Common Tasks

### Adding Protocol Types

1. Define in `src/protocol.rs`
2. Add serde derives
3. Write unit tests
4. Update documentation

### Implementing Transport Features

1. Extend `src/transport.rs`
2. Add error handling
3. Test with mock I/O
4. Verify JSONL compliance

### Updating Permissions

1. Modify `src/permissions.rs`
2. Map to Codex flags
3. Test permission combinations
4. Document security implications

## Performance Considerations

- Keep allocations minimal
- Use `&str` over `String` where possible
- Stream large payloads
- Avoid blocking operations in async code

## Security Guidelines

- Validate all inputs
- Never expose internal paths
- Sanitize error messages
- Use secure defaults

## Dependencies

Core dependencies only:

```toml
[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
tokio = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
```

## Documentation

Generate docs:

```bash
cargo doc -p acp-lazy-core --no-deps --open
```

## Quick Reference

```bash
# Build
cargo build -p acp-lazy-core

# Test
cargo test -p acp-lazy-core --all-features

# Check
cargo check -p acp-lazy-core

# Lint
cargo clippy -p acp-lazy-core -- -D warnings
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-27T11:37:10Z"
document:
    type: "agent-memory"
    path: "crates/acp-lazy-core/AGENTS.md"
    version: "1.0.0"
    last_updated: "2025-09-27T11:37:10Z"
    dependencies:
        - "../AGENTS.md"
        - "../../AGENTS.md"
        - "../../.specify/memory/constitution.md"
```
