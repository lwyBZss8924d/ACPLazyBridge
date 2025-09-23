# CLAUDE.md (_artifacts/legacy/)

## Authority

- See ../../../sdd-rules/CLAUDE.md and ../../../sdd-rules/AGENTS.md
- Evidence requirements: ../../../sdd-rules/lifecycle.md

## Purpose

Evidence collection directory for SDD compliance. All test results, execution logs, and analysis reports are stored here to provide traceability and validation for specifications.

## Directory Structure

```tree
_artifacts/
├── tests/              # Test execution results
│   └── <task>/        # Per-task test evidence
├── logs/              # Runtime and debug logs
│   └── <task>/        # Per-task execution logs
├── jq/                # JSON query analysis
│   └── <task>/        # Per-task JSON extracts
└── reports/           # Coverage and analysis
    └── <task>/        # Per-task reports
```

## Evidence Collection Guidelines

### Naming Conventions

```bash
# Test files
test_<YYYYMMDD>_<HHMMSS>.log
test_<module>_<YYYYMMDD>.json
test_summary_<task>.md

# Log files
run_<YYYYMMDD>_<HHMMSS>.log
acp_protocol_<YYYYMMDD>.jsonl
debug_<component>_<YYYYMMDD>.log

# Reports
coverage_<YYYYMMDD>.html
performance_<YYYYMMDD>.json
analysis_<task>.md
```

### Task Organization

Each task should have its own subdirectory:

```bash
<task>/
├── README.md           # Task context and summary
├── test_results.log    # Consolidated test output
├── execution.log       # Runtime logs
└── coverage.html       # Coverage report
```

## Common Evidence Types

### Test Evidence

```bash
# Capture test output
cargo test --workspace 2>&1 | tee tests/<task>/test_$(date +%Y%m%d_%H%M%S).log

# Save test results as JSON
cargo test --workspace --format json | tee tests/<task>/results.json

# Document test summary
echo "## Test Summary - $(date)" > tests/<task>/summary.md
echo "Total: X, Passed: Y, Failed: Z" >> tests/<task>/summary.md
```

### Protocol Evidence

```bash
# Capture ACP protocol interaction
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp 2>&1 | \
  tee logs/<task>/acp_$(date +%Y%m%d_%H%M%S).jsonl

# Validate JSONL output
cat logs/<task>/output.jsonl | jq -c 'select(.jsonrpc)' > jq/<task>/valid_messages.json
```

### Performance Evidence

```bash
# Benchmark execution
hyperfine --export-json reports/<task>/benchmark.json \
  'cargo run --release -p codex-cli-acp < test.jsonl'

# Memory profiling
valgrind --tool=massif --massif-out-file=reports/<task>/memory.out \
  target/release/codex-cli-acp
```

### Coverage Evidence

```bash
# Generate HTML coverage
cargo tarpaulin --out Html \
  --output-dir reports/<task>/

# Generate JSON coverage for analysis
cargo tarpaulin --out Json \
  --output-dir reports/<task>/
```

## Evidence Requirements by Phase

### Specification Phase

- [ ] Requirements documented
- [ ] User stories captured
- [ ] Acceptance criteria defined

### Planning Phase

- [ ] Technical decisions documented
- [ ] Risk assessment completed
- [ ] Dependencies identified

### Implementation Phase

- [ ] Test results (RED phase)
- [ ] Test results (GREEN phase)
- [ ] Refactoring evidence

### Validation Phase

- [ ] All tests passing
- [ ] Coverage meets threshold
- [ ] Performance benchmarks
- [ ] Security scan results

## Traceability Matrix

Link evidence to specifications:

| Requirement | Test | Evidence | Status |
|-------------|------|----------|--------|
| REQ-001 | test_init | tests/001/test_20250111.log | ✅ |
| REQ-002 | test_auth | tests/001/auth_20250111.log | ✅ |
| NFR-001 | benchmark | reports/001/perf.json | ✅ |

## Evidence Validation

### Checklist for PR

- [ ] Test logs present in tests/<task>/
- [ ] Execution logs in logs/<task>/
- [ ] Coverage report in reports/<task>/
- [ ] All tests passing
- [ ] No security issues

### Automated Validation

```bash
# Check evidence exists
./scripts/ci/validate-evidence.sh <task>

# Verify test results
grep -c "test result: ok" tests/<task>/*.log

# Check coverage threshold
jq '.coverage_percentage' reports/<task>/coverage.json
```

## Retention Policy

### Keep Forever

- Release evidence
- Security audit results
- Performance baselines

### Keep 30 Days

- Development test runs
- Debug logs
- Intermediate builds

### Clean Up

```bash
# Remove old development evidence
find . -name "*.log" -mtime +30 -delete

# Archive completed tasks
tar -czf archive/<task>_$(date +%Y%m%d).tar.gz <task>/
```

## Quick Commands

### Collect All Evidence

```bash
# Run full evidence collection
task=<task-name>
mkdir -p {tests,logs,jq,reports}/$task

# Tests
cargo test --workspace 2>&1 | tee tests/$task/test_$(date +%Y%m%d_%H%M%S).log

# Coverage
cargo tarpaulin --out Html --output-dir reports/$task/

# Logs
RUST_LOG=debug cargo run -p codex-cli-acp < test.jsonl 2>logs/$task/debug.log
```

### Generate Evidence Report

```bash
# Create summary report
cat > reports/$task/summary.md <<EOF
# Evidence Summary: $task

## Tests
$(ls -la tests/$task/)

## Logs
$(ls -la logs/$task/)

## Coverage
$(jq '.coverage_percentage' reports/$task/coverage.json)

## Validation
- [ ] All tests pass
- [ ] Coverage > 80%
- [ ] No security issues
EOF
```

---

---

```yaml
Constitution version: 1.0.1
Document: Last Updated
Document version: 2025
Last Updated: 2025-09-17
```
