# CLAUDE.md (.claude/)

## Authority

- Constitution: ../.specify/memory/constitution.md (Articles I, III, VII, VIII, IX)
- SDD Integration: ../.specify/CLAUDE.md (operational context)
- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md

## Purpose

This directory contains Claude Code's custom configuration for the ACPLazyBridge SDD workflow, including slash commands, validation hooks, and specialized sub-agents.

## Custom Configuration

### Slash Commands (.claude/commands/)

- `/specify` → `.specify/commands/specify.md` → `.specify/templates/spec-template.md`
- `/plan` → `.specify/commands/plan.md` → `.specify/templates/plan-template.md`
- `/tasks` → `.specify/commands/tasks.md` → `.specify/templates/tasks-template.md`
- `/sdd-task` → `.claude/commands/sdd-task.md` → Complete SDD workflow from GitHub issue

### Validation Hooks (.claude/hooks/)

- `inject-datetime.sh` - Automatic UTC timestamp injection
- `validate-sdd-compliance.sh` - Pre-tool SDD validation
- `post-sdd-check.sh` - Post-tool SDD validation
- `sdd-task-fetch.sh` - GitHub issue helper script

### Specialized Sub-Agents (.claude/agents/)

- `document-retriever.md` - High-signal document retrieval
- `code-retriever.md` - AST-aware code search
- `code-analyzer.md` - Repository-wide rule audits
- `sdd-doc-validator.md` - SDD documentation validation and fixing

## SDD Workflow with /sdd-task

The `/sdd-task` command streamlines the entire SDD workflow:

```bash
/sdd-task 42                    # From issue number
/sdd-task https://github.com/lwyBZss8924d/ACPLazyBridge/issues/42  # From URL
/sdd-task 42 "performance focus"  # With context
```

This command:

1. Fetches issue details using `gh` CLI
2. Creates appropriate git worktree and branch
3. Triggers `/specify` → `/plan` → `/tasks`
4. Ensures constitutional compliance
5. Tracks progress with TodoWrite tool
6. Creates evidence in `_artifacts/[NNN-slug]/`

## Operating Rules

### File Operations

- Hooks validate SDD compliance automatically
- Pre-tool: Constitutional requirements checked
- Post-tool: Document validation executed
- Results reported back to Claude

### Evidence Collection

- Primary: `_artifacts/{tests,logs,reports,jq}/<task>/`
- Legacy: `dev-docs/review/_artifacts/{tests,logs,reports,jq}/<task>/`

### Git Workflow

- Always use worktrees for development
- Branch types: feature | fix | perf | chore | docs
- Never develop on main branch

## Integration Points

### Scripts

- `scripts/sdd/create-new-feature.sh` - Feature initialization
- `scripts/sdd/setup-plan.sh` - Plan creation
- `scripts/sdd/check-task-prerequisites.sh` - Task validation
- `scripts/sdd/validate-sdd-docs.sh` - Document validation

### Templates

- `.specify/templates/spec-template.md`
- `.specify/templates/plan-template.md`
- `.specify/templates/tasks-template.md`

### Constitution Gates

- Article I: Library-First approach
- Article III: Test-First development (RED→GREEN→REFACTOR)
- Article VII: Simplicity (≤3 projects)
- Article VIII: Anti-Abstraction
- Article IX: Integration-First

## Configuration

### settings.local.json

Contains:

- Hook registrations
- Tool permissions (100+ allowed commands)
- MCP server configurations
- Agent allowlists

### Permissions

Key allowed tools:

- `Bash(./scripts/sdd/*:*)`
- `Bash(gh issue view:*)`
- `Bash(git worktree:*)`
- `Bash(./scripts/ci/run-local-ci.sh:*)`
- `Read(///**)`

## Technical Decisions

### Shell Scripts (not Python)

- Rust project - avoid Python dependencies
- Native Unix tools (jq, grep, sed)
- Direct integration with SDD scripts
- Fast execution without interpreter overhead

### Direct Command Execution

- Use Bash tool directly (not markdown execution)
- Better error handling
- Proper argument passing
- Timeout protection

## Security

All hooks and scripts:

- Validate JSON inputs with jq
- Use proper shell quoting
- Include timeouts (10-30 seconds)
- Read-only validation (no modifications)
- Never log sensitive information

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-22T15:20:00Z"
document:
    type: "claude-memory"
    path: ".claude/CLAUDE.md"
    version: "1.0.3"
    last_updated: "2025-09-22T15:20:00Z"
    dependencies:
        - "./CLAUDE.md"
        - ".specify/memory/lifecycle.md"
        - ".specify/README.md"
        - "sdd-rules/rules/README.md"
        - ".claude/commands/specify.md"
        - ".specify/templates/spec-template.md"
        - ".claude/commands/plan.md"
        - ".specify/templates/plan-template.md"
        - ".claude/commands/tasks.md"
        - ".specify/templates/tasks-template.md"
        - "dev-docs/references/acp.md"
```
