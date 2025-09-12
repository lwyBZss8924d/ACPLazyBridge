# CLAUDE.md - ACPLazyBridge Repository

## Context-Specific Memory Imports

@specs/CLAUDE.md
@dev-docs/CLAUDE.md
@crates/CLAUDE.md
@.github/CLAUDE.md
@sdd-rules/CLAUDE.md

## Repository Overview

ACPLazyBridge is an ACP (Agent Client Protocol) bridge that connects AI agents and agent-tools plugins with IDEs, editors, and development tools. It provides native adapters for various AI systems while maintaining protocol consistency and developer workflow integration.

## SDD Developer Team Workflow - The Three-Horse Carriage

Our development follows the **Specification-Driven Development (SDD)** methodology, coordinated through three synchronized channels:

### 🎯 The Three-Horse Carriage Model

1. **Specifications** (`specs/<NNN>-<slug>/`) - The source of truth
2. **Development Docs** (`dev-docs/`) - Evidence and artifacts
3. **GitHub Issues** - Coordination and tracking

These three components work together to ensure **dynamic consistency** and **standard-driven development**.

## SDD Workflow Commands

### Primary SDD Commands

| Command | Purpose | Phase | Output |
|---------|---------|-------|--------|
| `/specify` | Create feature specification and branch | 1. Requirements | `specs/<NNN>-<slug>/spec.md` |
| `/plan` | Generate implementation plan | 2. Design | `specs/<NNN>-<slug>/plan.md` |
| `/tasks` | Derive executable tasks | 3. Planning | `specs/<NNN>-<slug>/tasks.md` |

### Quick Workflow

```bash
# Step 1: Start a new feature
/specify Real-time chat system with message history

# Step 2: Create implementation plan  
/plan WebSocket messaging, PostgreSQL history, Redis presence

# Step 3: Generate tasks
/tasks

# Result: Complete SDD artifact tree in specs/<NNN>-feature/
```

## Navigation Quick Reference

### Critical Paths

| Path | Purpose | Key Files |
|------|---------|-----------|
| `specs/` | Feature specifications | `spec.md`, `plan.md`, `tasks.md` |
| `sdd-rules/` | Development rules & templates | `spec-driven.md`, `lifecycle.md`, `AGENTS.md` |
| `dev-docs/review/_artifacts/` | Evidence collection | `tests/`, `logs/`, `reports/` |
| `crates/` | Rust workspace | `acp-lazy-core/`, `codex-cli-acp/` |
| `scripts/` | Automation & CI | `ci/`, `sdd/`, `create-new-feature.sh` |

### Worktree Management

```bash
# ALWAYS create worktree for development
git -C /Users/arthur/dev-space/ACPLazyBridge worktree add \
  /Users/arthur/dev-space/acplb-worktrees/<task-dir> \
  origin/main -b <branch>

# Optional IDE navigation
ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> \
  /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>
```

## Authority and Governance

### Normative Documents (Authoritative)

- **SDD Principles**: [sdd-rules/spec-driven.md](sdd-rules/spec-driven.md)
- **Engineering Rules**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **SDD Lifecycle**: [sdd-rules/lifecycle.md](sdd-rules/lifecycle.md)
- **Team Coordination**: [sdd-rules/AGENTS.md](sdd-rules/AGENTS.md)
- **Rules Index**: [sdd-rules/rules/README.md](sdd-rules/rules/README.md)

### Non-Normative References

- Engineering guides: `dev-docs/engineering/*`
- Chinese documentation: `dev-docs/zh-CN/` (with disclaimer)

## Quality Gates & Validation

### Local CI Suite

```bash
# Run complete validation suite
scripts/ci/run-local-ci.sh

# Individual checks
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features --locked
scripts/ci/run-sdd-structure-lint.sh
scripts/ci/check-language-policy.sh
```

### Constitutional Gates

| Gate | Article | Requirement | Validation |
|------|---------|-------------|------------|
| **Simplicity** | VII | ≤3 projects, no future-proofing | Review complexity |
| **Anti-Abstraction** | VIII | Use frameworks directly | Check for wrappers |
| **Integration-First** | IX | Contracts before code | Verify contracts exist |
| **Test-First** | III | RED→GREEN→REFACTOR | Tests fail first |

## ACP Protocol Standards

### Current Version

**ACP v1** (protocolVersion: 1 as integer)

### Protocol Testing

```bash
# Basic handshake test
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | codex proto

# Full session test with evidence
cat test/acp_messages.jsonl | codex proto -c approval_policy="never" 2>&1 | \
  tee dev-docs/review/_artifacts/<task>/logs/acp_test_$(date +%Y%m%d_%H%M%S).log
```

### Key Conventions

- **stdout**: JSONL protocol messages only
- **stderr**: All logs and diagnostics
- **Format**: One JSON message per line

## Development Commands Reference

### Build & Test

```bash
# Workspace operations
cargo build --workspace --all-features
cargo test --workspace --all-features --locked

# Specific crate
cargo build -p codex-cli-acp
cargo test -p acp-lazy-core
```

### Evidence Collection

```bash
# Create evidence directory
mkdir -p dev-docs/review/_artifacts/<task>/{tests,logs,reports}

# Capture test results
cargo test --workspace 2>&1 | tee dev-docs/review/_artifacts/<task>/logs/test_$(date +%Y%m%d_%H%M%S).log
```

## Team AI Agents Coordination

### Agent Roles in SDD Workflow

| Agent | Role | Primary Phase | Tools |
|-------|------|---------------|-------|
| **Claude Code** | Lead developer & orchestrator | Implementation | All tools + MCP |
| **Warp** | Project manager & reviewer | Planning/Review | Terminal + MCP |
| **Gemini** | Research & documentation | Research | CLI + Web |
| **Cursor** | Pair programming | Implementation | IDE integrated |
| **Codex** | Code analysis | Optimization | CLI analysis |

### Coordination Points

1. **Specification Phase**: Warp + Claude collaborate on requirements
2. **Planning Phase**: Claude generates, Warp validates against gates
3. **Implementation**: Claude leads with Cursor support
4. **Validation**: Warp orchestrates review with all agents

## PR & Commit Standards

### Commit Message Format

```text
<type>(<scope>): <subject>

[TASK-NNN] or [BUG-NNN]

<body>

<footer>
```

### PR Requirements

- Links to `specs/<NNN>-<slug>/`
- Evidence in `dev-docs/review/_artifacts/`
- Quality gates passed
- CI summary included

## Security & Compliance

### Critical Rules

- No secrets in code or logs
- Environment variables for sensitive data
- GitHub secrets for CI/CD
- Code scanning enabled

### Language Policy

- **Normative**: English only (specs, plans, tasks, issues)
- **Non-normative**: Any language (discussions, dev-docs/zh-CN/)

## Implementation Status Dashboard

### Current Milestone: M1

| Component | Status | Priority |
|-----------|--------|----------|
| Codex native adapter | In Progress | High |
| stdio loop | ✅ Complete | - |
| Streaming support | In Progress | High |
| Tool call mapping | In Progress | High |
| Permission system | Planned | Medium |
| Smoke testing | Planned | Medium |

### Roadmap

- **M0**: ✅ Workspace bootstrap, References, SDD integration
- **M1**: 🔄 Codex native adapter
- **M2**: 📋 Proxy adapter, Plugin system v0
- **M3**: 📋 Native adapters, HTTP/SSE bridge

---

## Quick Actions

### Start New Feature

```bash
/specify <feature-description>
/plan <technical-approach>
/tasks
```

### Run Validation

```bash
scripts/ci/run-local-ci.sh
```

### Create PR

```bash
gh pr create --title "<type>(<scope>): <description>" \
  --body "$(cat pr_description.md)"
```

---

Specification Version: 1.0.3 | CLAUDE.md Format: 2.0 | Last Updated: 2025-09-11
