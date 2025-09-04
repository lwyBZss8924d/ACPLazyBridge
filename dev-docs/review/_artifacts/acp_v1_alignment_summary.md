# ACP v1 Alignment Implementation Summary

## Completed Tasks

### 1. acp-v1-init-and-session-alignment ✅

**Initialize Response Compliance:**
- `protocolVersion`: Integer value (1) instead of string
- `agentCapabilities`: Properly named field (not "capabilities")
- `promptCapabilities`: Nested under `agentCapabilities`
- `authMethods`: Required field included (empty array)
- No `fs` capability advertised (client-side only)

**Session/new Validation:**
- `cwd`: Primary parameter (absolute path required)
- `workingDirectory`: Supported as fallback alias
- `mcpServers`: Required array parameter with validation
- No defaulting to "." for relative paths

### 2. codex-cli-shell-params-and-output-alignment ✅

**Vec<String> Command Support:**
- Updated `extract_shell_command()` to handle both string and Vec<String> formats
- Aligns with Codex's `ShellToolCallParams.command: Vec<String>`
- Joins array elements with spaces for title display
- Filters non-string elements gracefully

**Output Preservation:**
- Full output preserved in `raw_output` field for completed/failed tool calls
- 2KB preview in content blocks (75% prefix, 25% suffix)
- Proper handling of stdout, stderr, and exit_code

### 3. claude-acp-best-practices-streaming-and-updates ✅

**Event Structure:**
- Single `update` object with `sessionUpdate` discriminator
- `ToolCall` for initial events, `ToolCallUpdate` for status changes
- Minimal update payloads (only changed fields)
- No repetition of title/kind in updates

**Error Handling:**
- Errors in tool context mapped to `ToolCallUpdate` with `status: failed`
- Errors outside tool context sent as `AgentMessageChunk`
- Proper tracking of tool call states with HashMap

**Deduplication:**
- Skip duplicate status updates when nothing changed
- Track tool call states to avoid duplicate pending events
- Deduplicate identical message chunks

## Test Results

All tests pass successfully:
- 6 unit tests in lib.rs
- 10 tests in main.rs
- 5 playback tests
- 8 session update format tests
- 8 tool call tests

## Evidence Files

### Test Artifacts
- `/dev-docs/review/_artifacts/tests/acp_v1_alignment.jsonl` - ACP v1 compliance test
- `/dev-docs/review/_artifacts/tests/vec_string_command.jsonl` - Vec<String> command test
- `/dev-docs/review/_artifacts/tests/handshake.jsonl` - Basic handshake test

### Implementation Files Modified
1. `src/main.rs` - Initialize response and session/new validation
2. `src/tool_calls.rs` - Vec<String> command support
3. `src/codex_proto.rs` - Error mapping and deduplication improvements

## Validation Output

Initialize response confirms ACP v1 compliance:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": 1,
    "agentCapabilities": {
      "loadSession": false,
      "promptCapabilities": {
        "audio": false,
        "embeddedContext": false,
        "image": false
      }
    },
    "authMethods": []
  }
}
```

## Acceptance Criteria Met

✅ Initialize response exactly matches ACP v1 spec
✅ Session/new enforces cwd absolute path and validates mcpServers
✅ Tool calls support Vec<String> commands from Codex
✅ Completed/failed tool calls include full rawOutput
✅ Minimal update payloads with proper deduplication
✅ Errors in tool context map to ToolCallUpdate with status=failed
✅ All tests pass without regression