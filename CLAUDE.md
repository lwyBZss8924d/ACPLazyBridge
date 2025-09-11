# CLAUDE.md - ACPLazyBridge Repository

## Repository Overview
ACPLazyBridge is an ACP (Agent Client Protocol) bridge that connects AI agents and agent-tools plugins with IDEs, editors, and development tools. It provides native adapters for various AI systems while maintaining protocol consistency and developer workflow integration.

## Authority and Governance

### Normative Documents (Authoritative)
- **Engineering Ground Rules**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **SDD Principles & Workflow**: [sdd-rules/spec-driven.md](sdd-rules/spec-driven.md)
- **SDD Lifecycle**: [sdd-rules/lifecycle.md](sdd-rules/lifecycle.md)
- **Team Rules**: [sdd-rules/AGENTS.md](sdd-rules/AGENTS.md)
- **Rules Index**: [sdd-rules/rules/README.md](sdd-rules/rules/README.md)

### Non-Normative References
- Engineering guides: `dev-docs/engineering/*` (each file links back to authority)
- Chinese documentation: `dev-docs/zh-CN/` (with disclaimer)

## SDD Developer Team Workflow

### Specification-Driven Development
Every feature or change follows the SDD workflow:
1. **Specify**: Create specification under `specs/<NNN>-<slug>/spec.md`
2. **Plan**: Technical design in `specs/<NNN>-<slug>/plan.md`
3. **Tasks**: Breakdown in `specs/<NNN>-<slug>/tasks.md`
4. **Implement**: Follow worktree-first development
5. **Validate**: Local CI checks and evidence collection
6. **Review**: PR with full traceability

### SDD Commands
- `/specify` - Generate feature specification
- `/plan` - Create implementation plan
- `/tasks` - Derive executable tasks

See [sdd-rules/commands/](sdd-rules/commands/) for details.

## Project Navigation

### Core Structure
```
├── crates/              # Rust workspace
│   ├── acp-lazy-core/   # Core protocol implementation
│   └── codex-cli-acp/   # Codex CLI adapter
├── scripts/             # CI and automation
│   ├── ci/              # CI scripts
│   └── sdd/             # SDD validation
├── specs/               # SDD specifications
├── sdd-rules/           # Development rules
│   ├── rules/           # Categorized rules
│   └── commands/        # SDD command docs
├── dev-docs/            # Development docs
│   ├── review/          # Review artifacts
│   │   └── _artifacts/  # Evidence storage
│   └── engineering/     # Non-normative guides
└── issues/              # Issue templates
```

### Key Paths
- Evidence: `dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/`
- Worktrees: `/Users/arthur/dev-space/acplb-worktrees/`
- IDE links: `.worktrees/` (symlinks to worktrees)

## ACP Protocol Implementation

### Version
**Current**: ACP v1 (protocolVersion: 1 as integer)

### Protocol Examples
```jsonl
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1,"capabilities":{}}}
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"capabilities":{},"serverInfo":{"name":"codex-cli-acp","version":"0.1.0"}}}
{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"session_123","content":"Processing..."}}
```

### Key Conventions
- **stdout**: Reserved for JSON-RPC/JSONL protocol messages only
- **stderr**: All logs, debug output, and diagnostics
- **Format**: One JSON message per line, newline-terminated

## Quality Gates

### Local CI Checks (Must Pass)
```bash
# Rust quality
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features --locked

# SDD compliance
scripts/ci/run-sdd-structure-lint.sh
scripts/ci/check-language-policy.sh

# Local comprehensive check
scripts/ci/run-local-ci.sh
```

### Constitutional Gates
- **Simplicity** (Article VII): ≤3 projects, no future-proofing
- **Anti-Abstraction** (Article VIII): Use framework features directly
- **Integration-First** (Article IX): Contracts defined before implementation
- **Test-First** (Article III): Tests fail (RED) before implementation

## Development Workflow

### Branch Management
- **Categories**: `feature | fix | perf | chore | docs`
- **Format**: `<category>/<NNN>-<module>-<description>`
- **Worktree-first**: Never develop on main

### Worktree Creation
```bash
# Create worktree from origin/main
git -C /Users/arthur/dev-space/ACPLazyBridge worktree add \
  /Users/arthur/dev-space/acplb-worktrees/<task-dir> \
  origin/main -b <branch>

# Optional IDE navigation
ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> \
  /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>
```

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

### Running the Codex Adapter
```bash
# Run with default settings
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

## Security and Compliance

### Security Policies
- No secrets in code or logs
- Use environment variables for sensitive data
- GitHub secrets for CI/CD
- Code scanning enabled (GitHub + local CodeQL)

### Language Policy
- **Normative artifacts**: English only (specs, plans, tasks, issues)
- **Chinese docs**: Allowed as non-normative under `dev-docs/zh-CN/`
- **Conversations**: Any language acceptable

## Team AI Agents

AI Developer coding agents team members include:
- **CLAUDE** "Claude Code Agent" (anthropic Claude Code CLI client link - `CLAUDE.md`)
- **WARP** "WARP Agent" (WARP.dev Terminal link - `WARP.md`)
- **GEMINI** "GEMINI CLI Agent" (google gemini-cli link - `GEMINI.md`)
- **CURSOR** "Cursor Agent" (Cursor IDE cursor-agent link - cursor rules file `.cursorrules`)
- **CODEX** "CODEX Agent" (OpenAI codex-cli link - `AGENTS.md`)

All AI Developers coding agent's client rules in this codebase ~/ root file are uniformly linked to [sdd-rules/AGENTS.md](sdd-rules/AGENTS.md)

## Implementation Status

### Completed (M0)
- Rust workspace bootstrapped
- References vendored
- SDD rules integrated

### In Progress (M1)
- Codex native adapter
  - stdio loop implementation
  - Streaming support
  - Tool call mapping
  - Permission system
  - Smoke testing

### Planned
- Proxy adapter
- Plugin system v0
- Native adapters
- HTTP/SSE bridge

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
- `fs/create_directory`: Create directories (requires `fs.createDirectory` capability)
- `fs/delete`: Delete files/directories (requires `fs.delete` capability)
- `fs/list_directory`: List directory contents (requires `fs.listDirectory` capability)

For complete implementation details, see [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)
