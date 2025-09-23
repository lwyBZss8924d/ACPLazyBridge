# Additional ACP v1 Optimizations

## Implemented Enhancements

### 1. Extended Shell Parameter Support ✅

**File**: `src/tool_calls.rs`

Added comprehensive support for all Codex ShellToolCallParams fields:

- `command`: Supports both string and Vec<String> formats
- `workdir`: Extracted with fallback support for `cwd` and `working_directory`
- `timeout_ms`: Supports both `timeout_ms` and `timeout` aliases
- `with_escalated_permissions`: Supports both field name and `sudo` alias
- `justification`: Extracted with fallback to `reason`

**New Struct**:

```rust
pub struct ExtractedShellParams {
    pub command: Option<String>,
    pub workdir: Option<String>,
    pub timeout_ms: Option<u64>,
    pub with_escalated_permissions: Option<bool>,
    pub justification: Option<String>,
}
```

### 2. Enhanced Error Code Mapping ✅

**File**: `src/codex_proto.rs`

Implemented semantic error categorization for better user feedback:

- `timeout` / `TIMEOUT` → "Tool execution timed out"
- `permission_denied` / `PERMISSION_DENIED` → "Permission denied"
- `not_found` / `NOT_FOUND` → "Resource not found"
- `cancelled` / `CANCELLED` → "Tool execution cancelled"
- `rate_limit` / `RATE_LIMIT` → "Rate limit exceeded"

Error messages are now more descriptive and include category information in `raw_output`.

### 3. Improved Tool Call Handling ✅

**Enhancements**:

- Better error context preservation in tool failures
- Semantic error messages based on error codes
- Extended parameter extraction for shell tools
- Support for alternative field names for compatibility

## Test Coverage

Added comprehensive tests for:

- Shell parameter extraction with all fields
- Alternative field name handling
- Error categorization logic
- Vec<String> command format support

## Benefits

1. **Full Codex Compatibility**: Complete support for ShellToolCallParams structure
2. **Better User Experience**: Clear, semantic error messages
3. **Flexible Integration**: Supports various field name conventions
4. **Enhanced Debugging**: Preserves error context and categories
5. **Future-Proof**: Ready for additional Codex tool types

## API Improvements

The new functions provide:

- `extract_shell_params()`: Complete parameter extraction for shell tools
- Enhanced error mapping with categories
- Backward compatibility with existing field names
- UTF-8 safe output truncation

## Compliance Status

✅ ACP v1 Protocol Compliance:

- Initialize response with integer protocolVersion
- Proper agentCapabilities structure
- Session validation with cwd and mcpServers

✅ Codex Integration:

- Full ShellToolCallParams support
- Error code mapping
- Tool event lifecycle management

✅ Best Practices:

- Minimal update payloads
- Deduplication of events
- Stable tool_call_id tracking
