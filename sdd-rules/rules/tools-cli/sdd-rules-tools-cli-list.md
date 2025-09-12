# SDD Rules - Tools - CLI - Index

This document provides a comprehensive index of CLI tools used by AI engineers
in the SDD workflow. Each tool is categorized by its primary function and
includes usage patterns optimized for non-interactive execution.

## Tool Categories

### Code Analysis & Search

#### ast-grep

- **Purpose**: AST-based structural code search and transformation
- **Documentation**: [sdd-rules-tools-cli-astgrep.md](./sdd-rules-tools-cli-astgrep.md)
- **Key Commands**:
  - `ast-grep --pattern '$FUNC($$$ARGS)' --lang python`
  - `ast-grep run --rule rule.yml --json`
  - `ast-grep scan --format sarif`
- **Use Cases**: Pattern matching, refactoring, security scanning

#### ripgrep (rg)

- **Purpose**: Fast text-based recursive search
- **Key Commands**:
  - `rg -t py "def.*test" --json`
  - `rg -l "TODO" --glob "!node_modules"`
- **Use Cases**: Text search, TODO tracking, quick grepping

#### fd

- **Purpose**: Fast file and directory finder
- **Key Commands**:
  - `fd -e py -x ast-grep --pattern '$_' --lang python {}`
  - `fd -t f -0 | xargs -0 wc -l`
- **Use Cases**: File discovery, batch operations

### Code Quality & Linting

#### markdownlint-cli2

- **Purpose**: Markdown style enforcement
- **Documentation**:
  [sdd-rules-documentation-markdownlint.md](../documentation-style/sdd-rules-documentation-markdownlint.md)
- **Key Commands**:
  - `markdownlint-cli2 "**/*.md" --config .markdownlint.json`
  - `markdownlint-cli2-fix "**/*.md"`
- **Use Cases**: Documentation quality, style consistency

#### shellcheck

- **Purpose**: Shell script analysis
- **Key Commands**:
  - `shellcheck -f json scripts/*.sh`
  - `shellcheck -S error -e SC2086`
- **Use Cases**: Script validation, security checks

### Testing & Validation

#### cargo (Rust)

- **Purpose**: Rust build and test orchestration
- **Key Commands**:
  - `cargo test --workspace --all-features --locked`
  - `cargo clippy -- -D warnings`
  - `cargo fmt --all -- --check`
- **Use Cases**: Build validation, test execution

#### pytest

- **Purpose**: Python testing framework
- **Key Commands**:
  - `pytest -v --json-report --json-report-file=report.json`
  - `pytest --cov=src --cov-report=json`
- **Use Cases**: Unit testing, coverage analysis

### Version Control

#### git

- **Purpose**: Source control management
- **Key Commands**:
  - `git log --oneline --format=json`
  - `git diff --staged --stat`
- **Use Cases**: History analysis, change tracking

#### gh (GitHub CLI)

- **Purpose**: GitHub operations
- **Key Commands**:
  - `gh pr list --json number,title,state`
  - `gh issue create --title "$TITLE" --body-file spec.md`
- **Use Cases**: PR management, issue tracking

### Data Processing

#### jq

- **Purpose**: JSON processor
- **Key Commands**:
  - `cat results.json | jq '.matches[] | .file'`
  - `jq -s 'group_by(.severity) | map({severity: .[0].severity, count: length})'`
- **Use Cases**: Result parsing, data transformation

#### yq

- **Purpose**: YAML processor
- **Key Commands**:
  - `yq eval '.rules[] | select(.severity == "error")' rules.yml`
  - `yq -o json config.yml`
- **Use Cases**: Config processing, YAML to JSON conversion

## Integration Patterns

### Pipeline Composition

```bash
# Find Python files → Extract functions → Count complexity
fd -e py | xargs -I{} ast-grep --pattern 'def $FUNC($$$):' --lang python {} \
  | jq -s 'length'

# Search patterns → Filter results → Generate report
ast-grep scan --format json | jq '.results[] | select(.severity == "error")' \
  | tee errors.json
```

### Evidence Collection

```bash
# Capture analysis with timestamps
ast-grep scan --format sarif 2>&1 \
  | tee "dev-docs/review/_artifacts/$(date +%Y%m%d_%H%M%S)_scan.sarif"

# Aggregate multiple tool outputs
{
  echo '{"timestamp": "'$(date -Iseconds)'",'
  echo '"ast_grep":' && ast-grep scan --format json
  echo ',"markdownlint":' && markdownlint-cli2 "**/*.md" --json
  echo '}'
} > analysis.json
```

### Batch Operations

```bash
# Apply transformation to all matching files
ast-grep --pattern '$OLD' --rewrite '$NEW' --lang python \
  $(fd -e py)

# Validate all shell scripts
fd -e sh -x shellcheck -f json {} \; \
  | jq -s 'flatten | group_by(.file)'
```

## Tool Selection Guidelines

### When to Use ast-grep vs ripgrep

- **Use ast-grep when**:
  - Searching for code structures (functions, classes, patterns)
  - Performing safe refactoring
  - Analyzing code semantics
  - Need language-aware matching

- **Use ripgrep when**:
  - Searching for text literals
  - Quick file content scanning
  - Cross-language text search
  - Performance is critical

### When to Use Native vs Wrapped Commands

- **Use native commands when**:
  - Tool has built-in JSON output
  - Need specific tool features
  - Performance matters

- **Use wrapped/piped commands when**:
  - Combining multiple tools
  - Transforming output format
  - Creating evidence trails

## Performance Considerations

### Parallel Execution

```bash
# Use GNU parallel for CPU-bound operations
fd -e py | parallel -j+0 'ast-grep --pattern "TODO" --lang python {}'

# Use xargs for I/O-bound operations
fd -e md -0 | xargs -0 -n100 markdownlint-cli2
```

### Output Size Management

```bash
# Limit output size for large codebases
ast-grep scan --max-results 1000 --format json

# Stream processing for memory efficiency
ast-grep scan --format jsonl | head -n 1000 | jq -c '.'
```

## Common Troubleshooting

### Exit Codes

Most tools follow standard conventions:

- `0`: Success
- `1`: General error or findings
- `2`: Misuse of command
- `127`: Command not found

### Debugging Commands

```bash
# Verbose output
ast-grep --debug-query --pattern '$_'

# Dry run
ast-grep --pattern '$OLD' --rewrite '$NEW' --dry-run

# Version check
ast-grep --version && rg --version && fd --version
```

---

specification_version: 1.0.5 | sdd-rules-tools-cli-list.md Format: 1.1 |
Last Updated: 2025-09-12
