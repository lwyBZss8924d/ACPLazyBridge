# CLAUDE.md (scripts/)

## Authority

- Constitution: ../.specify/memory/constitution.md (Articles III, VII, IX)
- SDD Integration: ../.specify/CLAUDE.md (operational context)
- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- SDD Commands: ../.specify/commands/{specify,plan,tasks}.md

## Purpose

Automation scripts for SDD workflow, CI/CD, and development tasks. These scripts implement the core SDD automation that drives the specification-driven development process.

## SDD Integration

For comprehensive SDD workflow details, see **[../.specify/CLAUDE.md](../.specify/CLAUDE.md)**

### Constitutional Requirements for Scripts

- **Article III (Test-First)**: Scripts support RED→GREEN→REFACTOR workflow
- **Article VII (Simplicity)**: Scripts are simple, single-purpose tools
- **Article IX (Integration-First)**: Scripts validate contracts before implementation

## Scripts Overview

### Core SDD Scripts (sdd/)

| Script | Purpose | Usage |
|--------|---------|-------|
| `create-new-feature.sh` | Initialize new feature with spec | Called by `/specify` |
| `setup-plan.sh` | Bootstrap implementation plan | Called by `/plan` |
| `check-task-prerequisites.sh` | Validate task readiness | Called by `/tasks` |
| `get-feature-paths.sh` | Resolve feature directory paths | Helper utility |
| `update-agent-context.sh` | Sync agent memory files | Memory management |
| `common.sh` | Shared utilities and functions | Sourced by other scripts |

### CI/CD Scripts (ci/)

| Script | Purpose | When Run |
|--------|---------|----------|
| `run-local-ci.sh` | Complete validation suite | Before PR |
| `run-sdd-structure-lint.sh` | Validate SDD structure | Every commit |
| `check-language-policy.sh` | Enforce English normative | Every commit |
| `run-markdown-style.sh` | Markdown linting | Documentation changes |

### SDD Validation Scripts (sdd/)

| Script | Purpose | Validates |
|--------|---------|-----------|
| `validate-sdd-docs.sh` | Check spec/plan/tasks structure | YAML metadata, templates |
| `check_language.sh` | Language policy enforcement | English-only normative |
| `check-markdown.sh` | Markdown quality check | MD format compliance |
| `fix-markdown.sh` | Auto-fix markdown issues | Linting corrections |
| `lint_docs.sh` | Documentation quality | Overall doc standards |
| `run_semantic_checks.sh` | Cross-reference validation | Links and references |

### Metadata Management Scripts (sdd/)

| Script | Purpose | Capabilities |
|--------|---------|-----------|
| `validate-metadata.sh` | Validate YAML metadata | Syntax, structure, required fields |
| `query-metadata.sh` | Query documents by metadata | Filter by type, version, date |
| `check-sdd-consistency.sh` | Check global consistency | Constitution versions, dependencies |
| `migrate-to-yaml-metadata.sh` | Migrate metadata format | Convert to unified YAML format |
| `lib/metadata-utils.sh` | Shared utilities | YAML/JSON parsing functions |

### AST-grep Scripts (ast-grep/)

| Script | Purpose | Analyzes |
|--------|---------|-----------|
| `sg-scan.sh` | Run full codebase scan | All configured rules |
| `sg-scan-file.sh` | Scan specific file | Single file analysis |
| `sg-baseline-acp-rust-dbg.sh` | Check for dbg! macros | Rust debug code |
| `sg-baseline-acp-rust-no-unwrap.sh` | Find unwrap() usage | Rust error handling |
| `sg-baseline-acp-rust-todo.sh` | Find TODO comments | Code debt markers |
| `sg-fix.sh` | Apply auto-fixes | Correctable issues |

## Usage Guidelines

### Security Requirements

```bash
# NEVER echo secrets
❌ echo "Token: $GITHUB_TOKEN"
✅ export GITHUB_TOKEN  # Use environment variables

# NEVER log sensitive data
❌ echo "$response" | tee log.txt
✅ echo "$response" | sed 's/token=.*/token=***/' | tee log.txt
```

### Non-Interactive Execution

```bash
# Prefer non-interactive commands
❌ git add -i
✅ git add specific-file.rs

# Avoid pagers
❌ git log
✅ git log --no-pager
```

### Error Handling

```bash
# Always check exit codes
set -e  # Exit on error
set -u  # Exit on undefined variable
set -o pipefail  # Pipe failures propagate

# Provide clear error messages
if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo not found. Please install Rust." >&2
    exit 1
fi
```

## SDD Automation Details

### Feature Creation Flow

```bash
# 1. scripts/sdd/create-new-feature.sh
# - Determines next feature number (NNN)
# - Creates branch from origin/main
# - Initializes specs/NNN-<slug>/ directory
# - Copies spec template
# - Returns JSON with paths

# Example output:
{
  "BRANCH_NAME": "feature/001-chat-system",
  "SPEC_FILE": "/path/to/specs/001-chat-system/spec.md",
  "FEATURE_NUMBER": "001"
}
```

### Plan Setup Flow

```bash
# 2. scripts/sdd/setup-plan.sh
# - Validates spec exists
# - Copies plan template
# - Creates supporting directories
# - Returns JSON with paths

# Example output:
{
  "FEATURE_SPEC": "/path/to/specs/001-chat-system/spec.md",
  "IMPL_PLAN": "/path/to/specs/001-chat-system/plan.md",
  "SPECS_DIR": "/path/to/specs/001-chat-system",
  "BRANCH": "feature/001-chat-system"
}
```

### CI Validation Flow

```bash
# 3. scripts/ci/run-local-ci.sh
# Executes in order:
1. Rust formatting check
2. Clippy linting
3. Test execution
4. SDD structure validation
5. Language policy check
6. Markdown style check
7. Semantic validation

# Returns consolidated status
```

## Common Patterns

### JSON Output for Tool Integration

```bash
#!/bin/bash
# Scripts should output JSON for tool consumption

output_json() {
    cat <<EOF
{
  "status": "success",
  "data": {
    "key": "value"
  }
}
EOF
}
```

### Path Resolution

```bash
# Always use absolute paths
REPO_ROOT="$(git rev-parse --show-toplevel)"
SPEC_FILE="$REPO_ROOT/specs/$FEATURE_NUMBER-$SLUG/spec.md"
```

### Template Processing

```bash
# Replace placeholders in templates
sed -e "s/\[FEATURE_NAME\]/$FEATURE_NAME/g" \
    -e "s/\[FEATURE_NUMBER\]/$FEATURE_NUMBER/g" \
    template.md > output.md
```

## Integration with SDD Commands

### /specify Command Integration

```bash
# Called by: /specify "feature description" (see ../.specify/commands/specify.md)
scripts/sdd/create-new-feature.sh --json "$FEATURE_DESC"
# Creates: Branch, spec directory, initial spec.md
```

### /plan Command Integration

```bash
# Called by: /plan "technical approach" (see ../.specify/commands/plan.md)
scripts/sdd/setup-plan.sh --json
# Creates: plan.md, data-model.md, contracts/, research.md, quickstart.md
```

### /tasks Command Integration

```bash
# Called by: /tasks "context" (see ../.specify/commands/tasks.md)
scripts/sdd/check-task-prerequisites.sh --json
# Validates prerequisites, then tasks.md is generated
# Creates: tasks.md with executable, ordered task list
```

### /sdd-task Command Integration

```bash
# Called by: /sdd-task <issue-number> (see ../.specify/commands/sdd-task.md)
gh issue view "$1" --json title,body,number,url,state,labels
# Creates: Worktree, branch from issue, triggers full SDD workflow
```

## Metadata Management

### YAML Metadata Format

All SDD documents use embedded YAML metadata at the document footer:

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "claude-memory"  # or sdd-rule, sdd-command, etc.
    path: "./path/to/doc.md"
    version: "1.0.1"
    last_updated: "2025-09-20T08:02:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
```

### Validation Workflows

```bash
# Validate single file metadata
./scripts/sdd/validate-metadata.sh --file CLAUDE.md --verbose

# Validate all SDD documents
./scripts/sdd/validate-metadata.sh

# Generate JSON validation report
./scripts/sdd/validate-metadata.sh --format json > validation-report.json

# Check constitution version consistency
./scripts/sdd/validate-metadata.sh --check-consistency --strict
```

### Querying Documents

```bash
# Find all Claude memory files
./scripts/sdd/query-metadata.sh --type claude-memory

# Find outdated documents (not updated in 30 days)
./scripts/sdd/query-metadata.sh --outdated 30

# Find documents with old constitution version
./scripts/sdd/query-metadata.sh --constitution-version 1.0.0

# Get all documents sorted by date in JSON format
./scripts/sdd/query-metadata.sh --all --sort date --format json
```

### Consistency Checking

```bash
# Full consistency check with details
./scripts/sdd/check-sdd-consistency.sh --verbose

# Generate JSON consistency report
./scripts/sdd/check-sdd-consistency.sh --format json > consistency-report.json

# Check without dependency validation (faster)
./scripts/sdd/check-sdd-consistency.sh --no-dependencies
```

### Metadata Migration

```bash
# Dry run to preview changes
./scripts/sdd/migrate-to-yaml-metadata.sh --dry-run

# Migrate all files
./scripts/sdd/migrate-to-yaml-metadata.sh

# Migrate specific file
./scripts/sdd/migrate-to-yaml-metadata.sh --file sdd-rules/AGENTS.md
```

### Integration with CI

The metadata validation is integrated into the CI pipeline:

```bash
# Run as part of local CI
./scripts/ci/run-local-ci.sh

# Or run metadata checks standalone
./scripts/sdd/validate-metadata.sh && \
./scripts/sdd/check-sdd-consistency.sh
```

### Common Use Cases

1. **Before PR submission**: Validate all metadata

   ```bash
   ./scripts/sdd/check-sdd-consistency.sh --verbose
   ```

2. **Find specific document types**: Query by type

   ```bash
   ./scripts/sdd/query-metadata.sh --type sdd-rule --format paths
   ```

3. **Update constitution version globally**: After constitution update

   ```bash
   # First validate current state
   ./scripts/sdd/validate-metadata.sh --check-consistency
   # Then migrate if needed
   ./scripts/sdd/migrate-to-yaml-metadata.sh
   ```

4. **Track document freshness**: Find stale documents

   ```bash
   ./scripts/sdd/query-metadata.sh --outdated 7 --format json | \
     jq '.results[] | {path: .path, last_updated: .last_updated}'
   ```

## AST-grep Integration

### Code Analysis with sgconfig.yml

```bash
# Run full scan with all rules
./scripts/ast-grep/sg-scan.sh

# Check specific issues
./scripts/ast-grep/sg-baseline-acp-rust-no-unwrap.sh  # Find unwrap() calls
./scripts/ast-grep/sg-baseline-acp-rust-dbg.sh        # Find dbg! macros
./scripts/ast-grep/sg-baseline-acp-rust-todo.sh       # Find TODO comments

# Scan individual file
./scripts/ast-grep/sg-scan-file.sh src/main.rs

# Apply automatic fixes
./scripts/ast-grep/sg-fix.sh
```

### Evidence Collection

```bash
# Collect AST-grep evidence (primary location)
ast-grep scan -c sgconfig.yml . 2>&1 | tee _artifacts/reports/<task>/ast_grep_$(date +%Y%m%d_%H%M%S).log

# Legacy location
ast-grep scan -c sgconfig.yml . 2>&1 | tee _artifacts/reports/legacy/<task>/ast_grep_$(date +%Y%m%d_%H%M%S).log
```

## Quick Reference

### Running Scripts

```bash
# From repo root
./scripts/sdd/create-new-feature.sh "Chat system"
./scripts/ci/run-local-ci.sh
./scripts/sdd/validate-sdd-docs.sh

# Run shell-based validation
./scripts/sdd/validate-sdd-docs.sh

# With environment variables
SKIP_TESTS=1 ./scripts/ci/run-local-ci.sh
```

### Script Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Missing dependencies |
| 3 | Invalid arguments |
| 4 | SDD validation failure |
| 5 | Quality gate failure |

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `REPO_ROOT` | Repository root path | Auto-detected |
| `SKIP_TESTS` | Skip test execution | false |
| `VERBOSE` | Verbose output | false |
| `CI` | Running in CI environment | false |

## Troubleshooting

### Common Issues

1. **Script not executable**

   ```bash
   chmod +x scripts/*.sh
   chmod +x scripts/**/*.sh
   ```

2. **Path issues**

   ```bash
   # Always run from repo root
   cd $(git rev-parse --show-toplevel)
   ./scripts/script.sh
   ```

3. **Missing dependencies**

   ```bash
   # Check required tools
   command -v cargo || echo "Install Rust"
   command -v jq || echo "Install jq"
   command -v yq || echo "Install yq (for metadata tools)"
   command -v ast-grep || echo "Install ast-grep"
   ```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "claude-memory"
    path: "./scripts/CLAUDE.md"
    version: "1.0.1"
    last_updated: "2025-09-20T08:02:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - "sdd-rules/rules/README.md"
        - ".specify/templates/spec-template.md"
        - ".specify/templates/plan-template.md"
        - ".specify/templates/tasks-template.md"
        - "./CLAUDE.md"
```
