# Suggested Commands for ACPLazyBridge Development

## SDD Validation Commands

### Full Local CI Suite

```bash
# Run complete local CI validation (ALWAYS run before PR)
scripts/ci/run-local-ci.sh

# Individual SDD checks
scripts/ci/run-sdd-structure-lint.sh    # Validate SDD structure
scripts/ci/check-language-policy.sh     # Check language compliance
scripts/sdd/run_semantic_checks.sh      # Semantic validation

# Python SDD validator
python scripts/sdd/validate_structure.py

# Markdown style check
markdownlint . --config .markdownlint.json
```

## Build Commands

```bash
# Build entire workspace
cargo build --workspace

# Build with all features
cargo build --workspace --all-features

# Build specific crate
cargo build -p codex-cli-acp
cargo build -p acp-lazy-core

# Release build
cargo build --workspace --release
```

## Test Commands

```bash
# Run all tests in workspace
cargo test --workspace --all-targets

# Run tests with all features
cargo test --workspace --all-targets --all-features

# Run tests for specific crate
cargo test -p acp-lazy-core
cargo test -p codex-cli-acp

# Run with coverage (if tarpaulin installed)
cargo tarpaulin --workspace --all-features

# Run streaming benchmark
cargo test --package codex-cli-acp --test streaming_benchmark --release
```

## Code Quality Commands

```bash
# Format all code
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check

# Run clippy linter
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Quick check without building
cargo check --workspace

# Full quality gate (REQUIRED before commits)
cargo fmt --all -- --check && \
cargo clippy --workspace --all-targets --all-features -- -D warnings && \
cargo test --workspace --all-features --locked
```

## Running the Codex Adapter

```bash
# Run with default settings
cargo run -p codex-cli-acp

# Run with verbose logging
RUST_LOG=info cargo run -p codex-cli-acp

# Debug mode with backtrace
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -p codex-cli-acp

# Trace level for protocol debugging
RUST_LOG=trace cargo run -p codex-cli-acp 2>&1 | tee debug.log
```

## ACP Protocol Testing

```bash
# Test basic ACP handshake
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp

# Test with JSONL file
cat test/acp_messages.jsonl | cargo run -p codex-cli-acp

# Test with Codex proto command
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  codex proto

# Test with permission modes
codex proto -c approval_policy="never" < test/session.jsonl
codex proto -c sandbox_mode="read-only" < test/readonly.jsonl
codex proto -c sandbox_mode="workspace-write" < test/edit.jsonl

# Validate JSON-RPC responses
codex proto < test/requests.jsonl | jq -c 'select(.jsonrpc == "2.0")'
```

## Documentation

```bash
# Build documentation
cargo doc --workspace --no-deps

# Build and open docs in browser
cargo doc --workspace --no-deps --open

# Generate test documentation
cargo test --workspace --doc
```

## Git Workflow Commands

### Worktree Management

```bash
# Create new worktree for feature (ALWAYS use this)
git -C /Users/arthur/dev-space/ACPLazyBridge worktree add \
  /Users/arthur/dev-space/acplb-worktrees/<task-dir> \
  origin/main -b <branch-name>

# Example for SDD feature
git -C /Users/arthur/dev-space/ACPLazyBridge worktree add \
  /Users/arthur/dev-space/acplb-worktrees/001-feature-name \
  origin/main -b feature/001-feature-name

# List worktrees
git worktree list

# Remove worktree after merge
git worktree remove /Users/arthur/dev-space/acplb-worktrees/<task-dir>

# Create IDE navigation symlink (optional)
ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> \
  /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>
```

### Git Status and Diff

```bash
# Check status
git status

# View changes
git diff                    # Unstaged changes
git diff --staged          # Staged changes
git diff origin/main       # All changes vs main

# Show commit history
git log --oneline -10     # Last 10 commits
git log --graph --oneline # Branch visualization
```

### Creating Pull Requests

```bash
# Push to remote
git push -u origin <branch-name>

# Create PR with GitHub CLI
gh pr create --title "feat(module): description" \
  --body "$(cat pr_description.md)"

# List PRs
gh pr list

# View PR
gh pr view <number>
```

## Evidence Collection

```bash
# Create evidence directory for task
mkdir -p dev-docs/review/_artifacts/<task>/{tests,logs,reports}

# Run tests with evidence
cargo test --workspace 2>&1 | \
  tee dev-docs/review/_artifacts/<task>/logs/test_$(date +%Y%m%d_%H%M%S).log

# Capture ACP protocol test
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp 2>&1 | \
  tee dev-docs/review/_artifacts/<task>/logs/acp_$(date +%Y%m%d_%H%M%S).log

# Run quality checks with evidence
scripts/ci/run-local-ci.sh 2>&1 | \
  tee dev-docs/review/_artifacts/<task>/reports/ci_$(date +%Y%m%d_%H%M%S).log
```

## System Utils (Darwin/macOS)

```bash
# File operations
ls -la                    # List files with details
find . -name "*.rs"       # Find Rust files
rg "pattern"              # Ripgrep search (faster than grep)
fd "pattern"              # Find files (faster than find)

# Process monitoring
ps aux | grep codex       # Find codex processes
lsof -i :8080            # Check port usage
kill -9 <PID>            # Force kill process

# Debugging
lldb target/debug/codex-cli-acp  # Debug with LLDB
rust-gdb target/debug/codex-cli-acp  # Debug with GDB

# Performance
time cargo build          # Measure build time
hyperfine 'cargo test'    # Benchmark command execution
```

## Cleanup Commands

```bash
# Clean build artifacts
cargo clean

# Clean specific package
cargo clean -p codex-cli-acp

# Remove worktree
git worktree remove <path>

# Prune remote branches
git remote prune origin

# Remove local branches merged to main
git branch --merged main | grep -v main | xargs -n 1 git branch -d
```

## Quick Validation Before PR

```bash
# One-liner for all checks
scripts/ci/run-local-ci.sh && echo "✅ Ready for PR!" || echo "❌ Fix issues first!"
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "serena-memories"
    memories: "suggested_commands"
    status: "expired"
    path: ".serena/memories/suggested_commands.md"
    version: "1.0.1"
    last_updated: "2025-09-14T08:26:00Z"
```
