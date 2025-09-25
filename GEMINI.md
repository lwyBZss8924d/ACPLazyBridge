# GEMINI.md ("gemini")

Updated for the SDD global consistency refresh delivered in PR #47 (merged 2025-09-25); treat this version as the authoritative "gemini" AI Engineer memory until the next CONSTITUTION cycle.

```text
This file provides guidance to "gemini" (Gemini CLI) when working with code in this repository. as AI Engineers ("gemini" agent) working within our Project - Repository "ACPLazyBridge" Specification‑Driven Development [SDD](.specify/spec-driven.md) team. It follows the SDD principles in (.specify/memory/constitution.md) . **Gemini AI Engineer** collaborates with a team that includes human developers and other AI engineers, team work with AI coding agents such as Claude Code, Gemini, and Warp. Together, the team plans and writes code that strictly follows the specification. It complements CONSTITUTION.
```

- **Project Name**: `ACPLazyBridge` (Rust workspace)
- **Project Repository URL**: <https://github.com/lwyBZss8924d/ACPLazyBridge>
- **ACP (Agent Client Protocol) Protocol**: <https://agentclientprotocol.com/protocol>
- **ACP Protocol Schema**: <https://agentclientprotocol.com/protocol/schema>
- **ACP official Rust library**: `cargo add agent-client-protocol`
- **ACP official TypeScript library**: `npm install @zed-industries/agent-client-protocol`
- **ACP Agents adapter best practice (@zed-industries/claude-code-acp)**: [Claude Code SDK from ACP-compatible clients for Zed IDE external-agents Custom Agents as ACP client adapter](https://github.com/zed-industries/claude-code-acp)
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)
- **(ACP) Protocol Lastest Version**: `v0.4.2` (2025-09-22)

```text
ACPLazyBridge is an ACP (Agent Client Protocol) bridge that connects AI agents and agent-tools plugins with IDEs, editors, and development tools. It provides native adapters for various AI systems while maintaining protocol consistency and developer workflow integration through Specification-Driven Development (SDD).
```

```text
Team's AI Engineer member: ("gemini")'s role and operating rules for **ACPLazyBridge**. It is a role-specific guide. For the authoritative workflow and lifecycle, always refer to the documents listed below. and always refer to the SDD Constitution. wen update any SDD document and sdd-rules document, MUST follow the SDD Constitution Update Checklist. All SDD document and sdd-rules document and normative artifacts (specify, plan, tasks, issues, PRDs, commits, etc.) MUST be English‑only.
```

**Authority and scope**

- Normative authority:
    - (.specify/) all SDD documents
    - (.specify/memory/constitution.md) with constitution's lifecycle (.specify/memory/lifecycle.md) (ACPLazyBridge SDD Constitution; authoritative governance)
    - (sdd-rules/) all sdd-rules documents. **AI Engineer SDD All Rules Index**: (sdd-rules/rules/README.md)
- Team/agent rules:
    - `sdd-rules/AGENTS.md` (team member AI Engineers BASE LINE roles for all AI Engineers, command allowlist, dynamic consistency workflow)
    - (../`CLAUDE.md`), (../`WARP.md`), (../`AGENTS.md`) e.g. Team member AI Engineers's root memory.
- (specs/) all SDD TASKs work specs files
- (dev-docs/) all dev-docs files (`_issues_drafts`, `_projects`, `_requirements`, `architecture`, `core_servers`, `references`, etc.)
- (crates/) all project crates codebase project directory
- (scripts/) all scripts (AI-Engineer's own for Dev scripts Tools and custom CLI-Tools, CI scripts, SDD scripts, and E2E-TESTS / Dev project scripts CLI etc., which need to be unified in the future.)
- (.worktrees) all main worktrees's branches symbolic linked for IDE navigation
- (`_artifacts`) all SDD TASKs artifacts outputs
- (.github) all GitHub Actions CI workflows

This file applies to all contributors (human and AI). ouher Agent-specific files `WARP.md` / `CLAUDE.md` / `GEMINI.md` etc.) must align with this file.

---

## Specification-Driven Development (SDD)

**What is SDD?**

Spec-Driven Development flips the script on traditional software development. For decades, code has been king — specifications were just scaffolding we built and discarded once the "real work" of coding began.
Spec-Driven Development changes this: specifications become executable, directly generating working implementations rather than just guiding them.

[spec-driven.md](.specify/spec-driven.md)

**SDD-CONSTITUTION**

- [SDD Constitution](.specify/memory/constitution.md)
- [SDD Lifecycle](.specify/memory/lifecycle.md)
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

## AI Engineer MUST Flow Project's SDD - CONSTITUTION

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
- Evidence: store all local scenario outputs and jq validations under `_artifacts/{tests,logs,jq,reports}/<task>/` (legacy archives remain in `_artifacts/{tests,logs,jq,reports}/legacy/`).
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
    - Evidence-URIs: `_artifacts/{tests|logs|jq|reports}/<task>/...` (legacy audits may reference `_artifacts/{tests|logs|jq|reports}/legacy/`)
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

## You have Augmented CLI Development tools chain and compose for codebase Code Analysis

Tip: When you need to do Code Search and Retrieval and any Codebase Analysis Operations, Can use subagent: "code-retriever" or "code-analyzer"
Tip: When you need to validate documentation, fix markdown violations, or ensure SDD compliance, Can use subagent: "sdd-doc-validator"

Advanced code analysis techniques: @sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md

**BASE Command line Tools**:

- Find Files: `fd`
- Find Text: `rg` (ripgrep) `search` and `parse`
- Find Code Structure: `ast-grep`
- Select among matches: pipe to `fzf`
- JSON: `jq`
- YAML/XML: `yq`

### Augmented CLI Development Tooling

> (sdd-rules/rules/tools-cli/sdd-rules-tools-cli-list.md)

### `ast-grep` (AST-based Code Analysis)

> (sdd-rules/rules/tools-cli/sdd-rules-tools-cli-astgrep.md)

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

#### Parse CLI Help

```bash
parse --help
```

#### Search CLI Help

```bash
$ search --help
A CLI tool for fast semantic keyword search

Usage: search [OPTIONS] <QUERY> [FILES]...

Arguments:
  <QUERY>     Query to search for (positional argument)
  [FILES]...  Files or directories to search

Options:
  -n, --n-lines <N_LINES>            How many lines before/after to return as context [default: 3]
      --top-k <TOP_K>                The top-k files or texts to return (ignored if max_distance is set) [default: 3]
  -m, --max-distance <MAX_DISTANCE>  Return all results with distance below this threshold (0.0+)
  -i, --ignore-case                  Perform case-insensitive search (default is false)
  -h, --help                         Print help
  -V, --version                      Print version
```

#### Workspace CLI Help

```bash
$ workspace --help
Manage semtools workspaces

Usage: workspace <COMMAND>

Commands:
  use     Use or create a workspace (prints export command to run)
  status  Show active workspace and basic stats
  prune   Remove stale or missing files from store
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Repository Overview (Milestone 0.1.0)

Synchronized with the metadata + global consistency refresh completed in PR #47 (merged 2025-09-25). Use the snapshot below to orient AI-Engineer before running live discovery commands (`fd`, `rg`), and rerun `scripts/sdd/check-sdd-consistency.sh` after every pull.

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

Aligned with the 2025-09-25 governance sweep. AI-Engineer must:

- run `scripts/sdd/validate-metadata.sh` and `scripts/sdd/check-sdd-consistency.sh` whenever updating agent docs;
- cross-check changes against `sdd-rules/AGENTS.md` and `.specify/CLAUDE.md` for parity;
- cite constitution articles (III, VII, IX) when documenting workflow rules.

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

### Implementation status

Status snapshot sourced from the milestone tracker after PR #47 (2025-09-25); revise whenever phases change or new milestones are opened.

- Completed (M0): Rust workspace bootstrapped; references vendored
- In progress (M1): Codex native adapter (stdio loop, streaming, tool calls, permission mapping, smoke testing)
- Planned: Proxy adapter, plugin system v0, native adapters, HTTP/SSE bridge

### Architecture (high level)

Post-refresh architecture outline—keeps Codex aligned with codex-cli-acp runtime responsibilities and the metadata-aware workflows documented in `dev-docs/architecture/acplb-architecture.md`.

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

### Commands you'll use most

Commands below now include the metadata + consistency tooling introduced by PR #47; treat the validation scripts as mandatory pre-PR gates alongside fmt/clippy/test.

- Prerequisites
    - Rust stable toolchain is pinned (rust-toolchain.toml). Ensure cargo, rustfmt, clippy are available.
    - For documentation style checks, scripts expect markdownlint-cli2 (install globally or use npx as shown in scripts/ci/run-markdown-style.sh).

- Build and test
    - Build everything

    ```bash path=null start=null
    cargo build --workspace --all-features
    ```

    - Build specific crate

    ```bash path=null start=null
    cargo build -p codex-cli-acp
    cargo build -p acp-lazy-core
    ```

    - Test everything (locked deps)

    ```bash path=null start=null
    cargo test --workspace --all-features --locked
    ```

    - Test a single crate

    ```bash path=null start=null
    cargo test -p codex-cli-acp
    cargo test -p acp-lazy-core
    ```

    - Run a single test (example names from this repo)

    ```bash path=null start=null
    cargo test -p codex-cli-acp initialize_accepts_string_and_integer_protocol_version
    cargo test -p acp-lazy-core test_message_classification
    ```

- Lint, format, docs
    - Format check and clippy (deny warnings)

    ```bash path=null start=null
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    ```

    - Build API docs (no deps)

    ```bash path=null start=null
    cargo doc --workspace --no-deps
    ```

- Local CI (structure + language + markdown + semantic checks)
    - Run the consolidated pre‑PR validation suite

    ```bash path=null start=null
    ./scripts/ci/run-local-ci.sh
    ```

    - Individual checks mirrored in CI

    ```bash path=null start=null
    ./scripts/ci/run-sdd-structure-lint.sh
    ./scripts/ci/check-language-policy.sh
    ./scripts/ci/run-markdown-style.sh
    ./scripts/sdd/run_semantic_checks.sh
    ```

- Run the adapter (ACP server)
    - Start the server (reads JSON-RPC 2.0 messages from stdin; writes JSONL to stdout; logs to stderr)

    ```bash path=null start=null
    cargo run -p codex-cli-acp
    ```

    - Minimal handshake (initialize → expect result.protocolVersion = 1)

    ```bash path=null start=null
    echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' \
      | cargo run -p codex-cli-acp
    ```

    - Create a session (cwd must be absolute; mcpServers must be an array)

    ```bash path=null start=null
    echo '{"jsonrpc":"2.0","id":2,"method":"session/new","params":{"cwd":"/Users/arthur/dev-space/ACPLazyBridge","mcpServers":[],"permissionMode":"default"}}' \
      | cargo run -p codex-cli-acp
    ```

    - Prompt within a session (streamed session/update events will be emitted)

    ```bash path=null start=null
    SESSION_ID="session_$(uuidgen | tr 'A-Z' 'a-z')"  # Example placeholder; use an actual ID returned by session/new
    printf '{"jsonrpc":"2.0","id":3,"method":"session/prompt","params":{"sessionId":"%s","prompt":{"role":"user","content":"Say hello"}}}\n' "$SESSION_ID" \
      | CODEX_CMD=${CODEX_CMD:-codex} cargo run -p codex-cli-acp
    ```

    - JSONL playback helper (feeds a JSONL file into a fresh server process)

    ```bash path=null start=null
    cargo run -p codex-cli-acp --bin playback < _artifacts/tests/protocol-baseline/<scenario>.jsonl
    ```

- Coverage (optional; if cargo-tarpaulin is installed)

  ```bash path=null start=null
  cargo tarpaulin --workspace --out Html --output-dir _artifacts/reports/legacy/<task>/
  ```

- Metadata and consistency
    - Validate YAML frontmatter and document headers:

    ```bash path=null start=null
    scripts/sdd/validate-metadata.sh
    ```

    - Run the global consistency audit (must pass before PR):

    ```bash path=null start=null
    scripts/sdd/check-sdd-consistency.sh
    ```

    - Inspect metadata programmatically (optional example):

    ```bash path=null start=null
    scripts/sdd/query-metadata.sh --type spec --format table
    ```

### Configuration and environment

Configuration defaults incorporate the notify + permission mapping refinements from the refresh; ensure environment variables stay in sync with `scripts/sdd/check-sdd-consistency.sh`.

- External Codex CLI
    - The adapter shells out to Codex. Configure path via CODEX_CMD (defaults to codex on PATH).

  ```bash path=null start=null
  export CODEX_CMD=/usr/local/bin/codex  # if codex is not in PATH
  ```

- Permission mapping (non‑interactive defaults)
    - default → approval_policy=never, sandbox_mode=read-only, network_access=false
    - plan → approval_policy=never, sandbox_mode=read-only, network_access=false
    - acceptEdits → approval_policy=never, sandbox_mode=workspace-write, network_access=false
    - bypassPermissions → approval_policy=never, sandbox_mode=workspace-write, network_access=true
    - "YOLO" danger mode is explicit opt‑in only (maps to danger-full-access).

- Notify integration (optional)
    - End turns immediately when Codex emits a notify event.
    - Environment variables:
        - ACPLB_NOTIFY_PATH: path to sink (file or FIFO).
        - ACPLB_NOTIFY_KIND: file | fifo (default: file).
        - ACPLB_NOTIFY_INJECT: auto | never | force (default: auto) — whether to inject acplb-notify-forwarder.
        - ACPLB_NOTIFY_CMD: custom notify program array (JSON) to override injection.
        - ACPLB_IDLE_TIMEOUT_MS: idle timeout (default: 1200).
        - ACPLB_POLLING_INTERVAL_MS: poll interval for timeouts/notify (default: 100).
    - Examples

    ```bash path=null start=null
    # File-based sink
    export ACPLB_NOTIFY_PATH=/tmp/codex-notify.jsonl
    export ACPLB_NOTIFY_KIND=file
    cargo run -p codex-cli-acp

    # FIFO sink
    mkfifo /tmp/codex-notify.fifo
    export ACPLB_NOTIFY_PATH=/tmp/codex-notify.fifo
    export ACPLB_NOTIFY_KIND=fifo
    cargo run -p codex-cli-acp

    # Disable auto-injection when Codex is already configured with a notify program
    export ACPLB_NOTIFY_INJECT=never
    cargo run -p codex-cli-acp
    ```

- Logging
    - stdout is reserved for protocol JSON lines.
    - All logs go to stderr (via tracing subscriber). Control with RUST_LOG (e.g., info, debug, trace).

  ```bash path=null start=null
  RUST_LOG=info cargo run -p codex-cli-acp 2>debug.log
  ```

## Handy workflows

Validated workflows reflecting the refreshed protocol and consistency toolchain—capture evidence under `_artifacts/` and link it in specs/ tasks.


- End‑to‑end smoke of initialize → session/new → session/prompt

  ```bash path=null start=null
  {
    echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}'
    echo '{"jsonrpc":"2.0","id":2,"method":"session/new","params":{"cwd":"/Users/arthur/dev-space/ACPLazyBridge","mcpServers":[]}}'
    echo '{"jsonrpc":"2.0","id":3,"method":"session/prompt","params":{"sessionId":"<REPLACE_WITH_RETURNED_ID>","prompt":{"role":"user","content":"hello"}}}'
  } | CODEX_CMD=${CODEX_CMD:-codex} RUST_LOG=info cargo run -p codex-cli-acp
  ```

- Develop with streaming logs and notify sink (FIFO)

  ```bash path=null start=null
  mkfifo /tmp/codex-notify.fifo
  export ACPLB_NOTIFY_PATH=/tmp/codex-notify.fifo
  export ACPLB_NOTIFY_KIND=fifo
  RUST_LOG=debug cargo run -p codex-cli-acp 2>stderr.log
  ```

- Replay saved protocol scenarios (JSONL)

  ```bash path=null start=null
  cargo run -p codex-cli-acp --bin playback < _artifacts/tests/protocol-baseline/<scenario>.jsonl
  ```

### Protocol implementation guidelines (ACP v1 examples)

- **ACP Protocol Schema**: <https://agentclientprotocol.com/protocol/schema>
- **ACP official Rust library**: `cargo add agent-client-protocol`
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)
- **(ACP) Protocol Lastest Version**: `v0.4.2` (2025-09-22)

Updated to match ACP v1 schema guidance (agent-client-protocol v0.4.2 released 2025-09-22); keep examples synchronized with `dev-docs/references/acp.md`.

- All examples use ACP v1: "protocolVersion": 1
- JSON-RPC 2.0 message structure:

```json
  Request:
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": { "protocolVersion": 1, "capabilities": {} }
  }
  Response:
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
      "protocolVersion": 1,
      "capabilities": {},
      "serverInfo": { "name": "codex-cli-acp", "version": "0.1.0" }
    }
  }
  Notification:
  {
    "jsonrpc": "2.0",
    "method": "session/update",
    "params": { "sessionId": "session_123", "content": "Processing request..." }
  }
  Error:
  {
    "jsonrpc": "2.0",
    "id": 1,
    "error": { "code": -32600, "message": "Invalid Request", "data": "Additional error details" }
  }
```

### Event streaming specifications

Session/update payloads must follow Codex streaming semantics: emit `agent_message_chunk` deltas, `tool_call` status transitions (pending → in_progress → completed/failed), and `notify_end_turn` markers for boundaries.

- `agent_message_chunk`: stream Codex assistant deltas in order; include role metadata when available.
- `tool_call`: publish lifecycle updates (pending → in_progress → completed/failed) with consistent `toolCallId` handles.
- `notify_end_turn`: signal early turn completion when acplb-notify-forwarder emits an agent-turn-complete event.

### JSONL communication format

Codex CLI emits one JSON object per line; do not pretty-print or buffer multiple messages—flush each newline-terminated payload immediately.

- One JSON message per line; newline-terminated; no pretty-printing
- Keep stdout reserved for protocol messages; send diagnostics to stderr via `tracing`.

### Error handling requirements

Map adapter errors to canonical JSON-RPC codes and include a `data` object with context (e.g., permission mode, command line) when available.

- Use standard JSON-RPC 2.0 error codes:
    - -32700 Parse error
    - -32600 Invalid Request
    - -32601 Method not found
    - -32602 Invalid params
    - -32603 Internal error
- Include descriptive messages and optional data field

### Practical examples (updated to ACP v1)

Run these sanity checks after pulling to confirm protocol fidelity; they exercise the refreshed defaults and should generate evidence if behaviour shifts.

- Test initialize handshake:
  echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | codex proto
- With custom model and permissions:
  codex proto -c model="openai/gpt-5" -c approval_policy="never" -c sandbox_mode="read-only" < test_messages.jsonl
- Debug with verbose logging:
  RUST_LOG=debug codex proto 2>debug.log

### Non-mock testing plan (WARP-Agent + Zed smoke)

Coordinate Warp + Zed smoke runs by storing logs in `_artifacts/tests/<task>/` and rerunning metadata + consistency scripts before sharing results.

- Evidence path: (_artifacts/{tests,logs,jq,reports}/<task>/)
- Do not echo secrets; use environment variables (e.g., OPENAI_API_KEY, ANTHROPIC_API_KEY, GEMINI_API_KEY)

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

## Notes

- This file is AI Engineer("warp")'s role-specific. If it conflicts with CONTRIBUTING.md or sdd-rules/lifecycle.md, those take precedence.

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-23T04:56:00Z"
document:
    type: "gemini-memory"
    path: "./GEMINI.md"
    version: "1.0.5"
    last_updated: "2025-09-25T02:30:00Z"
    changelog: "Refreshed constitution metadata after rerunning checklist"
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
        - "sdd-rules/AGENTS.md"
```
