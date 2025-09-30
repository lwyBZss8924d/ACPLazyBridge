# Codex to ACP Mapping Recommendations

**Analysis Date:** 2025-09-30
**Purpose:** Design recommendations for mapping Codex CLI protocol to Agent Client Protocol (ACP)

---

## Executive Summary

This document provides concrete recommendations for how `codex-cli-acp` (the Codex ACP adapter) should map Codex protocol concepts to ACP's session-based model. The analysis is based on comprehensive review of Codex source code and the existing `codex-cli-acp` implementation.

**Key Challenge:** Codex uses a Submission Queue / Event Queue pattern with rich, granular event types, while ACP uses a session-based JSON-RPC model with consolidated `session/update` events.

---

## 1. Session Lifecycle Mapping

### ACP Session Creation → Codex Session Initialization

**ACP Request:** `session/new`

```json
{
  "id": 1,
  "method": "session/new",
  "params": {
    "cwd": "/absolute/path",
    "mcpServers": [...],
    "permissionMode": "ask"
  }
}
```

**Codex Equivalent:** Start `codex proto` process and wait for `SessionConfigured` event

**Implementation:**

1. Spawn `codex proto` with config overrides for permissions
2. Read initial `SessionConfigured` event from stdout
3. Store `session_id` from event
4. Return ACP response with `sessionId` and `protocolVersion: 1`

**Permission Mapping:**

| ACP `permissionMode` | Codex Config Override |
|----------------------|----------------------|
| `ask` | `-c approval_policy=on-request -c sandbox_mode=workspace-write` |
| `always_allow` | `-c approval_policy=never -c sandbox_mode=workspace-write` |
| `readonly` | `-c approval_policy=untrusted -c sandbox_mode=read-only` |

---

### ACP Session Prompt → Codex UserTurn Submission

**ACP Request:** `session/prompt`

```json
{
  "id": 2,
  "method": "session/prompt",
  "params": {
    "sessionId": "sess-123",
    "messages": [
      { "role": "user", "content": "Fix the bug in main.rs" }
    ]
  }
}
```

**Codex Submission:**

```json
{
  "id": "2",
  "op": {
    "type": "user_turn",
    "items": [
      { "type": "text", "text": "Fix the bug in main.rs" }
    ],
    "cwd": "/absolute/path",
    "approval_policy": "on-request",
    "sandbox_policy": { "mode": "workspace-write" },
    "model": "gpt-5-codex",
    "effort": "medium",
    "summary": "auto"
  }
}
```

**Implementation:**

1. Extract text content from ACP messages array
2. Construct `UserTurn` submission with stored session configuration
3. Write JSONL submission to Codex stdin
4. Begin streaming Codex events back to ACP client via `session/update` notifications

---

### ACP Session Cancel → Codex Interrupt Submission

**ACP Request:** `session/cancel`

```json
{
  "id": 3,
  "method": "session/cancel",
  "params": {
    "sessionId": "sess-123"
  }
}
```

**Codex Submission:**

```json
{
  "id": "3",
  "op": {
    "type": "interrupt"
  }
}
```

**Implementation:**

1. Write `Interrupt` submission to stdin
2. Expect `TurnAborted` event from Codex
3. Return ACP success response

---

## 2. Event Stream Mapping

### Core Principle: Aggregate Codex Events → ACP session/update

ACP uses a single `session/update` notification with different event types. Codex has many granular event types that must be intelligently mapped.

---

### Agent Message Streaming

**Codex Events:**

```txt
AgentMessageDelta { delta: "Fix" }
AgentMessageDelta { delta: "ing" }
AgentMessage { message: "Fixing the bug..." }
```

**ACP Mapping:**

```json
{
  "method": "session/update",
  "params": {
    "sessionId": "sess-123",
    "event": {
      "type": "AgentMessageChunk",
      "message": "Fixing the bug..."
    }
  }
}
```

**Implementation Strategy:**

- Buffer `AgentMessageDelta` events
- De-duplicate: if `AgentMessage` contains same text as buffered deltas, don't emit duplicate chunk
- Emit final `AgentMessageChunk` with complete message from `AgentMessage` event

**Reference:** `codex_proto.rs` lines 150-200

---

### Tool Call Execution Mapping

#### Command Execution (local_shell)

**Codex Event Sequence:**

```txt
ExecApprovalRequest → (approval) → ExecCommandBegin → ExecCommandOutputDelta* → ExecCommandEnd
```

**ACP Mapping:**

**ToolCallUpdate (pending):**

```json
{
  "method": "session/update",
  "params": {
    "sessionId": "sess-123",
    "event": {
      "type": "ToolCallUpdate",
      "toolCall": {
        "id": "call_abc123",
        "kind": "execute",
        "status": "pending",
        "parameters": {
          "command": "cargo build",
          "workdir": "/path",
          "timeout": 120000
        }
      }
    }
  }
}
```

**ToolCallUpdate (in_progress):**

```json
{
  "event": {
    "type": "ToolCallUpdate",
    "toolCall": {
      "id": "call_abc123",
      "status": "in_progress",
      "outputPreview": "Compiling main v0.1.0..."
    }
  }
}
```

**ToolCallUpdate (completed):**

```json
{
  "event": {
    "type": "ToolCallUpdate",
    "toolCall": {
      "id": "call_abc123",
      "status": "completed",
      "output": "Full stdout/stderr...",
      "exitCode": 0
    }
  }
}
```

**Implementation:**

- Use `call_id` from Codex events as ACP `toolCall.id`
- Map `ExecCommandBegin` → `pending` status
- Map `ExecCommandOutputDelta` → `in_progress` with incremental `outputPreview`
- Map `ExecCommandEnd` → `completed`/`failed` with final output
- Truncate `outputPreview` to 1000 chars for streaming updates

**Reference:** `tool_calls.rs`, `codex_proto.rs` lines 250-350

---

#### Patch Application (apply_patch)

**Codex Event Sequence:**

```txt
ApplyPatchApprovalRequest → (approval) → PatchApplyBegin → PatchApplyEnd
```

**ACP Mapping:**

**ToolCallUpdate (pending):**

```json
{
  "event": {
    "type": "ToolCallUpdate",
    "toolCall": {
      "id": "call_patch_456",
      "kind": "edit",
      "status": "pending",
      "filesAffected": ["src/main.rs", "tests/test.rs"]
    }
  }
}
```

**ToolCallUpdate (completed):**

```json
{
  "event": {
    "type": "ToolCallUpdate",
    "toolCall": {
      "id": "call_patch_456",
      "status": "completed",
      "output": "Applied patch to 2 files"
    }
  }
}
```

**Implementation:**

- Extract file paths from `PatchApplyBegin.changes` HashMap
- Map patch kinds: `Add` → "edit", `Delete` → "delete", `Update` → "edit"
- Include unified diffs in final output

---

#### MCP Tool Calls

**Codex Event Sequence:**

```txt
McpToolCallBegin → McpToolCallEnd
```

**ACP Mapping:**

```json
{
  "event": {
    "type": "ToolCallUpdate",
    "toolCall": {
      "id": "call_mcp_789",
      "kind": "other",
      "status": "in_progress",
      "name": "github/list_repos",
      "outputPreview": "Calling github MCP server..."
    }
  }
}
```

**Implementation:**

- Use `server` and `tool` from `McpInvocation` to construct display name
- Map `result` to `completed`/`failed` status
- Include `CallToolResult.content` in final output

---

### Reasoning Events

**Codex Events:**

```txt
AgentReasoning { text: "I need to check the file first..." }
AgentReasoningDelta { delta: "..." }
```

**ACP Mapping:**

```json
{
  "event": {
    "type": "AgentMessageChunk",
    "reasoning": "I need to check the file first..."
  }
}
```

**Implementation:**

- Buffer `AgentReasoningDelta` events
- Emit complete reasoning text from `AgentReasoning` event
- Include in `AgentMessageChunk` under optional `reasoning` field (ACP extension)

---

### Error Handling

**Codex Events:**

```txt
Error { message: "Connection lost" }
StreamError { message: "SSE disconnected" }
```

**ACP Mapping:**

```json
{
  "event": {
    "type": "Error",
    "error": {
      "code": -32603,
      "message": "Connection lost"
    }
  }
}
```

**Implementation:**

- Map Codex errors to ACP JSON-RPC error codes
- Include original error message
- For `StreamError`, include retry information if available

---

## 3. Tool Call Classification and Parameters

### Shell Command Parameters

**Source:** Codex `ExecCommandBeginEvent`

**Extraction Logic:**

```rust
pub struct ShellToolCallParams {
    pub command: String,          // from command array joined
    pub workdir: Option<String>,  // from cwd
    pub timeout: Option<u64>,     // hardcoded or from config
    pub sudo: Option<bool>,       // inferred from command[0] == "sudo"
}
```

**Implementation:**

- Join `command: Vec<String>` into single string for display
- Extract `workdir` from `cwd` field
- Detect `sudo` by checking first element of command array
- Default timeout from Codex config or 120000ms

**Reference:** `tool_calls.rs` lines 100-150

---

### Tool Call Kind Classification

**Mapping Table:**

| Codex Tool/Event | ACP `kind` | Detection Logic |
|------------------|------------|-----------------|
| `ExecCommandBegin` | `execute` | Event type |
| `PatchApplyBegin` (Add) | `edit` | `FileChange::Add` |
| `PatchApplyBegin` (Update) | `edit` | `FileChange::Update` |
| `PatchApplyBegin` (Delete) | `delete` | `FileChange::Delete` |
| `McpToolCallBegin` (read) | `read` | MCP tool name heuristic |
| `McpToolCallBegin` (write) | `edit` | MCP tool name heuristic |
| `McpToolCallBegin` (search) | `search` | MCP tool name heuristic |
| `McpToolCallBegin` (other) | `other` | Default |
| `WebSearchBegin` | `fetch` | Event type |
| `PlanUpdate` | `think` | Event type (TODO list) |

**Implementation:** See `tool_calls.rs` for heuristic-based classification.

---

## 4. State Management

### Session State Tracking

The adapter must maintain:

```rust
pub struct SessionState {
    pub session_id: String,
    pub cwd: PathBuf,
    pub approval_policy: AskForApproval,
    pub sandbox_policy: SandboxPolicy,
    pub model: String,
    pub codex_process: Child,
    pub pending_approvals: HashMap<String, ApprovalRequest>,
    pub active_tool_calls: HashMap<String, ToolCallState>,
}
```

**Key State Transitions:**

- Track pending approvals by `call_id`
- Track tool call lifecycle from Begin → End
- Buffer incomplete messages/reasoning chunks
- Store conversation history for context

---

### Tool Call State Machine

```txt
pending → in_progress → completed/failed
```

**State Tracking:**

```rust
pub struct ToolCallState {
    pub id: String,
    pub kind: ToolCallKind,
    pub status: ToolCallStatus,
    pub output_buffer: String,
    pub start_time: Instant,
}
```

**Transitions:**

- `ExecCommandBegin` → `pending`
- First `ExecCommandOutputDelta` → `in_progress`
- `ExecCommandEnd` → `completed`/`failed`

---

## 5. Approval Flow Handling

### Permission Mode Integration

ACP sessions have a `permissionMode` set at creation. Codex needs this translated to config overrides:

**Decision Matrix:**

| Operation | `ask` Mode | `always_allow` Mode | `readonly` Mode |
|-----------|------------|---------------------|-----------------|
| Read file | Auto-approve | Auto-approve | Auto-approve |
| Execute safe command | Auto-approve | Auto-approve | Block |
| Execute unsafe command | Request approval | Auto-approve | Block |
| Write file | Request approval | Auto-approve | Block |
| Network access | Request approval | Auto-approve | Block |

**Implementation:**

1. Set Codex `approval_policy` and `sandbox_mode` at session creation
2. When `ExecApprovalRequest`/`ApplyPatchApprovalRequest` received:
   - If `permissionMode=always_allow`: Auto-respond with `Approved`
   - If `permissionMode=ask`: Return error to ACP client (no mid-session approval in ACP v0.4)
   - If `permissionMode=readonly`: Auto-respond with `Denied`

**Future Enhancement:** ACP could add `session/approval_request` and `session/approval_response` for interactive approval.

---

## 6. Notification Integration

### Turn Completion Detection

**Codex Signals:**

1. `TaskComplete` event
2. `notify` program invocation with `agent-turn-complete` JSON
3. Idle timeout (e.g., 30s with no events)

**ACP Response:**

```json
{
  "id": 2,
  "result": {
    "stopReason": "end_turn",
    "lastMessage": "Bug fixed successfully!"
  }
}
```

**Implementation:**

- Subscribe to Codex `notify` via environment variable `ACPLB_NOTIFY_PATH`
- Watch notify sink for `{"type":"agent-turn-complete"}` message
- Immediately end turn and return response
- Fallback to `TaskComplete` event or idle timeout

**Reference:** `notify_source.rs`, existing `acplb-notify-forwarder` binary

---

## 7. Error Categorization

### Codex Error → ACP Error Code Mapping

| Codex Error | ACP Error Code | Description |
|-------------|----------------|-------------|
| Invalid submission JSON | -32700 | Parse error |
| Unknown op type | -32601 | Method not found |
| Missing required field | -32602 | Invalid params |
| Command execution failed | -32603 | Internal error |
| Model connection lost | -32603 | Internal error |
| Patch application failed | -32603 | Internal error |

**Implementation:**

- Parse Codex `Error` events and `ExecCommandEnd.exit_code != 0`
- Map to appropriate ACP JSON-RPC error codes
- Include original Codex error message in `data` field

---

## 8. Testing Strategy

### JSONL Scenario Playback

**Approach:** Use existing `playback` binary to test adapter with recorded scenarios.

**Test Scenarios:**

1. Basic handshake and session creation
2. Simple prompt → agent message → completion
3. Command execution with approval
4. Patch application
5. MCP tool call
6. Error handling (command failure, model timeout)
7. Turn completion via notify

**Location:** `_artifacts/tests/protocol-baseline/`

---

### Integration Test Structure

```rust
#[tokio::test]
async fn test_session_prompt_with_command_execution() {
    // Setup: spawn adapter
    // Send: session/new
    // Assert: receive session/new response
    // Send: session/prompt
    // Assert: receive AgentMessageChunk
    // Assert: receive ToolCallUpdate (pending, in_progress, completed)
    // Assert: receive session/prompt response with stopReason
}
```

**Reference:** `tests/acp_integration_test.rs`

---

## 9. Performance Considerations

### Streaming Chunk Frequency

**Issue:** Codex emits many granular events (e.g., `AgentMessageDelta` every few characters).

**Solution:**

- Debounce delta events: collect for 100ms, then emit single `AgentMessageChunk`
- Truncate `outputPreview` to 1000 chars to limit message size
- Use UTF-8 safe truncation to avoid splitting multi-byte characters

**Reference:** `tool_calls.rs` `truncate_utf8_safe()`

---

### Memory Management

**Issue:** Long-running sessions accumulate event history.

**Solution:**

- Limit `active_tool_calls` HashMap to last 100 entries
- Clear completed tool call states after final event emitted
- Use `VecDeque` with max capacity for message buffers

---

## 10. Future Enhancements

### Image Attachment Support

**Codex Capability:** `InputItem::LocalImage`

**ACP Extension:**

```json
{
  "messages": [
    {
      "role": "user",
      "content": "Analyze this diagram",
      "attachments": [
        { "type": "image", "path": "/path/to/image.png" }
      ]
    }
  ]
}
```

**Implementation:**

- Accept ACP `attachments` field
- Map to Codex `LocalImage` input items

---

### Review Mode Support

**Codex Capability:** `Op::Review` with structured findings

**ACP Extension:**

```json
{
  "method": "session/review",
  "params": {
    "sessionId": "sess-123",
    "prompt": "Review the changes in this PR"
  }
}
```

**Response:**

```json
{
  "result": {
    "findings": [
      {
        "title": "Potential null pointer",
        "location": { "file": "main.rs", "line": 42 },
        "severity": "high"
      }
    ]
  }
}
```

---

### Slash Command Forwarding

**Codex Capability:** `/compact`, `/diff`, `/status`, etc.

**ACP Extension:**

```json
{
  "messages": [
    { "role": "user", "content": "/compact" }
  ]
}
```

**Implementation:**

- Detect slash command prefix in message content
- Map to corresponding Codex Op (e.g., `/compact` → `Op::Compact`)
- Return results via standard event stream

---

## 11. Reference Implementation Checklist

### Core Adapter (codex-cli-acp)

- [x] `initialize` method with `protocolVersion: 1` (integer)
- [x] `session/new` spawns Codex proto with permission overrides
- [x] `session/prompt` constructs `UserTurn` submission
- [x] `session/cancel` sends `Interrupt` submission
- [x] Event streaming via `session/update` notifications
- [x] `AgentMessageChunk` de-duplication
- [x] `ToolCallUpdate` lifecycle tracking
- [x] Error mapping to JSON-RPC codes
- [ ] Approval flow auto-responses based on `permissionMode`
- [x] Notify-based turn completion
- [ ] JSONL scenario regression tests

### Event Mapping (codex_proto.rs)

- [x] `AgentMessage`/`AgentMessageDelta` → `AgentMessageChunk`
- [x] `ExecCommandBegin`/`End` → `ToolCallUpdate` (execute)
- [x] `PatchApplyBegin`/`End` → `ToolCallUpdate` (edit/delete)
- [x] `McpToolCallBegin`/`End` → `ToolCallUpdate` (other)
- [ ] `AgentReasoning` → `AgentMessageChunk.reasoning`
- [x] `Error` → ACP Error event
- [x] `TaskComplete` → `stopReason: "end_turn"`

### Tool Parameter Extraction (tool_calls.rs)

- [x] Shell command parameter extraction
- [x] Tool call kind classification
- [x] UTF-8 safe output truncation
- [ ] MCP tool name heuristics for kind detection

---

## Appendix: Key Code Files

| Component | File Path | Purpose |
|-----------|-----------|---------|
| Main adapter | `crates/codex-cli-acp/src/main.rs` | ACP server implementation |
| Event mapping | `crates/codex-cli-acp/src/codex_proto.rs` | Codex → ACP event translation |
| Tool calls | `crates/codex-cli-acp/src/tool_calls.rs` | Tool call classification and parameters |
| Notify source | `crates/codex-cli-acp/src/notify_source.rs` | Turn completion detection |
| Validation | `crates/codex-cli-acp/src/validation.rs` | Request validation helpers |
| Integration tests | `crates/codex-cli-acp/tests/acp_integration_test.rs` | End-to-end ACP tests |
| Protocol baseline | `_artifacts/tests/protocol-baseline/` | JSONL test scenarios |

---

**End of Document**
