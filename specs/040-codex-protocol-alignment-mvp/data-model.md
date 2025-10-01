```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/040-codex-protocol-alignment-mvp
feature_branch: feature/040-codex-protocol-alignment-mvp
date: 2025-10-01T04:22:17Z
created: 2025-09-30T15:35:21Z
last_updated: 2025-10-01T04:22:17Z
status: validated
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/52
specs:
    constitution: "1.0.1"
    type: data-model
    feature_number: 040
```

---

# Data Model: Codex Protocol Alignment MVP

## Core Entities

### 1. McpBridge

**Purpose**: TCP server that mediates between Codex MCP client and ACP ecosystem

```rust
pub struct McpBridge {
    /// TCP server address (127.0.0.1:port)
    address: SocketAddr,

    /// Server listener handle
    listener: TcpListener,

    /// Spawned acp_mcp_server process handle
    mcp_server_process: Child,

    /// Bridge session ID (distinct from ACP session)
    session_id: String,

    /// Lifecycle state
    state: BridgeState,
}

pub enum BridgeState {
    Starting,
    Running,
    ShuttingDown,
    Stopped,
}
```

**Relationships**:

- Owned by `SessionState` (1:1)
- Spawns `acp_mcp_server` binary (1:1)
- Accepts connections from Codex CLI MCP client (1:N)

**Lifecycle**:

1. Created on `session/new`
2. Binds to `127.0.0.1:0` (OS-assigned port)
3. Spawns `acp_mcp_server` with connection info
4. Transitions to `Running` when server accepts first connection
5. Cleaned up on `session/cancel` or session end

**Validation Rules**:

- Port must be >1024 and <65535
- Server must be listening before Codex CLI spawn
- Process cleanup must occur even on crash

---

### 2. McpServerProcess (acp_mcp_server binary)

**Purpose**: MCP server exposing 4 filesystem tools, translating to ACP Client API

```rust
pub struct McpServerConfig {
    /// TCP address to connect back to McpBridge
    bridge_address: SocketAddr,

    /// ACP client handle (for API calls)
    acp_client: Arc<dyn AcpClientApi>,

    /// Staged edits manager
    staged_edits: Arc<Mutex<StagedEditsManager>>,

    /// Tool registry
    tools: HashMap<String, ToolDefinition>,
}

pub struct ToolDefinition {
    name: String,
    description: String,
    input_schema: Value,  // JSON Schema
    handler: ToolHandler,
}
```

**Tool Handlers**:

- `read_text_file` → `handle_read_tool`
- `write_text_file` → `handle_write_tool`
- `edit_text_file` → `handle_edit_tool`
- `multi_edit_text_file` → `handle_multi_edit_tool`

**Relationships**:

- Spawned by `McpBridge` (1:1)
- Calls `AcpClientApi` for filesystem operations (N:1)
- Manages `StagedEditsManager` for multi-edit diffs (1:1)

---

### 3. SessionState (Enhanced)

**Purpose**: Extended session tracking for dual protocol support

```rust
pub struct SessionState {
    // Existing ACP session fields
    pub acp_session_id: SessionId,
    pub cwd: PathBuf,
    pub permission_mode: PermissionMode,

    // New bridge fields
    pub fs_session_id: Option<String>,
    pub mcp_bridge: Option<Arc<McpBridge>>,

    // New tracking fields
    pub reasoning_sections: Vec<String>,
    pub current_approval: Option<AskForApproval>,
    pub current_sandbox: SandboxPolicy,
    pub token_usage: Option<TokenUsage>,

    // Enhanced tool tracking
    pub tool_calls: HashMap<ToolCallId, ToolCallState>,

    // Existing fields
    pub messages: Vec<Message>,
    pub mcp_servers: Vec<McpServerConfig>,
}
```

**New Fields Explained**:

- `fs_session_id`: Bridge-specific session ID for debugging
- `mcp_bridge`: Optional bridge handle (None if no bridge needed)
- `reasoning_sections`: Accumulated reasoning text for aggregation
- `current_approval`: Pending approval request (ExecApproval or PatchApproval)
- `current_sandbox`: Current sandbox policy (from Codex CLI)
- `token_usage`: Accumulated token counts from Codex events

**Relationships**:

- Owns `McpBridge` (1:0..1)
- Tracks multiple `ToolCallState` instances (1:N)
- References ACP `SessionId` (1:1)

**Lifecycle**:

1. Created on `session/new` with ACP session ID
2. Optionally spawns `McpBridge` if Codex requires MCP
3. Accumulates state during `session/prompt` turns
4. Cleaned up on `session/cancel` or error

---

### 4. ToolCallState

**Purpose**: Extended tool call tracking with lifecycle metadata

```rust
pub struct ToolCallState {
    // Core ACP fields
    pub id: ToolCallId,
    pub kind: ToolKind,
    pub status: ToolCallStatus,
    pub title: String,

    // Enhanced tracking
    pub raw_input: Value,
    pub raw_output: Option<Value>,
    pub locations: Vec<ToolCallLocation>,
    pub content: Vec<ToolCallContent>,

    // Lifecycle metadata
    pub started_at: Instant,
    pub updated_at: Instant,
    pub is_truncated: bool,
    pub total_output_bytes: usize,
}

impl ToolCallState {
    pub fn begin(id: ToolCallId, kind: ToolKind, input: Value) -> Self;
    pub fn update_status(&mut self, status: ToolCallStatus);
    pub fn append_content(&mut self, content: ToolCallContent);
    pub fn complete(&mut self, output: Value);
    pub fn fail(&mut self, error: AcpError);
}
```

**Status Transitions**:

```txt
Pending ──(begin)──> InProgress ──(update)──> InProgress ──(complete)──> Completed
   │                     │
   └─────────────────────┴──────────(fail)─────────────────────────────> Failed
```

**Relationships**:

- Owned by `SessionState` (N:1)
- Maps to ACP `ToolCall` + `ToolCallUpdate` (1:N updates)

**Validation Rules**:

- Status transitions must be valid (e.g., Pending → InProgress → Completed)
- Cannot transition from Completed or Failed
- Output preview truncated at 10KB (MAX_OUTPUT_PREVIEW_BYTES)

---

### 5. StagedEditsManager

**Purpose**: In-memory file versioning for multi-edit operations with diff generation

```rust
pub struct StagedEditsManager {
    /// Staged file versions
    staged_files: HashMap<PathBuf, StagedFile>,
}

pub struct StagedFile {
    /// Original file content (from ACP client read)
    original_content: String,

    /// Currently staged content (after edits)
    staged_content: String,

    /// Applied edits history
    edits_applied: Vec<EditOp>,

    /// Cumulative diff (unified format)
    cumulative_diff: String,
}

pub struct EditOp {
    pub old_string: String,
    pub new_string: String,
    pub line_start: usize,
    pub line_end: usize,
}

impl StagedEditsManager {
    pub fn stage_file(&mut self, path: PathBuf, content: String);
    pub fn apply_edit(&mut self, path: &Path, edit: EditOp) -> Result<()>;
    pub fn generate_diff(&self, path: &Path) -> String;
    pub fn commit(&mut self, path: &Path) -> String;
    pub fn rollback(&mut self, path: &Path);
}
```

**Operations**:

1. **Stage**: Load original content into memory
2. **Apply Edit**: Modify staged content, track edit operation
3. **Generate Diff**: Produce unified diff between original and staged
4. **Commit**: Return final content, clear staged version
5. **Rollback**: Discard staged changes, restore original

**Diff Format** (Unified):

```diff
--- config.toml (original)
+++ config.toml (staged)
@@ -5,3 +5,3 @@
 [server]
-port = 8080
+port = 3000
 host = "localhost"
```

**Relationships**:

- Owned by `McpServerConfig` (1:1)
- Manages multiple `StagedFile` instances (1:N)

---

### 6. CodexEvent → ACP SessionUpdate Mappings

**Purpose**: Complete event mapping table for all 25 events

#### Category 1: Message Streaming (Existing)

| Codex Event | ACP SessionUpdate | Mapping Logic |
|-------------|-------------------|---------------|
| AgentMessage | AgentMessageChunk | Direct text content |
| AgentMessageDelta | AgentMessageChunk | Append delta to buffer |
| AgentReasoning | AgentThoughtChunk | Direct reasoning text |
| AgentReasoningDelta | AgentThoughtChunk | Append delta to buffer |

#### Category 2: Tool Lifecycle (Existing + New)

| Codex Event | ACP SessionUpdate | Mapping Logic |
|-------------|-------------------|---------------|
| ToolCall | ToolCall | Map tool name → ToolKind, status = Pending |
| ToolCalls | ToolCall (batch) | Emit multiple ToolCall updates |
| **ExecCommandBegin** | **ToolCall** | kind = Execute, status = Pending, input = command |
| **ExecCommandStdout** | **ToolCallUpdate** | status = InProgress, content = stdout |
| **ExecCommandStderr** | **ToolCallUpdate** | status = InProgress, content = stderr |
| **ExecCommandEnd** | **ToolCallUpdate** | status = Completed/Failed, output = final |
| **PatchApplyBegin** | **ToolCall** | kind = Edit/Delete, status = Pending, diff preview |
| **PatchApplyEnd** | **ToolCallUpdate** | status = Completed/Failed |
| **McpToolCallBegin** | **ToolCall** | kind = inferred from tool name, status = Pending |
| **McpToolCallEnd** | **ToolCallUpdate** | status = Completed/Failed |
| **ToolProgress** | **ToolCallUpdate** | status = InProgress, progress metadata |

#### Category 3: Plan & Reasoning (Existing + New)

| Codex Event | ACP SessionUpdate | Mapping Logic |
|-------------|-------------------|---------------|
| PlanUpdate | Plan | Map CodexPlanItem → PlanEntry with status |
| **PlanChunk** | **Plan** | Incremental plan update |
| **PlanComplete** | **Plan** | Final plan state |
| **ReasoningSection** | **AgentThoughtChunk** | Accumulate in reasoning_sections |
| **ReasoningComplete** | **AgentThoughtChunk** | Emit complete reasoning |

#### Category 4: Approval & Mode (New)

| Codex Event | ACP SessionUpdate | Mapping Logic |
|-------------|-------------------|---------------|
| **ExecApprovalRequest** | **ToolCall** | Emit pending ExecCommand, store in current_approval |
| **ExecApproved** | _(internal)_ | Update approval state, resume Codex CLI |
| **PatchApprovalRequest** | **ToolCall** | Emit pending PatchApply, store in current_approval |
| **PatchApproved** | _(internal)_ | Update approval state, resume Codex CLI |
| SessionConfigured | CurrentModeUpdate | Extract model info, emit mode update |

#### Category 5: Session Lifecycle (Existing)

| Codex Event | ACP SessionUpdate | Mapping Logic |
|-------------|-------------------|---------------|
| TaskComplete | _(internal)_ | Set stop_reason = EndTurn |
| Error | ToolCallUpdate | status = Failed, error details |
| McpListToolsResponse | AvailableCommandsUpdate | Map MCP tools + slash commands |

**Total Events**: 25 (11 existing + 14 new)

---

### 7. AskForApproval (New Entity)

**Purpose**: Pending approval request tracking

```rust
pub enum AskForApproval {
    ExecCommand {
        id: ToolCallId,
        command: Vec<String>,
        cwd: PathBuf,
        timeout_ms: Option<u64>,
        with_escalated_permissions: bool,
    },
    PatchApply {
        id: ToolCallId,
        file_path: PathBuf,
        diff: String,
        operation: PatchOperation,
    },
}

pub enum PatchOperation {
    Edit,
    Delete,
    Create,
}

impl AskForApproval {
    pub fn to_tool_call(&self) -> ToolCall;
    pub fn approve(&self) -> Value;  // Response to send to Codex CLI
    pub fn deny(&self) -> Value;
}
```

**Lifecycle**:

1. Created on `ExecApprovalRequest` or `PatchApprovalRequest`
2. Stored in `SessionState::current_approval`
3. Converted to ACP `ToolCall` with status = Pending
4. Resolved on user approval/denial
5. Cleared after resolution

---

### 8. SlashCommand (New Entity)

**Purpose**: Built-in slash command definitions and handlers

```rust
pub struct SlashCommand {
    pub name: String,
    pub description: String,
    pub handler: CommandHandler,
}

pub type CommandHandler = Arc<dyn Fn(&SessionState) -> Result<String> + Send + Sync>;

impl SlashCommand {
    pub fn status() -> Self;
    pub fn model() -> Self;
    pub fn approvals() -> Self;
    pub fn compact() -> Self;
    pub fn review() -> Self;
}
```

**Command Implementations**:

```rust
// /status - Show session info
fn handle_status(state: &SessionState) -> Result<String> {
    format!(
        "Session: {}\nModel: {}\nApproval: {:?}\nSandbox: {:?}\nTools: {}",
        state.acp_session_id,
        state.current_model,
        state.permission_mode,
        state.current_sandbox,
        state.tool_calls.len()
    )
}

// /model - Show current model
fn handle_model(state: &SessionState) -> Result<String> {
    format!("Current model: {}", state.current_model)
}

// /approvals - Show approval policy
fn handle_approvals(state: &SessionState) -> Result<String> {
    format!("Approval policy: {:?}", state.permission_mode)
}

// /compact - Compact message history (Codex internal)
fn handle_compact(state: &SessionState) -> Result<String> {
    Ok("Message history compacted".to_string())
}

// /review - Trigger code review mode (Codex feature)
fn handle_review(state: &SessionState) -> Result<String> {
    Ok("Code review mode activated".to_string())
}
```

**Relationships**:

- Registered globally (N:0)
- Executed within `SessionState` context (N:1)
- Emits `AgentMessageChunk` with result

---

## Data Flow Diagrams

### Flow 1: Session Initialization with Bridge

```txt
1. Zed → session/new (ACP)
   ↓
2. Create SessionState
   ↓
3. Spawn McpBridge
   │  ├─ Bind TCP 127.0.0.1:0
   │  ├─ Get port from OS
   │  └─ Store address
   ↓
4. Spawn acp_mcp_server binary
   │  └─ Pass bridge address as arg
   ↓
5. Wait for MCP server ready
   ↓
6. Return session/new response
```

### Flow 2: Tool Call with Bridge

```txt
1. Codex CLI → MCP tool call (read_text_file)
   ↓
2. acp_mcp_server receives call
   ↓
3. acp_mcp_server → ACP client.read_text_file()
   ↓
4. Zed → file content (ACP response)
   ↓
5. acp_mcp_server → format MCP response
   ↓
6. Codex CLI receives MCP response
   ↓
7. codex-cli-acp → emit ToolCall (pending)
   ↓
8. codex-cli-acp → emit ToolCallUpdate (completed)
```

### Flow 3: Multi-Edit with Staged Diffs

```txt
1. Codex CLI → MCP multi_edit_text_file
   ↓
2. acp_mcp_server.stage_file(path, original)
   ↓
3. For each edit in edits:
   │  ├─ apply_edit(old_str, new_str)
   │  └─ Update staged_content
   ↓
4. generate_diff(staged vs. original)
   ↓
5. acp_mcp_server → ACP client.write_text_file(staged)
   ↓
6. Return cumulative diff to Codex CLI
   ↓
7. codex-cli-acp → emit ToolCall + ToolCallUpdate
```

### Flow 4: Approval Flow

```txt
1. Codex → ExecApprovalRequest
   ↓
2. Create AskForApproval::ExecCommand
   ↓
3. Store in SessionState::current_approval
   ↓
4. Emit ToolCall (status = Pending, approval hint)
   ↓
5. Zed → user sees approval UI
   ↓
6. User approves/denies
   ↓
7. Zed → write approval response to Codex stdin
   ↓
8. Codex → ExecApproved event
   ↓
9. Clear SessionState::current_approval
   ↓
10. Emit ToolCallUpdate (status = InProgress)
```

---

## Validation Rules

### McpBridge

- ✅ TCP port must be available (retry on conflict)
- ✅ Server must be listening before Codex CLI spawn
- ✅ acp_mcp_server process must start within 1 second
- ✅ Cleanup must occur even if Codex CLI crashes

### SessionState

- ✅ `acp_session_id` must be unique
- ✅ `fs_session_id` generated independently
- ✅ `reasoning_sections` deduplicated (no consecutive duplicates)
- ✅ `current_approval` cleared after resolution
- ✅ `tool_calls` HashMap bounded (max 1000 per session)

### ToolCallState

- ✅ Status transitions must be valid (state machine enforced)
- ✅ Cannot transition from terminal states (Completed, Failed)
- ✅ `raw_output` truncated at 10KB
- ✅ `is_truncated` flag set if truncation occurs
- ✅ `total_output_bytes` tracks full size

### StagedEditsManager

- ✅ Original content immutable after staging
- ✅ Edits applied sequentially (order matters)
- ✅ Diff generated in unified format
- ✅ Commit clears staged version
- ✅ Rollback restores original

---

## Performance Considerations

### Memory Bounds

| Entity | Per-Instance | Per-Session | Mitigation |
|--------|--------------|-------------|------------|
| McpBridge | ~4KB | ~4KB | One per session |
| McpServerProcess | ~2MB | ~2MB | One per session |
| SessionState | ~16KB | ~16KB | One per session |
| ToolCallState | ~2KB | ~2MB (1000 max) | Bounded HashMap |
| StagedFile | ~50KB | ~500KB (10 max) | Auto-commit old files |

**Total per session**: ~7MB (well under 10MB target)

### Latency Targets

| Operation | Target | Measured (Reference) |
|-----------|--------|----------------------|
| Bridge spawn | <100ms | 15ms |
| Tool call round-trip | <5ms | 2-3ms |
| Diff generation | <10ms | 5ms (for 1KB file) |
| Event mapping | <1ms | 0.3ms |

---

## IMPORTANT TECHNICAL STANDARDS

- [ACP Protocol](https://github.com/zed-industries/agent-client-protocol) - ACPLazyBridge follows ACP Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - ACPLazyBridge follows ACP JSON Schema
- **ACP Repository local path**: ~/dev-space/agent-client-protocol
- **ACP Rust Library Version**: `agent-client-protocol = "0.4.4"`

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_

---
