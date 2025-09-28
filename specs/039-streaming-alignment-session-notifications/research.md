# Research: Align Streaming Notifications with ACP Models

## Research Summary

Analysis of the official `agent_client_protocol` types and their compatibility with existing Codex streaming implementation.

## Type Analysis

### Current Custom Types (To Replace)

1. **SessionUpdate** (codex_proto.rs:73-77)
   - Custom struct with jsonrpc, method, params fields
   - Maps to: `agent_client_protocol::SessionNotification` (`session_id`, `update`, `_meta`)

2. **ContentBlock** (codex_proto.rs:88-92)
   - Custom enum with Text variant only
   - Maps to: `agent_client_protocol::ContentBlock` (`text`, `image`, `audio`, `resource_link`, `resource`)

3. **ToolCallStatus** (codex_proto.rs:96-104)
   - Custom enum: Pending, InProgress, Completed, Failed
   - Maps to: Official status progression in protocol (no `Cancelled` state)

4. **SessionUpdateContent** (codex_proto.rs:106-147)
   - Custom tagged enum for update types
   - Maps to: `agent_client_protocol::SessionUpdate` variants (ToolCall / ToolCallUpdate struct variants, Plan, etc.)

### Official Type Structures

Based on agent-client-protocol v0.4.2:

```rust
// From agent_client_protocol crate (v0.4.2)
pub struct SessionNotification {
    pub session_id: SessionId,
    pub update: SessionUpdate,
    pub meta: Option<Value>,
}

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

#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text(TextContent),
    Image(ImageContent),
    Audio(AudioContent),
    ResourceLink(ResourceLink),
    Resource(EmbeddedResource),
}

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

pub struct ToolCallUpdate {
    pub id: ToolCallId,
    pub fields: ToolCallUpdateFields,
    pub meta: Option<Value>,
}

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


## Migration Strategy

### Phase 1: Type Imports

- Add `use agent_client_protocol::{SessionNotification, SessionUpdate, ContentBlock, ToolCall, ToolCallUpdate};`
- Remove custom type definitions

### Phase 2: Serialization Compatibility

- Official types use the same JSON field names and discriminator tags as the custom structs (`session/update` envelope, `sessionUpdate` tag).
- Serde attributes already match the ACP schema; struct variant migration is additive.
- No breaking changes to JSONL output when fields are mapped 1:1.

### Phase 3: Deduplication Adaptation

- Current: Suppresses only the last text chunk via `last_sent_chunk` comparison
- New: Retain the single-last-chunk guard using official `ContentBlock::Text` values, documenting extension points for richer content types
- Maintain lightweight state to avoid broader hash sets; evaluate removal if downstream tooling expects raw streams

## Key Findings

1. **Direct Compatibility**: Official types serialize to same JSON structure
2. **Extended Capabilities**: Official types support more variants (future-ready)
3. **Status Alignment**: Tool call states map 1:1 to official enum
4. **No Breaking Changes**: JSONL output remains identical

## Risks and Mitigations

| Risk | Mitigation |
| --- | --- |
| Serde attribute differences | Snapshot tests verify exact JSON output |
| Missing field mappings | Comprehensive field-by-field migration |
| Deduplication behavior change | Explicit dedup tests with official types |
| Performance regression | Benchmark before/after comparison |

## Implementation Notes

### Tool Call Lifecycle

Current flow preserved:

1. Initial ToolCall with status: pending
2. ToolCallUpdate with status: in_progress
3. ToolCallUpdate with status: completed/failed

### Metadata Requirements

Official types require:

- `tool_call_id`: Already provided
- `raw_input`: Can extract from arguments
- `raw_output`: Can capture from tool execution
- `locations`: Optional, can add if available

## Validation Approach

1. **Snapshot Testing**: Capture serialized JSON for each type using the shared SnapshotHarness (tests/support/mod.rs)
2. **Round-trip Tests**: Deserialize → serialize → compare
3. **JSONL Regression**: Full scenario replay comparison
4. **Performance Tests**: Measure serialization overhead

## References

- agent-client-protocol source: ~/dev-space/agent-client-protocol/rust/src/
- Claude Code ACP adapter: ~/dev-space/claude-code-acp/src/
- Current implementation: crates/codex-cli-acp/src/codex_proto.rs
- Test baseline: _artifacts/038-adopt-acp-runtime/tests/

```yaml
constitution:
  version: "1.0.1"
  last_checked: "2025-09-25T10:14:27Z"
document:
  type: "research"
  path: "specs/039-streaming-alignment-session-notifications/research.md"
  version: "0.1.1"
  last_updated: "2025-09-25T10:14:27Z"
  issue_uri: "https://github.com/lwyBZss8924d/ACPLazyBridge/issues/45"
  related:
    spec: "specs/039-streaming-alignment-session-notifications/spec.md"
    plan: "specs/039-streaming-alignment-session-notifications/plan.md"
```
