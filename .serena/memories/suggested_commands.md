# Suggested Commands for ACPLazyBridge Development

## Build Commands
```bash
# Build entire workspace
cargo build --workspace

# Build with all features
cargo build --workspace --all-features

# Build specific crate
cargo build -p codex-cli-acp
cargo build -p acp-lazy-core
```

## Test Commands
```bash
# Run all tests in workspace
cargo test --workspace --all-targets

# Run tests with all features
cargo test --workspace --all-targets --all-features

# Run tests for specific crate
cargo test -p acp-lazy-core

# Run streaming benchmark
cargo test --package codex-cli-acp --test streaming_benchmark --release
```

## Code Quality (Required before commits)
```bash
# Format all code
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check

# Run clippy linter
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Quick check without building
cargo check --workspace

# Full quality gate (run before PR)
cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace --all-features --locked
```

## Running the Codex Adapter
```bash
# Run with default settings
cargo run -p codex-cli-acp

# Run with verbose logging
RUST_LOG=info cargo run -p codex-cli-acp

# Debug mode with backtrace
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -p codex-cli-acp

# Test ACP protocol compliance
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | cargo run -p codex-cli-acp
```

## Documentation
```bash
# Build documentation
cargo doc --workspace --no-deps

# Build and open docs in browser
cargo doc --workspace --no-deps --open
```

## Git Workflow
```bash
# Create new worktree for feature
git worktree add ../feature-name origin/main -b feature/feature-name

# List worktrees
git worktree list

# Check status
git status
```

## System Utils (Darwin/macOS)
```bash
# File operations
ls -la          # List files with details
find . -name    # Find files
grep -r         # Recursive search
rg              # ripgrep (faster alternative)

# Process monitoring
ps aux | grep rust-analyzer
lsof -i :PORT   # Check port usage

# Debugging
lldb            # macOS debugger
dtruss          # System call trace (macOS)
```