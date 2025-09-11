# ACPLazyBridge Development Workflow

## Team Development Policy
All AI developers (Claude Code, WARP Agent, GEMINI CLI, Cursor, CODEX) follow this unified workflow.

## 1. Task Selection
- Source: `dev-docs/plan/issues/m1-issue-list.md`
- Each issue contains:
  - Design specifications
  - References to specs
  - Acceptance criteria
  - Implementation hints

## 2. Worktree-First Development

### Create New Worktree
```bash
# Standard format
git worktree add ../task-dir origin/main -b feature/module-id

# Example
git worktree add ../codex-proto-1 origin/main -b feature/codex-proto-1

# Container path (required)
/Users/arthur/dev-space/acplb-worktrees/<task-dir>

# Optional symlink for IDE navigation
ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>
```

### Branch Naming Convention

- `feature/<module>-<id>` - New features
- `fix/<module>-<id>` - Bug fixes
- `perf/<module>-<id>` - Performance work
- `docs/<module>-<id>` - Documentation
- `chore/<module>-<id>` - Maintenance

## 3. Development Process

### Pre-Implementation

1. Review issue template
2. Check references: (dev-docs/references/)
   - (dev-docs/references/acp.md) - ACP spec
   - (dev-docs/references/zed_ide.md) - Zed IDE documentation
   - (dev-docs/references/acp_adapters/claude_code_acp.md) - ACP adapters for Claude Code documentation
   - (dev-docs/references/cli_agents/) - CLI agents documentation
3. Review implementation plan

### Implementation

1. Write code following conventions
2. Maintain protocol discipline:
   - stderr for logs
   - stdout for JSONL only
3. Add tests as you go
4. Document complex logic

### Testing Protocol

```bash
# Create test scenarios
cat > dev-docs/review/_artifacts/tests/feature_test.jsonl << EOF
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}
EOF

# Run with logging
cargo run -p codex-cli-acp < dev-docs/review/_artifacts/tests/feature_test.jsonl \
  2>&1 | tee dev-docs/review/_artifacts/logs/run_$(date +%Y%m%d_%H%M%S).log

# Extract snapshots with jq
cat log.jsonl | jq -c 'select(.method == "session/update")'
```

## 4. Quality Assurance

### Local Validation

```bash
# Full quality gate
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features --locked
```

### Evidence Collection

- Test files: `dev-docs/review/_artifacts/tests/*.jsonl`
- Execution logs: `dev-docs/review/_artifacts/logs/*.log`
- JQ snapshots: Apply filters from `dev-docs/review/_artifacts/jq/filters.md`

## 5. Documentation & Traceability

### Update Tracking Files

1. `dev-docs/review/_artifacts/IMPL.csv`:

   ```csv
   Symbol,File:Line,Mapped_IDs
   handle_initialize,main.rs:111,ACP-INIT-01
   ```

2. `dev-docs/review/_artifacts/traceability.csv`:

   ```csv
   REQ_ID,SPEC_REF,Status
   ACP-INIT-01,ACP/initialize,Verified
   ```

## 6. Git Commit

### Commit Message Format

```markdown
feat(codex): implement initialize handler

- Add JSON-RPC 2.0 compliant initialization
- Map ACP capabilities to Codex format
- Include protocol version negotiation

References: ACP-INIT-01, CODEX-SPEC-3.2
Evidence: dev-docs/review/_artifacts/tests/init_test.jsonl
```

### Commit Command

```bash
git add -A
git commit -m "feat(module): description

- Detail 1
- Detail 2

References: ISSUE-ID
Evidence: path/to/evidence"
```

## 7. Pull Request

### PR Template

```markdown
## Summary
Implements [feature] as specified in #ISSUE

## Changes
- Added X to handle Y
- Modified Z for compatibility

## Testing
- Test scenario: dev-docs/review/_artifacts/tests/test.jsonl
- Execution log: dev-docs/review/_artifacts/logs/run_20250101.log
- All quality gates pass

## References
- ACP Spec: Section X.Y
- Codex Docs: Page Z
- Issue: #ISSUE-ID
```

### PR Commands

```bash
# Push to remote
git push -u origin feature/module-id

# Create PR via GitHub CLI
gh pr create --title "feat(module): description" \
  --body "$(cat pr_description.md)"
```

## 8. Post-Merge Cleanup

```bash
# After squash merge
git worktree remove ../task-dir

# Update main
git checkout main
git pull origin main

# Clean up remote branches
git remote prune origin
```

## Multi-Worktree Management

### List Active Worktrees

```bash
git worktree list
```

### Switch Between Worktrees

```bash
cd /Users/arthur/dev-space/acplb-worktrees/feature-name
```

### Port Isolation (if running multiple instances)

```bash
# Use unique ports per worktree
ACPLB_PORT=8001 cargo run  # Worktree 1
ACPLB_PORT=8002 cargo run  # Worktree 2
```

## Collaboration Notes

- Always start from `origin/main`
- Never commit directly to main
- Use squash merge for clean history
- Keep evidence files for review
- Update tracking documents immediately
- Coordinate on shared modules via issues

---

Specification Version: 1.0.3 | development_workflow.md ("serena" MCP's memories) Format: 1.0 | Last Updated: 2025-09-11
