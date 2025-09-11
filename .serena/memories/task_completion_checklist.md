# Task Completion Checklist for ACPLazyBridge

## Before Starting a Task

1. Create new worktree from origin/main:

   ```bash
   git worktree add ../task-name origin/main -b feature/task-name
   ```

2. Review relevant issue in `dev-docs/plan/issues/`
3. Check implementation plan in `dev-docs/plan/m1-technical-implementation-plan.md`
4. Check references: (dev-docs/references/)
   - (dev-docs/references/acp.md) - ACP spec
   - (dev-docs/references/zed_ide.md) - Zed IDE documentation
   - (dev-docs/references/acp_adapters/claude_code_acp.md) - ACP adapters for Claude Code documentation
   - (dev-docs/references/cli_agents/) - CLI agents documentation

## During Development

1. Follow ACP specifications strictly (check `dev-docs/references/acp.md`)
2. Maintain protocol discipline:
   - Logs to stderr only
   - stdout for JSONL messages only
   - Use absolute paths
   - 1-based line numbers

## Quality Gates (MANDATORY before commit)

Run all checks in sequence:

```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Linting
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. Tests
cargo test --workspace --all-features --locked

# Combined command:
cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace --all-features --locked
```

## Evidence Collection

1. Create test scenarios under `dev-docs/review/_artifacts/tests/*.jsonl`
2. Capture execution logs:

   ```bash
   cargo run -p codex-cli-acp < test.jsonl 2>&1 | tee dev-docs/review/_artifacts/logs/run_$(date +%Y%m%d_%H%M%S).log
   ```

3. Use jq filters for snapshots (see `dev-docs/review/_artifacts/jq/filters.md`)

## Traceability Updates

1. Update `dev-docs/review/_artifacts/IMPL.csv` with symbol mappings
2. Update `dev-docs/review/_artifacts/traceability.csv` with verification status
3. Link requirements to implementation

## Git Commit

1. Stage relevant files (avoid committing logs with secrets)
2. Write descriptive commit message
3. Reference issue ID in commit
4. Include evidence file references

## Pull Request

1. Link to the issue
2. Explain design decisions
3. Include evidence (test results, logs)
4. Reference spec lines (ACP/REQ/ARC/CODEX/ZED)
5. Use squash merge after approval

## Post-Merge

1. Remove worktree:

   ```bash
   git worktree remove ../task-name
   ```

2. Update issue status
3. Document any new patterns or findings

## Security Checklist

- [ ] No secrets or API keys in code
- [ ] No secrets in logs
- [ ] Permission modes properly documented
- [ ] YOLO mode has explicit warnings
- [ ] Network access controlled by mode

---

Specification Version: 1.0.3 | task_completion_checklist.md ("serena" MCP's memories) Format: 1.0 | Last Updated: 2025-09-11
