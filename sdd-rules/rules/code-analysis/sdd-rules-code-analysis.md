# SDD Rules - Code Analysis

## Code Analysis - ast-grep

You run in an environment where `ast-grep` is available; whenever a search requires syntax-aware or structural matching, default to `ast-grep --lang rust -p '<pattern>'` (or set `--lang` appropriately) and avoid falling back to text-only tools like `rg` or `grep` unless I explicitly request a plain-text search.

### Project configuration and rule discovery

- Prefer project-local `sgconfig.yml` at repo root (committed). Tools and editors discover it automatically.
- For rule-level `files:` globs, include at least one positive pattern (e.g. `"**/*.rs"`) before negative excludes. Negative-only lists may match nothing.

### Important Limitations Discovered (Issue #34)

**File exclusion patterns don't work with ruleDirs:**

- When rules are loaded via `ruleDirs` in sgconfig.yml, the `files:` field in individual rule YAML files is ignored
- This is an undocumented limitation of ast-grep's configuration system
- The `ignores` section in sgconfig.yml only affects file traversal, not rule application

**Suppression comment requirements:**

- Suppression comments must be on the line immediately before the code to suppress
- Module-level or file-level suppressions don't cascade to nested functions
- Use `// ast-grep-ignore` or `// ast-grep-ignore: rule-id` format

**Workaround for test code:**

- Add individual suppression comments before each `unwrap()`/`expect()` in test files
- Cannot rely on file patterns to exclude test directories when using `ruleDirs`
- See CONTRIBUTING.md for detailed suppression examples

### Recommended CLI flows

### Rule Development Checklist (CLI-only, inspired by ast-grep guidance)

- Clarify intent and scope; break complex queries into sub-rules.
- Author minimal examples (one that SHOULD match, one that SHOULD NOT).
- Start atomic (kind or pattern), then add relational rules (inside/has) with `stopBy: end`.
- Inspect AST/pattern with `--debug-query` for ambiguous cases.
- Add constraints/composites incrementally; validate with rule-tests/ (valid/invalid).
- Produce streaming JSON evidence and a grouped-by-file summary; declare scope (globs/includes/excludes) in the report.

### CLI mapping for iterative rule authoring

- AST view: `--debug-query=ast` (or `pattern`/`cst`)
- Unit-test rules: `ast-grep test` (rule-tests/*)
- Repo scan: `scan -c sgconfig.yml --filter ... --globs ...`
- Project diagnostics summary:

  ```bash
  ast-grep scan -c ./sgconfig.yml --inspect summary .
  ```

- Subset rules by ID pattern:

  ```bash
  ast-grep scan -c ./sgconfig.yml --filter '^rust-no-' .
  ```

- Scope by globs (e.g., non-test Rust):

  ```bash
  ast-grep scan -c ./sgconfig.yml --globs '**/*.rs' --globs '!**/tests/**' .
  ```

- Ad-hoc structural search:

  ```bash
  ast-grep -p '$EXPR.unwrap()' -l rust crates/
  ast-grep -p 'dbg!($$$ARGS)' -l rust crates/
  ```

- Verify suppression effectiveness:

  ```bash
  # Count warnings for specific rule
  ast-grep scan -c sgconfig.yml --filter '^rust-no-unwrap$' . | grep -c warning

  # Test rule file directly (bypasses ruleDirs limitation)
  ast-grep scan --rule-file sdd-rules/rules/code-analysis/ast-grep/rust/no-unwrap.yml .
  ```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-19T04:32:00Z"
rules:
    name: "code-analysis"
    category: "code-analysis"
    version: "1.0.2"
document:
    type: "sdd-rule"
    path: "sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md"
    last_updated: "2025-09-19T12:00:00Z"
    changelog: "Added ast-grep limitations discovered in Issue #34"
    related:
        - "sdd-rules/rules/tools-cli/sdd-rules-tools-cli-list.md"
        - "sdd-rules/rules/tools-cli/sdd-rules-tools-cli-astgrep.md"
        - "sdd-rules/rules/code-analysis/ast-grep/rust/no-dbg.yml"
        - "sdd-rules/rules/code-analysis/ast-grep/rust/no-unwrap.yml"
        - "sdd-rules/rules/code-analysis/ast-grep/rust/rust-mutex-lock.yml"
        - "sdd-rules/rules/code-analysis/ast-grep/rust/todo-comment.yml"
        - "sdd-rules/rules/code-analysis/ast-grep/go/no-fmt-println.yml"
        - "sdd-rules/rules/code-analysis/ast-grep/js/no-console-log.yml"
        - "sdd-rules/rules/code-analysis/ast-grep/js/no-only-in-tests.yml"
        - "sdd-rules/rules/code-analysis/ast-grep/python/no-pdb.yml"
        - "sdd-rules/rules/code-analysis/ast-grep/python/no-print.yml"
```
