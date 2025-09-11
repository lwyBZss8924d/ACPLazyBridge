# CLAUDE.md (scripts/)

## Authority

- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- SDD Commands: ../sdd-rules/commands/{specify,plan,tasks}.md

## Purpose

Automation scripts for SDD workflow, CI/CD, and development tasks. These scripts implement the core SDD automation that drives the specification-driven development process.

## Scripts Overview

### Core SDD Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| `create-new-feature.sh` | Initialize new feature with spec | Called by `/specify` |
| `setup-plan.sh` | Bootstrap implementation plan | Called by `/plan` |
| `check-task-prerequisites.sh` | Validate task readiness | Pre-execution checks |
| `update-agent-context.sh` | Sync agent memory files | Memory management |

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
| `validate_structure.py` | Check spec/plan/tasks structure | Directory layout |
| `check_language.sh` | Language policy enforcement | Normative artifacts |
| `lint_docs.sh` | Documentation quality | Markdown format |
| `run_semantic_checks.sh` | Cross-reference validation | Links and references |

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
# 1. create-new-feature.sh
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
# 2. setup-plan.sh
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
# 3. run-local-ci.sh
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
# Called by: /specify "feature description"
scripts/create-new-feature.sh --json "$FEATURE_DESC"
# Creates: Branch, spec directory, initial spec.md
```

### /plan Command Integration

```bash
# Called by: /plan "technical approach"
scripts/setup-plan.sh --json
# Creates: plan.md, data-model.md, contracts/, research.md
```

### /tasks Command Integration

```bash
# Called after plan exists
# Derives tasks from plan and contracts
# Creates: tasks.md with executable task list
```

## Quick Reference

### Running Scripts

```bash
# From repo root
./scripts/create-new-feature.sh "Chat system"
./scripts/ci/run-local-ci.sh
./scripts/sdd/validate_structure.py

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
   ```

---

Specification Version: 1.0.3 | scripts/CLAUDE.md Format: 1.0 | Last Updated: 2025-09-11
