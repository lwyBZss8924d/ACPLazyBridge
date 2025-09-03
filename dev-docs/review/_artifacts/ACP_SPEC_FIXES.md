# ACP Spec Compliance Fixes

## Date: 2025-09-03

## Summary
Fixed critical ACP protocol specification compliance issues identified in WARP review to ensure compatibility with typed ACP clients like Zed.

## Critical Fixes Applied

### 1. ✅ Initialize Response Structure
**Issue**: Response structure violated ACP spec with incorrect types and field names
**Fixes Applied**:
- Changed `protocolVersion` from string "2024-11-05" to integer `1`
- Renamed "capabilities" → "agentCapabilities" per spec
- Nested `promptCapabilities` under `agentCapabilities`
- Added required `authMethods` field (empty array)
- Removed non-spec fields: `fs` capabilities and `serverInfo`
- Accept client `protocolVersion` as either integer or string for compatibility

**File**: `crates/codex-cli-acp/src/main.rs:38-74`

### 2. ✅ Session/New Parameter Names
**Issue**: Using "workingDirectory" instead of spec-required "cwd"
**Fixes Applied**:
- Primary parameter name changed to "cwd" per spec
- Added fallback to "workingDirectory" for backwards compatibility
- Removed default to "." - now requires absolute path
- Added validation for required "mcpServers" parameter
- Proper error mapping with -32602 for missing parameters

**File**: `crates/codex-cli-acp/src/main.rs:77-131`

### 3. ✅ Removed Agent-Side fs/* Handlers  
**Issue**: fs/read_text_file and fs/write_text_file are client methods, not agent methods
**Fix**: Removed handlers completely - agent sends these TO client, doesn't receive them
**Files Modified**: 
- Removed `handle_read_text_file` function
- Removed `handle_write_text_file` function
- Removed cases from `process_message` switch

### 4. ✅ Added Spec Compliance Tests
**New Tests**:
- `test_initialize_response_spec_compliance` - Verifies correct structure
- `test_session_new_cwd_parameter` - Verifies cwd parameter acceptance
- `test_session_new_validation_requirements` - Tests path validation

**File**: `crates/codex-cli-acp/tests/session_update_format.rs:170-266`

## Spec-Compliant Response Examples

### Initialize Response (Correct)
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

### Session/New Request (Correct)
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "session/new",
  "params": {
    "cwd": "/absolute/path/to/project",
    "mcpServers": []
  }
}
```

## Test Results
```bash
cargo test -p codex-cli-acp --tests
# All 8 tests passing including new spec compliance tests
```

## Build Status
```bash
cargo build -p codex-cli-acp
# Builds successfully with expected warnings for removed code
```

## Breaking Changes
- Clients must send integer `protocolVersion` (or we accept string for compatibility)
- Response structure changed to match spec exactly
- fs/* methods no longer handled by agent

## Verification
These changes ensure full compatibility with typed ACP clients like Zed that strictly validate response schemas.