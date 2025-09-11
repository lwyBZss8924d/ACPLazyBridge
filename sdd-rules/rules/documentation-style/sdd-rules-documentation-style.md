# SDD Documentation Style

SDD Documentation Style follows the [Google developer documentation style guide](https://developers.google.com/style).

## Text Style

- Use an imperative style. "Run a prompt using the API."
- Use sentence case in titles/headings.
- Use short titles/headings: "Download the data", "Call the API", "Process the results".
- Use the [Google developer documentation style guide](https://developers.google.com/style).
- Use second person: "you" rather than "we".
- When using links between notebooks, use relative ones as they'll work better in IDEs and Colab. Use absolute ones to link to folders or markdown files.

## Markdown — markdownlint Configuration

This repository standardizes Markdown style using markdownlint with a project-wide configuration, now optimized for `markdownlint-cli2`.

### Configuration Files

- **Config**: `.markdownlint.json` (repository root) - Main configuration with rule definitions
- **Ignore**: `.markdownlintignore` (repository root) - Patterns for files to exclude
- **Reference**: [markdownlint-config.md](./markdownlint-config.md) - Detailed rule documentation

### Base Rules

Aligned with the Google Developer Documentation Style Guide:

#### Headings

- **MD003**: ATX style (`#`, `##`, `###`) - no underline-style headings
- **MD024**: Allow duplicate headings (but not at same level)
- **MD025**: Single top-level heading per document
- **MD026**: Allow punctuation in headings (. , ; : !)
- **MD041**: First line doesn't need to be a heading (disabled)

#### Lists

- **MD004**: Dash bullets (`-`) for unordered lists
- **MD007**: 2 spaces for list indentation
- **MD029**: Ordered lists should be ordered (1, 2, 3)

#### Code Blocks

- **MD040**: Language specifiers required (e.g., ` ```bash `, ` ```yaml `)
- **MD046**: Fenced style with backticks
- **MD048**: Use backticks for code fences

#### Emphasis & Formatting

- **MD049**: Underscores for italic (`_italic_`)
- **MD050**: Asterisks for bold (`**bold**`)
- **MD036**: Emphasis-as-heading allowed for certain patterns (disabled)

#### Spacing & Layout

- **MD010**: No hard tabs (use spaces)
- **MD012**: Multiple blank lines allowed (disabled for readability)
- **MD013**: Line length limit disabled (better for diffs)
- **MD035**: Horizontal rules use `---`

#### Tables

- **MD055**: Pipes at beginning and end of table rows

#### Links & HTML

- **MD033**: Inline HTML allowed when necessary
- **MD051**: Link fragment checking disabled (too many false positives)
- **no-bare-urls**: Bare URLs allowed

### Tool Selection: markdownlint-cli2

We use `markdownlint-cli2` as the primary tool because:

1. **Better performance**: Faster scanning of large repositories
2. **Glob support**: Native glob patterns without shell expansion
3. **Ignore file**: Built-in `.markdownlintignore` support
4. **Fix capability**: Auto-fix support with `--fix` flag
5. **Better output**: Clearer error messages and formatting

### Tools and Workflow

#### Local Development (Non-blocking)

1. **Check markdown** (informational):

   ```bash
   scripts/sdd/check-markdown.sh
   ```

   - Provides detailed report without failing
   - Shows auto-fixable vs manual issues
   - Suggests fixes

2. **Auto-fix markdown**:

   ```bash
   scripts/sdd/fix-markdown.sh
   ```

   - Automatically fixes most common issues
   - Preserves intentional formatting
   - Creates backup before fixing

3. **Legacy lint** (deprecated):

   ```bash
   scripts/sdd/lint_docs.sh  # Will be removed in future
   ```

#### CI/CD (Blocking)

1. **CI markdown check**:

   ```bash
   scripts/ci/run-markdown-style.sh
   ```

   - Runs markdownlint-cli2 with strict checking
   - Exits non-zero on violations
   - Used in GitHub Actions and pre-commit hooks

2. **Aggregated local CI**:

   ```bash
   scripts/ci/run-local-ci.sh
   ```

   - Includes markdown style checks
   - Runs all quality gates in sequence

### Scope

#### Included (lint targets)

- `dev-docs/**/*.md`
- `specs/**/*.md`
- `sdd-rules/**/*.md`
- Top-level docs: `README.md`, `CONTRIBUTING.md`, `WARP.md`, `AGENTS.md`, `CLAUDE.md`
- Any new `*.md` files unless explicitly excluded

#### Excluded (via .markdownlintignore)

- `.worktrees/` - Git worktree directories
- `.codeql-db/` - CodeQL database files
- `node_modules/`, `target/`, `dist/`, `build/` - Build artifacts
- `.venv/`, `venv/` - Python virtual environments
- `sdd-rules/rules/changelog/semver.md` - External canonical specification

### Special Cases and Exceptions

#### Intentional Numbering Patterns

Some files use non-standard numbering that should be preserved:

1. **Chinese documentation** - Uses `1)`, `2)`, `3)` style numbering
2. **External specifications** - Files like `semver.md` maintain original formatting
3. **Reference documentation** - External content preserves source formatting

#### When to Disable Rules

Create file-specific overrides only when:

- Preserving external/canonical content
- Following specific framework conventions
- Maintaining compatibility with other tools

Use inline comments to disable specific rules:

```markdown
<!-- markdownlint-disable MD029 -->
1) First item with intentional numbering
2) Second item
3) Third item
<!-- markdownlint-enable MD029 -->
```

### Migration from markdownlint-cli to markdownlint-cli2

If you have `markdownlint-cli` (v1) installed:

1. It will still work but is deprecated
2. Install `markdownlint-cli2`: `npm install -g markdownlint-cli2`
3. Scripts automatically prefer cli2 when available
4. Configuration is compatible between versions

### Troubleshooting

#### Common Issues

1. **"MD040: Fenced code blocks should have a language"**
   - Add language identifier: ` ```text ` for plain text, ` ```bash ` for shell

2. **"MD029: Ordered list numbering"**
   - Use sequential numbers (1, 2, 3) unless intentionally different
   - Add to `.markdownlintignore` if intentional

3. **"MD055: Table pipe style"**
   - Ensure pipes `|` at start and end of table rows

4. **False positives**
   - Check if file should be in `.markdownlintignore`
   - Use inline disable comments for specific sections
   - Report persistent issues for config updates

---

specification_version: 1.0.4 | sdd-rules-documentation-style.md Format: 1.1 | Last Updated: 2025-09-12
