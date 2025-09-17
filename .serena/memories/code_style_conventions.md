# Code Style and Conventions for ACPLazyBridge

## Rust Conventions

- Use Rust stable toolchain (defined in rust-toolchain.toml)
- Follow standard Rust naming conventions:
    - snake_case for functions and variables
    - PascalCase for types and traits
    - SCREAMING_SNAKE_CASE for constants
- Use `Result<T>` for fallible operations
- Prefer `anyhow::Result` for error handling in binaries
- Use `thiserror` for library error types

## Project Structure

- Workspace with two main crates:
    - `acp-lazy-core`: Shared utilities and core functionality
    - `codex-cli-acp`: Binary implementation for Codex adapter
- Modular design with clear separation of concerns
- Each module should have a focused responsibility

## ACP Protocol Requirements

- Strict JSON-RPC 2.0 message format
- Line-separated JSON (JSONL) - one message per line
- All file paths MUST be absolute
- Line numbers are 1-based
- Messages use proper error codes: -32700/-32600/-32601/-32602/-32603

## Logging and Output Discipline

- **CRITICAL**: stdout must be strict JSONL only
- All logs must go to stderr
- Use `tracing` crate for structured logging
- Never print debug info to stdout
- Protocol messages only on stdout

## Permission Mapping

- Default to non-interactive modes (`approval_policy=never`)
- Map ACP modes to Codex parameters explicitly
- Document security implications clearly
- YOLO/danger modes require explicit opt-in with warnings

## Documentation

- Document public APIs with `///` doc comments
- Include examples in doc comments where helpful
- Update CLAUDE.md for AI agent instructions
- Chinese docs in dev-docs/ for implementation details

## Testing

- Write unit tests for protocol parsing
- Integration tests for message flows
- Mock provider responses for testing
- Test permission mapping thoroughly

## Import Organization

- Group imports: std, external crates, internal modules
- Use explicit imports rather than glob imports
- Prefer importing types/traits at module level

## Error Handling

- Use descriptive error messages
- Include context in errors
- Chain errors appropriately
- Never panic in library code

## Async Code

- Use `tokio` runtime
- Prefer async/await over raw futures
- Handle cancellation properly
- Avoid blocking operations in async context

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "serena-memories"
    memories: "code_style_conventions"
    status: "expired"
    path: ".serena/memories/code_style_conventions.md"
    version: "1.0.1"
    last_updated: "2025-09-14T08:26:00Z"
```
