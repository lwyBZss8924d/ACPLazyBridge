# Data Model: Align Streaming Notifications with ACP Models

## Entity Overview

This document defines the data model entities for streaming notification alignment with official ACP protocol types.

## Core Entities

### SessionNotification

**Purpose**: Primary wrapper for all session updates sent to clients
**Source**: `agent_client_protocol::SessionNotification`

```rust
pub struct SessionNotification {
    pub session_id: SessionId,
    pub update: SessionUpdate,
    pub meta: Option<Value>,
}
```

**Key Notes**:

- `session_id` is the strong `SessionId` newtype used across ACP, not a bare `String`.
- `update` carries a `SessionUpdate` enum value describing the concrete notification payload.
- `_meta` (when present) must remain untouched to preserve forward-compatibility.

### SessionUpdate

**Purpose**: Discriminated union of streaming updates emitted during a prompt turn
**Source**: `agent_client_protocol::SessionUpdate`

```rust
#[serde(tag = "sessionUpdate", rename_all = "snake_case")]
pub enum SessionUpdate {
    UserMessageChunk { content: ContentBlock },
    AgentMessageChunk { content: ContentBlock },
    AgentThoughtChunk { content: ContentBlock },
    ToolCall(ToolCall),
    ToolCallUpdate(ToolCallUpdate),
    Plan(Plan),
    AvailableCommandsUpdate { available_commands: Vec<AvailableCommand> },
    CurrentModeUpdate { current_mode_id: SessionModeId },
}
```

- Variants in ACP v0.4.3 cover user/agent streaming text, tool-call lifecycle, planning, and session-mode/command updates. The JSON discriminator is the `sessionUpdate` field.

### ContentBlock

**Purpose**: Rich content representation for messages, tool results, and plans
**Source**: `agent_client_protocol::ContentBlock`

```rust
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text(TextContent),
    Image(ImageContent),
    Audio(AudioContent),
    ResourceLink(ResourceLink),
    Resource(EmbeddedResource),
}
```

- `TextContent` carries the rendered string plus optional annotations/meta.
- `ImageContent` embeds base64 data, MIME type, and optional URI metadata.
- `AudioContent` mirrors the image structure for audio payloads.
- `ResourceLink` and `EmbeddedResource` model external and inline artifacts respectively.
- These blocks appear in streaming chunks, tool-call content, and planning updates; clients must handle the full variant set when migrating.

### ToolCall

**Purpose**: Represents a tool invocation request in flight
**Source**: `agent_client_protocol::ToolCall`

```rust
pub struct ToolCall {
    pub id: ToolCallId,
    pub title: String,
    pub kind: ToolKind,
    pub status: ToolCallStatus,
    pub content: Vec<ToolCallContent>,
    pub locations: Vec<ToolCallLocation>,
    pub raw_input: Option<Value>,
    pub raw_output: Option<Value>,
    pub meta: Option<Value>,
}
```

- `ToolKind` is the ACP enum (`read`, `edit`, `delete`, `move`, `search`, `execute`, `think`, `fetch`, `switch_mode`, `other`).
- `ToolCallContent` wraps either a nested `ContentBlock`, a diff, or a terminal reference.
- `ToolCallLocation` identifies affected paths/lines for IDE follow-along features.

### ToolCallUpdate and ToolCallUpdateFields

```rust
pub struct ToolCallUpdate {
    pub id: ToolCallId,
    pub fields: ToolCallUpdateFields,
    pub meta: Option<Value>,
}

#[derive(Default)]
pub struct ToolCallUpdateFields {
    pub kind: Option<ToolKind>,
    pub status: Option<ToolCallStatus>,
    pub title: Option<String>,
    pub content: Option<Vec<ToolCallContent>>,
    pub locations: Option<Vec<ToolCallLocation>>,
    pub raw_input: Option<Value>,
    pub raw_output: Option<Value>,
}
```

- Updates are sparse: only changed fields are populated.
- `TryFrom<ToolCallUpdate>` ensures the runtime can synthesize a `ToolCall` when only updates are received.

### ToolCallStatus

**Purpose**: Enumeration of tool execution states
**Source**: `agent_client_protocol::ToolCallStatus`

```rust
pub enum ToolCallStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}
```

- `Pending` is the default; `Cancelled` is _not_ part of ACP v0.4.3.
- Tool calls transition forward only (pending → in_progress → {completed | failed}).

## Supporting Entities

### ToolCallContent

**Purpose**: Structured tool outputs
**Source**: `agent_client_protocol::ToolCallContent`

Variants:

- `Content { content: ContentBlock }` – embeds standard content blocks.
- `Diff { diff: Diff }` – captures file diffs with `old_text`/`new_text`.
- `Terminal { terminal_id }` – references an ACP terminal session.

### ToolCallLocation

**Purpose**: Identifies paths affected by a tool
**Source**: `agent_client_protocol::ToolCallLocation`

```rust
pub struct ToolCallLocation {
    pub path: PathBuf,
    pub line: Option<u32>,
    pub meta: Option<Value>,
}
```

### StopReason

**Purpose**: Session termination reason reported via `session/prompt`
**Source**: `agent_client_protocol::StopReason`

```rust
pub enum StopReason {
    EndTurn,
    MaxTokens,
    MaxTurnRequests,
    Refusal,
    Cancelled,
}
```

## Deduplication Model

### LastChunkGuard

**Purpose**: Prevent immediate duplicate text chunks from being re-sent
**Strategy**: Cache only the most recent serialized text block and suppress repeats

```rust
struct LastChunkGuard {
    last_text: Option<String>,
}

impl LastChunkGuard {
    fn should_emit(&mut self, content: &str) -> bool {
        if self.last_text.as_deref() == Some(content) {
            false
        } else {
            self.last_text = Some(content.to_owned());
            true
        }
    }
}
```

**Rules**:

- Only text content is deduplicated today; additional block types can extend the guard when introduced.
- Duplicate suppression is limited to the most recent chunk to avoid unbounded memory usage.
- Reset or bypass the guard when non-text updates (tool events, plans) occur to preserve ordering.
- Snapshot harness tests (tests/streaming_snapshots_test.rs, T013) ensure the guard suppresses duplicate agent chunks without impacting tool events.

## Type Migration Mappings

| Custom Type | Official Type | Notes |
| --- | --- | --- |
| `SessionUpdate` | `SessionNotification` | Map to `session_id` + `update` (enum) |
| `SessionUpdateContent` | `SessionUpdate` variants | Adopt ACP struct variants (ToolCall / ToolCallUpdate) |
| `ContentBlock` (custom) | `ContentBlock` (official) | Replace inline text-only representation |
| `ToolCallStatus` (custom) | `ToolCallStatus` (official) | Drop unsupported states |
| `ToolCall` (custom fields) | `ToolCall` (official) | Use `id`, `title`, `kind`, `content`, `locations`, raw IO |
| `ToolCallUpdate` (inline) | `ToolCallUpdate` + `ToolCallUpdateFields` | Flatten update-fields pattern |

## Serialization Requirements

### JSON Field Mappings

```json
// SessionNotification
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "...",
    "update": {
      "sessionUpdate": "agent_message_chunk",
      "content": { "type": "text", "text": "..." }
    }
  }
}
```

### Field Name Conversions

- Rust: `tool_call_id` → JSON: `toolCallId`
- Rust: `raw_input` → JSON: `rawInput`
- Rust: `raw_output` → JSON: `rawOutput`
- Rust: `session_id` → JSON: `sessionId`

## Invariants

1. **Session ID Consistency**: All notifications for a session use same ID
2. **Tool Call ID Uniqueness**: No duplicate tool_call_id in same session
3. **Status Monotonicity**: Tool status only progresses forward
4. **Content Immutability**: Content blocks never modified after creation
5. **JSONL Atomicity**: Each line is complete, valid JSON

## Performance Characteristics

| Operation | Current | Target | Notes |
| --- | --- | --- | --- |
| Serialize notification | <1ms | <1ms | No regression |
| Deduplicate chunk | O(1) | O(1) | String compare |
| Tool status update | <1ms | <1ms | Direct field update |
| JSONL write | <5ms | <5ms | Line-buffered |

## Dependencies

- `agent_client_protocol` v0.4.3
- `serde` v1.0 with derive
- `serde_json` v1.0
- `tokio` v1.x (async runtime)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_

```yaml
constitution:
  version: "1.0.1"
  last_checked: "2025-09-25T10:14:27Z"
document:
  type: "data-model"
  path: "specs/039-streaming-alignment-session-notifications/data-model.md"
  version: "0.1.1"
  last_updated: "2025-09-25T10:14:27Z"
  issue_uri: "https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45"
  related:
    spec: "specs/039-streaming-alignment-session-notifications/spec.md"
    plan: "specs/039-streaming-alignment-session-notifications/plan.md"
    tasks: "specs/039-streaming-alignment-session-notifications/tasks.md"
```
