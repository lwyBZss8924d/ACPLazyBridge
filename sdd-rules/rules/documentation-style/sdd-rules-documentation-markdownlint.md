# SDD Documentation - Markdownlint Configuration Reference

This document provides a detailed reference for the `markdownlint`
configuration used in this repository, with justifications based on the
[Google Markdown Style Guide](https://google.github.io/styleguide/docguide/style.html).

## Configuration File: .markdownlint.json

The configuration is stored in `.markdownlint.json` at the repository root.

## Rule Reference

### General Settings

- `default: true`: Enables all rules by default, which are then explicitly
  configured or disabled below.

### Heading Rules

#### MD003 - Heading style

- **Configuration**: `{ "style": "atx" }`
- **Description**: Enforces ATX-style headings (`#`, `##`).
- **Rationale**: Google's guide requires ATX headings for readability and
  easier maintenance.

**Correct:**

```markdown
# Heading 1
## Heading 2
```

**Incorrect:**

```markdown
Heading 1
=========
```

#### MD022 - Headings should be surrounded by blank lines

- **Configuration**: `true`
- **Description**: Requires a blank line before and after each heading.
- **Rationale**: Aligns with Google's guideline to "Add spacing to headings"
  for readability.

#### MD024 - Multiple headings with the same content

- **Configuration**: `{ "siblings_only": true }`
- **Description**: Allows duplicate heading text, but not at the same level
  (as siblings).
- **Rationale**: A practical choice to prevent accidental duplication while
  allowing logical sectioning.

#### MD025 - Multiple top-level headings

- **Configuration**: `true`
- **Description**: Enforces a single H1 (`#`) heading per document.
- **Rationale**: Google's guide specifies that the first heading should be a
  single level-one heading that serves as the document title.

#### MD026 - Trailing punctuation in heading

- **Configuration**: `{ "punctuation": ".,;:!" }`
- **Description**: Allows common punctuation at the end of headings.

#### MD041 - First line in file should be a top-level heading

- **Configuration**: `false`
- **Description**: The first line does not need to be a heading, allowing for
  frontmatter or comments.

### List Rules

#### MD004 - Unordered list style

- **Configuration**: `{ "style": "dash" }`
- **Description**: Enforces dashes (`-`) for unordered list items.

#### MD007 - Unordered list indentation

- **Configuration**: `{ "indent": 4 }`
- **Description**: Nested list items must be indented with 4 spaces.
- **Rationale**: Google's guide explicitly requires a 4-space indent for
  nested lists.

**Correct:**

```markdown
- Item 1
    - Nested item
        - Deeply nested
```

#### MD029 - Ordered list item prefix

- **Configuration**: `{ "style": "ordered" }`
- **Description**: Enforces sequential numbering (`1.`, `2.`, `3.`).
- **Note**: Google's guide also suggests "lazy numbering" (`1.`, `1.`, `1.`)
  for long, complex lists that may change often. Our configuration prefers
  sequential numbering for source readability.

#### MD030 - Spaces after list markers

- **Configuration**: `false`
- **Description**: Does not enforce specific spacing after list markers,
  allowing flexibility in list formatting.
- **Rationale**: Disabled to accommodate various list formatting styles without
  strict spacing requirements.

### Code Block Rules

#### MD040 - Fenced code blocks should have a language specified

- **Configuration**: `true`
- **Description**: Requires a language identifier for all fenced code blocks.
- **Rationale**: Google's guide requires declaring the language to enable
  correct syntax highlighting and parsing.

#### MD046 - Code block style

- **Configuration**: `{ "style": "fenced" }`
- **Description**: Enforces fenced code blocks (```) over indented blocks.
- **Rationale**: Google's guide mandates fenced blocks because they are
  unambiguous and allow language specifiers.

#### MD048 - Code fence style

- **Configuration**: `{ "style": "backtick" }`
- **Description**: Enforces backticks (```) for code fences, not tildes
  (`~~~`).

### Emphasis Rules

#### MD049 - Emphasis style (italic)

- **Configuration**: `{ "style": "underscore" }`
- **Description**: Use underscores for italic: `_italic_`.

#### MD050 - Strong style (bold)

- **Configuration**: `{ "style": "asterisk" }`
- **Description**: Use asterisks for bold: `**bold**`.

#### MD036 - Emphasis used instead of a heading

- **Configuration**: `false`
- **Description**: Disabled to allow emphasis where appropriate without
  flagging it as a heading.

### Spacing and Layout Rules

#### MD009 - Trailing spaces

- **Configuration**: `{ "br_spaces": 0 }`
- **Description**: Prohibits trailing whitespace.
- **Rationale**: Google's guide forbids trailing whitespace to avoid
  unintentional line breaks and ensure clean diffs.

#### MD010 - Hard tabs

- **Configuration**: `true`
- **Description**: Prohibits the use of hard tab characters.

#### MD012 - Multiple consecutive blank lines

- **Configuration**: `false`
- **Description**: Allows multiple blank lines, which can be useful for
  visually separating sections.

#### MD013 - Line length

- **Configuration**: `false`
- **Description**: Line length checking is disabled.
- **Rationale**: This repository does not enforce a specific line length limit
  to accommodate long URLs, code examples, technical documentation, and
  AI-generated content in files like CLAUDE.md and AGENTS.md. Code review
  handles readability concerns on a case-by-case basis.

#### MD035 - Horizontal rule style

- **Configuration**: `{ "style": "---" }`
- **Description**: Enforces `---` for horizontal rules.

### Table Rules

#### MD055 - Table pipe style

- **Configuration**: `{ "style": "leading_and_trailing" }`
- **Description**: Requires table rows to start and end with a pipe (`|`)
  character.

### Link and HTML Rules

#### MD033 - Inline HTML

- **Configuration**: `false`
- **Description**: Allows the use of inline HTML for complex formatting not
  supported by standard Markdown.

#### MD051 - Link fragments

- **Configuration**: `false`
- **Description**: Disables checking of link fragments (e.g., `#section-name`),
  as it can be unreliable with dynamic content.

#### no-bare-urls

- **Configuration**: `false`
- **Description**: Allows URLs to be pasted directly without requiring them to
  be formatted as links.

## Troubleshooting Common Issues

## MD040: Fenced code blocks should have a language

Add a language identifier after the opening backticks.

**Correct:** Use ` ```bash ` or ` ```text ` or any appropriate language.

**Incorrect:** Using ` ``` ` without a language identifier.

## MD029: Ordered list numbering

Use sequential numbers unless intentionally different:

```markdown
# Correct
1. First item
2. Second item
3. Third item

# For intentional numbering, disable the rule:
<!-- markdownlint-disable MD029 -->
1) First item
2) Second item
<!-- markdownlint-enable MD029 -->
```

## MD007: Incorrect list indentation

Use 4 spaces for nested lists:

```markdown
# Correct
- First level
    - Second level (4 spaces)
        - Third level (8 spaces)

# Incorrect
- First level
  - Second level (2 spaces)
```

## Automated Validation and Fixing

The `sdd-doc-validator` sub-agent provides comprehensive markdown validation and fixing capabilities:

- Runs markdownlint with project configuration
- Auto-fixes violations where possible
- Creates tracking lists for manual fixes
- Manages long-term documentation quality improvements

For large-scale markdown fixes or comprehensive validation, delegate to the sdd-doc-validator agent.

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-29T22:47:00Z"
rules:
    name: "markdownlint"
    category: "documentation-style"
    version: "1.0.4"
document:
    type: "sdd-rule"
    path: "sdd-rules/rules/documentation-style/sdd-rules-documentation-markdownlint.md"
    version: "1.0.4"
    last_updated: "2025-09-29T22:47:00Z"
    changelog: "Aligned documentation with actual .markdownlint.json configuration: MD013 disabled, MD030 documented"
    related:
        - "sdd-rules/rules/documentation-style/sdd-rules-documentation-style.md"
        - "sdd-rules/rules/documentation-style/google-markdown-style-guide.md"
```
