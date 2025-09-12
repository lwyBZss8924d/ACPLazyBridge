# SDD Rules - Tools - CLI - ast-grep

This document provides comprehensive guidelines for using `ast-grep` within
the SDD workflow. ast-grep is a powerful AST-based structural code search and
transformation tool that enables language-aware pattern matching, refactoring,
and code analysis.

## Installation and Setup

### Installation Methods

```bash
# Cargo (Rust)
cargo install ast-grep --locked

# Homebrew (macOS/Linux)
brew install ast-grep

# npm (Node.js)
npm install -g @ast-grep/cli

# Pre-built binaries
curl -fsSL https://get.ast-grep.com | sh
```

### Version Verification

```bash
# Check installation
ast-grep --version

# Verify language support
ast-grep --lang list
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
# Search with pattern
ast-grep --pattern 'TODO' --lang python

# Search with regex filter
ast-grep --pattern '$FUNC($$$)' --lang python \
  --filter 'FUNC: "test_.*"'

# Search specific files
ast-grep --pattern 'async function $NAME' --lang ts \
  src/**/*.ts

# Output formats
ast-grep --pattern '$_' --lang rust --json
ast-grep --pattern '$_' --lang rust --format compact
```

### Advanced Search Features

```bash
# Context lines
ast-grep --pattern 'throw $_' --lang js \
  --before 2 --after 2

# Limit results
ast-grep --pattern 'console.log' --lang js \
  --max-results 10

# Interactive mode
ast-grep --pattern '$_' --lang python --interactive

# Debug query
ast-grep --pattern 'class $NAME' --lang python \
  --debug-query
```

## Rule Configuration

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

#### Security: SQL Injection Detection

```yaml
id: sql-injection-risk
language: python
severity: error
message: Potential SQL injection vulnerability

rule:
  all:
    - pattern: |
        cursor.execute($QUERY, $$$ARGS)
    - any:
        - pattern: $QUERY
          regex: '.*\+.*'
        - pattern: $QUERY
          regex: '.*%.*'
    - not:
        inside:
          pattern: |
            def $TEST_FUNC($$$):
              $$$
          regex:
            TEST_FUNC: '^test_'

fix: |
  # Use parameterized query
  cursor.execute($QUERY, $$$ARGS)
```

#### Refactoring: Modernize Code

```yaml
id: use-optional-chaining
language: typescript
message: Use optional chaining operator

rule:
  pattern: |
    $OBJ && $OBJ.$PROP

fix: '$OBJ?.$PROP'

constraints:
  OBJ:
    regex: '^[a-zA-Z_][a-zA-Z0-9_]*$'
```

#### Performance: Optimize Loops

```yaml
id: optimize-array-includes
language: javascript
message: Use Set for better performance

rule:
  inside:
    pattern: |
      for ($_ of $ARRAY) {
        $$$
      }
  pattern: $ARRAY.includes($_)

fix: |
  // Consider using Set for O(1) lookup:
  // const set = new Set($ARRAY);
  // set.has($_)
  $ARRAY.includes($_)
```

## Transformation and Refactoring

### Basic Transformations

```bash
# Simple replacement
ast-grep --pattern 'var $NAME = $_' \
  --rewrite 'const $NAME = $_' \
  --lang js

# Dry run (preview changes)
ast-grep --pattern 'print($MSG)' \
  --rewrite 'logger.info($MSG)' \
  --lang python \
  --dry-run

# Apply to specific files
ast-grep --pattern 'assertEquals' \
  --rewrite 'assert.equal' \
  --lang js \
  test/**/*.js
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

### Test Configuration

```yaml
# test-rule.yml
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
# Test single rule
ast-grep test rule.yml

# Test all rules in directory
ast-grep test rules/

# Verbose output
ast-grep test --verbose rule.yml
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
# Cache parsed ASTs
export AST_GREP_CACHE_DIR=/tmp/ast-grep-cache
ast-grep --pattern '$_' --cache

# Incremental analysis
ast-grep scan --incremental --since HEAD~1
```

## Common Patterns Library

### Error Handling

```yaml
# Catch empty catch blocks
pattern: |
  try {
    $$$
  } catch($_) {}

# Find unhandled promises
pattern: |
  $PROMISE()
  not: await $PROMISE()
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

## Troubleshooting

### Common Issues

```bash
# Debug pattern matching
ast-grep --pattern 'class $_' --debug-query

# Show AST structure
ast-grep --pattern '$_' --lang python --show-ast file.py

# Validate rule syntax
ast-grep test --validate-only rule.yml

# Check language support
ast-grep --lang list
```

### Error Messages

| Error | Cause | Solution |
|-------|-------|----------|
| "Pattern parse error" | Invalid pattern syntax | Check metavariable usage |
| "No matches found" | Pattern too specific | Generalize pattern |
| "Language not supported" | Missing parser | Install language support |
| "Rule validation failed" | YAML syntax error | Validate YAML structure |

---

specification_version: 1.0.5 | sdd-rules-tools-cli-astgrep.md Format: 2.0 |
Last Updated: 2025-09-12
