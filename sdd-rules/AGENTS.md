# AGENTS.md (AI-Engineer Team Member roles)

```text
This document defines AGENTS.md (AI-Engineer Team Member roles) in ACPLazyBridge. It inherits the global rules from CONTRIBUTING.md and sdd-rules/AGENTS.md, and defines the Developer Team's AI Engineers-"codex" roles, responsibilities, and coordination model for AI Engineers ("agents") working within our Specification‑Driven Development (SDD) team. It follows the SDD principles in .specify/spec-driven.md: specifications are the primary artifacts; plans and code serve the spec. AI Engineers development rules apply to human engineers and other AI Engineers team members (Claude Code, WARP, Gemini, Codex, etc.). It complements CONTRIBUTING.md and .specify/memory/lifecycle.md.
```

## Authority and Scope

- [SDD Constitution](../.specify/memory/constitution.md) - v1.0.1 with 9 core articles
- [SDD Lifecycle](../.specify/memory/lifecycle.md) - Supplementary Rules of the CONSTITUTION
- [SDD Principles](../.specify/spec-driven.md) - Core SDD principles

## AI-Engineer Team Member Key Rules

- **Development approach**: Worktree-first; branch categories: feature | fix | perf | chore | docs
- **Protocol compliance**: Stdout strictly JSONL; logs to stderr only
- **Evidence paths**:
    - Primary: `_artifacts/{tests,logs,jq,reports}/<task>/`
    - Legacy archives: `_artifacts/{tests,logs,jq,reports}/legacy/`
- **Permission mapping**: Non-interactive defaults: approval_policy=never; sandbox_mode per task; network access only when explicitly required
- **Protocol version**: Examples MUST use ACP v1: "protocolVersion": 1 (integer, not string)

## Submission Checklist (AI-Engineer Team Member PRs)

- Links to Spec/Plan/Tasks (`specs/<NNN>-<slug>/`)
- Evidence links (tests/logs/jq/reports) from both primary and legacy paths
- Risks/rollback section
- CI summary (fmt/clippy/test/replay)
- Constitutional gate verification (Articles I, II, III, VII, VIII, IX)

## About SDD

## SDD Constitution

- [SDD Constitution](../.specify/memory/constitution.md) - v1.0.1 with 9 core articles
- [SDD Lifecycle](../.specify/memory/lifecycle.md) - Supplementary Rules of the CONSTITUTION
- [SDD Constitution Update Checklist](../.specify/memory/constitution_update_checklist.md)
- [SDD Principles](../.specify/spec-driven.md) - Core SDD principles

### Base SDD Documentation

- [.specify/commands/sdd-task.md](../.specify/commands/sdd-task.md) - SDD-TASK command
- [.specify/commands/specify.md](../.specify/commands/specify.md) - SPECIFY command
- [.specify/commands/plan.md](../.specify/commands/plan.md) - PLAN command
- [.specify/commands/tasks.md](../.specify/commands/tasks.md) - TASKS command
- [.specify/templates/spec-template.md](../.specify/templates/spec-template.md) - Specification template
- [.specify/templates/plan-template.md](../.specify/templates/plan-template.md) - Plan template
- [.specify/templates/tasks-template.md](../.specify/templates/tasks-template.md) - Tasks template
- [AGENTS.md](./AGENTS.md) - Team coordination and roles
- [CLAUDE.md](./CLAUDE.md) - Claude-specific rules (this file)

(.specify/)

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

## SDD - (/specs)

Work in `./specs/`

### Current SDD Rules Structure

(/sdd-rules/rules/)

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

## SDD - Scripts & CI/CD

all scripts (AI-Engineer's own for Dev scripts Tools and custom CLI-Tools, CI scripts, SDD scripts, and E2E-TESTS / Dev project scripts CLI etc., which need to be unified in the future.)

```bash
ACPLazyBridge/scripts
❯ tree
.
├── CLAUDE.md
├── README.md
├── ast-grep
│   ├── sg-baseline-acp-rust-dbg.sh
│   ├── sg-baseline-acp-rust-no-unwrap.sh
│   ├── sg-baseline-acp-rust-todo.sh
│   ├── sg-fix.sh
│   ├── sg-scan-file.sh
│   └── sg-scan.sh
├── ci
│   ├── check-language-policy.sh
│   ├── json-to-sarif.jq
│   ├── run-local-ci.sh
│   ├── run-markdown-style.sh
│   ├── run-sdd-gates.sh
│   └── run-sdd-structure-lint.sh
└── sdd
    ├── check-markdown.sh
    ├── check-sdd-consistency.sh
    ├── check-task-prerequisites.sh
    ├── check_language.sh
    ├── common.sh
    ├── create-new-feature.sh
    ├── fix-markdown.sh
    ├── get-feature-paths.sh
    ├── lib
    │   └── metadata-utils.sh
    ├── lint_docs.sh
    ├── migrate-to-yaml-metadata.sh
    ├── query-metadata.sh
    ├── review-constitution-changes.sh
    ├── run_semantic_checks.sh
    ├── setup-plan.sh
    ├── update-agent-context.sh
    ├── upstream
    │   └── lib
    │       └── common.sh
    ├── validate-claude-md.sh
    ├── validate-metadata.sh
    └── validate-sdd-docs.sh
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

- [codex] Codex CLI — Code analysis and optimization                         `codex --help`
- [claude] Claude Code (CLI/VS Code) — Primary dev agent and orchestrator    `claude --help`
- [warp] Warp Agent (Terminal/CLI) — Project manager, planner, reviewer      `warp-preview agent run --help`
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

### SDD-TASKs INITIALIZATION WORKFLOW

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

### New Feature Workflow (spec → plan → tasks → code)

1. **codex**: Co‑define requirements with human devs; capture the WHAT and WHY (no HOW). If needed, open/triage a GitHub Issue.
2. **claude**: Create a feature branch and worktree (auto‑numbered) and initialize `specs/NNN-feature/` using `/specify` or `/sdd-task <issue>` for issue-based initialization.
3. **claude**: Generate implementation plan via `/plan`, producing `plan.md`, and supporting docs (`data-model.md`, `contracts/`, `research.md`, `quickstart.md`).
4. **codex**: Validate plan against SDD gates (Simplicity, Anti‑Abstraction, Integration‑First, Test‑First). Mark ambiguities as `[NEEDS CLARIFICATION]`.
   - Library‑First Gate (Article I):
     - [ ] Feature implemented as a library first (package/module skeleton present)
     - [ ] Minimal testable structure exists (contract/integration scaffolds)
     - [ ] Build/test jobs include the library target
   - CLI Interface Gate (Article II):
     - [ ] CLI entrypoint(s) defined and discoverable (`<tool> --help`)
     - [ ] CLI supports stdin/stdout and JSON for structured IO
     - [ ] CLI contract tests present (help/usage snapshot + sample IO cases)
5. **codex**: Generate executable `tasks.md` via `/tasks`. Mark parallelizable tasks.
6. **codex**: Implement via strict TDD (contract → integration → e2e → unit), only writing code to make tests pass.
7. All Team members AI-Engineer: (Dev Cooking 🚧)
8. **codex**: Review SDD TASKs artifacts in `specs/NNN-feature/`, update progress, and link the branch/commits to the Issue.
9. **claude + warp**: Run local checks (structure, language policy, semantic, template drift). pre-PR and TASKs Review Pass. Then Push branch and open PR.
10. Monitor GitHub Actions CI/CD and PR Review & Team members AI-Engineer Observation and Fix Review comments for Loop Phase.
11. **claude**: Merge, clean up worktree, pull main, run SDD consistency pass, and update team‑wide SDD docs if required.

### Bug Fix Workflow (spec‑first, reproduction‑driven)

Use the feature workflow adapted for bug reproduction and prevention. Code changes must be specification‑driven, not patch‑first.

1. **claude**: Open/triage a GitHub Issue. Create a bugfix worktree/branch `NNN-bug-[slug]`.
2. **codex**: In `specs/NNN-bug-[slug]/spec.md`, document:
   - Title, context, impacted versions, severity
   - Minimal Reproduction Steps (MRS)
   - Expected vs. Actual behavior
   - Scope (components, contracts, data)
   - Non‑functional impacts (perf, security, compatibility)
3. **codex**: Generate `plan.md` with root‑cause hypotheses and proposed fix strategies. Record validation points and potential regressions.
4. **codex**: Write failing tests first derived from MRS (contract/integration/e2e). No implementation until tests are red.
5. **codex**: Implement the fix to make tests pass; update contracts if behavior is clarified. Keep changes minimal per Simplicity/Anti‑Abstraction gates.
   - If the fix touches behavior contracts:
     - [ ] Update CLI help/usage and examples accordingly
     - [ ] Update CLI contract tests (help snapshot + sample IO)
     - [ ] Record rationale and impact in `spec.md`/`plan.md`
6. **codex**: Ensure the change lands in a replaceable library unit (Article I) and the CLI surface remains consistent (Article II).
7. **codex**: Update `tasks.md` for the bugfix, mark status, and link commit messages to the Issue `[BUG-NNN]` (or `[TASK-XXX]` if unified).
8. **claude**: Run local CI (structure, language, semantic, drift). Push branch and open PR with reproduction, fix rationale, and test evidence.
9. **claude + warp**: Address PR feedback. If the bug implies spec ambiguity, update feature specs to remove `[NEEDS CLARIFICATION]` markers system‑wide.
10. **claude**: Merge, clean up branch. Backport if needed. Update CHANGELOG/Release notes.

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

#### CLAUDE.md (such as AGENTS.md / GEMINI.md / WARP.md for Team member AI all Engineers's memory)

Claude Code "claude" / Codex "codex" / Gemini "gemini" / Warp "warp" etc. can remember project's SDD rules preferences across sessions, like style guidelines and common commands in SDD-DeveloperTeam workflow.

##### "Claude-Code" AI-Engineer SDD rules and memory type

Example CLAUDE.md (outer AGENTS.md etc. similarly)

Claude Code offers four memory locations in a hierarchical structure, each serving a different purpose:

| Memory Type                | Location                                                                                                                                                | Purpose                                             | Use Case Examples                                                    | Shared With                     |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------- |
| **Enterprise policy**      | macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />Linux: `/etc/claude-code/CLAUDE.md`<br /> | Organization-wide instructions managed by IT/DevOps | Company coding standards, security policies, compliance requirements | All users in organization       |
| **Project memory**         | `./CLAUDE.md`                                                                                                                                           | Team-shared instructions for the project            | Project architecture, coding standards, common workflows             | Team members via source control |
| **User memory**            | `~/.claude/CLAUDE.md`                                                                                                                                   | Personal preferences for all projects               | Code styling preferences, personal tooling shortcuts                 | Just you (all projects)         |
| **Project memory (local)** | `./CLAUDE.local.md`                                                                                                                                     | Personal project-specific preferences               | **(Deprecated, see below)** Your sandbox URLs, preferred test data     | Just you (current project)      |

All memory files are automatically loaded into Claude Code's context when launched. Files higher in the hierarchy take precedence and are loaded first, providing a foundation that more specific memories build upon.

##### CLAUDE.md imports

Example CLAUDE.md (outer AGENTS.md etc. similarly):

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

##### How Claude looks up memories (such as Codex / Gemini / Warp etc.)

Example CLAUDE.md (outer AGENTS.md etc. similarly):

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

##### Organization-level SDD rules and memory management (such as Codex / Gemini / Warp etc.)

Enterprise organizations can deploy centrally managed CLAUDE.md files that apply to all users.

To set up organization-level memory management:

1). Create the enterprise memory file in the appropriate location for your operating system:

- macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
- Linux/WSL: `/etc/claude-code/CLAUDE.md`

2). Deploy via your configuration management system (MDM, Group Policy, Ansible, etc.) to ensure consistent distribution across all developer machines.

##### SDD rules and memory best practices (such as Codex / Gemini / Warp etc.)

- **Be specific**: "Use 2-space indentation" is better than "Format code properly".
- **Use structure to organize**: Format each individual memory as a bullet point and group related memories under descriptive markdown headings.
- **Review periodically**: Update memories as your project evolves to ensure Claude is always using the most up to date information and context.

#### AI-Engineer developer team members

### (Codex CLI) AI-Engineer "codex"

**Role**: Project-Co-Founder, Project Manager, Issues Planner, and task‑artifact Reviewer, Full-stack development
**Capabilities**: Command execution, log analysis, debugging
**Access Level**: All Permissions Always allow with selective execution (Apply code diffs / Read files / Create plans / Execute commands)
**Command allowlist**: Always allow
`which .*` `ls(\s.*)?` `grep(\s.*)?` `ast-grep(\s.*)?` `find .*` `echo(\s.*)?` `bash(\s.*)?` `zsh(\s.*)?` `fish(\s.*)?` `wget(\s.*)?` `rm(\s.*)?` `source(\s.*)?` `eval(\s.*)?` `curl(\s.*)?` `sh(\s.*)?` `pwsh(\s.*)?` e.g. all config allowed CLI tools
**MCP Servers**: Always allow All available with config MCP tools ( context7 / jina-mcp / deepwiki / serena / e.g. )

### (Claude Code CLI) AI-Engineer "claude"

**Role**: Project-Co-Founder, Project Manager, Issues Planner, and task‑artifact Reviewer, Full-stack development
**Capabilities**: Command execution, log analysis, debugging
**Access Level**: All Permissions Always allow with selective execution (Apply code diffs / Read files / Create plans / Execute commands)
**Command allowlist**: Always allow
`which .*` `ls(\s.*)?` `grep(\s.*)?` `ast-grep(\s.*)?` `find .*` `echo(\s.*)?` `bash(\s.*)?` `zsh(\s.*)?` `fish(\s.*)?` `wget(\s.*)?` `rm(\s.*)?` `source(\s.*)?` `eval(\s.*)?` `curl(\s.*)?` `sh(\s.*)?` `pwsh(\s.*)?` e.g. all config allowed CLI tools
**MCP Servers**: Always allow All available with config MCP tools ( context7 / jina-mcp / deepwiki / serena / e.g. )

### (Warp) Terminal & CLI AI-Engineer "warp"

**Role**: Code analysis and optimization and GitHub operationser
**Capabilities**: Performance profiling, security scanning
**Access Level**: All Permissions Always allow with selective execution
**Primary Tools**: Bash / Task / Todo / Write / Edit / Read / Glob / Grep / Bash(ast-grep) and (/allowed-tools) all local CLI tools
**MCP Servers**: All available with config MCP tools

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

### "Gemini CLI" Agents

**Role**: Researcher / Documentation Retriever / Code Retriever / Codebase Analyzer
**Capabilities**: Technical research, API exploration, Code & Document Search any Codebase Analysis Operations
**Access Level**: All Permissions Always allow with selective execution
**Primary Tools**: Bash / Task / Todo / Write / Edit / Read / Glob / Grep / Bash(ast-grep) and (/allowed-tools) all local CLI tools
**MCP Servers**: All available with config MCP tools

```bash
gemini research "JWT implementation best practices"
gemini document --spec specs/001-feature/
```

### "Cursor" IDE & CLI Agents

**Role**: Pair programming developer
**Capabilities**: Real-time code suggestions, refactoring
**Access Level**: All Permissions Always allow with selective execution
**Primary Tools**: Bash / Task / Todo / Write / Edit / Read / Glob / Grep / Bash(ast-grep) and (/allowed-tools) all local CLI tools
**MCP Servers**: All available with config MCP tools
**Invocation**: Integrated in Cursor IDE

## Agent Coordination

### Task Distribution

```yaml
specification_phase:
  lead: codex
  support: [cladue, claude_code_subagents, warp_agent, gemini]

planning_phase:
  lead: codex
  reviewers: [claude, warp_agent]

implementation_phase:
  lead: codex
  pair: claude
  reviewers: [codex, claude, warp_agent, cursor_agent]

validation_phase:
  lead: codex
  support: [cladue, claude_code_subagents, warp_agent, gemini]
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

## Version Compatibility

| Agent | Min Version | Recommended | Notes |
|-------|------------|-------------|-------|
| Claude Code | 1.0.123 | Latest | Primary agent |
| warp-preview | v0.2025.09.17.08.11.preview_01 | Latest | CLI required |
| codex-cli | 0.41.0 | Latest | Optional |
| gemini-cli | 0.6.1 | Latest | Research focus |
| cursor-agent | 2025.09.18-7ae6800 | Latest | Cursor IDE's CLI agent |

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-22T15:20:00Z"
document:
    type: "ai-engineer-memory"
    path: "./sdd-rules/CLAUDE.md"
    version: "1.0.3"
    last_updated: "2025-09-25T02:30:00Z"
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
        - "./AGENTS.md"
```
