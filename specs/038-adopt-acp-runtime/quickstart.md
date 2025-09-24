# Quickstart: ACP Runtime Testing Guide

## Prerequisites

- Rust 1.89+ installed
- cargo and rustc in PATH
- agent-client-protocol crate available
- Existing JSONL test scenarios

## Setup

```bash
# Navigate to worktree
cd ../acplb-worktrees/038-adopt-acp-runtime

# Optional: capture runtime evidence emitted by RuntimeServer
export ACPLB_EVIDENCE_PATH="_artifacts/038-adopt-acp-runtime/logs/runtime_events.jsonl"

# Build the project
cargo build --workspace
```

## Testing the Implementation

### 1. Unit Tests

Run the Agent trait contract tests:

```bash
# Test runtime module
cargo test -p acp-lazy-core --test runtime_test

# Test CodexAgent implementation
cargo test -p codex-cli-acp codex_agent
```

### 2. Integration Tests

Test the full session lifecycle:

```bash
# Run integration test suite
cargo test --test acp_integration_test -- --nocapture

# Test notify handling
cargo test --test acp_integration_test notify_signal_causes_early_stop_reason

# Test timeout behavior
cargo test --test acp_integration_test idle_timeout_without_notify_returns_end_turn
```

### 3. JSONL Regression Tests

Validate backward compatibility:

```bash
# Run handshake scenario with the playback helper
cargo run -p codex-cli-acp --bin playback < _artifacts/tests/protocol-baseline/handshake.jsonl

# Run full scenario suite
cargo test --test jsonl_regression_test -- --nocapture
```


### 4. Performance Validation

Measure message processing time:

```bash
# Run performance benchmark
cargo run --release -p codex-cli-acp --bin playback < _artifacts/tests/protocol-baseline/prompt_with_mock_codex.jsonl

# Check timing logs
grep "message_processing_time" debug.log | awk '{print $NF}' | sort -n
```

### 5. Notify Source Testing

Test external turn completion:

```bash
# Create notify FIFO
mkfifo /tmp/notify.pipe

# Run with notify configured
ACPLB_NOTIFY_PATH=/tmp/notify.pipe \
ACPLB_NOTIFY_KIND=fifo \
  cargo run -p codex-cli-acp &

# Send turn completion signal
echo '{"type":"agent-turn-complete"}' > /tmp/notify.pipe
```

### 6. Cancellation Testing

Test session cancellation:

```bash
# Replay cancellation fixture and stream updates
cargo run -p codex-cli-acp --bin playback < _artifacts/tests/protocol-baseline/prompt_and_cancel.jsonl

# Unit-level coverage
cargo test --test playback test_cancel_notification -- --nocapture
```

## Validation Checklist

- [ ] All Agent trait methods respond correctly
- [ ] Session state persists across prompts (inspect `runtime_events.jsonl`)
- [ ] Notify events end turns immediately
- [ ] Idle timeout triggers after configured duration
- [ ] Cancellation kills child processes
- [ ] JSONL output remains valid and compatible
- [ ] Performance meets ≤150 ms requirement (Roadmap FR-0105)
- [ ] No memory leaks under load

## Evidence Collection

Capture test results for PR:

```bash
# Create evidence directory
mkdir -p _artifacts/038-adopt-acp-runtime/{tests,logs,reports}

# Run full test suite with evidence
cargo test --workspace 2>&1 | \
  tee _artifacts/038-adopt-acp-runtime/tests/full_suite_$(date +%Y%m%d_%H%M%S).log

# Capture JSONL regression results
cargo test --test jsonl_regression_test -- --nocapture 2>&1 | \
  tee _artifacts/038-adopt-acp-runtime/tests/jsonl_regression_$(date +%Y%m%d_%H%M%S).log

# Generate coverage report
cargo tarpaulin --workspace --out Html \
  --output-dir _artifacts/038-adopt-acp-runtime/reports/
```

## Troubleshooting

### LocalSet Errors

If you see "!Send future" errors:

- Ensure all ACP operations are within LocalSet::run_until
- Use spawn_local instead of spawn
- Don't pass connections across thread boundaries

### JSONL Compatibility Issues

If output differs from baseline:

- Check protocol version is integer (1) not string ("1")
- Verify stdout has only JSONL, no debug output
- Compare outputs with `cargo test -- --nocapture` to inspect differences

### Permission Mapping Problems

If Codex doesn't respect permissions:

- Verify CLI arguments are correctly formatted (inspect stderr and runtime_events.jsonl)
- Check environment variable overrides
- Confirm permission mode is stored in session (see session_mode_changed events)


## Next Steps

After all tests pass:

1. Remove legacy JSON-RPC code
2. Update documentation
3. Run SDD validation
4. Create PR with evidence
5. Request review
