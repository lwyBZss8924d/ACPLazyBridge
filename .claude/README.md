# Claude Code SDD Workflow Integration

This directory contains custom slash commands, hooks, and sub-agents to streamline the Specification-Driven Development (SDD) workflow for ACPLazyBridge.

## Directory Structure

```tree
.claude/
├── agents/                   # Custom sub-agents for specialized tasks
│   ├── document-retriever.md # Document search and parsing agent
│   ├── code-retriever.md    # AST-based code search agent
│   ├── code-analyzer.md     # Repository analysis agent
│   └── sdd-doc-validator.md # SDD documentation validation agent
├── commands/                 # Custom slash commands
│   ├── specify.md           # Create feature specifications
│   ├── plan.md              # Generate implementation plans
│   ├── tasks.md             # Create task breakdowns
│   └── sdd-task.md          # Initialize from GitHub issues
├── hooks/                    # Automated validation hooks
│   ├── inject-datetime.sh          # Auto-inject UTC timestamps (UserPromptSubmit)
│   ├── validate-sdd-compliance.sh  # Pre-tool validation (PreToolUse: Write|Edit|MultiEdit)
│   ├── pre-tool-use-approval.sh    # Auto-approve Read for docs (PreToolUse: Read)
│   ├── post-sdd-check.sh           # Post-tool validation (PostToolUse: Write|Edit|MultiEdit)
│   ├── markdown-formatter.sh       # Post-tool markdown normalization (PostToolUse)
│   └── sdd-task-fetch.sh           # GitHub issue helper
├── settings.local.json       # Local configuration
├── CLAUDE.md                # Claude-specific guidance
└── README.md                # This file
```

## Features

### 1. Custom Slash Commands

#### `/sdd-task` - Initialize from GitHub Issues

Start SDD workflow directly from a GitHub issue:

```bash
/sdd-task 42                    # Using issue number
/sdd-task https://github.com/lwyBZss8924d/ACPLazyBridge/issues/42  # Using URL
/sdd-task 42 "performance focus"  # With additional context
```

Features:

- Fetches issue details via GitHub CLI
- Creates appropriate worktree and branch
- Triggers complete SDD workflow (/specify → /plan → /tasks)
- Ensures constitutional compliance
- Tracks progress with TodoWrite tool
- Creates evidence in `_artifacts/[NNN-slug]/`

#### `/specify` - Create Specifications

Generate feature specifications from natural language:

```bash
/specify "Real-time chat system with message history"
```

#### `/plan` - Generate Implementation Plans

Create technical plans from specifications:

```bash
/plan "WebSocket for real-time, PostgreSQL for history"
```

#### `/tasks` - Create Task Breakdowns

Generate executable task lists:

```bash
/tasks
```

### 2. Automated Hooks

#### Date/Time Injection (`inject-datetime.sh`)

- Automatically adds UTC timestamp to every prompt
- Eliminates manual date entry
- Provides ISO format, day of week, and temporal context

#### Pre-Tool Validation (`validate-sdd-compliance.sh`)

- Checks constitutional compliance before file operations
- Verifies Test-First principle
- Ensures stdout/stderr usage in Rust code
- Validates spec directory naming conventions
- Checks required metadata in SDD documents

#### Pre-Tool Auto Approval (`pre-tool-use-approval.sh`)

- Automatically approves `Read` tool calls for documentation files
- Extensions: `.md`, `.mdx`, `.txt`, `.json`
- Reduces friction for non-sensitive reads (PreToolUse: `Read`)

#### Post-Tool Validation (`post-sdd-check.sh`)

- Runs after spec/plan/task files are created/modified
- Executes appropriate validation scripts
- Reports validation results back to Claude

#### Markdown Formatter (`markdown-formatter.sh`)

- Normalizes Markdown after edits/writes (PostToolUse: `Write|Edit|MultiEdit`)
- Adds missing language tags to code fences
- Trims excessive blank lines outside code blocks
- Idempotent; safe to re-run

### 3. Specialized Sub-Agents

#### Document Retriever

- High-signal document retrieval using SemTools
- Searches `sdd-rules/`, `dev-docs/`, and other documentation
- Parses non-text formats (PDF, DOCX, XLSX)

#### Code Retriever

- AST-aware code search using ast-grep
- Falls back to ripgrep when needed
- Returns precise file:line citations

#### Code Analyzer

- Repository-wide rule audits
- Uses sgconfig.yml for Rust/JS/Python/Go
- Produces JSON/SARIF reports

#### SDD Doc Validator

- Comprehensive markdown validation and fixing
- SDD compliance checking across all documents
- Auto-fixes common violations with markdownlint
- Manages long-term documentation quality improvements
- Tracks progress across multiple sessions for large-scale fixes

## Configuration

### settings.local.json

Contains:

- Hook registrations
- Tool permissions
- MCP server configurations
- Agent allowlists

### Hook Configuration

Registered hooks:

- UserPromptSubmit:
    - `$CLAUDE_PROJECT_DIR/.claude/hooks/inject-datetime.sh`
- PreToolUse:
    - `matcher: Write|Edit|MultiEdit` → `$CLAUDE_PROJECT_DIR/.claude/hooks/validate-sdd-compliance.sh`
    - `matcher: Read` → `$CLAUDE_PROJECT_DIR/.claude/hooks/pre-tool-use-approval.sh`
- PostToolUse:
    - `matcher: Write|Edit|MultiEdit` → `$CLAUDE_PROJECT_DIR/.claude/hooks/post-sdd-check.sh`
    - `matcher: Write|Edit|MultiEdit` → `$CLAUDE_PROJECT_DIR/.claude/hooks/markdown-formatter.sh`

Note: The sensitive prompt blocker (Rust `user-prompt-submit`) is present but not registered; `inject-datetime` handles the UserPromptSubmit context injection.

Hooks are shell scripts for:

- Zero external language dependencies
- Direct integration with SDD scripts
- Fast execution without interpreter overhead
- Native Unix tool usage (jq, grep, sed)

## Usage Examples

### Starting a New Feature

```bash
# From GitHub issue
/sdd-task 28

# Manual specification
/specify "Add dark mode toggle to settings"
/plan "Use CSS variables for theming"
/tasks
```

### Validation Workflow

```bash
# Automatic hooks run on file operations
# Manual validation available:
./scripts/sdd/validate-sdd-docs.sh
./scripts/ci/run-local-ci.sh
```

## Integration Points

### With SDD Scripts

- `scripts/sdd/create-new-feature.sh` - Used by /specify
- `scripts/sdd/setup-plan.sh` - Used by /plan
- `scripts/sdd/check-task-prerequisites.sh` - Used by /tasks
- `scripts/sdd/validate-sdd-docs.sh` - Used by hooks

### With Templates

- `.specify/templates/spec-template.md`
- `.specify/templates/plan-template.md`
- `.specify/templates/tasks-template.md`

### With Constitution

Enforces ACPLazyBridge SDD Constitution v1.0.1:

- Article I: Library-First approach
- Article III: Test-First development
- Article VII: Simplicity (≤3 projects)
- Article VIII: Anti-Abstraction
- Article IX: Integration-First

## Troubleshooting

If hooks aren't working:

1. Check hook registration: `/hooks`
2. Verify executable permissions: `chmod +x .claude/hooks/*.sh`
3. Enable debug mode: `claude --debug`
4. Check configuration: `.claude/settings.local.json`

## Technical Implementation

### Shell Scripts vs Python

The hooks are implemented as shell scripts to:

- Eliminate external language dependencies in this Rust project
- Leverage native Unix tools (`jq`, `date`, `grep`, `sed`)
- Integrate directly with existing SDD validation scripts
- Provide faster execution without interpreter overhead
- Maintain consistency with the project's shell-based tooling

### Command Architecture

The `/sdd-task` command uses a direct execution approach:

- Claude executes `gh` CLI commands directly using the Bash tool
- No bash execution in markdown (avoiding `!command` syntax issues)
- Arguments are passed as context for Claude to use
- Clear step-by-step instructions for Claude to follow
- Better error handling through direct command execution

## Security

All hooks and scripts:

- Validate inputs with jq
- Use proper shell quoting
- Include timeouts to prevent hanging
- Only validate, never modify without permission
- Never log sensitive information

## References

- [SDD Constitution](../.specify/memory/constitution.md)
- [SDD Lifecycle](../.specify/memory/lifecycle.md)
- [SDD Rules](../sdd-rules/rules/README.md)
- [Contributing](../CONTRIBUTING.md)

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-22T15:20:00Z"
document:
    type: "claude-memory"
    path: ".claude/README.md"
    version: "1.0.1"
    last_updated: "2025-09-28T17:50:00Z"
    changelog: "Documented sdd-doc-validator integration and refreshed constitution checklist evidence"
    dependencies:
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - "sdd-rules/rules/README.md"
        - ".claude/CLAUDE.md"
        - ".claude/agents/sdd-doc-validator.md"
```
