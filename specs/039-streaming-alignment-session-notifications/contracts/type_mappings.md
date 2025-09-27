# Type Mapping Contracts

## Contract Overview

This document defines the binding contracts for migrating custom streaming types to official agent_client_protocol types.

## Type Migration Contracts

### Contract 1: SessionUpdate → SessionNotification

**Input Type**: legacy `codex_proto::SessionUpdate`

```rust
pub struct SessionUpdate {
    pub jsonrpc: String,
    pub method: String,
    pub params: SessionUpdateParams,
}
```

**Output Type**: `agent_client_protocol::SessionNotification`

```rust
pub struct SessionNotification {
    pub session_id: SessionId,
    pub update: SessionUpdate,
    pub meta: Option<Value>,
}
```

**Mapping Rules**:

1. Map `params.session_id` → `SessionId` newtype (validate format before wrapping).
2. Convert inline `SessionUpdateContent` enums to official `SessionUpdate` variants (see Contract 4).
3. Preserve the JSON-RPC envelope (`jsonrpc`, `method = "session/update"`) unchanged.

**Validation**:

- JSON output must remain identical
- Field names must use camelCase
- Method must remain "session/update"

### Contract 2: ContentBlock Migration

**Input Type**: text-only `codex_proto::ContentBlock`

```rust
pub enum ContentBlock {
    Text { text: String },
}
```

**Output Type**: `agent_client_protocol::ContentBlock`

```rust
pub enum ContentBlock {
    Text(TextContent),
    Image(ImageContent),
    Audio(AudioContent),
    ResourceLink(ResourceLink),
    Resource(EmbeddedResource),
}
```

**Mapping Rules**:

1. Wrap plain text as `ContentBlock::Text(TextContent { text, annotations: None, meta: None })`.
2. For future block types (images/audio/resources) forward the upstream structure untouched.
3. Avoid flattening nested structs—serde derives already match ACP schema.

**Validation**:

- Existing Text blocks serialize identically
- PartialEq implementation for deduplication

### Contract 3: ToolCallStatus Migration

**Input Type**: `codex_proto::ToolCallStatus`

```rust
pub enum ToolCallStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}
```

**Output Type**: `agent_client_protocol::ToolCallStatus`

- Same variant set; drop any custom `Cancelled` mapping.
- Ensure pending is treated as default when `status` is missing.

**Validation**:

- JSON representation unchanged for existing states
- State machine transitions preserved

### Contract 4: SessionUpdateContent → SessionUpdate

**Input Type**: `codex_proto::SessionUpdateContent`

```rust
pub enum SessionUpdateContent {
    AgentMessageChunk { content: ContentBlock },
    ToolCall { /* fields */ },
    ToolCallUpdate { /* fields */ },
}
```

**Output Type**: `agent_client_protocol::SessionUpdate`

**Mapping Rules**:

1. `AgentMessageChunk` → `SessionUpdate::AgentMessageChunk { content: ContentBlock }` (no wrapping Vec).
2. Inline tool-call structs → `SessionUpdate::ToolCall(ToolCall)` using Contract 5.
3. Inline tool-call updates → `SessionUpdate::ToolCallUpdate(ToolCallUpdate)` using Contract 6.
4. Unhandled variants (plan, commands, mode) must be added before migration completes; fail fast otherwise.

**Validation**:

- Tagged enum discriminator preserved
- All fields mapped without loss

## Tool Call Contracts

### Contract 5: Tool Call Creation

**Requirement**: Create official ToolCall from Codex events

**Input**: Codex ToolCall event

```json
{
  "type": "tool_call",
  "id": "call_123",
  "name": "fs/read_file",
  "arguments": { "path": "file.txt" }
}
```

**Output**: ACP ToolCall

```rust
ToolCall {
    id: ToolCallId(Arc::from("call_123")),
    title: "fs/read_file".into(),
    kind: map_tool_kind("fs"),
    status: ToolCallStatus::Pending,
    content: Vec::new(),
    locations: Vec::new(),
    raw_input: Some(json!({ "path": "file.txt" })),
    raw_output: None,
    meta: None,
}
```

### Contract 6: Tool Call Updates

**Requirement**: Generate `ToolCallUpdate` (with `ToolCallUpdateFields`) for status changes

**Status Transitions**:

```text
Pending → InProgress (tool execution starts)
InProgress → Completed (success)
InProgress → Failed (error)
```

**Update Generation**:

1. Always populate `id` with the canonical `ToolCallId`.
2. Set fields via `ToolCallUpdateFields` (e.g., `status`, `content`, `raw_output`) instead of bespoke struct shapes.
3. Preserve existing lists: when content/locations are unchanged, omit them to avoid empty overwrites.

## Deduplication Contract

### Contract 7: LastChunkGuard Deduplication

**Requirement**: Mirror the existing single-last-chunk suppression used in the adapter

**Algorithm**:

```rust
if last_chunk_guard.should_emit(&text) {
    // forward chunk
}
```

**Rules**:

- Guard tracks only the most recent `ContentBlock::Text` payload.
- Non-text updates bypass the guard to maintain ordering.
- Consider lifting the guard entirely if downstream clients prefer raw streams (documented trade-off).
- Snapshot coverage: tests/streaming_snapshots_test.rs (T013) verifies duplicate suppression without affecting tool or plan updates.

## Serialization Contract

### Contract 8: JSONL Protocol Format

**Requirement**: Maintain exact JSONL compatibility

**Format**:

```json
{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"...","update":{...}}}
```

**Rules**:

1. One complete JSON object per line
2. No newlines within JSON
3. UTF-8 encoding
4. Immediate flush after write

## Error Handling Contract

### Contract 9: Error Mapping

**Input**: Codex Error event

```json
{
  "type": "error",
  "message": "File not found",
  "code": "ENOENT"
}
```

**Output**: ACP ErrorInfo

```rust
ErrorInfo {
    code: "ENOENT",
    message: "File not found",
    details: None,
}
```

## Testing Contracts

### Contract 10: Snapshot Testing

**Requirement**: Verify exact JSON output

**Test Pattern**:

```rust
#[test]
fn test_notification_serialization() {
    let notification = create_notification();
    let json = serde_json::to_string(&notification).unwrap();
    insta::assert_json_snapshot!(json);
}
```

**Coverage Required**:

- All SessionUpdate variants
- All ContentBlock types
- All ToolCallStatus states
- Error conditions

## Performance Contract

### Contract 11: Latency Requirements

**Baseline** (from Task 038): ≤150ms prompt latency

**Requirements**:

- Type conversion: <1ms overhead
- Serialization: <5ms for typical message
- Deduplication check: O(1) lookup
- No blocking operations in stream path

## Backwards Compatibility Contract

### Contract 12: JSONL Regression

**Requirement**: Existing scenarios produce identical output

**Validation**:

```bash
# Run regression with old implementation
cat scenario.jsonl | old_binary > old_output.jsonl

# Run with new implementation
cat scenario.jsonl | new_binary > new_output.jsonl

# Must be identical
diff old_output.jsonl new_output.jsonl
```

**Scenarios to Test**:

- Basic handshake
- Session creation
- Tool execution
- Error handling
- Cancellation

### Contract 13: Direct Codex Streaming (Fallback Removal)

**Requirement**: Eliminate the simulated fallback branch so that Codex stdio is the single source of streaming updates.

**Rules**:

- `CodexProviderAdapter::spawn_and_stream_codex` must always forward Codex stdout through the official conversion pipeline; simulated responses are forbidden.
- Notify/timeout handling operates on real Codex output; no synthetic completion messages may be injected.
- Regression tests must confirm that scenarios previously relying on the fallback now succeed via the primary path.

**Validation**:

- Unit tests or JSONL replays observe identical output before/after fallback removal.
- Code review verifies the fallback branch is deleted and not replaced with another simulation pathway.

```yaml
constitution:
  version: "1.0.1"
  last_checked: "2025-09-25T10:14:27Z"
document:
  type: "contract"
  path: "specs/039-streaming-alignment-session-notifications/contracts/type_mappings.md"
  version: "0.1.1"
  last_updated: "2025-09-25T10:14:27Z"
  issue_uri: "https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45"
  related:
    spec: "specs/039-streaming-alignment-session-notifications/spec.md"
    plan: "specs/039-streaming-alignment-session-notifications/plan.md"
    tasks: "specs/039-streaming-alignment-session-notifications/tasks.md"
```
