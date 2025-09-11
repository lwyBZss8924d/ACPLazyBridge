# ACPLazyBridge Architecture and Structure

## High-Level Architecture

```text
┌─────────────────┐     ┌──────────────┐     ┌─────────────┐
│   IDE/Editor    │────▶│  ACPLazyBridge│────▶│  AI Agent   │
│  (ACP Client)   │◀────│   (Bridge)    │◀────│  (Provider) │
└─────────────────┘     └──────────────┘     └─────────────┘
      JSON-RPC              stdio/JSONL          Native CLI
```

## Workspace Structure

### Root Configuration Files

- `Cargo.toml` - Workspace configuration
- `rust-toolchain.toml` - Rust version specification
- `.markdownlint.json` - Markdown style configuration
- `CLAUDE.md` - Claude Code agent instructions
- `AGENTS.md` - Team AI agents rules (links to sdd-rules/AGENTS.md)
- `WARP.md` - WARP agent instructions
- `CONTRIBUTING.md` - Engineering ground rules
- `README.md` - Project documentation
- `ROADMAP.md` - Project roadmap

### Core Directories

#### `crates/` - Rust Workspace

##### `crates/acp-lazy-core/` - Core Library

Shared utilities and foundational components:

- **`src/permissions.rs`**
  - Maps ACP permission modes to provider-specific parameters
  - Handles non-interactive approval policies
  - Sandbox mode configuration

- **`src/transport.rs`**
  - Spawn/stdio communication utilities
  - Line-based JSON reading/writing
  - Stream handling primitives

- **`src/logging.rs`**
  - Tracing initialization
  - Log configuration
  - stderr/stdout separation

##### `crates/codex-cli-acp/` - Codex Native Adapter

Binary implementation for Codex CLI integration:

- **`src/main.rs`**
  - Entry point and CLI handling
  - ACP server implementation
  - Message routing and dispatch

- **`src/codex_proto.rs`**
  - Codex event parsing
  - Stream management
  - Protocol translation layer

- **`src/bin/`**
  - `acplb_notify_forwarder.rs` - Notification forwarding utility
  - `playback.rs` - Test playback tool

#### `sdd-rules/` - Specification-Driven Development Rules

**Normative authority for development process:**

- **`spec-driven.md`** - SDD principles and workflow
- **`lifecycle.md`** - SDD lifecycle phases
- **`AGENTS.md`** - Team AI agents rules
- **`commands/`** - SDD command documentation
  - `/specify` command docs
  - `/plan` command docs
  - `/tasks` command docs
- **`templates/`** - SDD document templates
  - `spec-template.md`
  - `plan-template.md`
  - `tasks-template.md`
- **`rules/`** - Categorized development rules
  - `documentation-style/` - Markdown and doc standards
  - `git/` - Git workflow rules (worktree, PR, issues)
  - `ci/` - CI/CD requirements
  - `tests/` - Testing standards
  - `code-analysis/` - Code quality rules
  - `tools-cli/` - CLI tool guidelines
  - `tools-mcp/` - MCP tool guidelines
  - `research/` - Research methodology
  - `changelog/` - Changelog standards

#### `specs/` - SDD Specifications

Feature specifications following SDD lifecycle:

```tree
specs/
├── 000-example/           # Example specification
│   ├── spec.md           # Feature specification
│   ├── TASK-PLAN.md      # Implementation plan
│   └── ISSUE.md          # Issue template
└── 001-claude-memory-sdd-alignment/
    ├── spec.md           # Feature specification
    ├── plan.md           # Technical plan
    └── tasks.md          # Task breakdown
```

#### `scripts/` - Automation Scripts

##### `scripts/ci/` - CI Scripts

- `run-local-ci.sh` - Complete local CI validation
- `run-sdd-structure-lint.sh` - SDD structure validation
- `check-language-policy.sh` - Language policy enforcement
- Various other validation scripts

##### `scripts/sdd/` - SDD Validation

- `validate_structure.py` - Python SDD structure validator
- `run_semantic_checks.sh` - Semantic validation script

#### `dev-docs/` - Development Documentation

##### `dev-docs/references/` - Reference Materials

- `acp.md` - ACP specification
- `acp_adapters/` - Adapter documentation
  - `claude_code_acp.md` - Claude Code ACP adapter
- `cli_agents/` - CLI agent references
  - `ClaudeCode/` - Claude Code documentation
  - `codex.md` - Codex CLI documentation
  - `gemini.md` - Gemini CLI documentation
- `zed_ide.md` - Zed IDE integration

##### `dev-docs/engineering/` - Engineering Guides

Non-normative guides linking to authority:

- `workflow.md` - Reference maintenance workflow
- `codeql.md` - Security analysis

##### `dev-docs/review/` - Review Artifacts

- `_artifacts/` - Evidence storage
  - `tests/` - Test scenarios (JSONL)
  - `logs/` - Execution logs
  - `jq/` - JSON processing filters
  - `reports/` - Test reports
  - `IMPL.csv` - Symbol mapping
  - `traceability.csv` - Requirements tracking

##### `dev-docs/zh-CN/` - Chinese Documentation

Non-normative Chinese language documentation (with disclaimer)

### Supporting Directories

#### `.github/` - GitHub Configuration

- `workflows/` - GitHub Actions
- Issue and PR templates

#### `.serena/` - Serena MCP Memories

Project context and knowledge base

#### `queries/` - CodeQL Queries

Custom security analysis queries

#### `memory/` - Project Memory

Historical context and decisions

## Protocol Flow

1. **Initialize**: Capability negotiation
2. **Session Management**: Create/load sessions
3. **Prompt Processing**: Handle user prompts
4. **Tool Calls**: Execute and respond to tool requests
5. **Streaming**: Real-time response delivery

## Permission System

- Maps ACP modes to provider parameters
- Non-interactive by default
- Configurable sandbox levels:
  - `read-only` - No file system writes
  - `workspace-write` - Write within workspace
  - `full` - Unrestricted access
- Network access control

## Event Processing

- Codex event deserialization
- Stream deduplication
- Idle timeout handling
- Turn completion detection

## Module Dependencies

```tree
codex-cli-acp
    ├── acp-lazy-core (permissions, transport, logging)
    ├── serde_json (JSON handling)
    ├── tokio (async runtime)
    └── anyhow (error handling)

acp-lazy-core
    ├── tracing (logging)
    ├── serde (serialization)
    └── tokio (async primitives)
```

## Extension Points

- Plugin system (planned)
- Additional provider adapters
- HTTP/SSE bridge
- Custom permission policies

## Key Paths

- **Evidence**: `dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/`
- **Worktrees**: `/Users/arthur/dev-space/acplb-worktrees/`
- **IDE links**: `.worktrees/` (symlinks to worktrees)
- **Specs**: `specs/<NNN>-<slug>/`
- **SDD Rules**: `sdd-rules/rules/`

---

Specification Version: 1.0.3 | architecture_structure.md ("serena" MCP's memories) Format: 1.0 | Last Updated: 2025-09-11
