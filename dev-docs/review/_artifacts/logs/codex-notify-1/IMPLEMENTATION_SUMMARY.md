# codex-notify-1 Implementation Summary

## Task: Notify sink integration + idle fallback + dedup

**Issue**: [#16](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/16)
**Branch**: feature/codex-notify-1
**Date**: 2025-09-04

## Implementation Overview

Successfully implemented notify sink integration for immediate turn completion via external notifications, with idle timeout as fallback and enhanced deduplication.

## Key Components Implemented

### 1. acplb-notify-forwarder Binary

- **File**: `crates/codex-cli-acp/src/bin/acplb_notify_forwarder.rs`
- Minimal forwarder that reads JSON from argv[1]
- Writes to ACPLB_NOTIFY_PATH (file or FIFO)
- Supports ACPLB_NOTIFY_KIND (file/fifo)

### 2. NotifySource Abstraction

- **File**: `crates/codex-cli-acp/src/notify_source.rs`
- Trait-based monitoring for notification sinks
- FileNotifySource: Polls regular files for new lines
- FifoNotifySource: Monitors FIFO for notifications
- Detects "agent-turn-complete" events for immediate completion

### 3. Notify Injection Logic

- **File**: `crates/codex-cli-acp/src/main.rs`
- Resolves forwarder path (sibling, target dirs, PATH fallback)
- Auto-injects forwarder when ACPLB_NOTIFY_PATH is set
- Respects ACPLB_NOTIFY_INJECT policy (auto/never/force)
- Supports ACPLB_NOTIFY_CMD for custom notify programs

### 4. Immediate Completion Semantics

- Monitors both stdout and notify sink concurrently
- Immediate turn completion on:
  - task_complete from Codex stdout
  - agent-turn-complete from notify sink
  - idle timeout (1200ms default) as fallback
- No duplicate final chunks

### 5. Configuration

Environment variables:

- ACPLB_NOTIFY_PATH: Path to notify sink
- ACPLB_NOTIFY_KIND: file|fifo (default: file)
- ACPLB_NOTIFY_INJECT: auto|never|force (default: auto)
- ACPLB_NOTIFY_CMD: Custom notify command (JSON array)
- ACPLB_IDLE_TIMEOUT_MS: Idle timeout (default: 1200)
- ACPLB_POLLING_INTERVAL_MS: Polling interval (default: 100)

## Quality Gates Passed

✅ **Formatting**: `cargo fmt --all -- --check`
✅ **Linting**: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
✅ **Tests**: `cargo test --workspace --all-features --locked`

- All 4 notify tests passing
- Existing tests remain green

✅ **JSONL Scenarios**:

- Created `notify_idle.jsonl` test scenario
- ACP protocol compliance verified

## Documentation Updated

✅ **README.md**: Added Configuration section with notify integration details
✅ **CLAUDE.md**: Added notify environment variables to Environment Variable Overrides
✅ **CONTRIBUTING.md**: Added notify integration test instructions

## Key Design Decisions

1. **Auto-injection by default**: When ACPLB_NOTIFY_PATH is set and no custom notify command provided, automatically inject forwarder
2. **Absolute path resolution**: Forwarder path resolved to absolute to avoid PATH issues
3. **M1 scope**: File and FIFO support only; Unix domain socket deferred to M2
4. **No ACP wire changes**: Maintained existing session/update structure
5. **Immediate completion priority**: Notify/task_complete trigger immediate turn end without waiting for idle

## Files Changed

### New Files

- `crates/codex-cli-acp/src/bin/acplb_notify_forwarder.rs`
- `crates/codex-cli-acp/src/notify_source.rs`
- `crates/codex-cli-acp/tests/notify_test.rs`
- `dev-docs/review/_artifacts/tests/notify_idle.jsonl`

### Modified Files

- `crates/codex-cli-acp/Cargo.toml` - Added forwarder binary, async-trait, tempfile deps
- `crates/codex-cli-acp/src/main.rs` - Inject notify, integrate monitoring
- `crates/codex-cli-acp/src/codex_proto.rs` - Enhanced completion signaling
- `crates/codex-cli-acp/src/lib.rs` - Export notify module
- `README.md` - Added Configuration section
- `CLAUDE.md` - Added notify env vars
- `CONTRIBUTING.md` - Added notify test instructions

## Test Evidence

```bash
# Test run saved to:
dev-docs/review/_artifacts/logs/codex-notify-1/test_run_*.log

# All tests passing:
- test_notify_forwarder_writes_to_file
- test_notify_forwarder_appends_to_existing_file  
- test_notify_forwarder_fails_without_env
- test_file_notify_source_reads_new_lines
```

## Next Steps

- Ready for PR submission
- Future M2 enhancement: Unix domain socket support
- Future enhancement: More sophisticated file monitoring (inotify/FSEvents)
