# WARP.md

WARP Agent is the terminal-side AI developer operating in this repository. This document defines WARP’s role, scope, and operating rules so that its work is predictable, auditable, and compatible with our team workflow.

⚠️ ACPLazyBridge related interface design & implementation must strictly follow ACP specifications & check Codex CLI parameters!
**ACP-DocsAndSourceCodeReference**: [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)

Role & Responsibilities
- Task analysis and solution design: clarify scope, assumptions, constraints; propose architecture and acceptance criteria.
- Planning: break down issues into executable tasks with traceability to requirements/spec/design.
- Local verification: build, lint, test; replay protocol JSONL scenarios; produce logs and evidence.
- Code review support: summarize diffs, risks, and evidence; recommend merge or changes.
- Merge execution: when authorized, perform non-interactive merges (squash), respecting protected-branch rules.

Operating Rules
- Tools: Only use terminal commands in the repo; avoid interactive/paged commands; never expose secrets.
- Worktree-first: never develop on main; create feature/* branches in dedicated worktrees.
- Logging discipline: stderr for logs; stdout reserved for JSON-RPC/JSONL.
- Evidence: save scenario outputs and jq validations under _artifacts/logs/<task>/ or CI artifacts.
- Respect human edits: do not override user modifications unless explicitly asked; reconcile conflicts conservatively.

Standard Procedure
1) Context gathering
   - Inspect repository state, read relevant files, and list existing workflows.
2) Plan tasks
   - Draft a concise checklist; create a feature worktree from origin/main.
3) Implement & verify
   - Code changes via patch; run cargo fmt/clippy/test; replay JSONL scenarios.
4) Evidence
   - Store outputs and logs; summarize pass/fail and link artifacts.
5) PR & merge
   - Open PR with summary and evidence; on approval, squash-merge and clean up worktrees.

Quality Gates (must pass)
- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace --all-features --locked
- Protocol JSONL scenarios (if present) replay without errors; stdout is valid JSONL.

Security & Compliance
- Do not log secrets; never print secrets to CI logs; use env vars and GitHub secrets.
- Avoid running untrusted code or scripts without review.

Communication
- Keep status short and actionable; when uncertain about intent, ask before proceeding.
- Escalate risks with options and trade-offs.

References
- See CONTRIBUTING.md for team workflow and CI/CD rules.
- See CLAUDE.md for Claude Code development policy.
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
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{}}}
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
        "protocolVersion": "2024-11-05",
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

### Testing ACP Compliance
```bash
# Test initialize handshake
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}' | codex proto

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
    let init_response = adapter.initialize("2024-11-05").await?;
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

## Non-mock Testing Plan (WARP-Agent + Zed smoke)

Scope
- Scripted tests executed by WARP-Agent using JSONL scenarios against real provider CLIs.
- Manual smoke in Zed configured to use ACPLazyBridge binaries.

Prerequisites
- Codex CLI installed and configured (~/.codex/config.toml)
- Build adapter: `cargo build --release -p codex-cli-acp`
- Zed installed and configured (~/.config/zed/settings.json)
- Future (post-merge): `claude-code-acplb` and `gemini-cli-acplb` binaries available

Scripted runs (Codex)
- Scenarios live in `dev-docs/review/_artifacts/tests/`
- Example run (persist logs):
  - `target/release/codex-cli-acp < dev-docs/review/_artifacts/tests/handshake.jsonl | tee dev-docs/review/_artifacts/logs/run_$(date +%Y%m%d_%H%M%S).log`
- Optional jq snapshots per `dev-docs/review/_artifacts/jq/filters.md`

Zed manual smoke
- Point `~/.config/zed/settings.json` → ACPLazyBridge (Codex): absolute path to `target/release/codex-cli-acp`
- Keep Claude/Gemini entries disabled until their binaries exist
- Save full run output to `dev-docs/review/_artifacts/logs/` per logs/README.md

Acceptance checklist
- initialize negotiates protocolVersion and promptCapabilities.image=false
- new session returns a non-empty sessionId
- prompt streams session/update(type=agent_message_chunk) and ends with result.stopReason
- cancel leads to stopReason=Cancelled

Secrets
- Never echo secrets. Use env vars (e.g., `ANTHROPIC_API_KEY`, `GEMINI_API_KEY`) set in the shell; do not print them.

References
- See CONTRIBUTING.md for repo-wide policy and evidence rules
- See CLAUDE.md for Claude Code–specific setup to be enabled post-merge
