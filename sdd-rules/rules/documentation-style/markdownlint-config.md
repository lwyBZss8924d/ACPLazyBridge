# Markdownlint Configuration Reference

This document provides detailed information about the markdownlint configuration used in ACPLazyBridge.

## Configuration File: .markdownlint.json

The configuration is stored in `.markdownlint.json` at the repository root. The file uses standard JSON format without comments (JSON doesn't support inline comments).

## Rule Reference

### General Settings

```json
{
  "default": true
}
```

- `default: true` - Enable all rules by default, then override specific ones

### Heading Rules

#### MD003 - Heading style

```json
"MD003": { "style": "atx" }
```

- **Style**: ATX (`#`, `##`, `###`)
- **Not**: Setext (underlines with `===` or `---`)

 **Good:**

```markdown
# Heading 1
## Heading 2
```

L **Bad:**

```markdown
Heading 1
=========
```

#### MD024 - Multiple headings with the same content

```json
"MD024": { "siblings_only": true }
```

- Allow duplicate headings at different levels
- Prevent duplicates at the same level

 **Good:**

```markdown
# Setup
## Database Setup
## API Setup
```

L **Bad:**

```markdown
## Setup
## Setup  # Duplicate at same level
```

#### MD025 - Multiple top-level headings

```json
"MD025": true
```

- Only one H1 (`#`) per document

#### MD026 - Trailing punctuation in heading

```json
"MD026": { "punctuation": ".,;:!" }
```

- Allow these punctuation marks in headings
- Useful for headings like "What is this?" or "Important!"

#### MD041 - First line in file should be a top-level heading

```json
"MD041": false
```

- Disabled - first line doesn't need to be a heading
- Allows starting with metadata or comments

### List Rules

#### MD004 - Unordered list style

```json
"MD004": { "style": "dash" }
```

- Use dashes (`-`) for unordered lists
- Not asterisks (`*`) or plus signs (`+`)

 **Good:**

```markdown
- Item 1
- Item 2
```

#### MD007 - Unordered list indentation

```json
"MD007": { "indent": 2 }
```

- Use 2 spaces for nested list indentation

 **Good:**

```markdown
- Item 1
  - Nested item
    - Deeply nested
```

#### MD029 - Ordered list item prefix

```json
"MD029": { "style": "ordered" }
```

- Use sequential numbers (1, 2, 3)
- Not all 1s (1, 1, 1)

 **Good:**

```markdown
1. First
2. Second
3. Third
```

L **Bad:**

```markdown
1. First
1. Second
1. Third
```

### Code Block Rules

#### MD040 - Fenced code blocks should have a language specified

```json
"MD040": true
```

- Always specify language after opening backticks

 **Good:**

```markdown
```bash
echo "Hello"
```
```

L **Bad:**

```markdown
```
echo "Hello"
```
```

#### MD046 - Code block style

```json
"MD046": { "style": "fenced" }
```

- Use fenced code blocks (```)
- Not indented code blocks

#### MD048 - Code fence style

```json
"MD048": { "style": "backtick" }
```

- Use backticks (```) not tildes (~~~)

### Emphasis Rules

#### MD049 - Emphasis style (italic)

```json
"MD049": { "style": "underscore" }
```

- Use underscores for italic: `_italic_`
- Not asterisks: `*italic*`

#### MD050 - Strong style (bold)

```json
"MD050": { "style": "asterisk" }
```

- Use asterisks for bold: `**bold**`
- Not underscores: `__bold__`

#### MD036 - Emphasis used instead of a heading

```json
"MD036": false
```

- Disabled - allows using bold/italic for emphasis in certain contexts
- Some documentation styles use this pattern

### Spacing Rules

#### MD010 - Hard tabs

```json
"MD010": false
```

- Disabled (handled by `no-hard-tabs` rule)

#### MD012 - Multiple consecutive blank lines

```json
"MD012": false
```

- Disabled - allows multiple blank lines for readability
- Useful for separating major sections

#### MD013 - Line length

```json
"MD013": false,
"line-length": false
```

- Both disabled - no line length limit
- Better for diffs and side-by-side viewing

#### MD035 - Horizontal rule style

```json
"MD035": { "style": "---" }
```

- Use three dashes for horizontal rules

### Table Rules

#### MD055 - Table pipe style

```json
"MD055": { "style": "leading_and_trailing" }
```

- Tables must have pipes at start and end of rows

 **Good:**

```markdown
| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
```

L **Bad:**

```markdown
Header 1 | Header 2
-------- | --------
Cell 1   | Cell 2
```

### Link and HTML Rules

#### MD033 - Inline HTML

```json
"MD033": false
```

- Disabled - allows inline HTML when needed
- Useful for complex formatting

#### MD051 - Link fragments

```json
"MD051": false
```

- Disabled - doesn't check link fragments
- Too many false positives with dynamic content

#### no-bare-urls

```json
"no-bare-urls": false
```

- Disabled - allows bare URLs
- Useful in documentation and examples

### Other Rules

#### no-trailing-spaces

```json
"no-trailing-spaces": true
```

- No trailing spaces at end of lines

#### no-hard-tabs

```json
"no-hard-tabs": true
```

- Use spaces, not tabs

#### no-emphasis-as-heading

```json
"no-emphasis-as-heading": true
```

- Don't use bold/italic as pseudo-headings
- Use proper heading syntax

#### no-duplicate-heading

```json
"no-duplicate-heading": { "siblings_only": true }
```

- Same as MD024 - prevent duplicate headings at same level

## Ignore Patterns

The `.markdownlintignore` file excludes:

- **Build directories**: `node_modules/`, `target/`, `dist/`, `build/`
- **Virtual environments**: `.venv/`, `venv/`
- **Git directories**: `.git/`, `.worktrees/`
- **Database files**: `.codeql-db/`
- **External specs**: `sdd-rules/rules/changelog/semver.md`

## Special Cases

### Inline Disabling

For specific sections that need different rules:

```markdown
<!-- markdownlint-disable MD029 -->
1) Special numbering
2) That should be preserved
<!-- markdownlint-enable MD029 -->
```

### File-specific Configuration

Create `.markdownlint.yaml` in a directory to override for that directory:

```yaml
extends: ../../.markdownlint.json
rules:
  MD029: false  # Disable for this directory
```

## Command Line Usage

### Check files

```bash
markdownlint-cli2 "**/*.md"
```

### Auto-fix issues

```bash
markdownlint-cli2 "**/*.md" --fix
```

### Use custom config

```bash
markdownlint-cli2 "**/*.md" --config .markdownlint.json
```

### Ignore certain files

```bash
markdownlint-cli2 "**/*.md" --ignore ".worktrees/**"
```

## Migration Notes

### From markdownlint-cli to markdownlint-cli2

1. **Config compatibility**: Same `.markdownlint.json` works for both
2. **Ignore file**: cli2 supports `.markdownlintignore`, cli doesn't
3. **Performance**: cli2 is faster for large repositories
4. **Glob patterns**: cli2 handles globs better

### Common Issues When Migrating

1. **JSON comments**: Remove any comments from `.markdownlint.json`
2. **Ignore patterns**: Move exclusions to `.markdownlintignore`
3. **Fix flag**: Use `--fix` with cli2 for auto-fixing

---

specification_version: 1.0.1 | markdownlint-config.md Format: 1.0 | Last Updated: 2025-09-12
