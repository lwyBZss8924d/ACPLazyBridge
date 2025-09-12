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

- **Configuration**:
    `{ "line_length": 80, "code_blocks": false, "tables": false }`
- **Description**: Enforces an 80-character line length for prose.
- **Rationale**: This directly implements the Google guide's 80-character
  limit, which improves readability in code-centric tools. The configuration
  correctly excludes code blocks and tables from this limit, as specified in
  the guide.

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

## MD013: Line length exceeded

Wrap prose at 80 characters. Code blocks and tables are automatically excluded:

```markdown
# Prose should wrap at 80 characters
This is a long line that should be wrapped to stay within the 80 character
limit for better readability.

# Tables are excluded from line length limits
| This can be a very long header | Another long header | And another one |
| ------------------------------- | ------------------- | --------------- |
| Long content is allowed here   | More long content   | Even more       |
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

---

specification_version: 1.0.2 | sdd-rules-documentation-markdownlint.md
Format: 1.1 | Last Updated: 2025-09-12
