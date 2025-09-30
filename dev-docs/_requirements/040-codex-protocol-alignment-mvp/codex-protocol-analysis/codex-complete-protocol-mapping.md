# Codex CLI Complete Protocol Mapping

**Analysis Date:** 2025-09-30
**Source Directories:**

- `/Users/arthur/dev-space/codex/docs/`
- `/Users/arthur/dev-space/codex/codex-rs/`
- `/Users/arthur/dev-space/codex/sdk/typescript/`

**Purpose:** Comprehensive mapping of Codex CLI protocol, event types, tool structures, and capabilities for ACP bridge implementation.

---

## Table of Contents

1. [Protocol Overview](#protocol-overview)
2. [Event Types and Schemas](#event-types-and-schemas)
3. [Tool Call Structures](#tool-call-structures)
4. [Submission Format](#submission-format)
5. [Session Configuration](#session-configuration)
6. [Slash Commands](#slash-commands)
7. [Custom Prompts](#custom-prompts)
8. [Proto Mode vs JSON Mode](#proto-mode-vs-json-mode)
9. [Approval Flows](#approval-flows)
10. [MCP Server Integration](#mcp-server-integration)
11. [Notifications and Turn Completion](#notifications-and-turn-completion)

---

## 1. Protocol Overview

### Architecture

- **Source:** `codex-rs/docs/protocol_v1.md`, `codex-rs/protocol/src/protocol.rs`
- **Pattern:** Submission Queue (SQ) / Event Queue (EQ)
- **Communication:** Bidirectional streaming over stdin/stdout using newline-delimited JSON (JSONL)

### Core Entities

#### Codex Core Engine

- Runs locally in background thread or separate process
- Communicates via SQ (Submission Queue) and EQ (Event Queue)
- Takes user input, makes requests to Model, executes commands, applies patches

#### Session

- Current configuration and state of Codex
- Initialized by `Op::ConfigureSession` (deprecated, use `Op::UserTurn`)
- Can be reconfigured; running execution is aborted on reconfiguration

#### Task

- Codex executing work in response to user input
- Session has at most one Task running at a time
- Started by `Op::UserInput` or `Op::UserTurn`
- Consists of series of Turns
- Terminates when:
    - Model completes task with no output for next Turn
    - Additional `Op::UserInput` aborts current task
    - UI interrupts with `Op::Interrupt`
    - Fatal errors (e.g., Model connection exceeding retry limits)
    - Blocked by user approval

#### Turn

- One cycle of iteration in a Task
- Flow:
  1. Request to Model (prompt + optional `last_response_id` or previous turn output)
  2. Model streams responses via SSE until "completed" message
  3. Codex executes commands, applies patches, outputs messages
  4. Pauses for approval when necessary
- Output of one Turn is input to next Turn
- Turn yielding no output terminates the Task
- `response_id` stored in Session to resume thread

### Transport

- Supports: cross-thread channels, IPC, stdin/stdout, TCP, HTTP2, gRPC
- **Non-framed transports** (stdin/stdout, TCP): use newline-delimited JSON

---

## 2. Event Types and Schemas

### Submission Types (Op)

**Source:** `codex-rs/protocol/src/protocol.rs` (lines 49-179)

```rust
pub enum Op {
    // Core Operations
    Interrupt,
    UserInput { items: Vec<InputItem> },
    UserTurn {
        items: Vec<InputItem>,
        cwd: PathBuf,
        approval_policy: AskForApproval,
        sandbox_policy: SandboxPolicy,
        model: String,
        effort: Option<ReasoningEffortConfig>,
        summary: ReasoningSummaryConfig,
        final_output_json_schema: Option<Value>,
    },
    OverrideTurnContext {
        cwd: Option<PathBuf>,
        approval_policy: Option<AskForApproval>,
        sandbox_policy: Option<SandboxPolicy>,
        model: Option<String>,
        effort: Option<Option<ReasoningEffortConfig>>,
        summary: Option<ReasoningSummaryConfig>,
    },

    // Approval Operations
    ExecApproval { id: String, decision: ReviewDecision },
    PatchApproval { id: String, decision: ReviewDecision },

    // History Operations
    AddToHistory { text: String },
    GetHistoryEntryRequest { offset: usize, log_id: u64 },

    // Query Operations
    GetPath,
    ListMcpTools,
    ListCustomPrompts,
    Compact,
    Review { review_request: ReviewRequest },

    // Lifecycle
    Shutdown,
}
```

#### InputItem Types

```rust
pub enum InputItem {
    Text { text: String },
    Image { image_url: String },  // Pre-encoded data URI
    LocalImage { path: PathBuf },  // Converted to base64 data URL during serialization
}
```

#### AskForApproval Policy

```rust
pub enum AskForApproval {
    UnlessTrusted,  // Auto-approve only "known safe" read-only commands
    OnFailure,      // Auto-approve all, escalate on sandbox failure
    OnRequest,      // Model decides when to ask (default)
    Never,          // Never ask, failures returned to model
}
```

#### SandboxPolicy

```rust
pub enum SandboxPolicy {
    DangerFullAccess,  // No restrictions
    ReadOnly,          // Read-only file system access
    WorkspaceWrite {
        writable_roots: Vec<PathBuf>,
        network_access: bool,
        exclude_tmpdir_env_var: bool,
        exclude_slash_tmp: bool,
    },
}
```

#### ReviewDecision

```rust
pub enum ReviewDecision {
    Approved,             // Execute this command
    ApprovedForSession,   // Execute and auto-approve identical future commands
    Denied,               // Don't execute but continue session
    Abort,                // Don't execute and wait for next user command
}
```

---

### Event Types (EventMsg)

**Source:** `codex-rs/protocol/src/protocol.rs` (lines 420-522)

```rust
pub enum EventMsg {
    // Errors and Status
    Error(ErrorEvent),
    TaskStarted(TaskStartedEvent),
    TaskComplete(TaskCompleteEvent),
    TokenCount(TokenCountEvent),

    // Agent Messages
    AgentMessage(AgentMessageEvent),
    UserMessage(UserMessageEvent),
    AgentMessageDelta(AgentMessageDeltaEvent),

    // Reasoning Events
    AgentReasoning(AgentReasoningEvent),
    AgentReasoningDelta(AgentReasoningDeltaEvent),
    AgentReasoningRawContent(AgentReasoningRawContentEvent),
    AgentReasoningRawContentDelta(AgentReasoningRawContentDeltaEvent),
    AgentReasoningSectionBreak(AgentReasoningSectionBreakEvent),

    // Session Management
    SessionConfigured(SessionConfiguredEvent),

    // Tool Calls
    McpToolCallBegin(McpToolCallBeginEvent),
    McpToolCallEnd(McpToolCallEndEvent),
    WebSearchBegin(WebSearchBeginEvent),
    WebSearchEnd(WebSearchEndEvent),

    // Command Execution
    ExecCommandBegin(ExecCommandBeginEvent),
    ExecCommandOutputDelta(ExecCommandOutputDeltaEvent),
    ExecCommandEnd(ExecCommandEndEvent),
    ExecApprovalRequest(ExecApprovalRequestEvent),

    // Patch Application
    ApplyPatchApprovalRequest(ApplyPatchApprovalRequestEvent),
    PatchApplyBegin(PatchApplyBeginEvent),
    PatchApplyEnd(PatchApplyEndEvent),

    // Other Events
    BackgroundEvent(BackgroundEventEvent),
    StreamError(StreamErrorEvent),
    TurnDiff(TurnDiffEvent),
    GetHistoryEntryResponse(GetHistoryEntryResponseEvent),
    McpListToolsResponse(McpListToolsResponseEvent),
    ListCustomPromptsResponse(ListCustomPromptsResponseEvent),
    PlanUpdate(UpdatePlanArgs),
    TurnAborted(TurnAbortedEvent),
    ShutdownComplete,
    ConversationPath(ConversationPathResponseEvent),
    EnteredReviewMode(ReviewRequest),
    ExitedReviewMode(ExitedReviewModeEvent),
}
```

### Key Event Payload Structures

#### ExecCommandBeginEvent

```rust
pub struct ExecCommandBeginEvent {
    pub call_id: String,
    pub command: Vec<String>,
    pub cwd: PathBuf,
    pub parsed_cmd: Vec<ParsedCommand>,
}
```

#### ExecCommandEndEvent

```rust
pub struct ExecCommandEndEvent {
    pub call_id: String,
    pub stdout: String,
    pub stderr: String,
    pub aggregated_output: String,
    pub exit_code: i32,
    pub duration: Duration,
    pub formatted_output: String,
}
```

#### ExecCommandOutputDeltaEvent

```rust
pub struct ExecCommandOutputDeltaEvent {
    pub call_id: String,
    pub stream: ExecOutputStream,  // Stdout or Stderr
    pub chunk: Vec<u8>,  // Base64 encoded in JSON
}
```

#### TokenCountEvent

```rust
pub struct TokenCountEvent {
    pub info: Option<TokenUsageInfo>,
    pub rate_limits: Option<RateLimitSnapshot>,
}

pub struct TokenUsageInfo {
    pub total_token_usage: TokenUsage,
    pub last_token_usage: TokenUsage,
    pub model_context_window: Option<u64>,
}

pub struct TokenUsage {
    pub input_tokens: u64,
    pub cached_input_tokens: u64,
    pub output_tokens: u64,
    pub reasoning_output_tokens: u64,
    pub total_tokens: u64,
}
```

---

### Exec Thread Events (Simplified Event Model)

**Source:** `codex-rs/exec/src/exec_events.rs`

These are higher-level events used in the `codex exec` streaming API:

```rust
pub enum ThreadEvent {
    ThreadStarted(ThreadStartedEvent),
    TurnStarted(TurnStartedEvent),
    TurnCompleted(TurnCompletedEvent),
    TurnFailed(TurnFailedEvent),
    ItemStarted(ItemStartedEvent),
    ItemUpdated(ItemUpdatedEvent),
    ItemCompleted(ItemCompletedEvent),
    Error(ThreadErrorEvent),
}

pub enum ThreadItemDetails {
    AssistantMessage(AssistantMessageItem),
    Reasoning(ReasoningItem),
    CommandExecution(CommandExecutionItem),
    FileChange(FileChangeItem),
    McpToolCall(McpToolCallItem),
    WebSearch(WebSearchItem),
    TodoList(TodoListItem),
    Error(ErrorItem),
}
```

#### CommandExecutionItem

```rust
pub struct CommandExecutionItem {
    pub command: String,
    pub aggregated_output: String,
    pub exit_code: Option<i32>,
    pub status: CommandExecutionStatus,  // InProgress | Completed | Failed
}
```

#### FileChangeItem

```rust
pub struct FileChangeItem {
    pub changes: Vec<FileUpdateChange>,
    pub status: PatchApplyStatus,  // Completed | Failed
}

pub struct FileUpdateChange {
    pub path: String,
    pub kind: PatchChangeKind,  // Add | Delete | Update
}
```

---

## 3. Tool Call Structures

### Built-in Tools

**Source:** `codex-rs/core/src/openai_tools.rs` (lines 40-277)

```rust
pub enum OpenAiTool {
    Function(ResponsesApiTool),
    LocalShell {},
    WebSearch {},
    Freeform(FreeformTool),
}
```

### Shell Tool

#### Standard Shell Tool

```rust
// Parameters
{
    "command": ["array", "of", "strings"],
    "workdir": "string (optional)",
    "timeout_ms": "number (optional)",
    "with_escalated_permissions": "boolean (optional)",
    "justification": "string (required if with_escalated_permissions=true)"
}
```

**Required fields:** `command` (array of strings)

#### Unified Exec Tool (Experimental)

**Source:** `codex-rs/core/src/openai_tools.rs` (lines 161-206)

```rust
// Parameters
{
    "input": ["array", "of", "strings"],
    "session_id": "string (optional)",
    "timeout_ms": "number (optional)"
}
```

**Description:** Runs a command in a PTY. When `session_id` is omitted, spawns a new command. When provided, concatenates input strings and writes to the session's stdin.

### Apply Patch Tool

**Source:** `codex-rs/core/src/tool_apply_patch.rs`

Two formats available:

#### 1. Freeform Tool (custom XML-like format)

```rust
pub struct FreeformTool {
    pub name: String,
    pub description: String,
    pub format: FreeformToolFormat,
}

pub struct FreeformToolFormat {
    pub r#type: String,
    pub syntax: String,
    pub definition: String,
}
```

#### 2. Function Tool (JSON schema)

```rust
// Parameters
{
    "input": "string"
}
```

### Plan Tool

**Source:** `codex-rs/core/src/plan_tool.rs`, `codex-rs/protocol/src/plan_tool.rs`

```rust
pub struct UpdatePlanArgs {
    pub list_items: Vec<PlanItem>,
}

pub struct PlanItem {
    pub text: String,
    pub completed: bool,
}
```

### MCP Tool Invocation

**Source:** `codex-rs/protocol/src/protocol.rs` (lines 815-849)

```rust
pub struct McpInvocation {
    pub server: String,  // Name of MCP server from config
    pub tool: String,    // Name of tool from MCP server
    pub arguments: Option<serde_json::Value>,
}

pub struct McpToolCallBeginEvent {
    pub call_id: String,
    pub invocation: McpInvocation,
}

pub struct McpToolCallEndEvent {
    pub call_id: String,
    pub invocation: McpInvocation,
    pub duration: Duration,
    pub result: Result<CallToolResult, String>,
}
```

---

## 4. Submission Format

### UserInput Structure

**Source:** `codex-rs/protocol/src/protocol.rs` (lines 59-93)

```json
{
  "id": "unique-submission-id",
  "op": {
    "type": "user_input",
    "items": [
      { "type": "text", "text": "Your prompt here" },
      { "type": "image", "image_url": "data:image/png;base64,..." },
      { "type": "local_image", "path": "/path/to/image.png" }
    ]
  }
}
```

### UserTurn Structure (Preferred for Proto Mode)

```json
{
  "id": "unique-submission-id",
  "op": {
    "type": "user_turn",
    "items": [
      { "type": "text", "text": "Your prompt here" }
    ],
    "cwd": "/absolute/path/to/working/directory",
    "approval_policy": "on-request",
    "sandbox_policy": {
      "mode": "workspace-write",
      "writable_roots": [],
      "network_access": false,
      "exclude_tmpdir_env_var": false,
      "exclude_slash_tmp": false
    },
    "model": "gpt-5-codex",
    "effort": "medium",
    "summary": "auto"
  }
}
```

### Input Context Types

**Source:** `codex-rs/protocol/src/protocol.rs` (lines 723-763)

Three types of input message kinds:

```rust
pub enum InputMessageKind {
    Plain,              // Regular user text
    UserInstructions,   // Wrapped in <user_instructions>...</user_instructions>
    EnvironmentContext, // Wrapped in <environment_context>...</environment_context>
}
```

**XML Tag Constants:**

```rust
pub const USER_INSTRUCTIONS_OPEN_TAG: &str = "<user_instructions>";
pub const USER_INSTRUCTIONS_CLOSE_TAG: &str = "</user_instructions>";
pub const ENVIRONMENT_CONTEXT_OPEN_TAG: &str = "<environment_context>";
pub const ENVIRONMENT_CONTEXT_CLOSE_TAG: &str = "</environment_context>";
pub const USER_MESSAGE_BEGIN: &str = "## My request for Codex:";
```

---

## 5. Session Configuration

### Configuration Options

**Source:** `docs/config.md`

#### Model Configuration

```toml
model = "gpt-5-codex"
model_provider = "openai"
model_context_window = 200000
model_max_output_tokens = 16000
model_reasoning_effort = "medium"  # minimal | low | medium | high
model_reasoning_summary = "auto"   # auto | concise | detailed | none
model_verbosity = "medium"         # low | medium | high (GPT-5 only)
```

#### Approval and Sandbox Configuration

```toml
approval_policy = "on-request"  # untrusted | on-failure | on-request | never
sandbox_mode = "workspace-write"  # read-only | workspace-write | danger-full-access

[sandbox_workspace_write]
writable_roots = ["/path/to/extra/writable/root"]
network_access = false
exclude_tmpdir_env_var = false
exclude_slash_tmp = false
```

#### MCP Servers Configuration

```toml
[mcp_servers.server-name]
command = "npx"
args = ["-y", "mcp-server"]
env = { "API_KEY" = "value" }
startup_timeout_sec = 10
tool_timeout_sec = 60

# Streamable HTTP (requires experimental rmcp client)
[mcp_servers.figma]
url = "http://127.0.0.1:3845/mcp"
bearer_token = "<token>"
```

#### Shell Environment Policy

```toml
[shell_environment_policy]
inherit = "all"  # all | core | none
ignore_default_excludes = false
exclude = ["AWS_*", "AZURE_*"]
set = { CI = "1" }
include_only = ["PATH", "HOME"]
```

### SessionConfiguredEvent

**Source:** `codex-rs/protocol/src/protocol.rs` (lines 1182-1206)

```rust
pub struct SessionConfiguredEvent {
    pub session_id: ConversationId,
    pub model: String,
    pub reasoning_effort: Option<ReasoningEffortConfig>,
    pub history_log_id: u64,
    pub history_entry_count: usize,
    pub initial_messages: Option<Vec<EventMsg>>,
    pub rollout_path: PathBuf,
}
```

---

## 6. Slash Commands

**Source:** `codex-rs/tui/src/slash_command.rs`

### Built-in Slash Commands

| Command | Description | Available During Task |
|---------|-------------|----------------------|
| `/model` | Choose what model and reasoning effort to use | No |
| `/approvals` | Choose what Codex can do without approval | No |
| `/review` | Review my current changes and find issues | No |
| `/new` | Start a new chat during a conversation | No |
| `/init` | Create an AGENTS.md file with instructions for Codex | No |
| `/compact` | Summarize conversation to prevent hitting context limit | No |
| `/undo` | Restore workspace to last Codex snapshot (BETA) | No |
| `/diff` | Show git diff (including untracked files) | Yes |
| `/mention` | Mention a file | Yes |
| `/status` | Show current session configuration and token usage | Yes |
| `/mcp` | List configured MCP tools | Yes |
| `/logout` | Log out of Codex | No |
| `/quit` | Exit Codex | Yes |

### Custom Prompts

**Source:** `codex-rs/core/src/custom_prompts.rs`

Custom prompts are Markdown files stored in `$CODEX_HOME/prompts/` with optional YAML frontmatter:

```markdown
---
description: "Short description for popup"
argument-hint: "<optional arg hint>"
---

Prompt content goes here...
```

**Discovery:**

- Only `.md` files are recognized
- Files sorted alphabetically by name
- Built-in slash command names are excluded

**Response to `Op::ListCustomPrompts`:**

```rust
pub struct ListCustomPromptsResponseEvent {
    pub custom_prompts: Vec<CustomPrompt>,
}

pub struct CustomPrompt {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
    pub description: Option<String>,
    pub argument_hint: Option<String>,
}
```

---

## 7. Proto Mode vs JSON Mode

### Proto Mode

**Source:** `codex-rs/cli/src/proto.rs`

**Invocation:** `codex proto [--config key=value]`

**Characteristics:**

- Expects stdin to be a pipe (not terminal)
- Uses newline-delimited JSON (JSONL) for both input and output
- Logs go to stderr, protocol messages to stdout
- Starts with synthetic `SessionConfigured` event on stdout
- Bidirectional streaming: reads Submissions from stdin, writes Events to stdout

**Flow:**

1. CLI starts, loads config with overrides
2. Creates new ConversationManager and Conversation
3. Emits `SessionConfigured` event to stdout immediately
4. Spawns two concurrent tasks:
   - **SQ task:** Reads JSONL Submissions from stdin, forwards to conversation
   - **EQ task:** Reads Events from conversation, serializes to JSONL on stdout
5. Both tasks run until stdin closes, Ctrl-C, or fatal error

**Config Overrides:**
CLI supports `-c key=value` flags that override `config.toml`:

```bash
codex proto -c approval_policy=never -c sandbox_mode=workspace-write
```

### JSON Mode (HTTP API)

Not directly exposed by CLI; used internally by Codex's REST API endpoints.

---

## 8. Approval Flows

### Command Execution Approval

**Source:** `codex-rs/protocol/src/protocol.rs` (lines 1097-1108)

**Event:**

```rust
pub struct ExecApprovalRequestEvent {
    pub call_id: String,
    pub command: Vec<String>,
    pub cwd: PathBuf,
    pub reason: Option<String>,  // e.g., "retry without sandbox"
}
```

**Response:**

```json
{
  "id": "submission-id",
  "op": {
    "type": "exec_approval",
    "id": "call_id",
    "decision": "approved"  // approved | approved_for_session | denied | abort
  }
}
```

### Patch Application Approval

**Source:** `codex-rs/protocol/src/protocol.rs` (lines 1110-1121)

**Event:**

```rust
pub struct ApplyPatchApprovalRequestEvent {
    pub call_id: String,
    pub changes: HashMap<PathBuf, FileChange>,
    pub reason: Option<String>,
    pub grant_root: Option<PathBuf>,  // Request to allow writes under root for session
}

pub enum FileChange {
    Add { content: String },
    Delete { content: String },
    Update { unified_diff: String, move_path: Option<PathBuf> },
}
```

**Response:**

```json
{
  "id": "submission-id",
  "op": {
    "type": "patch_approval",
    "id": "call_id",
    "decision": "approved"
  }
}
```

---

## 9. MCP Server Integration

### Configuration

**Source:** `docs/config.md` (lines 339-408)

#### STDIO Transport

```toml
[mcp_servers.server-name]
command = "npx"
args = ["-y", "mcp-server"]
env = { "API_KEY" = "value" }
startup_timeout_sec = 20  # default: 10
tool_timeout_sec = 30     # default: 60
```

#### Streamable HTTP Transport

```toml
experimental_use_rmcp_client = true

[mcp_servers.figma]
url = "http://127.0.0.1:3845/mcp"
bearer_token = "<token>"
```

### MCP CLI Commands

```bash
# Add server
codex mcp add docs -- docs-server --port 4000

# List servers
codex mcp list
codex mcp list --json

# Show one server
codex mcp get docs
codex mcp get docs --json

# Remove server
codex mcp remove docs
```

### MCP Tool Listing

**Request:** `Op::ListMcpTools`

**Response:**

```rust
pub struct McpListToolsResponseEvent {
    pub tools: HashMap<String, McpTool>,  // Fully qualified name -> definition
}
```

---

## 10. Notifications and Turn Completion

### Turn Completion Notification

**Source:** `docs/config.md` (lines 562-641)

**Configuration:**

```toml
notify = ["python3", "/path/to/notify.py"]
```

**Notification JSON Format:**

```json
{
  "type": "agent-turn-complete",
  "turn-id": "12345",
  "input-messages": ["Rename `foo` to `bar` and update callsites."],
  "last-assistant-message": "Rename complete and verified `cargo build` succeeds."
}
```

**Current Notification Types:**

- `agent-turn-complete` (only type currently supported)

**Event in Protocol:**

```rust
pub struct TaskCompleteEvent {
    pub last_agent_message: Option<String>,
}
```

### TUI Notifications (In-Terminal)

**Source:** `docs/config.md` (lines 709-728)

```toml
[tui]
notifications = true  # Enable all notifications

# Or filter specific types
notifications = ["agent-turn-complete", "approval-requested"]
```

**Available Types:**

- `agent-turn-complete`
- `approval-requested`

**Note:** Uses terminal escape codes; not supported by all terminals (macOS Terminal.app, VS Code terminal don't support; iTerm2, Ghostty, WezTerm do).

---

## 11. Additional Capabilities

### History Management

**Add to history:**

```json
{
  "id": "sub-id",
  "op": {
    "type": "add_to_history",
    "text": "Message to store"
  }
}
```

**Get history entry:**

```json
{
  "id": "sub-id",
  "op": {
    "type": "get_history_entry_request",
    "offset": 0,
    "log_id": 12345
  }
}
```

**Response:**

```rust
pub struct GetHistoryEntryResponseEvent {
    pub offset: usize,
    pub log_id: u64,
    pub entry: Option<HistoryEntry>,
}
```

### Conversation Path

**Request:** `Op::GetPath`

**Response:**

```rust
pub struct ConversationPathResponseEvent {
    pub conversation_id: ConversationId,
    pub path: PathBuf,
}
```

### Compact (Summarization)

**Request:** `Op::Compact`

Uses existing context (conversation history or previous response_id) to generate summary, returned as `AgentMessage` event.

### Review Mode

**Request:**

```rust
pub struct ReviewRequest {
    pub prompt: String,
    pub user_facing_hint: String,
}
```

**Entry Event:** `EventMsg::EnteredReviewMode(ReviewRequest)`

**Exit Event:**

```rust
pub struct ExitedReviewModeEvent {
    pub review_output: Option<ReviewOutputEvent>,
}

pub struct ReviewOutputEvent {
    pub findings: Vec<ReviewFinding>,
    pub overall_correctness: String,
    pub overall_explanation: String,
    pub overall_confidence_score: f32,
}

pub struct ReviewFinding {
    pub title: String,
    pub body: String,
    pub confidence_score: f32,
    pub priority: i32,
    pub code_location: ReviewCodeLocation,
}
```

---

## Summary of Key Differences from ACP

### Protocol Structure

- **Codex:** SQ/EQ pattern with explicit Submission and Event types
- **ACP:** JSON-RPC 2.0 with method-based requests and notifications

### Session Management

- **Codex:** Single session per process, reconfigured via `OverrideTurnContext`
- **ACP:** Multiple sessions via `session/new`, `session/prompt`, `session/cancel`

### Tool Execution

- **Codex:** Tools sent to model, execution reported via events (Begin/End pattern)
- **ACP:** Client-side tool execution via ACP runtime adapter

### Streaming

- **Codex:** Native streaming via SSE from model, chunked output deltas
- **ACP:** `session/update` events for streaming agent messages and tool call progress

### Approval Model

- **Codex:** Explicit approval requests with ReviewDecision response
- **ACP:** Permission modes set at session creation, no mid-session approval requests

---

## File Citations

| Component | File Path | Key Lines |
|-----------|-----------|-----------|
| Protocol Overview | `/codex-rs/docs/protocol_v1.md` | 1-172 |
| Protocol Types | `/codex-rs/protocol/src/protocol.rs` | 40-1265 |
| Event Types | `/codex-rs/exec/src/exec_events.rs` | 1-199 |
| Proto Mode | `/codex-rs/cli/src/proto.rs` | 1-134 |
| OpenAI Tools | `/codex-rs/core/src/openai_tools.rs` | 1-300 |
| Slash Commands | `/codex-rs/tui/src/slash_command.rs` | 1-102 |
| Custom Prompts | `/codex-rs/core/src/custom_prompts.rs` | 1-200 |
| Config Reference | `/docs/config.md` | 1-783 |
| Advanced Features | `/docs/advanced.md` | 1-121 |
| TypeScript Events | `/sdk/typescript/src/events.ts` | 1-63 |

---

**End of Document**
