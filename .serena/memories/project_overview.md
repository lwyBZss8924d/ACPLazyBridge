# ACPLazyBridge Project Overview

## Purpose

ACPLazyBridge is an ACP (Agent Client Protocol) bridge that connects AI agents and agent-tools plugins with IDEs, editors, and development tools. It provides native adapters for various AI systems while maintaining protocol consistency and developer workflow integration through Specification-Driven Development (SDD).

## Tech Stack

- **Language**: Rust (stable toolchain)
- **Protocol**: ACP (Agent Client Protocol) v1 over stdio
- **Format**: JSON-RPC 2.0 with line-separated JSON (JSONL)
- **Build System**: Cargo with workspace configuration
- **Development Methodology**: Specification-Driven Development (SDD)

## Key Features

- Streaming support for real-time agent responses
- Tool call handling with permission mapping
- Non-interactive permission modes to avoid UI approval prompts
- Extensible plugin system (planned)
- Multi-agent support (Codex native, Claude/Gemini proxy planned)
- SDD-compliant development workflow

## Specification-Driven Development (SDD)

Every feature or change follows the SDD workflow:

1. **Specify**: Create specification under `specs/<NNN>-<slug>/spec.md`
2. **Plan**: Technical design in `specs/<NNN>-<slug>/plan.md`
3. **Tasks**: Breakdown in `specs/<NNN>-<slug>/tasks.md`
4. **Implement**: Follow worktree-first development
5. **Validate**: Local CI checks and evidence collection
6. **Review**: PR with full traceability

### SDD Commands

- `/specify` - Generate feature specification
- `/plan` - Create implementation plan
- `/tasks` - Derive executable tasks

## Authority and Governance

### Normative Documents (Authoritative)

- **Engineering Ground Rules**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **SDD Principles & Workflow**: [sdd-rules/spec-driven.md](sdd-rules/spec-driven.md)
- **SDD Lifecycle**: [sdd-rules/lifecycle.md](sdd-rules/lifecycle.md)
- **Team Rules**: [sdd-rules/AGENTS.md](sdd-rules/AGENTS.md)
- **Rules Index**: [sdd-rules/rules/README.md](sdd-rules/rules/README.md)

### Non-Normative References

- Engineering guides: `dev-docs/engineering/*` (each file links back to authority)
- Chinese documentation: `dev-docs/zh-CN/` (with disclaimer)

## Project Status

- **M0**: ✅ Completed - Basic workspace setup, SDD rules integrated
- **M1**: 🚧 In Progress - Codex Native Adapter implementation
  - stdio loop implementation
  - Streaming support
  - Tool call mapping
  - Permission system
  - Smoke testing
- **M2-M5**: 📋 Planned
  - Proxy adapters (Claude, Gemini)
  - Plugin system v0
  - Native adapters
  - HTTP/SSE bridge

## Repository Structure

```tree
├── crates/              # Rust workspace
│   ├── acp-lazy-core/   # Core protocol implementation
│   └── codex-cli-acp/   # Codex CLI adapter
├── scripts/             # CI and automation
│   ├── ci/              # CI scripts
│   └── sdd/             # SDD validation
├── specs/               # SDD specifications
├── sdd-rules/           # Development rules
│   ├── rules/           # Categorized rules
│   └── commands/        # SDD command docs
├── dev-docs/            # Development docs
│   ├── review/          # Review artifacts
│   │   └── _artifacts/  # Evidence storage
│   └── engineering/     # Non-normative guides
└── issues/              # Issue templates
```

## Quality Gates

### Constitutional Gates

- **Simplicity** (Article VII): ≤3 projects, no future-proofing
- **Anti-Abstraction** (Article VIII): Use framework features directly
- **Integration-First** (Article IX): Contracts defined before implementation
- **Test-First** (Article III): Tests fail (RED) before implementation

### Local CI Checks (Must Pass)

```bash
# Rust quality
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features --locked

# SDD compliance
scripts/ci/run-sdd-structure-lint.sh
scripts/ci/check-language-policy.sh

# Local comprehensive check
scripts/ci/run-local-ci.sh
```

## Team AI Agents

AI Developer coding agents team members include:

- **CLAUDE** "Claude Code Agent" (anthropic Claude Code CLI client link - `CLAUDE.md`)
- **WARP** "WARP Agent" (WARP.dev Terminal link - `WARP.md`)
- **GEMINI** "GEMINI CLI Agent" (google gemini-cli link - `GEMINI.md`)
- **CURSOR** "Cursor Agent" (Cursor IDE cursor-agent link - `.cursorrules`)
- **CODEX** "CODEX Agent" (OpenAI codex-cli link - `AGENTS.md`)

All AI Developers coding agent's client rules in this codebase ~/ root file are uniformly linked to [sdd-rules/AGENTS.md](sdd-rules/AGENTS.md)

---

Specification Version: 1.0.3 | project_overview.md ("serena" MCP's memories) Format: 1.0 | Last Updated: 2025-09-11
