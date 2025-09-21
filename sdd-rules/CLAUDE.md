# CLAUDE.md (Authoritative)

```text
This document defines Claude Code–specific rules in ACPLazyBridge. It inherits the global rules from CONTRIBUTING.md and sdd-rules/AGENTS.md, and defines the Developer Team's AI Engineers-"claude" roles, responsibilities, and coordination model for AI Engineers ("agents") working within our Specification‑Driven Development (SDD) team. It follows the SDD principles in .specify/spec-driven.md: specifications are the primary artifacts; plans and code serve the spec. AI Engineers development rules apply to human engineers and other AI Engineers team members (Claude Code, WARP, Gemini, Codex, etc.). It complements CONTRIBUTING.md and .specify/memory/lifecycle.md.
```

## Authority and Scope

- **Normative authority**: CONTRIBUTING.md, .specify/memory/lifecycle.md, sdd-rules/AGENTS.md
- **Constitutional authority**: .specify/memory/constitution.md (v1.0.1)
- This file provides Claude-specific clarifications and must remain consistent with the above

## Key Rules (Claude Code)

- **Development approach**: Worktree-first; branch categories: feature | fix | perf | chore | docs
- **Protocol compliance**: Stdout strictly JSONL; logs to stderr only
- **Evidence paths**:
    - Primary: `_artifacts/{tests,logs,jq,reports}/<task>/`
    - Legacy: `dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/`
- **Permission mapping**: Non-interactive defaults: approval_policy=never; sandbox_mode per task; network access only when explicitly required
- **Protocol version**: Examples MUST use ACP v1: "protocolVersion": 1 (integer, not string)

## Submission Checklist (Claude PRs)

- Links to Spec/Plan/Tasks (`specs/<NNN>-<slug>/`)
- Evidence links (tests/logs/jq/reports) from both primary and legacy paths
- Risks/rollback section
- CI summary (fmt/clippy/test/replay)
- Constitutional gate verification (Articles I, II, III, VII, VIII, IX)

## References

- [CONTRIBUTING.md](../CONTRIBUTING.md)
- [sdd-rules/AGENTS.md](./AGENTS.md)
- [.specify/memory/lifecycle.md](../.specify/memory/lifecycle.md)
- [.specify/memory/constitution.md](../.specify/memory/constitution.md)

---

## About SDD

- [SDD Principles](../.specify/spec-driven.md)

### Current SDD Rules Structure

```bash
ACPLazyBridge/sdd-rules/
├── AGENTS.md                   # Team AI engineer profiles
├── CLAUDE.md                   # Claude-specific rules (this file)
├── commands/                   # SDD command documentation
│   └── README.md              # Command workflow reference
├── rule-tests/                 # AST-grep rule tests
│   ├── rust-mutex-lock-test.yml
│   └── rust-no-unwrap-test.yml
└── rules/                      # All SDD rule categories
    ├── README.md              # Rules index
    ├── changelog/             # Version and change management
    ├── ci/                    # Continuous integration rules
    ├── code-analysis/         # Code quality and AST-grep rules
    ├── documentation-style/   # Documentation standards
    ├── git/                   # Git workflow rules
    ├── research/              # Research methodology
    ├── tests/                 # Testing standards
    ├── tools-cli/             # CLI tool documentation
    └── tools-mcp/             # MCP server configuration
```

### Base SDD Documentation

- [AGENTS.md](./AGENTS.md) - Team coordination and roles
- [CLAUDE.md](./CLAUDE.md) - Claude-specific rules (this file)
- [.specify/spec-driven.md](../.specify/spec-driven.md) - Core SDD principles
- [.specify/templates/spec-template.md](../.specify/templates/spec-template.md) - Specification template
- [.specify/templates/plan-template.md](../.specify/templates/plan-template.md) - Plan template
- [.specify/templates/tasks-template.md](../.specify/templates/tasks-template.md) - Tasks template

## SDD Constitution

- [SDD Constitution](../.specify/memory/constitution.md) - v1.0.1 with 9 core articles
- [SDD Constitution Update Checklist](../.specify/memory/constitution_update_checklist.md)

## SDD - (/specs)

Work in `./specs/`

## SDD - Scripts & CI/CD

```bash
ACPLazyBridge/scripts/
├── sdd/                        # SDD workflow automation (12 scripts)
│   ├── create-new-feature.sh   # Initialize features
│   ├── setup-plan.sh           # Create plans
│   ├── check-task-prerequisites.sh  # Validate tasks
│   ├── validate-sdd-docs.sh    # SDD document validation
│   ├── check_language.sh       # Language policy
│   ├── run_semantic_checks.sh  # Link validation
│   ├── check-markdown.sh       # Markdown checks
│   ├── fix-markdown.sh         # Auto-fix markdown
│   ├── lint_docs.sh            # Doc linting
│   ├── get-feature-paths.sh    # Path utilities
│   ├── update-agent-context.sh # Memory sync
│   └── common.sh               # Shared functions
├── ci/                         # CI/CD automation (4 scripts)
│   ├── run-local-ci.sh        # Main CI orchestrator
│   ├── run-sdd-structure-lint.sh  # SDD validation
│   ├── check-language-policy.sh   # English enforcement
│   └── run-markdown-style.sh  # Markdown linting
└── ast-grep/                   # Code analysis (6 scripts)
    ├── sg-scan.sh              # Full codebase scan
    ├── sg-scan-file.sh         # Single file scan
    ├── sg-baseline-acp-rust-dbg.sh     # Find dbg! macros
    ├── sg-baseline-acp-rust-no-unwrap.sh  # Find unwrap()
    ├── sg-baseline-acp-rust-todo.sh    # Find TODOs
    └── sg-fix.sh               # Apply auto-fixes
```

## SDD Rules Categories

```bash
ACPLazyBridge/sdd-rules/rules/
├── README.md                   # Rules index
├── changelog/                  # Version management
│   ├── sdd-rules-changelog.md
│   └── semver.md
├── ci/                         # CI/CD rules
│   └── sdd-rules-ci.md
├── code-analysis/              # Code quality rules
│   ├── sdd-rules-code-analysis.md
│   └── ast-grep/               # AST-grep rule definitions
│       ├── go/no-fmt-println.yml
│       ├── js/
│       │   ├── no-console-log.yml
│       │   └── no-only-in-tests.yml
│       ├── python/
│       │   ├── no-pdb.yml
│       │   └── no-print.yml
│       └── rust/
│           ├── no-dbg.yml
│           ├── no-unwrap.yml
│           ├── rust-mutex-lock.yml
│           └── todo-comment.yml
├── documentation-style/        # Documentation standards
│   ├── Google-developer-documentation-style-guide.md
│   ├── sdd-rules-documentation-markdownlint.md
│   └── sdd-rules-documentation-style.md
├── git/                        # Git workflow
│   ├── comments/sdd-rules-comments.md
│   ├── issues/sdd-rules-issues.md
│   ├── pr/sdd-rules-pr.md
│   └── worktree/sdd-rules-worktrees.md
├── research/                   # Research methodology
│   └── sdd-rules-research.md
├── tests/                      # Testing standards
│   └── sdd-rules-tests.md
├── tools-cli/                  # CLI tool documentation
│   ├── sdd-rules-tools-cli-list.md
│   ├── sdd-rules-tools-cli-astgrep.md
│   ├── sdd-rules-tools-cli-document-search-and-parsing.md
│   └── ast-grep.llms.txt
└── tools-mcp/                  # MCP server rules
    └── sdd-rules-tools-mcp.md
```

## SDD Templates Location

The actual SDD templates are located in `.specify/templates/`, not under sdd-rules:

```bash
ACPLazyBridge/.specify/templates/
├── agent-file-template.md      # Agent documentation template
├── plan-template.md            # Implementation plan template
├── spec-template.md            # Specification template
└── tasks-template.md           # Task list template
```

## AST-grep Configuration

```yaml
# sgconfig.yml at repository root
ignores:
  - node_modules/**
  - target/**
  - dist/**
  - build/**
  - .git/**
  - .venv/**
  - .cache/**
  - coverage/**
  - tmp/**

ruleDirs:
  - sdd-rules/rules/code-analysis/ast-grep/go
  - sdd-rules/rules/code-analysis/ast-grep/js
  - sdd-rules/rules/code-analysis/ast-grep/python
  - sdd-rules/rules/code-analysis/ast-grep/rust
```

## Team AI Engineer Profiles

The following agents compose our SDD Developer Team members. Names in brackets are short identifiers used throughout this document.

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
2. **warp**: Create a feature branch and worktree (auto‑numbered) and initialize `specs/NNN-feature/` using `/specify` or `/sdd-task <issue>` for issue-based initialization.
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
**Sub-agents**: "Claude-Code" developer team's sub-agents config (`~/.claude/agents/`) <https://docs.anthropic.com/en/docs/claude-code/sub-agents>

- `document-retriever` - High-signal document retrieval using SemTools
- `code-retriever` - Precise code retrieval using AST-aware patterns
- `code-analyzer` - Repository analysis via ast-grep scan
- `sdd-doc-validator` - SDD documentation validation and markdown fixing

**Settings**: config (`~/.claude/settings.json`) <https://docs.anthropic.com/en/docs/claude-code/settings>
**Hooks**: <https://docs.anthropic.com/en/docs/claude-code/hooks>
**CLAUDE.md**: All Claude Code's AI-Engineer memory and SDD rules files
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

"Claude-Code" developer team's sub-agents

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

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-21T14:51:00Z"
document:
    type: "claude-memory"
    path: "./sdd-rules/CLAUDE.md"
    version: "1.0.2"
    last_updated: "2025-09-21T14:51:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - "sdd-rules/rules/README.md"
        - ".specify/templates/spec-template.md"
        - ".specify/templates/plan-template.md"
        - ".specify/templates/tasks-template.md"
        - "./CLAUDE.md"
```
