# AGENTS.md (agents meta profile) — SDD Developer Team's AI Engineers

```text
This document defines the roles, responsibilities, and coordination model for AI Engineers ("agents") working within our Project - Repository "ACPLazyBridge" Specification‑Driven Development (SDD) team. It follows the SDD principles in (.specify/memory/constitution.md) . All **AI Engineer** collaborates with a team that includes human developers and other AI engineers, team work with AI coding agents such as (Claude Code, WARP, Gemini, Codex, etc.) Together, the team plans and writes code that strictly follows the specification. It complements CONTRIBUTING.
```

- **Project Name**: `ACPLazyBridge` (Rust workspace)
- **Project Repository URL**: <https://github.com/lwyBZss8924d/ACPLazyBridge>
- **ACP (Agent Client Protocol) Protocol**: <https://github.com/zed-industries/agent-client-protocol>
- **ACP JSON Schema**: <https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json>
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

```text
ACPLazyBridge is an ACP (Agent Client Protocol) bridge that connects AI agents and agent-tools plugins with IDEs, editors, and development tools. It provides native adapters for various AI systems while maintaining protocol consistency and developer workflow integration through Specification-Driven Development (SDD).
```

```text
Team's AI Engineer all members: Roles, responsibilities, and coordination model. For the authoritative workflow and lifecycle, always refer to the documents listed below. and always refer to the SDD Constitution. wen update any SDD document and sdd-rules document, MUST follow the SDD Constitution Update Checklist. All SDD document and sdd-rules document and normative artifacts (specify, plan, tasks, issues, PRDs, commits, etc.) MUST be English‑only.
```

**Authority and scope**

- Normative authority:
    - (.specify/) all SDD documents
    - (.specify/memory/constitution.md) (ACPLazyBridge SDD Constitution; authoritative governance)
    - (sdd-rules/) all sdd-rules documents. **AI Engineer SDD All Rules Index**: (sdd-rules/rules/README.md)
- Team/agent rules:
    - sdd-rules/AGENTS.md (team member AI Engineers BASE LINE roles for all AI Engineers, command allowlist, dynamic consistency workflow)
    - (../CLAUDE.md), (../WARP.md), (../AGENTS.md) e.g. Team member AI Engineers's root memory.
- (specs/) all TASK work specs files
- (dev-docs/) all dev-docs files
- (crates/) all crates codebase files
- (scripts/) all scripts
- (.worktrees) all worktrees branches linked

This file applies to all contributors (human and AI). ouher Agent-specific files (WARP.md / CLAUDE.md / GEMINI.md etc.) must align with this file.

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
  "Validate-Structure": "validate_structure.py"
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

## Team AI Engineer Profiles

The following agents compose our SDD Developer Team members. Names in brackets are short identifiers used throughout this document.

### Operating base Rules

- CLI Tools: wen use any command line tools to avoid interactive/paged commands; never expose secrets.
- Command allowlist & MCP servers: defer to <DeveloperTeamMembers AI-Engineer (Agents)>; do not duplicate here.
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

- /specify — generate a new feature specification and branch/worktree; see sdd-rules/commands/specify.md
- /plan — create implementation plan and design docs; see sdd-rules/commands/plan.md
- /tasks — derive executable tasks from the plan; see sdd-rules/commands/tasks.md
Notes:
- Use these commands to maintain the spec → plan → tasks flow described in (.specify/spec-driven.md) and (.specify/memory/lifecycle.md).

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
  git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/<task-dir> origin/main -b <branch>
- Optional IDE navigation:
  ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>

### Quality gates (must pass)

- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace --all-features --locked
- Protocol JSONL scenarios (if present) replay without errors; stdout is valid JSONL.
- Code scanning (GitHub Code Scanning) is enabled. For local custom CodeQL queries, see dev-docs/engineering/codeql.md.

### Constitutional gates (must pass)

- Simplicity (Article VII): ≤3 projects; no future-proofing; avoid unnecessary patterns. See .specify/memory/constitution.md
- Anti-Abstraction (Article VIII): Use framework features directly; single model representation. See .specify/memory/constitution.md
- Integration-First (Article IX): Contracts defined; contract tests written before implementation; use real dependencies where practical. See .specify/memory/constitution.md
- Test-First (Article III): Write tests first and confirm failing (RED) before implementation. See .specify/memory/constitution.md

### SDD checks (pre-PR)

- scripts/ci/run-local-ci.sh — runs structure, language, markdown, and semantic checks
- Or on macOS, run individually:
    - scripts/sdd/check_language.sh
    - scripts/sdd/lint_docs.sh
    - scripts/sdd/run_semantic_checks.sh

- Before submitting a PR, run the scripts in (scripts/sdd/) to perform the SDD consistency check and ensure compliance for (specs/) TASK's artifacts.

### Security & compliance

- Do not log secrets; never print secrets to CI logs; use env vars and GitHub secrets.
- Avoid running untrusted code or scripts without review.

### Communication

- Keep status short and actionable; when uncertain about intent, ask before proceeding.
- Escalate risks with options and trade-offs.

## Code Analysis command line Tools

Tip: When you need to do Code Search and Retrieval and any Codebase Analysis Operations, Can use subagent: "code-retriever" or "code-analyzer"

Advanced code analysis techniques: @sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md

**BASE Command line Tools**:

- Find Files: `fd`
- Find Text: `rg` (ripgrep) `search` and `parse`
- Find Code Structure: `ast-grep`
- Select among matches: pipe to `fzf`
- JSON: `jq`
- YAML/XML: `yq`

## Augmented CLI Development Tooling

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

### "SemTools" `search` and `parse` (Document Search and Parsing)

> (sdd-rules/rules/tools-cli/sdd-rules-tools-cli-document-search-and-parsing.md)

#### Parse CLI Help

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

#### Search CLI Help

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

### AI Engineers Roles

All AI Engineer MUST Flow Project's SDD - CONSTITUTION and SDD Rules.

- [claude] Claude Code (CLI/VS Code) — Primary dev agent and orchestrator  `claude --help`
- [warp] Warp Agent (Terminal/CLI) — Project manager, planner, reviewer      `warp-preview agent run --help`
- [codex] Codex CLI — Code analysis and optimization                         `codex --help`
- [gemini] Gemini CLI — Research and documentation                           `gemini --help`
- [cursor] Cursor Agent — Pair programming and refactors                     `cursor-agent --help`

```yaml
name: sdd-development
agents:
  - claude: primary
  - claude_subagents: members
  - cursor_agent: pair
mcp_servers: all
permissions:
  - read: all
  - write: all
  - execute: all

name: sdd-review
agents:
  - warp: primary
  - gemini: analyzer
mcp_servers: all
permissions:
  - read: all
  - write: all
  - execute: all

name: sdd-research
agents:
  - warp: primary
  - claude: researcher
mcp_servers: all
permissions:
  - read: all
  - write: all
  - execute: all
```

## SDD Team Workflows

### New Feature Workflow (spec → plan → tasks → code)

1. **warp**: Co‑define requirements with human devs; capture the WHAT and WHY (no HOW). If needed, open/triage a GitHub Issue.
2. **warp**: Create a feature branch and worktree (auto‑numbered) and initialize `specs/NNN-feature/` using `/specify`.
3. **claude**: Generate implementation plan via `/plan`, producing `plan.md`, and supporting docs (`data-model.md`, `contracts/`, `research.md`, `quickstart.md`).
4. **warp**: Validate plan against SDD gates (Simplicity, Anti‑Abstraction, Integration‑First, Test‑First). Mark ambiguities as `[NEEDS CLARIFICATION]`.
   - Library‑First Gate (Article I):
     - [ ] Feature implemented as a library first (package/module skeleton present)
     - [ ] Minimal testable structure exists (contract/integration scaffolds)
     - [ ] Build/test jobs include the library target
   - CLI Interface Gate (Article II):
     - [ ] CLI entrypoint(s) defined and discoverable (`<tool> --help`)
     - [ ] CLI supports stdin/stdout and JSON for structured IO
     - [ ] CLI contract tests present (help/usage snapshot + sample IO cases)
5. **claude**: Generate executable `tasks.md` via `/tasks`. Mark parallelizable tasks.
6. **claude**: Implement via strict TDD (contract → integration → e2e → unit), only writing code to make tests pass.
7. **warp**: Review artifacts in `specs/NNN-feature/`, update progress, and link the branch/commits to the Issue.
8. **warp**: Run local checks (structure, language policy, semantic, template drift). Push branch and open PR.
9. **warp + claude**: Monitor CI, process PR review, keep specs/tasks in sync with requested changes.
10. **warp**: Merge, clean up worktree, pull main, run SDD consistency pass, and update team‑wide SDD docs if required.

### Bug Fix Workflow (spec‑first, reproduction‑driven)

Use the feature workflow adapted for bug reproduction and prevention. Code changes must be specification‑driven, not patch‑first.

1. **warp**: Open/triage a GitHub Issue. Create a bugfix worktree/branch `NNN-bug-[slug]`.
2. **warp**: In `specs/NNN-bug-[slug]/spec.md`, document:
   - Title, context, impacted versions, severity
   - Minimal Reproduction Steps (MRS)
   - Expected vs. Actual behavior
   - Scope (components, contracts, data)
   - Non‑functional impacts (perf, security, compatibility)
3. **claude**: Generate `plan.md` with root‑cause hypotheses and proposed fix strategies. Record validation points and potential regressions.
4. **claude**: Write failing tests first derived from MRS (contract/integration/e2e). No implementation until tests are red.
5. **claude**: Implement the fix to make tests pass; update contracts if behavior is clarified. Keep changes minimal per Simplicity/Anti‑Abstraction gates.
   - If the fix touches behavior contracts:
     - [ ] Update CLI help/usage and examples accordingly
     - [ ] Update CLI contract tests (help snapshot + sample IO)
     - [ ] Record rationale and impact in `spec.md`/`plan.md`
6. **warp**: Ensure the change lands in a replaceable library unit (Article I) and the CLI surface remains consistent (Article II).
7. **warp**: Update `tasks.md` for the bugfix, mark status, and link commit messages to the Issue `[BUG-NNN]` (or `[TASK-XXX]` if unified).
8. **warp**: Run local CI (structure, language, semantic, drift). Push branch and open PR with reproduction, fix rationale, and test evidence.
9. **warp + claude**: Address PR feedback. If the bug implies spec ambiguity, update feature specs to remove `[NEEDS CLARIFICATION]` markers system‑wide.
10. **warp**: Merge, clean up branch. Backport if needed. Update CHANGELOG/Release notes.

### SDD Documentation & CI Dynamic Consistency Update Workflow

Purpose: keep specifications, plans, tasks, and CI checks aligned with reality after any change (feature, fix, or refactor).

1. **Triggering Events**
   - PR merged to main; upstream template changes; ecosystem/library updates; constitution amendments; recurring drift or semantic alerts.

2. **Detection & Audit (local/CI)**
   - Run `scripts/ci/run-local-ci.sh` or `specify doctor` to execute:
     - SDD structure lint (required directories, files)
     - Language policy (English‑only for normative artifacts)
     - Markdown lint (style, links)
     - Template drift (compare against upstream or pinned ref)
     - Semantic checks (broken cross‑refs, placeholders, `[NEEDS CLARIFICATION]`)
     - Library‑First conformance (Article I): library modules present; packaging/build targets configured
     - CLI conformance (Article II): entrypoints exist and are executable; `--help` output matches documented usage/examples

3. **Documentation Sync**
   - For any deviation, update `specs/*/(spec|plan|tasks).md` and supporting docs (`research.md`, `data-model.md`, `contracts/`).
   - If CI workflows or governance changed, update `dev-docs/sdd/*` and project‑level `WARP.md`, `AGENTS.md`, `CLAUDE.md`.
   - Ensure updates are minimal and traceable; link Issues/PRs.

4. **Template & Manifest Alignment (optional)**
   - If improvements are generic, promote them into the template set under `templates/` (not repository‑specific roots).
   - Record template version and migration notes. Prepare `templates diff`/`templates update`.

5. **Validation & Publication**
   - Re‑run local checks. Open a PR focused on doc/CI consistency. Ensure passing SDD gates.
   - On merge, if templates changed, cut a release of templates (not repository‑specific content). Communicate channel (stable/canary).

6. **Roles**
   - **warp**: Orchestrates audits, updates normative docs, drives CI corrections, opens/merges doc PRs.
   - **claude**: Proposes concrete doc changes from diffs and runtime evidence; generates checklists and regression tests.

Outcome: documentation, plans, tasks, and CI checks remain a living, executable representation of the system, continuously synchronized with the implementation and upstream norms.

## DeveloperTeamMembers AI-Engineer (Agents)

### "Claude-Code" AI-Engineer subteam profiles (Primary)

**Role**: Lead developer team and orchestrator
**Capabilities**: Full-stack development, architectural decisions, code generation
**Access Level**: Read/write within defined scopes
**Primary Tools**: Bash / Task / TodoWrite / Read / Write / Edit / Glob / Grep / MultiEdit / WebSearch / WebFetch / Bash(ast-grep) and (/allowed-tools) all local CLI tools
**MCP Servers**: All available `--scope user` with config MCP tools `claude mcp list` ( serena / context7 / jina-mcp / deepwiki / github-mcp / e.g. ) <https://docs.anthropic.com/en/docs/claude-code/mcp>
**sub-agents**: "Claude-Code" developer team's sub-agents config (`~/.claude/agents/`) <https://docs.anthropic.com/en/docs/claude-code/sub-agents>
**settings**: config (`~/.claude/settings.json`) <https://docs.anthropic.com/en/docs/claude-code/settings>
**hooks**: <https://docs.anthropic.com/en/docs/claude-code/hooks>
**CLAUDE.md**: All Claude Code’s AI-Engineer memory and SDD rules files
**Github Actions**: <https://docs.anthropic.com/en/docs/claude-code/github-actions>

#### CLAUDE.md

Claude Code can remember project's SDD rules preferences across sessions, like style guidelines and common commands in SDD-DeveloperTeam workflow.

##### "Claude-Code" AI-Engineer SDD rules and memory type

Claude Code offers four memory locations in a hierarchical structure, each serving a different purpose:

| Memory Type                | Location                                                                                                                                                | Purpose                                             | Use Case Examples                                                    | Shared With                     |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------- |
| **Enterprise policy**      | macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />Linux: `/etc/claude-code/CLAUDE.md`<br />Windows: `C:\ProgramData\ClaudeCode\CLAUDE.md` | Organization-wide instructions managed by IT/DevOps | Company coding standards, security policies, compliance requirements | All users in organization       |
| **Project memory**         | `./CLAUDE.md`                                                                                                                                           | Team-shared instructions for the project            | Project architecture, coding standards, common workflows             | Team members via source control |
| **User memory**            | `~/.claude/CLAUDE.md`                                                                                                                                   | Personal preferences for all projects               | Code styling preferences, personal tooling shortcuts                 | Just you (all projects)         |
| **Project memory (local)** | `./CLAUDE.local.md`                                                                                                                                     | Personal project-specific preferences               | **(Deprecated, see below)** Your sandbox URLs, preferred test data     | Just you (current project)      |

All memory files are automatically loaded into Claude Code's context when launched. Files higher in the hierarchy take precedence and are loaded first, providing a foundation that more specific memories build upon.

##### CLAUDE.md imports

CLAUDE.md files can import additional files using `@path/to/import` syntax. The following example imports 3 files:

```bash
See @README for project overview and @package.json for available npm commands for this project.

# Additional Instructions
- git workflow @docs/git-instructions.md
```

Both relative and absolute paths are allowed. In particular, importing files in user's home dir is a convenient way for your team members to provide individual instructions that are not checked into the repository.
Previously CLAUDE.local.md served a similar purpose, but is now deprecated in favor of imports since they work better across multiple git worktrees.

```bash
# Individual Preferences
- @~/.claude/my-project-instructions.md
```

To avoid potential collisions, imports are not evaluated inside markdown code spans and code blocks.

```bash
This code span will not be treated as an import: `@anthropic-ai/claude-code`
```

Imported files can recursively import additional files, with a max-depth of 5 hops. You can see what memory files are loaded by running `/memory` command.

##### How Claude looks up memories

Claude Code reads memories recursively: starting in the cwd, Claude Code recurses up to (but not including) the root directory **/** and reads any CLAUDE.md or CLAUDE.local.md files it finds.
This is especially convenient when working in large repositories where you run Claude Code in **foo/bar/**, and have memories in both **foo/CLAUDE.md** and **foo/bar/CLAUDE.md**.

Claude will also discover CLAUDE.md nested in subtrees under your current working directory. Instead of loading them at launch, they are only included when Claude reads files in those subtrees.

##### Quickly add memories with the `#` shortcut

The fastest way to add a memory is to start your input with the `#` character:

```bash
# Always use descriptive variable names
```

You'll be prompted to select which memory file to store this in.

##### Directly edit memories with `/memory`

Use the `/memory` slash command during a session to open any memory file in your system editor for more extensive additions or organization.

##### Set up project SDD rules and memory

Suppose you want to set up a CLAUDE.md file to store important project information, conventions, and frequently used commands.

Bootstrap a CLAUDE.md for your codebase with the following command:

```bash
> /init
```

<Tip>
  Tips:
- Include frequently used commands (build, test, lint) to avoid repeated searches
- Document code style preferences and naming conventions
- Add important architectural patterns specific to your project
- CLAUDE.md memories can be used for both instructions shared with your team and for your individual preferences.
</Tip>

##### Organization-level SDD rules and memory management

Enterprise organizations can deploy centrally managed CLAUDE.md files that apply to all users.

To set up organization-level memory management:

1). Create the enterprise memory file in the appropriate location for your operating system:

- macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
- Linux/WSL: `/etc/claude-code/CLAUDE.md`
- Windows: `C:\ProgramData\ClaudeCode\CLAUDE.md`

2). Deploy via your configuration management system (MDM, Group Policy, Ansible, etc.) to ensure consistent distribution across all developer machines.

##### SDD rules and memory best practices

- **Be specific**: "Use 2-space indentation" is better than "Format code properly".
- **Use structure to organize**: Format each individual memory as a bullet point and group related memories under descriptive markdown headings.
- **Review periodically**: Update memories as your project evolves to ensure Claude is always using the most up to date information and context.

#### "Claude-Code" CLI reference

> Complete reference for Claude Code command-line interface, including commands and flags.

##### CLI commands

| Command                            | Description                                    | Example                                                            |
| :--------------------------------- | :--------------------------------------------- | :----------------------------------------------------------------- |
| `claude`                           | Start interactive REPL                         | `claude`                                                           |
| `claude "query"`                   | Start REPL with initial prompt                 | `claude "explain this project"`                                    |
| `claude -p "query"`                | Query via SDK, then exit                       | `claude -p "explain this function"`                                |
| `cat file \| claude -p "query"`    | Process piped content                          | `cat logs.txt \| claude -p "explain"`                              |
| `claude -c`                        | Continue most recent conversation              | `claude -c`                                                        |
| `claude -c -p "query"`             | Continue via SDK                               | `claude -c -p "Check for type errors"`                             |
| `claude -r "<session-id>" "query"` | Resume session by ID                           | `claude -r "abc123" "Finish this PR"`                              |
| `claude update`                    | Update to latest version                       | `claude update`                                                    |
| `claude mcp`                       | Configure Model Context Protocol (MCP) servers | See the [Claude Code MCP documentation](/en/docs/claude-code/mcp). |

##### CLI flags

Customize Claude Code's behavior with these command-line flags:

| Flag                             | Description                                                                                                                                              | Example                                                                    |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------- |
| `--add-dir`                      | Add additional working directories for Claude to access (validates each path exists as a directory)                                                      | `claude --add-dir ../apps ../lib`                                          |
| `--allowedTools`                 | A list of tools that should be allowed without prompting the user for permission, in addition to [settings.json files](/en/docs/claude-code/settings)    | `"Bash(git log:*)" "Bash(git diff:*)" "Read"`                              |
| `--disallowedTools`              | A list of tools that should be disallowed without prompting the user for permission, in addition to [settings.json files](/en/docs/claude-code/settings) | `"Bash(git log:*)" "Bash(git diff:*)" "Edit"`                              |
| `--print`, `-p`                  | Print response without interactive mode (see [SDK documentation](/en/docs/claude-code/sdk) for programmatic usage details)                               | `claude -p "query"`                                                        |
| `--append-system-prompt`         | Append to system prompt (only with `--print`)                                                                                                            | `claude --append-system-prompt "Custom instruction"`                       |
| `--output-format`                | Specify output format for print mode (options: `text`, `json`, `stream-json`)                                                                            | `claude -p "query" --output-format json`                                   |
| `--input-format`                 | Specify input format for print mode (options: `text`, `stream-json`)                                                                                     | `claude -p --output-format json --input-format stream-json`                |
| `--include-partial-messages`     | Include partial streaming events in output (requires `--print` and `--output-format=stream-json`)                                                        | `claude -p --output-format stream-json --include-partial-messages "query"` |
| `--verbose`                      | Enable verbose logging, shows full turn-by-turn output (helpful for debugging in both print and interactive modes)                                       | `claude --verbose`                                                         |
| `--max-turns`                    | Limit the number of agentic turns in non-interactive mode                                                                                                | `claude -p --max-turns 3 "query"`                                          |
| `--model`                        | Sets the model for the current session with an alias for the latest model (`sonnet` or `opus`) or a model's full name                                    | `claude --model claude-sonnet-4-20250514`                                  |
| `--permission-mode`              | Begin in a specified [permission mode](iam#permission-modes)                                                                                             | `claude --permission-mode plan`                                            |
| `--permission-prompt-tool`       | Specify an MCP tool to handle permission prompts in non-interactive mode                                                                                 | `claude -p --permission-prompt-tool mcp_auth_tool "query"`                 |
| `--resume`                       | Resume a specific session by ID, or by choosing in interactive mode                                                                                      | `claude --resume abc123 "query"`                                           |
| `--continue`                     | Load the most recent conversation in the current directory                                                                                               | `claude --continue`                                                        |
| `--dangerously-skip-permissions` | Skip permission prompts (use with caution)                                                                                                               | `claude --dangerously-skip-permissions`                                    |

<Tip>
  The `--output-format json` flag is particularly useful for scripting and
  automation, allowing you to parse Claude's responses programmatically.
</Tip>

For detailed information about print mode (`-p`) including output formats,
streaming, verbose logging, and programmatic usage, see the
[SDK documentation](/en/docs/claude-code/sdk).

#### "Claude-Code" developer team's sub-agents

### "Warp" Terminal & CLI Agents

**Role**: Project Manager, Issues Planner, and task‑artifact Reviewer
**Capabilities**: Command execution, log analysis, debugging
**Access Level**: All Permissions Always allow with selective execution (Apply code diffs / Read files / Create plans / Execute commands)
**Command allowlist**: Always allow
`which .*` `ls(\s.*)?` `grep(\s.*)?` `ast-grep(\s.*)?` `find .*` `echo(\s.*)?` `bash(\s.*)?` `zsh(\s.*)?` `fish(\s.*)?` `wget(\s.*)?` `rm(\s.*)?` `source(\s.*)?` `eval(\s.*)?` `curl(\s.*)?` `sh(\s.*)?` `pwsh(\s.*)?` e.g. all config allowed CLI tools
**MCP Servers**: Always allow All available with config MCP tools ( github-mcp / serena / context7 / jina-mcp / deepwiki / e.g. )

"Warp" CLI (`warp-preview`)

```bash
warp-preview

warp-preview --help

warp-preview agent --help
```

```bash
# List available profiles
warp-preview agent profile list

# Run with profile and MCP
warp-preview agent run \
  --profile {{PROFILE_UUID}} \
  --mcp-server {{MCP_UUID}} \
  -C /path/to/repo \
  --prompt "Review @dev-docs/sdd/lifecycle.md"

# GUI mode for interactive review
warp-preview agent run --gui \
  --prompt "Validate SDD compliance for PR #123"
```

### "Codex CLI" Agents

**Role**: Code analysis and optimization
**Capabilities**: Performance profiling, security scanning
**Access Level**: All Permissions Always allow with selective execution
**Primary Tools**: Bash / Task / Todo / Write / Edit / Read / Glob / Grep / Bash(ast-grep) and (/allowed-tools) all local CLI tools
**MCP Servers**: All available with config MCP tools

"Codex (codex)" CLI

```bash
codex

codex --version

codex --help
```

```bash
codex-cli analyze --spec specs/001-feature/spec.md
codex-cli optimize --file src/module.py
```

### "Gemini CLI" Agents

**Role**: Research and documentation
**Capabilities**: Technical research, API exploration
**Access Level**: All Permissions Always allow with selective execution
**Primary Tools**: Bash / Task / Todo / Write / Edit / Read / Glob / Grep / Bash(ast-grep) and (/allowed-tools) all local CLI tools
**MCP Servers**: All available with config MCP tools

"gemini-cli (gemini)" CLI

```bash
gemini

gemini --version

gemini --help
```

```bash
gemini research "JWT implementation best practices"
gemini document --spec specs/001-feature/
```

### "Cursor" IDE & CLI Agents

**Role**: Pair programming assistant
**Capabilities**: Real-time code suggestions, refactoring
**Access Level**: All Permissions Always allow with selective execution
**Primary Tools**: Bash / Task / Todo / Write / Edit / Read / Glob / Grep / Bash(ast-grep) and (/allowed-tools) all local CLI tools
**MCP Servers**: All available with config MCP tools
**Invocation**: Integrated in Cursor IDE

"Cursor (cursor-agent)" CLI

```bash
cursor-agent

cursor-agent --version

cursor-agent --help
```

## Agent Coordination

### Task Distribution

```yaml
specification_phase:
  lead: claude_code
  support: [claude_code_subagents, gemini_cli]

planning_phase:
  lead: claude_code
  reviewers: [warp_agent, codex_cli]

implementation_phase:
  lead: claude_code
  pair: cursor_agent
  reviewers: [warp_agent]

validation_phase:
  lead: warp_agent
  support: [claude_code_subagents]
```

### Communication Channels

- **Git branches**: on GitHub — Isolated work streams
- **PR comments**: on GitHub — Review feedback
- **Issue tracking**: on GitHub — Task coordination

### Commit & PR Conventions (traceability)

- Commit messages MUST include the task reference: `[TASK-NNN]` or `[BUG-NNN]`.
- Conventional commit style is encouraged: `feat:`, `fix:`, `docs:`, `chore:`, etc.
- PR template MUST answer:
    - Does this change affect a library interface? (Article I) [yes/no]
    - Does this change affect a CLI surface? (Article II) [yes/no]
    - Contracts updated? (paths)
    - Tests updated? (contract/integration/e2e/unit)
- Link the PR to the corresponding Issue and the `specs/NNN-*/` folder.

## MCP Servers Configuration

```bash
# MCP Servers Authorization Token
export GITHUB_TOKEN=$(security find-generic-password -s github)
export JINA_API_KEY=$(security find-generic-password -s jina)
export ANTHROPIC_API_KEY=$(security find-generic-password -s anthropic)
```

### "github-mcp"

- [MCP] <https://github.com/github/github-mcp-server>

**Purpose**: Repository operations
**install**: remote HTTP server
**Agents**: All
**Config**:

```json
{
  "mcpServers": {
    "github": {
      "url": "https://api.githubcopilot.com/mcp/",
      "headers": {
        "Authorization": "Bearer YOUR_GITHUB_PAT"
      }
    }
  }
}
```

**For Claude Code CLI:**

```bash
claude mcp add --transport http github https://api.githubcopilot.com/mcp -H "Authorization: Bearer YOUR_GITHUB_PAT"
```

### "jina-mcp"

- [MCP] <https://github.com/jina-ai/MCP>

**Purpose**: Web research and extraction
**install**: remote HTTP server
**Agents**: All
**Config**:

```json
{
  "mcpServers": {
    "jina-mcp-server": {
      "url": "https://mcp.jina.ai/mcp",
      "headers": {
        "Authorization": "Bearer ${JINA_API_KEY}"
      }
    }
  }
}
```

**For Claude Code CLI:**

```bash
claude mcp add --transport sse jina-mcp https://mcp.jina.ai/sse \
  --header "X-API-Key: Bearer ${JINA_API_KEY}"
```

### "context7"

- [MCP] <https://github.com/upstash/context7>

**Purpose**: Library documentation
**install**: remote HTTP server
**Agents**: All
**Config**:

```json
{
  "mcpServers": {
    "context7": {
      "url": "https://mcp.context7.com/mcp",
      "headers": {
        "CONTEXT7_API_KEY": "{CONTEXT7_API_KEY}"
      }
    }
  }
}
```

**For Claude Code CLI:**

```bash
claude mcp add --transport http context7 https://mcp.context7.com/mcp --header "CONTEXT7_API_KEY: YOUR_API_KEY"
```

### "deepwiki"

- [MCP] <https://mcp.deepwiki.com>

**Purpose**: any Github Repository research
**install**: remote HTTP server
**Agents**: All
**Config**:

```json
{
  "mcpServers": {
    "deepwiki": {
      "serverUrl": "https://mcp.deepwiki.com/mcp"
    }
  }
}
```

**For Claude Code CLI:**

```bash
claude mcp add -s user -t http deepwiki https://mcp.deepwiki.com/mcp
```

### "serena"

- [MCP] <https://github.com/oraios/serena>

**Purpose**: Semantic code analysis
**install**: local stdio macp server

```bash
uvx --from git+https://github.com/oraios/serena serena start-mcp-server
```

**Agents**: All
**Config**:

```json
{
    "mcpServers": {
        "serena": {
            "command": "/abs/path/to/uvx",
            "args": ["--from", "git+https://github.com/oraios/serena", "serena", "start-mcp-server"]
        }
    }
}
```

**For Claude Code CLI:**

```bash
claude mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --context ide-assistant --project $(pwd)
```

## Version Compatibility

| Agent | Min Version | Recommended | Notes |
|-------|------------|-------------|-------|
| Claude Code | 1.0.117 | Latest | Primary agent |
| warp-preview | v0.2025.09.10.08.11.preview_01 | Latest | CLI required |
| codex-cli | 0.36.0 | Latest | Optional |
| gemini-cli | 0.4.1 | Latest | Research focus |
| cursor-agent | 2025.09.12-4852336 | Latest | Cursor IDE's CLI agent |

```fish
echo "=== AI Engineers CLI Version check ==="
echo ""
echo "Claude Code: "(command -v claude >/dev/null 2>&1; and claude -v 2>/dev/null; or echo "Not installed")
echo "warp-preview: "(command -v warp-preview >/dev/null 2>&1; and warp-preview dump-debug-info 2>/dev/null | grep "Warp version" | sed 's/.*: Some("\(.*\)")/\1/'; or echo "Not installed")
echo "codex-cli: "(command -v codex >/dev/null 2>&1; and codex -V 2>/dev/null; or echo "Not installed")
echo "gemini-cli: "(command -v gemini >/dev/null 2>&1; and gemini -v 2>/dev/null; or echo "Not installed")
echo "cursor-agent: "(command -v ~/.local/bin/cursor-agent >/dev/null 2>&1; and ~/.local/bin/cursor-agent --version 2>/dev/null; or echo "Not installed")
```

---

## Repository Overview

### Implementation status

- Completed (M0): Rust workspace bootstrapped; references vendored
- In progress (M1): Codex native adapter (stdio loop, streaming, tool calls, permission mapping, smoke testing)
- Planned: Proxy adapter, plugin system v0, native adapters, HTTP/SSE bridge

### Architecture (high level)

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
    cargo run -p codex-cli-acp --bin playback < dev-docs/review/_artifacts/tests/<scenario>.jsonl
    ```

- Coverage (optional; if cargo-tarpaulin is installed)

  ```bash path=null start=null
  cargo tarpaulin --workspace --out Html --output-dir dev-docs/review/_artifacts/reports/<task>/
  ```

### Configuration and environment

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
  cargo run -p codex-cli-acp --bin playback < dev-docs/review/_artifacts/tests/<scenario>.jsonl
  ```

### Protocol implementation guidelines (ACP v1 examples)

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

- Agent message chunks: session/update with type=agent_message_chunk
- Tool call events: pending → completed tool_call updates

### JSONL communication format

- One JSON message per line; newline-terminated; no pretty-printing

### Error handling requirements

- Use standard JSON-RPC 2.0 error codes:
    - -32700 Parse error
    - -32600 Invalid Request
    - -32601 Method not found
    - -32602 Invalid params
    - -32603 Internal error
- Include descriptive messages and optional data field

### Practical examples (updated to ACP v1)

- Test initialize handshake:
  echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | codex proto
- With custom model and permissions:
  codex proto -c model="openai/gpt-5" -c approval_policy="never" -c sandbox_mode="read-only" < test_messages.jsonl
- Debug with verbose logging:
  RUST_LOG=debug codex proto 2>debug.log

### Non-mock testing plan (WARP-Agent + Zed smoke)

- Evidence path: (_artifacts/{tests,logs,jq,reports}/<task>/)
- Do not echo secrets; use environment variables (e.g., OPENAI_API_KEY, ANTHROPIC_API_KEY, GEMINI_API_KEY)

## (dev-docs/) and References

- Project references: dev-docs/references/, dev-docs/references/acp_adapters/, dev-docs/references/cli_agents/, dev-docs/references/acp.md, dev-docs/references/zed_ide.md
- Design/Plan/Requirements: dev-docs/design/, dev-docs/plan/, dev-docs/requirements/

<dev-docs>

```tree
ACPLazyBridge/dev-docs
❯ tree
.
├── CLAUDE.md
├── design
│   └── acp-lazybridge-architecture.md
├── engineering
│   └── codeql.md
├── plan
│   ├── acp-lazybridge-project-plan.md
│   ├── issues
│   │   ├── TEMPLATE.md
│   │   ├── closed/
│   │   ├── m1-issue-list.md
│   │   ├── open
│   │   │   ├── normalize-jsonl-protocol-v1.md
│   │   │   └── refresh-docs-examples-protocol-v1.md
│   │   └── waiting
│   │       └── ci-replay-acp-v1-runner.md
│   └── m1-technical-implementation-plan.md
├── references
│   ├── acp.md
│   ├── acp_adapters
│   │   └── claude_code_acp.md
│   ├── cli_agents
│   │   ├── ClaudeCode
│   │   │   ├── ClaudeCode-Config.md
│   │   │   ├── cli-reference.md
│   │   │   ├── hooks.md
│   │   │   ├── sdk-headless.md
│   │   │   ├── sdk-overview.md
│   │   │   ├── sdk-python.md
│   │   │   ├── sdk-rust(Unofficial).md
│   │   │   ├── sdk-typescript.md
│   │   │   ├── slash-commands.md
│   │   │   └── troubleshooting.md
│   │   ├── CodexCLI-Config.md
│   │   ├── claude_code.md
│   │   ├── codex.md
│   │   └── gemini.md
│   └── zed_ide.md
├── requirements
│   └── acp-lazybridge-requirements.md
└── review
    ├── _artifacts
    │   ├── ACP_SPEC_FIXES.md
    │   ├── ARC.yml
    │   ├── CHANGES.md
    │   ├── CLAUDE.md
    │   ├── CODEX.yml
    │   ├── ENV.txt
    │   ├── IMPL.csv
    │   ├── REQ.yml
    │   ├── REVISION.txt
    │   ├── SPEC.yml
    │   ├── WARP_REVIEW_FIXES.md
    │   ├── ZED.yml
    │   ├── jq
    │   │   └── filters.md
    │   ├── logs/
    │   │   └── README.md
    ├── parsed_files.txt
    │   ├── shell_params_integration.md
    │   ├── tests/
    │   ├── traceability.csv
    │   └── traceability.md
    └──changes/
```

</dev-docs>

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "agents-memory"
    path: "./sdd-rules/AGENTS.md"
    version: "1.0.1"
    last_updated: "2025-09-17T08:26:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - "sdd-rules/rules/README.md"
        - ".specify/templates/spec-template.md"
        - ".specify/templates/plan-template.md"
        - ".specify/templates/tasks-template.md"
```
