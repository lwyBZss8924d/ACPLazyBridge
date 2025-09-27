# AGENTS.md (crates/)

Workspace-level guidance for all AI Engineers working with the ACPLazyBridge Rust crates. This document applies to the Rust workspace implementation of the ACP protocol bridge.

## Authority

- Constitution: ../.specify/memory/constitution.md (Articles I, III, VII, VIII, IX)
- Parent: ../AGENTS.md (repository-wide AI engineer guidance)
- SDD Rules: ../sdd-rules/AGENTS.md (team coordination)
- Workspace: ./AGENTS.md (Codex-specific crate guidance)

## Rust Workspace Context

This workspace contains the core ACPLazyBridge implementation:

```tree
crates/
├── acp-lazy-core/      # Core protocol library
└── codex-cli-acp/      # Codex CLI adapter
```

## Common Development Patterns

### Test-First Development (Article III)

```bash
# RED: Write failing test first
cargo test --workspace test_name
# ✗ Expected to fail

# GREEN: Minimal implementation
# ... implement feature ...
cargo test --workspace test_name
# ✓ Passes

# REFACTOR: Clean up
cargo test --workspace --all-features
# ✓ All pass
```

### Quality Gates (Must Pass)

```bash
# Format
cargo fmt --all -- --check

# Lint
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Test
cargo test --workspace --all-features --locked
```

### Protocol Testing

```bash
# Test handshake
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp

# Test with JSONL
cat ../_artifacts/tests/protocol-baseline/scenario.jsonl | cargo run -p codex-cli-acp
```

## Library-First Principles (Article I)

Every crate MUST:

1. Have a `src/lib.rs` (library interface)
2. Optionally have `src/main.rs` (CLI interface)
3. Export public API through the library
4. Keep CLI thin, logic in library

## Anti-Abstraction Rules (Article VIII)

✅ Good:

- Direct use of `serde`, `tokio`, `anyhow`
- Framework features used as-is
- Concrete types over generic ones

❌ Avoid:

- Wrapper traits around framework types
- Unnecessary abstractions
- Future-proofing without requirements

## Evidence Collection

```bash
# Primary location
cargo test --workspace 2>&1 | \
  tee ../_artifacts/tests/<task>/test_$(date +%Y%m%d_%H%M%S).log

# Alternative: legacy location
cargo test --workspace 2>&1 | \
  tee ../_artifacts/legacy/tests/<task>/test_$(date +%Y%m%d_%H%M%S).log
```

## ACP Protocol Requirements

- **stdout**: JSONL protocol messages only
- **stderr**: All logs and diagnostics
- **protocolVersion**: Integer `1`, never string `"1"`

## Crate-Specific Guidelines

### acp-lazy-core

Core library responsibilities:

- Protocol type definitions
- Transport abstractions
- Permission models
- Shared utilities

See: [acp-lazy-core/CLAUDE.md](acp-lazy-core/CLAUDE.md)

### codex-cli-acp

Adapter responsibilities:

- ACP server implementation
- Tool call mapping
- Streaming support
- Notification handling

See: [codex-cli-acp/CLAUDE.md](codex-cli-acp/CLAUDE.md)

## Common Tasks

### Adding New Protocol Method

1. Write failing test in `tests/`
2. Define types in `acp-lazy-core/src/protocol.rs`
3. Implement handler in `codex-cli-acp/src/codex_proto.rs`
4. Verify tests pass (RED→GREEN→REFACTOR)
5. Collect evidence in `_artifacts/`

### Updating Dependencies

```bash
# Check outdated
cargo outdated --workspace

# Update and test
cargo update
cargo test --workspace --all-features
```

### Performance Profiling

```bash
# CPU profiling
cargo build --release
valgrind --tool=callgrind target/release/codex-cli-acp

# Memory profiling
valgrind --tool=massif target/release/codex-cli-acp
```

## Security Considerations

- Never log secrets or tokens
- Validate all input paths
- Sanitize protocol messages
- Check permissions before operations
- Use `anyhow` for error context

## Debugging

```bash
# Verbose logging
RUST_LOG=trace cargo run -p codex-cli-acp 2>trace.log

# With backtrace
RUST_BACKTRACE=full cargo run -p codex-cli-acp

# Test specific input
echo '{"method":"test"}' | cargo run -p codex-cli-acp
```

## CI/CD Integration

```bash
# Run full CI locally
../scripts/ci/run-local-ci.sh

# Individual checks
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features --locked
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-27T11:37:10Z"
document:
    type: "agent-memory"
    path: "crates/AGENTS.md"
    version: "1.0.0"
    last_updated: "2025-09-27T11:37:10Z"
    dependencies:
        - "../AGENTS.md"
        - "../sdd-rules/AGENTS.md"
        - "../.specify/memory/constitution.md"
```
