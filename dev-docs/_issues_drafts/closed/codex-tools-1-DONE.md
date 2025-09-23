---

Issue status: "closed"

---

# ISSUE: codex-tools-1 — ToolCalls Standardization & 2KB Preview (COMPLETED)

⚠️ ACPLazyBridge related interface design & implementation must strictly follow ACP specifications & check Codex CLI parameters!
**ACP-DocsAndSourceCodeReference**: [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)

## Status: ✅ COMPLETED

## Branch

- **Name**: feature/codex-tools-1
- **Worktree**: /Users/arthur/dev-space/acplb-worktrees/codex-tools-1

## Summary

Successfully implemented comprehensive tool call support for the ACPLazyBridge Codex adapter, including:

- Tool call event parsing and status progression (pending → in_progress → completed/failed)
- Proper ToolKind mapping based on tool names
- 2KB stdout preview truncation for large outputs
- Batch tool call processing support
- Enhanced error handling and output formatting

## Implementation Details

### 1. Created `tool_calls.rs` Module

**File**: `crates/codex-cli-acp/src/tool_calls.rs`

- `map_tool_kind()`: Maps tool names to ACP ToolKind categories (read, edit, delete, move, search, execute, think, fetch, other)
- `truncate_output()`: Smart truncation preserving 75% beginning and 25% end with UTF-8 safety
- `extract_shell_command()`: Extracts command from tool arguments for shell executions
- `format_tool_output()`: Formats tool output with special handling for stdout/stderr/exit_code

### 2. Enhanced `codex_proto.rs`

**File**: `crates/codex-cli-acp/src/codex_proto.rs`

- Extended `CodexEvent` enum to include output and error fields
- Enhanced `CodexStreamManager` with tool call state tracking
- Implemented comprehensive `send_tool_call()` method with:
    - Status progression tracking (avoids duplicate events)
    - Title formatting (uses command for shell tools)
    - Content formatting with 2KB preview
    - Raw output preservation for complete data

### 3. Test Coverage

- **Unit Tests**: `crates/codex-cli-acp/src/tool_calls.rs` (4 tests)
    - Tool kind mapping
    - Output truncation
    - Shell command extraction
    - Output formatting

- **Integration Tests**: `crates/codex-cli-acp/tests/tool_calls_test.rs` (8 tests)
    - Single tool call progression
    - Batch tool calls processing
    - Shell command title formatting
    - Large output truncation
    - Error handling
    - Comprehensive tool kind mapping

- **JSONL Test Cases**: `_artifacts/tests/legacy/`
    - `tool_calls.jsonl`: Basic tool call scenario
    - `tool_calls_batch.jsonl`: Multiple simultaneous tool calls
    - `tool_calls_large_output.jsonl`: 2KB truncation test

## Acceptance Criteria Verification

✅ **Single tool calls progress through pending → in_progress → completed states**

- Implemented in `send_tool_call()` with state tracking via `tool_call_states` HashMap

✅ **Batch tool calls are processed individually with proper status tracking**

- `CodexEvent::ToolCalls` variant processes each call independently

✅ **local_shell commands show command as title and include 2KB stdout preview**

- `extract_shell_command()` extracts command for title
- `format_tool_output()` truncates stdout to 2KB with preserved beginning/end

✅ **Tool kinds are correctly mapped based on tool names**

- Comprehensive mapping with proper precedence (fetch → search → read → ...)

✅ **Output truncation preserves both beginning and end of content**

- Smart truncation: 75% prefix + truncation marker + 25% suffix

✅ **All tool call events conform to ACP schema specification**

- Uses proper ToolCall/ToolCallUpdate variants per ACP protocol

✅ **JSONL test cases pass validation with proper event sequences**

- All tests passing: 37 total tests, 0 failures

## Code Quality

### Compilation

- ✅ No errors
- ✅ No warnings (fixed unused variables and imports)
- ✅ Release build successful

### Test Results

```text
test result: ok. 37 passed; 0 failed; 0 ignored
```

## Files Modified/Created

### Created

1. `crates/codex-cli-acp/src/tool_calls.rs` - Core utilities module
2. `crates/codex-cli-acp/tests/tool_calls_test.rs` - Integration tests
3. `_artifacts/tests/legacy/tool_calls.jsonl` - JSONL test case
4. `_artifacts/tests/legacy/tool_calls_batch.jsonl` - Batch test case
5. `_artifacts/tests/legacy/tool_calls_large_output.jsonl` - Large output test

### Modified

1. `crates/codex-cli-acp/src/lib.rs` - Added tool_calls module export
2. `crates/codex-cli-acp/src/main.rs` - Added tool_calls module import
3. `crates/codex-cli-acp/src/codex_proto.rs` - Enhanced tool call handling
4. `crates/codex-cli-acp/tests/session_update_format.rs` - Existing tests remain passing

## Compliance with Requirements

### ACP Specification Compliance

- ✅ JSON-RPC 2.0 message format maintained
- ✅ Tool call events follow schema.json structure
- ✅ Proper use of ToolCall vs ToolCallUpdate

### Codex CLI Integration

- ✅ Parses Codex proto events correctly
- ✅ Maps tool_call and tool_calls events
- ✅ Handles status progression

### Zed Reference Alignment

- ✅ Matches Zed's ToolCall/ToolCallUpdate structure
- ✅ Includes all required fields (title, kind, status, content, locations, raw_input, raw_output)
- ✅ Proper status enum values (pending, in_progress, completed, failed)

## Next Steps

This implementation is complete and ready for:

1. Integration testing with actual Codex CLI
2. PR submission with evidence from test runs
3. Merge to main after review

## Evidence Location

- Test outputs: Can be generated via `cargo test --package codex-cli-acp 2>&1 | tee _artifacts/logs/legacy/test_$(date +%Y%m%d_%H%M%S).log`
- Build artifacts: `target/release/codex-cli-acp`
- JSONL test cases: `_artifacts/tests/legacy/tool_calls*.jsonl`
