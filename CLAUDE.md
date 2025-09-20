# CLAUDE.md

```text
This file provides guidance to CLAUDE (Claude Code Agents) when working with code in this repository. as AI Engineers ("claude" agent) working within our Project - Repository "ACPLazyBridge" Specification‑Driven Development (SDD) team. It follows the SDD principles in (.specify/memory/constitution.md) . ****Claude AI Engineer**** collaborates with a team that includes human developers and other AI engineers, team work with AI coding agents such as Claude Code, Gemini, and Codex. Together, the team plans and writes code that strictly follows the specification. It complements CONTRIBUTING.
```

- **Project Name**: `ACPLazyBridge` (Rust workspace)
- **Project Repository URL**: <https://github.com/lwyBZss8924d/ACPLazyBridge>
- **ACP (Agent Client Protocol) Protocol**: <https://github.com/zed-industries/agent-client-protocol>
- **ACP JSON Schema**: <https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json>
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

```text
Team's AI Engineer member: ("claude")'s role and operating rules for **ACPLazyBridge**. It is a role-specific guide. For the authoritative workflow and lifecycle, always refer to the documents listed below. and always refer to the SDD Constitution. wen update any SDD document and sdd-rules document, MUST follow the SDD Constitution Update Checklist. All SDD document and sdd-rules document and normative artifacts (specify, plan, tasks, issues, PRDs, commits, etc.) MUST be English‑only.
```

## Repository Overview

ACPLazyBridge is an ACP (Agent Client Protocol) bridge that connects AI agents and agent-tools plugins with IDEs, editors, and development tools.
It provides native adapters for various AI systems while maintaining protocol consistency and developer workflow integration through Specification-Driven Development (SDD).

## Architecture

The project consists of two main crates:

### `acp-lazy-core/` - Protocol Library

- Core ACP protocol types and transport layer
- Permission system and validation logic
- Shared utilities for all adapters

### `codex-cli-acp/` - Codex CLI Adapter

- Implements ACP server for Codex CLI
- Handles streaming with real-time `agent_message_chunk` events
- Maps tool calls between ACP and Codex formats
- Manages turn completion via notifications or idle timeout
- Key components:
    - `codex_proto.rs` - Main protocol handler and session management
    - `streaming.rs` - Real-time streaming implementation
    - `tool_calls.rs` - Tool call mapping and execution
    - `notify.rs` - External notification system for turn completion

## Common Development Commands

### Build & Test

```bash
# Full quality check (run before any commit)
cargo fmt --all -- --check && \
cargo clippy --workspace --all-targets --all-features -- -D warnings && \
cargo test --workspace --all-features --locked

# Quick build
cargo build --workspace

# Run tests
cargo test --workspace --all-features

# Run with debug logging
RUST_LOG=debug cargo run -p codex-cli-acp
```

### ACP Protocol Testing

```bash
# Test basic handshake (protocol version must be integer 1, not string)
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | \
  cargo run -p codex-cli-acp

# Test with JSONL scenarios
cat dev-docs/review/_artifacts/tests/handshake.jsonl | cargo run -p codex-cli-acp

# Test with Codex proto command
codex proto -c approval_policy="never" < dev-docs/review/_artifacts/tests/basic_session.jsonl
```

### Code Analysis command line Tools

Tip: When you need to do Code Search and Retrieval and any Codebase Analysis Operations, Can use subagent: "code-retriever" or "code-analyzer"

Advanced code analysis techniques: @sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md

**BASE Command line Tools**:

- Find Files: `fd`
- Find Text: `rg` (ripgrep) `search` and `parse`
- Find Code Structure: `ast-grep`
- Select among matches: pipe to `fzf`
- JSON: `jq`
- YAML/XML: `yq`

### You have Augmented CLI Development Tooling

> [sdd-rules-tools-cli-list](sdd-rules/rules/tools-cli/sdd-rules-tools-cli-list.md)

#### `ast-grep` (AST-based Code Analysis)

> [sdd-rules-tools-cli-astgrep](sdd-rules/rules/tools-cli/sdd-rules-tools-cli-astgrep.md)

```bash
# Scan for code issues
ast-grep scan -c ./sgconfig.yml --inspect summary .

# Check for unwrap() usage in Rust
./scripts/ast-grep/sg-baseline-acp-rust-no-unwrap.sh

# Check for dbg! macros
./scripts/ast-grep/sg-baseline-acp-rust-dbg.sh

# Check for TODO comments
./scripts/ast-grep/sg-baseline-acp-rust-todo.sh
```

#### "SemTools" `search` and `parse` (Document Search and Parsing)

> [sdd-rules-tools-cli-document-search-and-parsing](sdd-rules/rules/tools-cli/sdd-rules-tools-cli-document-search-and-parsing.md)

##### Parse CLI Help

```bash
parse --help
A CLI tool for parsing documents using various backends

Usage: parse [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Files to parse

Options:
  -c, --parse-config <PARSE_CONFIG>  Path to the config file. Defaults to ~/.parse_config.json
  -b, --backend <BACKEND>            The backend type to use for parsing. Defaults to `llama-parse` [default: llama-parse]
  -v, --verbose                      Verbose output while parsing
  -h, --help                         Print help
  -V, --version                      Print version
```

##### Search CLI Help

```bash
search --help
A CLI tool for fast semantic keyword search

Usage: search [OPTIONS] <QUERY> [FILES]...

Arguments:
  <QUERY>     Query to search for (positional argument)
  [FILES]...  Files to search (optional if using stdin)

Options:
  -n, --n-lines <N_LINES>            How many lines before/after to return as context [default: 3]
      --top-k <TOP_K>                The top-k files or texts to return (ignored if max_distance is set) [default: 3]
  -m, --max-distance <MAX_DISTANCE>  Return all results with distance below this threshold (0.0+)
  -i, --ignore-case                  Perform case-insensitive search (default is false)
  -h, --help                         Print help
  -V, --version                      Print version
```

### SDD Validation

```bash
# Run full local CI suite (includes all SDD checks)
scripts/ci/run-local-ci.sh

# Run individual checks
scripts/sdd/validate-sdd-docs.sh
scripts/sdd/run_semantic_checks.sh
scripts/sdd/check_language.sh
```

## Important Conventions

### Protocol Requirements

- **stdout**: JSONL protocol messages only (never logs)
- **stderr**: All logs and diagnostics
- **protocolVersion**: Always integer `1`, never string `"1"`

### Git Workflow

Always use worktrees for development:

```bash
# Create worktree for new feature
git worktree add ../acplb-worktrees/<task-name> origin/main -b feature/<task-name>

# After merge, clean up
git worktree remove ../acplb-worktrees/<task-name>
```

### Evidence Collection

Store test evidence for PRs:

```bash
# Run with evidence capture
cargo test --workspace 2>&1 | \
  tee dev-docs/review/_artifacts/<task>/logs/test_$(date +%Y%m%d_%H%M%S).log
```

## Testing Approach

The project uses JSONL files for protocol testing. Key test scenarios are in `dev-docs/review/_artifacts/tests/`:

- `handshake.jsonl` - Basic initialization
- `basic_session.jsonl` - Session creation and prompting
- `tool_calls.jsonl` - Tool execution flows
- `streaming_benchmark.jsonl` - Performance testing

## Notification System

The adapter supports external turn completion signals via environment variables:

- `ACPLB_NOTIFY_PATH`: Path to notification sink
- `ACPLB_NOTIFY_KIND`: `file` or `fifo`
- `ACPLB_NOTIFY_INJECT`: `auto`, `never`, or `force`

This allows immediate turn completion instead of waiting for idle timeout.

## Quick Start for New Features

1. Create worktree: `git worktree add ../acplb-worktrees/feature-name origin/main -b feature/name`
2. Run quality gates: `scripts/ci/run-local-ci.sh`
3. Test ACP protocol: `cargo run -p codex-cli-acp < test.jsonl`
4. Collect evidence: Store logs in `dev-docs/review/_artifacts/<task>/`
5. Create PR with links to specs and evidence

## Current Focus (M1)

- Codex native adapter implementation
- Streaming support with real-time chunks
- Tool call mapping and execution
- Turn completion via notifications

---

## Specification-Driven Development (SDD)

**What is SDD?**

Spec-Driven Development flips the script on traditional software development. For decades, code has been king — specifications were just scaffolding we built and discarded once the "real work" of coding began.
Spec-Driven Development changes this: specifications become executable, directly generating working implementations rather than just guiding them.

[spec-driven.md](.specify/spec-driven.md)

**SDD-CONSTITUTION**

- [SDD Constitution](.specify/memory/constitution.md)
- [SDD Constitution Update Checklist](.specify/memory/constitution_update_checklist.md)

**SDD-RULES**

root path: (`sdd-rules/`)

- SDD Rules Index: [README.md](sdd-rules/rules/README.md)

**SDD-SCRIPTS**

root path: (`scripts/sdd/`)

- `create-new-feature.sh` with (.specify/commands/specify.md) use (.specify/templates/spec-template.md)
- `setup-plan.sh` with (.specify/commands/plan.md) use (.specify/templates/plan-template.md)
- `check-task-prerequisites.sh` with (.specify/commands/tasks.md) use (.specify/templates/tasks-template.md)

```json
{
  "Check-Markdown": "check-markdown.sh",
  "Check-Language": "check_language.sh",
  "Create-New-Feature": "create-new-feature.sh",
  "Get-Feature-Paths": "get-feature-paths.sh",
  "Run-Semantic-Checks": "run_semantic_checks.sh",
  "Update-Agent-Context": "update-agent-context.sh",
  "Check-Task-Prerequisites": "check-task-prerequisites.sh",
  "Common": "common.sh",
  "Fix-Markdown": "fix-markdown.sh",
  "Lint-Docs": "lint_docs.sh",
  "Setup-Plan": "setup-plan.sh",
  "Validate-SDD-Docs": "validate-sdd-docs.sh"
}
```

**SDD-WORKSPACE**

root path: (`specs/`)

### SDD META DOCUMENTATION DIRECTORY: (.specify/)

root path: (`.specify/`)

<sdd-specify>

```tree
ACPLazyBridge/.specify
❯ tree
.
├── commands
│   ├── plan.md
│   ├── specify.md
│   └── tasks.md
├── commands-template
│   ├── plan.md
│   ├── specify.md
│   └── tasks.md
├── memory
│   ├── constitution.md
│   ├── constitution_update_checklist.md
│   └── lifecycle.md
├── memory-template
│   ├── constitution.md
│   └── constitution_update_checklist.md
├── scripts-template
│   └── bash
│       ├── check-task-prerequisites.sh
│       ├── common.sh
│       ├── create-new-feature.sh
│       ├── get-feature-paths.sh
│       ├── setup-plan.sh
│       └── update-agent-context.sh
├── spec-driven.md
└── templates
    ├── agent-file-template.md
    ├── plan-template.md
    ├── spec-template.md
    └── tasks-template.md
```

</sdd-specify>

### SDD-RULES - SDD All Detailed Rules Documentation

root path: (`sdd-rules/`)

- SDD Rules Index: [README.md](sdd-rules/rules/README.md)

<sdd-rules>

```tree
ACPLazyBridge/sdd-rules
❯ tree
.
├── AGENTS.md
├── CLAUDE.md
├── rule-tests
│   ├── rust-mutex-lock-test.yml
│   └── rust-no-unwrap-test.yml
└── rules
    ├── README.md
    ├── changelog
    │   ├── keep-a-changelog-index.html.haml
    │   ├── sdd-rules-changelog.md
    │   └── semver.md
    ├── ci
    │   └── sdd-rules-ci.md
    ├── code-analysis
    │   ├── ast-grep
    │   │   ├── go
    │   │   │   └── no-fmt-println.yml
    │   │   ├── js
    │   │   │   ├── no-console-log.yml
    │   │   │   └── no-only-in-tests.yml
    │   │   ├── python
    │   │   │   ├── no-pdb.yml
    │   │   │   └── no-print.yml
    │   │   └── rust
    │   │       ├── no-dbg.yml
    │   │       ├── no-unwrap.yml
    │   │       ├── rust-mutex-lock.yml
    │   │       └── todo-comment.yml
    │   └── sdd-rules-code-analysis.md
    ├── documentation-style
    │   ├── Google-developer-documentation-style-guide.md
    │   ├── sdd-rules-documentation-markdownlint.md
    │   └── sdd-rules-documentation-style.md
    ├── git
    │   ├── comments
    │   │   └── sdd-rules-comments.md
    │   ├── issues
    │   │   └── sdd-rules-issues.md
    │   ├── pr
    │   │   └── sdd-rules-pr.md
    │   └── worktree
    │       └── sdd-rules-worktrees.md
    ├── research
    │   └── sdd-rules-research.md
    ├── tests
    │   └── sdd-rules-tests.md
    ├── tools-cli
    │   ├── ast-grep.llms.txt
    │   ├── sdd-rules-tools-cli-astgrep.md
    │   └── sdd-rules-tools-cli-list.md
    └── tools-mcp
        └── sdd-rules-tools-mcp.md
```

</sdd-rules>


### Role and responsibilities

- Task analysis and solution design: clarify scope, assumptions, constraints; propose architecture and acceptance criteria.
- Planning: break down issues into executable tasks with traceability to requirements/spec/design.
- Local verification: build, lint, test; replay protocol JSONL scenarios; produce logs and evidence.
- Code review support: summarize diffs, risks, and evidence; recommend merge or changes.
- Merge execution: when authorized, perform non-interactive merges (squash), respecting protected-branch rules.

### Operating base Rules

- CLI Tools: wen use any command line tools to avoid interactive/paged commands; never expose secrets.
- Command allowlist & MCP servers: defer to sdd-rules/AGENTS.md; do not duplicate here.
- Worktree-first: never develop on main; create a feature branch in a dedicated worktree.
- Branch categories (canonical): feature | fix | perf | chore | docs (kebab-case). The feature/<module>-<id> style is allowed as an alternative but not the canonical example.
- Logging discipline: stderr for logs; stdout reserved for JSON-RPC/JSONL only.
- Evidence: store all local scenario outputs and jq validations under dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/.
- Respect human edits: do not override user modifications unless explicitly requested; reconcile conflicts conservatively.

### SDD compliance (must do for every task)

work in: (specs/)

- Create an SDD record under specs/<NNN>-<slug>/ with:
    - spec.md (WHAT/WHY; requirements and acceptance)
    - plan.md (technical plan; architecture and trade-offs)
    - tasks.md (subtasks, responsibilities, test plan)
- Add the following metadata block at the top of each file (and mirror in the GitHub Issue body):
    - Issue-URI: <link to the GitHub issue>
    - Spec-URI / Plan-URI / Tasks-URI: <self links>
    - Evidence-URIs: old task is in dev-docs/review/_artifacts/{tests|logs|jq|reports}/<task>/... new task is in root path
    (_artifacts/{tests,logs,jq,reports}/<task>/...) linked with (specs/) TASK's artifacts outputs.
    (Subsequent task evidence is stored under the root path)
- PR description must include: links to Spec/Plan/Tasks, evidence files (tests/logs/jq/reports), risks/rollback, and CI pass summary.

### SDD commands (artifact generation)

- `/specify` — generate a new feature specification and branch/worktree; see sdd-rules/commands/specify.md
- `/plan` — create implementation plan and design docs; see sdd-rules/commands/plan.md
- `/tasks` — derive executable tasks from the plan; see sdd-rules/commands/tasks.md
- `/sdd-task` — initialize SDD task from GitHub issue; see .specify/commands/sdd-task.md

> Notes:
> Use these commands to maintain the spec → plan → tasks flow described in (.specify/spec-driven.md) and (.specify/memory/lifecycle.md).

### Standard procedure

1) Context gathering
   - Inspect repository state, read relevant files, and list existing workflows.
2) Plan tasks
   - Draft a concise checklist; create a feature worktree from origin/main.
3) Implement & verify
   - Code changes via patch; run fmt/clippy/test; replay JSONL scenarios; record evidence.
4) Evidence
   - Store (specs/) TASK's artifacts outputs linked to (_artifacts/{tests,logs,jq,reports}/<task>/...; summarize pass/fail and link (specs/) TASK's (specs/) artifacts.
5) PR & merge
   - Open PR with summary and evidence; on approval, squash-merge and clean up worktrees.
   - After merge:
     - MUST re-run the SDD Documentation Dynamic Consistency Check Workflow (.specify/memory/constitution_update_checklist.md) first!
     - Then if needed to add any new sdd-rules or update .specify/memory/constitution.md and resync docs/templates if needed.

## SDD Rules

root path: (`sdd-rules/`)

**SDD-RULES**: When AI engineers update the (specs/) Initialize Tasks & Process Tasks workflow process in accordance with the requirements and in strict compliance with the CONSTITUTION & "CONSTITUTION" - Link outher SDD decs; The SDD artifact:
spec.md / plan.md / task.md needs to be explicitly linked to the specific rules (sdd-rules/rules/) {ssd-rules-xxx}
if it needs to refer to specific rules. plan.md / task.md need to explicitly link to specific rules when (sdd-rules/rules/) {ssd-rules-xxx}

### Branch and worktree (canonical example)

For every formal TASK (e.g., `specs/<NNN>-<slug>/`), create a new worktree and branch off `origin/main`.
(specs/) TASK's worktree branch Use existing GitHub Issues or create new ones, along with corresponding PRs, to track and manage the TASK’s status and progress any Issues and PRs comments fllow GitHub best practices.

- Branch categories: feature | fix | perf | chore | docs
- Create a new worktree and branch from origin/main:
  `git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/<task-dir> origin/main -b <branch>`
- Optional IDE navigation:
  ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>

### Quality gates (must pass)

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features --locked`
- Protocol JSONL scenarios (if present) replay without errors; stdout is valid JSONL.
- Code scanning (GitHub Code Scanning) is enabled. For local custom CodeQL queries, see (dev-docs/engineering/codeql.md) .

### Constitutional gates (must pass)

- **Simplicity (Article VII)**: ≤3 projects; no future-proofing; avoid unnecessary patterns. See .specify/memory/constitution.md
- **Anti-Abstraction (Article VIII)**: Use framework features directly; single model representation. See .specify/memory/constitution.md
- **Integration-First (Article IX)**: Contracts defined; contract tests written before implementation; use real dependencies where practical. See .specify/memory/constitution.md
- **Test-First (Article III)**: Write tests first and confirm failing (RED) before implementation. See .specify/memory/constitution.md

### SDD checks (pre-PR)

- `scripts/ci/run-local-ci.sh` — runs structure, language, markdown, and semantic checks
- Or on macOS, run individually:
    - `scripts/sdd/check_language.sh`
    - `scripts/sdd/lint_docs.sh`
    - `scripts/sdd/run_semantic_checks.sh`

- **Before submitting a PR, run the scripts in (scripts/sdd/) to perform the SDD consistency check and ensure compliance for (specs/) TASK's artifacts.**

### Security & compliance

- Do not log secrets; never print secrets to CI logs; use env vars and GitHub secrets.
- Avoid running untrusted code or scripts without review.

### Communication

- Keep status short and actionable; when uncertain about intent, ask before proceeding.
- Escalate risks with options and trade-offs.

---

## My SDD Integration Understanding

For detailed documentation of my (Claude Code's) complete understanding and operational context within the ACPLazyBridge SDD framework, see:

→ **[.specify/CLAUDE.md](.specify/CLAUDE.md)**

This document contains:

- SDD workflow execution patterns
- Constitutional gate check procedures
- Command integration (`/specify`, `/plan`, `/tasks`)
- Script execution and JSON parsing
- Template processing rules
- Evidence collection standards
- Sub-agent coordination
- Memory and state management

**Key operational insights:**

- Claude enforce Test-First (Article III) by ensuring tests are written and fail before implementation
- Claude validate Simplicity (Article VII) by rejecting >3 projects without justification
- Claude implement Library-First (Article I) by ensuring all features start as libraries
- Claude maintain evidence trails in `_artifacts/` for all tasks
- Claude use JSON mode for all script interactions to enable reliable parsing
- Claude delegate heavy retrieval to specialized sub-agents (document-retriever, code-retriever, code-analyzer)

---

## Claude's Sub‑agents for Code or Document Retrieval and any Codebase Analysis

To keep your working context focused and reduce repetitive retrieval loops, delegate heavy retrieval and repository‑wide analysis to the following use the document-retriever / code-retriever / code-analyzer etc. subagent.
They run non‑interactively, apply safe defaults, and return cited evidence and artifacts.

Available sub‑agents (installed per‑user):

- **document-retriever** — ~/.claude/agents/document-retriever.md
    - Purpose: High‑signal document retrieval over docs directories using SemTools (parse/search).
    - Best for: `sdd-rules/`, `dev-docs/`, and other documentation directories (including paths outside the codebase when explicitly scoped).
- **code-retriever** — ~/.claude/agents/code-retriever.md
    - Purpose: Precise code retrieval using AST‑aware patterns (ast-grep), with ripgrep fallback when needed.
    - Best for: “find code patterns” tasks, e.g., unwrap() in Rust, console.log in TS, etc.
- **code-analyzer** — ~/.claude/agents/code-analyzer.md
    - Purpose: Repository analysis via ast-grep scan using sgconfig.yml and curated rules (Rust/JS/Python/Go).
    - Best for: rule‑based audits (e.g., rust-no-unwrap, js-no-console-log) with JSON/SARIF output and summaries.

Headless discipline and SDD alignment

- Non‑interactive: they never prompt; they proceed with safe, documented defaults.
- Scope first, then search: always provide top‑level target directories; agents will auto‑narrow if candidates exceed caps.
- Ignores: by default, do not scan .git, node_modules, target, dist, build, .venv, .cache, coverage, tmp, logs, and archives.
- Evidence:
    - Human run logs: $HOME/.claude/runs/<retrieval|analysis>-<timestamp>.md
    - Repo artifacts (when running inside a repo): _artifacts/reports/<task>/* (JSON/JSONL/SARIF + summaries)
- Secrets: never log secrets; use env variables; do not print key values to logs.

When to delegate (decision guide)

- I need documents from `sdd-rules/`, `dev-docs/`, or other doc trees (possibly outside this repo)
  → Delegate to document-retriever with: scope paths + retrieval keywords; it will parse non‑text (PDF/DOCX/PPTX/XLSX) first, then semantically search and iterate until it finds high‑signal material.
- I need code examples or exact structural usages across the codebase
  → Delegate to code-retriever with: language + AST pattern(s) + scope paths; it returns precise, cited matches (file:line), optimizing breadth/precision automatically.
- I need a repo‑wide rule audit or security/quality sweep
  → Delegate to code-analyzer with: rule_filter (regex) + optional globs + desired format (json/jsonl/sarif/github); it uses sgconfig.yml and writes machine‑readable evidence plus summaries.

Delegation contract (what you provide to the sub‑agent)

- scope: one or more top‑level directories to search (e.g., `sdd-rules/rules/`, `dev-docs/`, `crates/`, docs outside the repo when explicitly allowed)
- query:
    - For document-retriever: keywords (prefer comma‑separated multi‑aspect queries)
    - For code-retriever: AST pattern(s) and language (e.g., -l rust, -l ts)
    - For code-analyzer: rule_filter (e.g., '^rust-no-unwrap$') and format (json|jsonl|sarif|github)
- outputs: optional task slug to group artifacts under _artifacts/reports/<task>/
- anchors (optional): strong keywords to pre‑filter very large corpora
- caps (optional): HEADLESS_MAX_FILES / HEADLESS_DISPLAY_CAP overrides if you need different limits

Examples (prompts you can give to the sub‑agents)

- document-retriever (docs search across SDD rules and dev docs)
  “Search `sdd-rules/rules/` and `dev-docs/` for ‘worktree, branch policy, SDD workflow’, prioritize `sdd-rules/rules/git/worktree`, return top citations with 4 lines of context. Save full results under _artifacts/reports/worktree-audit/.”
- code-retriever (AST code search)
  “In crates/**.rs, find all $EXPR.unwrap() (language: rust), exclude tests/benches. Return JSON matches with file and line, and write the full list to _artifacts/reports/rust-unwrap/.”
- code-analyzer (rule‑based audit via sgconfig.yml)
  “Run ast-grep scan with sgconfig.yml using rule_filter '^rust-no-unwrap$', format jsonl; produce a per‑file count summary and store artifacts under_artifacts/reports/rust-unwrap-audit/.”

Operating notes (outside-of-repo docs)

- For documentation searches beyond this repository, explicitly supply the allowed directories (do not scan $HOME by default).
- document-retriever will parse non‑text first and then semantically search; it records auto‑narrowing and thresholds in the run log.

Optional: Keep specs in sync via CLAUDE.md imports

- You may import the per‑user sub‑agent specs for quick reference:
  `@~/.claude/agents/document-retriever.md`
  `@~/.claude/agents/code-retriever.md`
  `@~/.claude/agents/code-analyzer.md`

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "claude-memory"
    path: "./CLAUDE.md"
    version: "1.0.2"
    last_updated: "2025-09-20T07:27:35Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - "sdd-rules/rules/README.md"
        - ".specify/templates/spec-template.md"
        - ".specify/templates/plan-template.md"
        - ".specify/templates/tasks-template.md"
```
