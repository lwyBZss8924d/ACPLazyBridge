# SDD Rules - Tools - CLI - ast-grep

This document provides comprehensive guidelines for using `ast-grep` within
the SDD workflow. ast-grep is a powerful AST-based structural code search and
transformation tool that enables language-aware pattern matching, refactoring,
and code analysis.

## ast-grep CLI Help

```bash
ast-grep --help
Search and Rewrite code at large scale using AST pattern.
Usage: ast-grep [OPTIONS] <COMMAND>
Commands:
  run          Run one time search or rewrite in command line. (default command)
  scan         Scan and rewrite code by configuration
  test         Test ast-grep rules
  new          Create new ast-grep project or items like rules/tests
  lsp          Start language server
  completions  Generate shell completion script
  help         Print this message or the help of the given subcommand(s)
Options:
  -c, --config <CONFIG_FILE>
          Path to ast-grep root config, default is sgconfig.yml
  -h, --help
          Print help (see a summary with '-h')
  -V, --version
          Print version
```

## Core Concepts

### AST-Based Matching

ast-grep operates on Abstract Syntax Trees (ASTs) rather than text, providing:

- **Semantic accuracy**: Matches code structure, not text patterns
- **Language awareness**: Understands syntax and semantics
- **Whitespace agnostic**: Ignores formatting differences
- **Type-safe transformations**: Preserves code validity

### Pattern Syntax

#### Basic Patterns

```bash
# Literal match
ast-grep --pattern 'console.log("debug")' --lang js
# Single wildcard ($_)
ast-grep --pattern 'console.log($_)' --lang js
# Multiple wildcards ($$$)
ast-grep --pattern 'function $FUNC($$$ARGS) { $$$BODY }' --lang js
```

#### Metavariable Types

| Metavar | Matches | Example |
|---------|---------|----------|
| `$NAME` | Single node | `$VAR = 42` |
| `$$NAME` | Zero or more nodes (list) | `func($$ARGS)` |
| `$$$NAME` | Zero or more nodes (statement) | `{ $$$STMTS }` |
| `$_` | Anonymous single node | `array[$_]` |
| `$$_` | Anonymous list | `func($$_)` |
| `$$$_` | Anonymous statements | `if (cond) { $$$_ }` |

## Command-Line Usage

### Basic Search Operations

```bash
# Note: 'run' is the default command and can be omitted
# These are equivalent:
ast-grep run --pattern 'print($_)' --lang python
ast-grep --pattern 'print($_)' --lang python
# Short flags are also available
ast-grep -p 'console.log($_)' -l js
# Search with pattern (Python)
ast-grep -p 'def $FUNC($$$PARAMS):' -l python scripts/
# Search specific files
ast-grep -p 'async function $NAME' -l ts src/**/*.ts
# Output formats
ast-grep -p 'println!($$$)' -l rust --json
ast-grep -p 'fmt.Println($_)' -l go --format compact
```

### Advanced Search Features

```bash
# Context lines
ast-grep -p 'throw $_' -l js \
  --before 2 --after 2
# Limit results
ast-grep -p 'console.log' -l js \
  --max-results 10
# Interactive mode for selective changes
ast-grep -p 'print($_)' -r 'logger.info($_)' -l python \
  --interactive
# Debug query (shows AST structure)
ast-grep -p 'class $NAME' -l python \
  --debug-query=ast  # or 'pattern' or 'cst'
# Use selector for sub-pattern matching
ast-grep -p 'if ($COND) { $$$BODY }' \
  --selector 'binary_expression' -l js
```

## Rule Configuration

### Files globs precedence (important)

- In rule YAML, `files:` should include at least one positive include (e.g. `"**/*.rs"`) before any negative excludes (e.g. `"!**/tests/**"`).
- Negative-only lists may result in zero matches.
- Prefer scoping at rule-level over global ignores when you want per-rule precision.
- For "non-test only" Rust rules, prefer scoping to `src/**/*.rs` in typical crates. For repos that keep Rust files at the package root (no src/), use `**/*.rs` with excludes like `*_test.rs`, `*tests.rs`, `tests/**`, and `benches/**`.

**⚠️ CRITICAL LIMITATION (Discovered in Issue #34):**

- **When rules are loaded via `ruleDirs` in sgconfig.yml, the `files:` field in individual rule YAML files is COMPLETELY IGNORED**
- This is an undocumented ast-grep limitation that affects all file exclusion patterns
- The `ignores` section in sgconfig.yml only affects file traversal, NOT rule application
- **Solution**: Use suppression comments instead of file patterns (see section below)

### YAML Rule Format

```yaml
# rule.yml
id: no-console-log
language: javascript
severity: warning
message: Remove console.log statements
rule:
  pattern: console.log($$$ARGS)
fix: ''
metadata:
  category: cleanup
  tags:
    - production
    - logging
```

### Complex Rule Examples

#### Pattern with Constraints

```yaml
id: find-hooks
language: javascript
message: Found React Hook
rule:
  pattern: $HOOK($$$ARGS)
constraints:
  HOOK:
    regex: '^use[A-Z]'
```

#### Security: SQL Injection Detection

```yaml
id: sql-injection-risk
language: python
severity: error
message: Potential SQL injection vulnerability
rule:
  all:
    - pattern: cursor.execute($QUERY)
    - any:
        - has:
            pattern: $QUERY + $_
        - has:
            pattern: f"{$$$}"
    - not:
        inside:
          kind: function_definition
          has:
            field: name
            regex: '^test_'
fix: |
  # Use parameterized query
  cursor.execute($QUERY, params)
```

#### Refactoring: Modernize Code

```yaml
id: use-optional-chaining
language: typescript
message: Use optional chaining operator
rule:
  pattern: $OBJ && $OBJ.$PROP
fix: '$OBJ?.$PROP'
constraints:
  OBJ:
    regex: '^[a-zA-Z_][a-zA-Z0-9_]*$'
```

#### Using Kind with Pattern

```yaml
id: find-test-functions
language: go
message: Found test function
rule:
  kind: function_declaration
  has:
    field: name
    regex: '^Test'
```

#### Performance: Optimize Loops

```yaml
id: optimize-array-includes
language: javascript
message: Use Set for better performance
rule:
  inside:
    pattern: for ($_ of $ARRAY) { $$$BODY }
  pattern: $ARRAY.includes($_)
fix: |
  // Consider using Set for O(1) lookup:
  // const set = new Set($ARRAY);
  // set.has($_)
  $ARRAY.includes($_)
```

## Suppression Comments (Critical for Test Code)

Due to the file pattern limitation with `ruleDirs`, suppression comments are the ONLY reliable way to exclude test code from warnings.

### Syntax and Requirements

```bash
# Basic suppression - suppresses ALL rules for next line
// ast-grep-ignore

# Rule-specific suppression
// ast-grep-ignore: rust-no-unwrap

# Multiple rules
// ast-grep-ignore: rust-no-unwrap, rust-mutex-lock
```

**⚠️ CRITICAL REQUIREMENTS:**

- Suppression comments MUST be on the line immediately before the code
- File-level and module-level suppressions DO NOT cascade to nested functions
- Each occurrence needs its own suppression comment

### Examples for Test Files

```rust
// DOESN'T WORK - module-level suppression doesn't cascade
#[cfg(test)]
mod tests {
    // ast-grep-ignore: rust-no-unwrap  // This won't suppress unwrap in test functions!
    use super::*;

    #[test]
    fn test_something() {
        let result = Some(42).unwrap(); // Still shows warning!
    }
}

// CORRECT - suppression on each line
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // ast-grep-ignore
        let result = Some(42).unwrap(); // No warning
    }
}
```

### Automated Suppression Script

For existing codebases with many test files, use this Python script:

```python
#!/usr/bin/env python3
import sys
import re

def add_suppressions(file_path):
    with open(file_path, 'r') as f:
        lines = f.readlines()

    pattern = re.compile(r'\.unwrap\(\)|\.expect\(')
    result = []

    for i, line in enumerate(lines):
        if pattern.search(line) and (i == 0 or '// ast-grep-ignore' not in lines[i-1]):
            indent = len(line) - len(line.lstrip())
            result.append(' ' * indent + '// ast-grep-ignore\n')
        result.append(line)

    with open(file_path, 'w') as f:
        f.writelines(result)

if __name__ == "__main__":
    for file_path in sys.argv[1:]:
        add_suppressions(file_path)
```

## Transformation and Refactoring

### Basic Transformations

```bash
# Simple replacement (short flags)
ast-grep -p 'var $NAME = $_' \
  -r 'const $NAME = $_' \
  -l js
# Interactive mode for selective changes
ast-grep -p 'print($MSG)' \
  -r 'logger.info($MSG)' \
  -l python \
  --interactive
# Apply to specific files
ast-grep -p 'assertEquals' \
  -r 'assert.equal' \
  -l js \
  test/**/*.js
# Using stdin/stdout
echo 'console.log("test")' | \
  ast-grep -p 'console.log($_)' -r 'debug($_)' -l js --stdin
```

### Complex Transformations

```bash
# Function to arrow function
ast-grep --pattern 'function $NAME($$$PARAMS) { return $EXPR }' \
  --rewrite 'const $NAME = ($$$PARAMS) => $EXPR' \
  --lang js
# Add type annotations
ast-grep --pattern 'def $FUNC($PARAM):' \
  --rewrite 'def $FUNC($PARAM: Any) -> None:' \
  --lang python
# Extract to variable
ast-grep --pattern 'if ($COND) { $$$BODY }' \
  --rewrite 'const condition = $COND;\nif (condition) { $$$BODY }' \
  --lang ts
```

## Testing and Validation

### Test Directory Structure

```bash
my-project/
  ├── rules/
  │   └── no-console.yml
  ├── rule-tests/
  │   └── no-console-test.yml
  └── sgconfig.yml
```

### Test Configuration

```yaml
# rule-tests/test-rule.yml
id: test-rule
language: python
rule:
  pattern: assert $ACTUAL == $EXPECTED
fix: assert_equal($ACTUAL, $EXPECTED)
tests:
  - name: basic assertion
    valid:
      - assert_equal(1, 1)
      - self.assertEqual(a, b)
    invalid:
      - assert x == y
      - assert result == expected
```

### Running Tests

```bash
# Test all rules in test directory (default: rule-tests/)
ast-grep test
# Test specific directory
ast-grep test -t tests/
# Filter tests by regex
ast-grep test -f 'console'
# Interactive snapshot update
ast-grep test -i
# Update all snapshots
ast-grep test -U
# Skip snapshot validation
ast-grep test --skip-snapshot-tests
```

## Project Configuration

### sgconfig.yml

```yaml
# Project-wide configuration
ruleDirs:
  - rules
  - security-rules
testDirs:
  - tests
files:
  - "src/**/*.ts"
  - "!src/**/*.test.ts"
  - "!node_modules"
language:
  ts: typescript
  tsx: tsx
  js: javascript
  jsx: jsx
ruleConfig:
  severity:
    default: warning
    security: error
  ignoreRules:
    - no-console-log
    - prefer-const
output:
  format: github  # github, sarif, json, compact
  reportFile: ast-grep-report.json
```

### Custom Language Support

```yaml
# Add custom language configuration
languages:
  vue:
    extensions: [.vue]
    parser: tree-sitter-vue
    scopeQuery: |
      (script_element
        (raw_text) @script)
```

## CI/CD Integration

### GitHub Actions

```yaml
name: AST-Grep Analysis
on: [push, pull_request]
jobs:
  ast-grep:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install ast-grep
        run: npm install -g @ast-grep/cli
      - name: Run security scan
        run: |
          ast-grep scan --rule-dir security-rules \
            --format sarif \
            --output results.sarif
      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: results.sarif
```

### Pre-commit Hook

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: ast-grep
        name: ast-grep
        entry: ast-grep scan
        language: system
        files: '\.(js|ts|py)$'
        pass_filenames: false
```

## API and Programmatic Usage

### Node.js API

```javascript
import { parse, findAll } from '@ast-grep/napi';
const code = 'console.log("hello"); console.error("error")';
const ast = parse('javascript', code);
const matches = findAll(ast, {
  pattern: 'console.$METHOD($_)'
});
matches.forEach(match => {
  console.log(`Found: ${match.text()}`);
  console.log(`Method: ${match.getMatch('METHOD').text()}`);
});
```

### Python API

```python
from ast_grep_py import SgRoot, Pattern
code = """def hello(): print("world")"""
root = SgRoot(code, "python")
pattern = Pattern('print($_)')
matches = root.find_all(pattern)
for match in matches:
    print(f"Found print at: {match.range()}")
    print(f"Content: {match.text()}")
```

## Editor Integration

### VS Code Extension

```json
// .vscode/settings.json
{
  "ast-grep.rules": [
    "./rules/**/*.yml"
  ],
  "ast-grep.languageMap": {
    "vue": "typescript"
  },
  "ast-grep.formatOnSave": true
}
```

### Neovim Plugin

```lua
-- nvim configuration
require('ast-grep').setup({
  cmd = {'ast-grep', 'lsp'},
  filetypes = {'javascript', 'typescript', 'python'},
  root_dir = require('lspconfig.util').root_pattern('sgconfig.yml')
})
```

## Performance Optimization

### Parallel Processing

```bash
# Use multiple threads
ast-grep --pattern '$_' --lang python -j 8
# Process large codebases
fd -e py -x ast-grep --pattern 'import $_' --lang python {} \;
# Batch processing
find . -name "*.js" -print0 | \
  xargs -0 -P 4 -n 50 ast-grep --pattern 'await $_'
```

### Caching Strategies

```bash
# Process files in parallel
ast-grep -p '$_' -l python -j 8
# Batch processing with fd
fd -e py -x ast-grep -p 'import $_' -l python {} \;
# Note: Incremental analysis is planned but not yet available
# Future: ast-grep scan --incremental --since HEAD~1
```

## Common Patterns Library

### Error Handling

```yaml
# Catch empty catch blocks (JavaScript)
rule:
  pattern: try { $$$TRY } catch($_) {}
# Find unhandled promises (JavaScript)
rule:
  all:
    - pattern: $PROMISE($$$)
    - not:
        has:
          pattern: await $PROMISE($$$)
constraints:
  PROMISE:
    regex: '^(fetch|axios|.*Async)$'
```

### Security

```yaml
# Detect eval usage
pattern: eval($_)
# Find hardcoded secrets
pattern: |
  $KEY = "$SECRET"
constraints:
  KEY:
    regex: '.*(password|token|secret|key).*'
  SECRET:
    regex: '^[A-Za-z0-9+/]{20,}.*'
```

### Performance

```yaml
# N+1 query detection
pattern: |
  for $ITEM in $COLLECTION:
    $$$
    $ITEM.$RELATION.all()
# Inefficient list comprehension
pattern: |
  [x for x in $LIST if $COND][0]
fix: next((x for x in $LIST if $COND), None)
```

## Rule Development Checklist (CLI-only)

1) Clarify intent and scope; break down complex queries into sub-rules
2) Author minimal examples (match / non-match)
3) Start atomic (kind or pattern), then add relational rules with `stopBy: end`
4) Inspect AST/pattern with `--debug-query` for ambiguous cases
5) Add constraints and composite rules incrementally
6) Validate with rule-tests/ (valid/invalid)
7) Produce streaming JSON evidence grouped by file; declare scope explicitly

### CLI mapping for development

- AST dump: `--debug-query=ast` (or `pattern`/`cst`)
- Rule unit tests: `ast-grep test` (rule-tests/*)
- Iterative scans: rules/<lang>/…, then `scan -c sgconfig.yml --filter ... --globs ...`

## Quick Reference

### Essential Commands

```bash
# Search patterns
ast-grep -p 'pattern' -l lang [files]
# Replace patterns
ast-grep -p 'old' -r 'new' -l lang
# Run rules from YAML
ast-grep scan --rule rule.yml
# Test rules
ast-grep test -t test-dir/
# Create new project
ast-grep new project
# Start LSP server
ast-grep lsp
```

### Language Codes

| Language | Code | Extensions |
|----------|------|------------|
| Python | `python` | `.py` |
| JavaScript | `js` | `.js` |
| TypeScript | `ts` | `.ts` |
| Rust | `rust` | `.rs` |
| Go | `go` | `.go` |
| Java | `java` | `.java` |
| C++ | `cpp` | `.cpp`, `.cc` |
| Ruby | `ruby` | `.rb` |

## Best Practices

### Pattern Design

1. **Start specific, generalize later**

   ```bash
   # Too broad
   ast-grep --pattern '$_'
   # Better
   ast-grep --pattern 'console.$METHOD($$$)'
   ```

2. **Use constraints for precision**

   ```yaml
   pattern: $FUNC($$$)
   constraints:
     FUNC:
       regex: '^(eval|exec|compile)$'
   ```

3. **Combine patterns with logic**

   ```yaml
   all:
     - pattern: if ($COND) { $$$THEN }
     - not:
         pattern: else { $$$ELSE }
   ```

### Performance Tips

- Use file filters to reduce scope
- Leverage incremental analysis for large repos
- Cache results when running multiple queries
- Use parallel processing for independent operations

### Integration Guidelines

- Run ast-grep in pre-commit hooks for immediate feedback
- Include in CI/CD for automated quality gates
- Generate SARIF reports for GitHub integration
- Use rule severity levels appropriately

## Real-World Examples

### Issue #34: Test File False Positives

This real-world case demonstrates the file pattern limitation and workaround:

```yaml
# This DOESN'T WORK when rule is loaded via ruleDirs:
# sdd-rules/rules/code-analysis/ast-grep/rust/no-unwrap.yml
id: rust-no-unwrap
files:
  - "**/*.rs"
  - "!**/tests/**"  # ❌ This exclusion is IGNORED!
rule:
  pattern: $EXPR.unwrap()
```

**Problem**: 85+ false positives in test files despite exclusion patterns.

**Root Cause**: When loaded via `ruleDirs` in sgconfig.yml, the `files:` field is ignored.

**Solution Applied**:

```rust
// Add suppression comments to each test file
#[test]
fn test_something() {
    // ast-grep-ignore
    let result = Some(42).unwrap();  // ✅ No warning
}
```

**Result**: 100% reduction in false positives (85 → 0 warnings).

### OpenAI SDK Migration

```yaml
# Migrate from v0 to v1
id: migrate-openai-client
language: python
rule:
  pattern: openai.api_key = $KEY
fix: |
  from openai import Client
  client = Client($KEY)
```

### React Hooks Detection

```yaml
id: detect-hooks
language: typescript
rule:
  all:
    - pattern: $FUNC($$$)
    - inside:
        kind: function_declaration
constraints:
  FUNC:
    regex: '^use[A-Z]'
```

### XState v5 Migration

```bash
# Migrate Machine to createMachine
ast-grep -p 'Machine($CONFIG)' \
  -r 'createMachine($CONFIG)' \
  -l typescript --interactive
# Update imports
ast-grep -p "import { Machine } from 'xstate'" \
  -r "import { createMachine } from 'xstate'" \
  -l typescript
```

## Troubleshooting

### Common Issues

```bash
# Debug pattern matching
ast-grep -p 'class $_' -l python --debug-query=ast
# Show pattern structure
ast-grep -p 'def $F(): $$$' -l python --debug-query=pattern
# Show CST (concrete syntax tree) with all tokens
ast-grep -p 'if True: pass' -l python --debug-query=cst
```

### Error Messages and Solutions

| Error | Cause | Solution |
|-------|-------|----------|
| "Pattern parse error" | Invalid pattern syntax | Check metavariable usage ($ for single, $$ for list, $$$ for statements) |
| "No matches found" | Pattern too specific | Use $_ for wildcards, check language syntax |
| "Language not supported" | Wrong language code | Check https://ast-grep.github.io/reference/languages.html |
| "Rule must specify AST kinds" | Missing pattern/kind in YAML | Add `pattern:` or `kind:` to rule |
| "Cannot parse rule" | Invalid YAML structure | Check indentation, constraints placement |
| "unexpected argument found" | Wrong command syntax | Remember `run` is default, check subcommand help |

### Common Issues from Real-World Usage (Issue #34)

| Issue | Root Cause | Solution |
|-------|------------|----------|
| File exclusions not working | Rules loaded via `ruleDirs` ignore `files:` patterns | Use suppression comments instead |
| Test files still showing warnings | File patterns in rules don't work with sgconfig.yml | Add `// ast-grep-ignore` before each occurrence |
| Suppressions not working | Module-level suppressions don't cascade | Must add suppression on line immediately before code |
| sgconfig.yml `ignores` not excluding from rules | `ignores` only affects traversal, not rule application | Use suppression comments or `--rule-file` directly |
| Too many false positives in tests | Can't exclude test directories via config | Automate adding suppressions with script |

### Verification Commands

```bash
# Verify suppressions are working
ast-grep scan -c sgconfig.yml --filter '^rust-no-unwrap$' . | grep -c warning

# Test rule file directly (bypasses ruleDirs limitation)
ast-grep scan --rule-file path/to/rule.yml .

# Compare rule behavior with/without sgconfig.yml
ast-grep scan --rule-file rules/no-unwrap.yml . | wc -l  # Direct rule
ast-grep scan -c sgconfig.yml --filter 'no-unwrap' . | wc -l  # Via config
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-19T04:32:00Z"
rules:
    name: "tools-cli-astgrep"
    category: "tools-cli"
    version: "1.0.2"
document:
    type: "sdd-rule"
    path: "sdd-rules/rules/tools-cli/sdd-rules-tools-cli-astgrep.md"
    last_updated: "2025-09-19T13:00:00Z"
    changelog: "Added critical limitations discovered in Issue #34 regarding ruleDirs and file patterns"
    related:
        - "sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md"
        - "sdd-rules/rules/tools-cli/sdd-rules-tools-cli-list.md"
        - "sdd-rules/rules/tools-cli/ast-grep.llms.txt"
```
