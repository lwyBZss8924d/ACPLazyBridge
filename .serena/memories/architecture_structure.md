# ACPLazyBridge Architecture and Structure

## High-Level Architecture
```
┌─────────────────┐     ┌──────────────┐     ┌─────────────┐
│   IDE/Editor    │────▶│  ACPLazyBridge│────▶│  AI Agent   │
│  (ACP Client)   │◀────│   (Bridge)    │◀────│  (Provider) │
└─────────────────┘     └──────────────┘     └─────────────┘
      JSON-RPC              stdio/JSONL          Native CLI
```

## Workspace Structure

### Root Files
- `Cargo.toml` - Workspace configuration
- `rust-toolchain.toml` - Rust version specification  
- `CLAUDE.md` / `AGENTS.md` - AI agent instructions
- `README.md` - Project documentation
- `CONTRIBUTING.md` - Development guidelines

### Crates Structure

#### `crates/acp-lazy-core/` - Core Library
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

#### `crates/codex-cli-acp/` - Codex Native Adapter
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

### Reference Materials (`local_refs/`)
- `agent-client-protocol/` - ACP specification
- `codex/` - Codex documentation
- `zed-acp-examples/` - Reference implementations

### Development Documentation (`dev-docs/`)

#### `requirements/`
- Project requirements and specifications
- Acceptance criteria

#### `design/`
- Architecture decisions
- Design documents
- Protocol mappings

#### `plan/`
- `issues/` - Task tracking
  - `m1-issue-list.md` - Current milestone tasks
  - `TEMPLATE.md` - Issue template
- `m1-technical-implementation-plan.md` - Implementation roadmap

#### `review/`
- `_artifacts/`
  - `tests/` - JSONL test scenarios
  - `logs/` - Execution logs
  - `jq/` - JSON processing filters
  - `IMPL.csv` - Symbol mapping
  - `traceability.csv` - Requirements tracking

## Key Components

### Protocol Flow
1. **Initialize**: Capability negotiation
2. **Session Management**: Create/load sessions
3. **Prompt Processing**: Handle user prompts
4. **Tool Calls**: Execute and respond to tool requests
5. **Streaming**: Real-time response delivery

### Permission System
- Maps ACP modes to provider parameters
- Non-interactive by default
- Configurable sandbox levels
- Network access control

### Event Processing
- Codex event deserialization
- Stream deduplication
- Idle timeout handling
- Turn completion detection

## Module Dependencies
```
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