# CLAUDE.md

Updated for the SDD global consistency refresh delivered in PR #47 (merged 2025-09-25); treat this version as Claude Code's authoritative memory until the next constitution checkpoint.

```text
This file provides guidance to CLAUDE (Claude Code Agents) when working with code in this repository. as AI Engineers ("claude" agent) working within our Project - Repository "ACPLazyBridge" Specification‑Driven Development [SDD](.specify/spec-driven.md) team. It follows the SDD principles in (.specify/memory/constitution.md) . ****Claude AI Engineer**** collaborates with a team that includes human developers and other AI engineers, team work with AI coding agents such as Claude Code, Gemini, and Codex. Together, the team plans and writes code that strictly follows the specification. It complements CONTRIBUTING.
```

- **Project Name**: `ACPLazyBridge` (Rust workspace)
- **Project Repository URL**: <https://github.com/lwyBZss8924d/ACPLazyBridge>
- **ACP (Agent Client Protocol) Protocol**: <https://agentclientprotocol.com/protocol>
- **ACP Protocol Schema**: <https://agentclientprotocol.com/protocol/schema>
- **ACP official Rust library**: `cargo add agent-client-protocol`
- **ACP official TypeScript library**: `npm install @zed-industries/agent-client-protocol`
- **ACP Agents adapter best practice (@zed-industries/claude-code-acp)**: [Claude Code SDK from ACP-compatible clients for Zed IDE external-agents Custom Agents as ACP client adapter](https://github.com/zed-industries/claude-code-acp)
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)
- **(ACP) Protocol Rust Library Lastest Version**: `agent-client-protocol = "0.4.4"` (2025-09-30) _always check latest version from (https://github.com/zed-industries/agent-client-protocol/releases)`_

```text
Team's AI Engineer member: ("claude")'s role and operating rules for **ACPLazyBridge**. It is a role-specific guide. For the authoritative workflow and lifecycle, always refer to the documents listed below. and always refer to the SDD Constitution. wen update any SDD document and sdd-rules document, MUST follow the SDD Constitution Update Checklist. All SDD document and sdd-rules document and normative artifacts (specify, plan, tasks, issues, PRDs, commits, etc.) MUST be English‑only.
```

## Repository Overview

ACPLazyBridge is an ACP (Agent Client Protocol) bridge that connects AI agents and agent-tools plugins with IDEs, editors, and development tools.
It provides native adapters for various AI systems while maintaining protocol consistency and developer workflow integration through Specification-Driven Development (SDD).

```bash
ACPLazyBridge/crates
❯ tree
.
├── CLAUDE.md
├── acp-lazy-core
│   ├── CLAUDE.md
│   ├── Cargo.toml
│   ├── README.md
│   ├── src
│   │   ├── lib.rs
│   │   ├── permissions.rs
│   │   ├── protocol.rs
│   │   ├── runtime
│   │   │   ├── adapter.rs
│   │   │   ├── mod.rs
│   │   │   ├── server.rs
│   │   │   └── session.rs
│   │   └── transport.rs
│   └── tests
│       └── runtime_test.rs
└── codex-cli-acp
    ├── CLAUDE.md
    ├── Cargo.toml
    ├── src
    │   ├── bin
    │   │   ├── acplb_notify_forwarder.rs
    │   │   └── playback.rs
    │   ├── codex_agent.rs
    │   ├── codex_proto.rs
    │   ├── lib.rs
    │   ├── main.rs
    │   ├── notify_source.rs
    │   ├── tool_calls.rs
    │   └── validation.rs
    └── tests
        ├── acp_integration_test.rs
        ├── jsonl_regression_test.rs
        ├── notify_test.rs
        ├── playback.rs
        ├── session_update_format.rs
        └── tool_calls_test.rs
```

### Repository Guidelines

Synchronized with the 2025-09-25 governance sweep. Claude must:

- run `scripts/sdd/validate-metadata.sh` and `scripts/sdd/check-sdd-consistency.sh` whenever editing agent docs;
- reconcile updates with `sdd-rules/CLAUDE.md`, `sdd-rules/AGENTS.md`, and `.specify/CLAUDE.md`;
- reference constitution Articles III, VII, and IX when documenting workflow obligations.

#### Project Structure & Module Organization

- Rust workspace with crates in `crates/`:
    - `crates/acp-lazy-core`: shared ACP bridge utilities.
    - `crates/codex-cli-acp`: Codex ACP adapter binaries (`codex-cli-acp`, `playback`, `acplb-notify-forwarder`).
- Tests: unit in each crate under `src/` modules; integration tests in `crates/*/tests/`.
- CI and helpers: `scripts/ci/` (pre-PR checks), `scripts/ast-grep/` (static analysis).
- Docs/specs: `dev-docs/`, `specs/`; rule sets in `sgconfig.yml`, `sdd-rules/`.

#### Build, Test, and Development Commands

- Format: `cargo fmt --all -- --check` — verify formatting.
- Lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no warnings allowed.
- Test: `cargo test --workspace --all-features --locked` — run all tests.
- Adapter (debug): `cargo run -p codex-cli-acp` — run the Codex ACP adapter.
- Adapter (release): `cargo build --release -p codex-cli-acp`.
- Local CI suite: `scripts/ci/run-local-ci.sh` — runs structure, language, markdown, and semantic checks.
- Static analysis: `ast-grep scan -c sgconfig.yml .`.

#### Coding Style & Naming Conventions

- Rust style via rustfmt (4-space indent); keep code clippy-clean.
- Avoid `unwrap/expect` in non-test code; prefer `anyhow`/`thiserror` and `Result`.
- Logging with `tracing` goes to stderr; stdout is reserved for JSON/JSONL protocol output.
- Names: crates and modules `snake-kebab`/`snake_case`; types `PascalCase`; functions/vars `snake_case`; constants `SCREAMING_SNAKE_CASE`.

#### Testing Guidelines

- Place integration tests in `crates/<name>/tests/`.
- For AST-grep in tests, add before uses of `unwrap()`:

  ```rust
  // ast-grep-ignore: rust-no-unwrap
  ```

- JSONL protocol scenarios (if used) live under `_artifacts/tests/protocol-baseline/` (legacy mirror: `_artifacts/tests/legacy/`) and can be piped into `codex-cli-acp`.

### Architecture (high level)

Post-refresh architecture outline—keeps Claude aligned with codex-cli-acp runtime responsibilities and the metadata-aware orchestration flows described in `dev-docs/architecture/acplb-architecture.md`.

- Workspace overview
    - crates/acp-lazy-core (library)
        - protocol.rs: JSON‑RPC 2.0 types and classification (requests, notifications, responses; Error codes −32700…−32603).
        - transport.rs: ProcessTransport (spawn child process with piped stdio, stderr severity logging), JSONL I/O helpers (read_lines, read_values, write_line), async reader tasks, MessageQueue.
        - permissions.rs: Maps ACP permission modes to Codex CLI overrides (-c approval_policy=…, -c sandbox_mode=…, network access toggles) with env overrides (ACPLB_*).
        - logging: tracing subscriber directed to stderr to keep stdout JSON‑only.
    - crates/codex-cli-acp (binary "codex-cli-acp" + utilities)
        - main.rs: Implements ACP server methods:
            - initialize: returns protocolVersion: 1 (integer) and agentCapabilities.promptCapabilities.
            - session/new: validates cwd is absolute, mcpServers is array; stores permissionMode; creates sessionId.
            - session/prompt: spawns Codex CLI in proto mode with permission overrides; optionally injects a notify forwarder; streams Codex stdout to ACP session/update events; ends on notify event or idle timeout; returns stopReason.
            - session/cancel: terminates the Codex child process.
        - codex_proto.rs: Maps Codex events (AgentMessage, AgentMessageDelta, ToolCall, ToolCalls, TaskComplete, Error) to ACP session/update payloads:
            - AgentMessage/Delta → AgentMessageChunk with de‑duplication.
            - ToolCall/ToolCallUpdate with status transitions (pending → in_progress → completed/failed), kind mapping, output previews, and error categorization.
        - tool_calls.rs: Tool categorization (read/edit/delete/move/search/execute/think/fetch/other), shell parameter extraction (command, workdir, timeout, sudo), UTF‑8 safe truncation previews.
        - notify_source.rs: File or FIFO notification sources; watches for {"type":"agent-turn-complete", …} to cut turns immediately; file mode uses polling; FIFO mode uses a blocking reader.
        - validation.rs: RPC error classification (InvalidParams, MethodNotFound, Internal) and helpers (absolute path validation, 1‑based line numbers).
        - bins:
            - acplb-notify-forwarder: small helper that writes Codex notify JSON to ACPLB_NOTIFY_PATH (file/FIFO) for immediate turn completion.
            - playback: test utility that builds and runs the server, forwards JSONL requests, and waits for responses.

- Data flow (session/prompt)
  1) Client calls session/prompt → server maps ACP permission mode to Codex overrides.
  2) Server spawns Codex CLI (proto) with args like: -c approval_policy=never, -c sandbox_mode=…; may inject acplb-notify-forwarder.
  3) Server writes a Codex request {"method":"prompt","params":{"messages":[…]}} to the child stdin.
  4) Server reads child stdout lines → codex_proto maps each to ACP session/update; writes to stdout as JSONL.
  5) Turn ends on notify event "agent-turn-complete" or after idle timeout (defaults below). Response carries {"stopReason":"end_turn"}.

## Common Development Commands

Command set reflects the PR #47 tooling refresh; treat metadata validation scripts as mandatory pre-PR gates alongside fmt/clippy/test.

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
cat _artifacts/tests/protocol-baseline/handshake.jsonl | cargo run -p codex-cli-acp

# Test with Codex proto command
codex proto -c approval_policy="never" < _artifacts/tests/protocol-baseline/basic_session.jsonl
```

### Metadata & Consistency Tooling

- Validate YAML frontmatter and document headers before writing specs or memories:

```bash
scripts/sdd/validate-metadata.sh
```

- Run the global consistency audit (must pass prior to PR handoff):

```bash
scripts/sdd/check-sdd-consistency.sh
```

- Query metadata when triaging cross-doc drift (optional example):

```bash
scripts/sdd/query-metadata.sh --owner claude --format table
```

## You have Augmented CLI Development tools chain and compose for codebase Code Analysis

Tip: When you need to do Code Search and Retrieval and any Codebase Analysis Operations, Can use subagent: "code-retriever" or "code-analyzer"

Advanced code analysis techniques: @sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md

**BASE Command line Tools**:

- Find Files: `fd`
- Find Text: `rg` (ripgrep) `search` and `parse`
- Find Code Structure: `ast-grep`
- Select among matches: pipe to `fzf`
- JSON: `jq`
- YAML/XML: `yq`

### Augmented CLI Development Tooling

> [sdd-rules-tools-cli-list](sdd-rules/rules/tools-cli/sdd-rules-tools-cli-list.md)

### `ast-grep` (AST-based Code Analysis)

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

### "SemTools" CLI Tools use to chain and compose for document retrievl

> (sdd-rules/rules/tools-cli/sdd-rules-tools-cli-document-search-and-parsing.md)

```bash
parse --help
```

```bash
search --help
```

```bash
workspace --help
```

### SDD Validation

Claude must integrate the metadata and consistency tooling into every validation pass; escalate any drift discovered by these scripts before shipping artifacts.

```bash
# Run full local CI suite (includes all SDD checks)
scripts/ci/run-local-ci.sh

# Run individual checks
scripts/sdd/validate-sdd-docs.sh
scripts/sdd/run_semantic_checks.sh
scripts/sdd/check_language.sh
scripts/sdd/validate-metadata.sh
scripts/sdd/check-sdd-consistency.sh
```

## Important Conventions

Conventions below incorporate the refreshed governance rules—ensure Claude's guidance stays in lockstep with `AGENTS.md` and the SDD constitution after each pull.

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
  tee _artifacts/legacy/<task>/logs/test_$(date +%Y%m%d_%H%M%S).log
```

## Testing Approach

Reaffirmed after the refresh: keep JSONL scenarios current, store evidence under `_artifacts/`, and rerun metadata + consistency checks before publishing results.

The project uses JSONL files for protocol testing. Key test scenarios are in `_artifacts/tests/protocol-baseline/` (legacy mirror retained under `_artifacts/tests/legacy/`):

- `handshake.jsonl` - Basic initialization
- `basic_session.jsonl` - Session creation and prompting
- `tool_calls.jsonl` - Tool execution flows
- `streaming_benchmark.jsonl` - Performance testing

## Notification System

Notification guidance now explicitly covers acplb-notify-forwarder behavior and environment defaults introduced during the refresh.

The adapter supports external turn completion signals via environment variables:

- `ACPLB_NOTIFY_PATH`: Path to notification sink
- `ACPLB_NOTIFY_KIND`: `file` or `fifo`
- `ACPLB_NOTIFY_INJECT`: `auto`, `never`, or `force`

This allows immediate turn completion instead of waiting for idle timeout.

## Current Focus (Milestone 0.1.0)

Status snapshot pulled from the milestone tracker immediately after PR #47 (2025-09-25); update when milestones shift or new scope opens.

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

```bash
ACPLazyBridge/.specify
❯ tree
.
├── CLAUDE.md
├── README.md
├── commands
│   ├── plan.md
│   ├── sdd-task.md
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

```bash
ACPLazyBridge/sdd-rules/rules
❯ tree
.
├── README.md
├── changelog
│   ├── examples.md
│   ├── keep-a-changelog-index.html.haml
│   ├── sdd-rules-changelog.md
│   └── semantic-versioning-2.0.0.md
├── ci
│   ├── claude-code-github-actions.md
│   └── sdd-rules-ci.md
├── code-analysis
│   ├── ast-grep
│   │   └── rust
│   │       ├── no-dbg.yml
│   │       ├── no-unwrap.yml
│   │       ├── rust-mutex-lock.yml
│   │       └── todo-comment.yml
│   └── sdd-rules-code-analysis.md
├── documentation-style
│   ├── google-developer-documentation-style-guide.md
│   ├── google-markdown-style-guide.md
│   ├── sdd-rules-documentation-markdownlint.md
│   └── sdd-rules-documentation-style.md
├── git
│   ├── comments
│   │   └── sdd-rules-comments.md
│   ├── commit
│   │   └── sdd-rules-commit-message.md
│   ├── issues
│   │   └── sdd-rules-issues.md
│   ├── pr
│   │   └── sdd-rules-pr.md
│   └── worktree
│       └── sdd-rules-worktrees.md
├── research
│   └── sdd-rules-research.md
├── sdd-validation
│   ├── needs-clarification.yml
│   ├── placeholders.yml
│   ├── task-numbering.yml
│   └── todo-markers.yml
├── tests
│   └── sdd-rules-tests.md
├── tools-cli
│   ├── ast-grep.llms.txt
│   ├── sdd-rules-tools-cli-astgrep.md
│   ├── sdd-rules-tools-cli-document-search-and-parsing.md
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
- Evidence: store all local scenario outputs and jq validations under _artifacts/legacy/{tests,logs,jq,reports}/<task>/.
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
    - Evidence-URIs: old task is in _artifacts/legacy/{tests|logs|jq|reports}/<task>/... new task is in root path
    (_artifacts/{tests,logs,jq,reports}/<task>/...) linked with (specs/) TASK's artifacts outputs.
    (Subsequent task evidence is stored under the root path)
- PR description must include: links to Spec/Plan/Tasks, evidence files (tests/logs/jq/reports), risks/rollback, and CI pass summary.

### SDD commands and SDD-TASKs Workflow

1. `/sdd-task` — initialize SDD task from GitHub issue; see .specify/commands/sdd-task.md
2. `/specify` — generate a new feature specification and branch/worktree; see sdd-rules/commands/specify.md
3. `/plan` — create implementation plan and design docs; see sdd-rules/commands/plan.md
4. `/tasks` — derive executable tasks from the plan; see sdd-rules/commands/tasks.md

#### Standard procedure

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

📌 When AI-Engineer SDD-TASKs Cooking Workflow can follow the BASELINE TEMPLATES work in (specs/): [AI-Engineer-SDD-Workflow-Baseline-templates](.specify/memory/AI-Engineer-SDD-Workflow-Baseline-templates.txt)

<SDD-TASK-INITIALIZATION-WORKFLOW>

```text
Any AI Engineers that specializes in Spec-Driven Development (SDD) task initialization. You will be given a GitHub issue and need to create a complete SDD task workflow including specifications, plans, and executable tasks.

Here is the GitHub issue you need to process:

<github_issue>
{{GITHUB_ISSUE}}
</github_issue>

## SDD Workflow Overview

You will follow this complete workflow:
ISSUES(#XXX) → SDD-TASKs Initialization → Specification Documents → Review → Development → Tests → Final Review → PR

The core SDD commands you need to simulate are:
1. `/sdd-task` — initialize SDD task from GitHub issue
2. `/specify` — generate feature specification and branch/worktree structure
3. `/plan` — create implementation plan and design docs
4. `/tasks` — derive executable tasks from the plan

## Step-by-Step Instructions

### Phase 1: Issue Analysis and Setup
First, analyze the GitHub issue thoroughly. Extract:
- Issue number and title
- Problem description and requirements
- Acceptance criteria
- Any technical constraints or dependencies

### Phase 2: Worktree Structure Creation
Create the following directory structure for the SDD task:

specs/<NNN>-<slug>/
├── spec.md
├── plan.md
├── tasks.md
└── [additional specification documents as needed]


Where XXX is a 3-digit number and <slug> is derived from the issue title.

### Phase 3: SDD TASKs Specification Documents Generation

**spec.md Requirements:**
- Must include UTC timestamp in YAML frontmatter: `date: YYYY-MM-DDTHH:MM:SSZ`
- Follow the spec-template structure
- Include problem statement, requirements, acceptance criteria
- Reference the original GitHub issue

**plan.md Requirements:**
- Must include UTC timestamp in YAML frontmatter
- Follow the plan-template structure
- Break down implementation approach
- Identify technical dependencies and risks
- Include design decisions and architecture considerations

**tasks.md Requirements:**
- Must include UTC timestamp in YAML frontmatter
- Follow the tasks-template structure
- Derive specific, executable tasks from the plan
- Include task priorities and dependencies
- Specify testing requirements

### Phase 4: Consistency and Alignment
Ensure all documents:
- Reference the SDD rules and constitution
- Maintain consistency with existing project structure
- Follow the lifecycle management guidelines
- Include proper cross-references between documents

## Output Requirements

Your response should contain:

1. **Worktree Information:**
   - Suggested worktree path: `/acplb-worktrees/XXX-<slug>`
   - Issue reference and URI

2. **Complete File Contents:**
   - Full content for `spec.md`
   - Full content for `plan.md`
   - Full content for `tasks.md`
   - Any additional specification documents needed

3. **Metadata:**
   - Current UTC timestamp for all documents
   - Proper YAML frontmatter for each file
   - Cross-references between documents

## Critical Requirements

⚠️ **MUST include current UTC timestamp** in format `YYYY-MM-DDTHH:MM:SSZ` in all document headers
⚠️ **MUST follow the template structures** referenced in the SDD commands
⚠️ **MUST create proper cross-references** between spec → plan → tasks
⚠️ **MUST align with SDD rules** and constitution guidelines

## Final Output Format

Structure your response with clear sections for each file, using appropriate headers and formatting. Include the complete file contents that would be created in the worktree, ready for immediate use in the SDD workflow.

Your final response should contain the complete, ready-to-use SDD task initialization package that can be directly implemented in the project worktree structure.

## Best Practice Example

(/ACPLazyBridge) | worktree: (acplb-worktrees/038-adopt-acp-runtime)

acplb-worktrees/038-adopt-acp-runtime/specs/038-adopt-acp-runtime
❯ tree
.
├── contracts
│   └── runtime_api.md
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md

```

</SDD-TASK-INITIALIZATION-WORKFLOW>

> Notes:
> Use these commands to maintain the spec → plan → tasks flow described in (.specify/spec-driven.md) and (.specify/memory/lifecycle.md).

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
- Code scanning (GitHub Code Scanning) is enabled.

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

## (dev-docs/) and References

<dev-docs>

```bash
.
├── CLAUDE.md
├── README.md
├── _issues_drafts
│   ├── TEMPLATE.md
│   ├── closed
│   │   ├── ...
│   │   └── #44-runtime-adoption-core-loop.md
│   └── open
│       ├── #45-streaming-alignment-session-notifications.md
│       └── #46-protocol-cleanup-official-models.md
├── _projects
│   └── migration-blueprint-project-management-plan.md
├── _requirements
│   ├── Roadmap.md
│   ├── acp-lazybridge-project-plan.md
│   ├── acp-lazybridge-requirements.md
│   ├── m1-issue-list.md
│   └── m1-technical-implementation-plan.md
├── architecture
│   └── acplb-architecture.md
├── changelogs
│   ├── 038-adopt-acp-runtime.md
│   ├── README.md
│   ├── codex-tools-1-code-changes-2025-09-04.md
│   └── codex-tools-1-review-2025-09-04.md
├── core_servers
│   └── acplb-core-runtime.md
└── references
    ├── acp.md
    ├── acp_adapters
    │   └── claude_code_acp.md
    ├── cli_agents
    │   ├── ClaudeCode
    │   │   ├── ClaudeCode-Config.md
    │   │   ├── cli-reference.md
    │   │   ├── hooks.md
    │   │   ├── sdk-headless.md
    │   │   ├── sdk-overview.md
    │   │   ├── sdk-python.md
    │   │   ├── sdk-rust(Unofficial).md
    │   │   ├── sdk-typescript.md
    │   │   ├── slash-commands.md
    │   │   └── troubleshooting.md
    │   ├── CodexCLI-Config.md
    │   ├── claude_code.md
    │   ├── codex.md
    │   └── gemini.md
    └── zed_ide.md
```

</dev-docs>

---

## My (Claude Code's) SDD Integration Understanding

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
- **sdd-doc-validator** — ~/.claude/agents/sdd-doc-validator.md
    - Purpose: Comprehensive SDD documentation validation, markdown linting, and automated fixing.
    - Best for: markdown style validation, SDD compliance checking, auto-fixing violations, managing long-term documentation quality improvements.

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
- I need markdown validation, SDD compliance checking, or documentation quality fixes
  → Delegate to sdd-doc-validator for: comprehensive markdown validation, auto-fixing violations, tracking long-term documentation improvements, and SDD compliance verification.

Delegation contract (what you provide to the sub‑agent)

- scope: one or more top‑level directories to search (e.g., `sdd-rules/rules/`, `dev-docs/`, `crates/`, docs outside the repo when explicitly allowed)
- query:
    - For document-retriever: keywords (prefer comma‑separated multi‑aspect queries)
    - For code-retriever: AST pattern(s) and language (e.g., -l rust, -l ts)
    - For code-analyzer: rule_filter (e.g., '^rust-no-unwrap$') and format (json|jsonl|sarif|github)
    - For sdd-doc-validator: validation type (markdown/sdd/both), fix mode (auto/manual/both), scope (paths)
- outputs: optional task slug to group artifacts under _artifacts/reports/<task>/
- anchors (optional): strong keywords to pre‑filter very large corpora
- caps (optional): HEADLESS_MAX_FILES / HEADLESS_DISPLAY_CAP overrides if you need different limits

Examples (prompts you can give to the sub‑agents)

- document-retriever (docs search across SDD rules and dev docs)
  “Search `sdd-rules/rules/` and `dev-docs/` for ‘worktree, branch policy, SDD workflow’, prioritize `sdd-rules/rules/git/worktree`, return top citations with 4 lines of context. Save full results under _artifacts/reports/worktree-audit/.”
- code-retriever (AST code search)
  “In crates/**.rs, find all $EXPR.unwrap() (language: rust), exclude tests/benches. Return JSON matches with file and line, and write the full list to _artifacts/reports/rust-unwrap/.”
- code-analyzer (rule‑based audit via sgconfig.yml)
  "Run ast-grep scan with sgconfig.yml using rule_filter '^rust-no-unwrap$', format jsonl; produce a per‑file count summary and store artifacts under_artifacts/reports/rust-unwrap-audit/."
- sdd-doc-validator (markdown validation and fixing)
  "Validate all markdown files for SDD compliance and style violations. Auto-fix what can be fixed, create a TodoWrite list for manual fixes, and store progress in _artifacts/reports/markdown-validation/."

Operating notes (outside-of-repo docs)

- For documentation searches beyond this repository, explicitly supply the allowed directories (do not scan $HOME by default).
- document-retriever will parse non‑text first and then semantically search; it records auto‑narrowing and thresholds in the run log.

Optional: Keep specs in sync via CLAUDE.md imports

- You may import the per‑user sub‑agent specs for quick reference:
  `~/.claude/agents/document-retriever.md`
  `~/.claude/agents/code-retriever.md`
  `~/.claude/agents/code-analyzer.md`
  `~/.claude/agents/sdd-doc-validator.md`

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-23T04:56:00Z"
document:
    type: "claude-memory"
    path: "./CLAUDE.md"
    version: "1.0.5"
    last_updated: "2025-09-27T10:09:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - ".specify/README.md"
        - "sdd-rules/rules/README.md"
        - ".claude/commands/sdd-task.md"
        - ".specify/commands/specify.md"
        - ".specify/templates/spec-template.md"
        - ".specify/commands/plan.md"
        - ".specify/templates/plan-template.md"
        - ".specify/commands/tasks.md"
        - ".specify/templates/tasks-template.md"
        - "(dev-docs/references/)"
        - "(dev-docs/_requirements/)"
        - "(dev-docs/_issues_drafts/)"
        - "(dev-docs/_projects/)"
        - "<https://github.com/zed-industries/agent-client-protocol/tree/main/docs>"
        - "<https://agentclientprotocol.com/protocol>"
        - "<https://agentclientprotocol.com/protocol/schema>"
        - "sdd-rules/CLAUDE.md"
        - ".claude/CLAUDE.md"
```
