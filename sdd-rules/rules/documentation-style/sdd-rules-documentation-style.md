# SDD Documentation Style Guide

This guide defines the documentation standards for the ACPLazyBridge
project. Our standards are based on two primary sources to ensure clarity,
consistency, and maintainability:

1. **Writing Style**: [Google Developer Documentation Style Guide][1]
2. **Markdown Syntax**: [Google Markdown Style Guide][2]

[1]: https://developers.google.com/style
[2]: https://google.github.io/styleguide/docguide/style.html

> **Migration Note**: The project is transitioning to full Google Markdown
> Style Guide compliance. Currently using relaxed rules (`.markdownlint.json`)
> for existing content, with strict rules (`.markdownlint-strict.json`)
> available for new content and future migration.

## Language Policy

All normative artifacts committed to the repository (specifications, plans,
technical documents) **must be in English**. Non-normative reference
documentation, such as translations, may be provided in other languages
(e.g., under `dev-docs/zh-CN/`) but must include a disclaimer clarifying
their reference status.

## Writing Style

These rules govern the prose, tone, and language of documentation, derived
from the Google Developer Documentation Style Guide:

- **Voice and Tone**: Be conversational and friendly. Write for a global,
    technical audience.
- **Person**: Use second person ("you") rather than "we" or "I".
- **Active Voice**: Use active voice to make it clear who is performing
    the action.
  - ✅ Correct: "Run the command to start the server."
  - ❌ Incorrect: "The server is started by running the command."
- **Headings**: Use sentence case for all titles and headings.
  - ✅ Correct: "Create a new virtual machine"
  - ❌ Incorrect: "Create a New Virtual Machine"
- **Imperative Mood**: Use imperative mood (command form) for instructions
    and short headings.
  - Examples: "Download the data", "Call the API", "Process the results"
- **Links**: Use descriptive link text that clearly indicates the
    destination. When linking between documents within this repository, use
    relative paths.

## Markdown Syntax and Formatting

These rules govern the structure and syntax of Markdown files, enforced
automatically by `markdownlint`. They are based on the Google Markdown Style
Guide:

### Current Configuration (Transitional)

- **Headings**: Use ATX-style headings (`#`, `##`). A document must contain
  exactly one top-level H1 heading.
- **Line Length**: Currently disabled for existing content. Target: 80
  characters for prose (excludes code blocks and tables).
- **Lists**:
  - Use a dash (`-`) for unordered lists
  - Indent nested lists with 2 spaces (current) or 4 spaces (target)
  - Number ordered lists sequentially (`1.`, `2.`, `3.`)
- **Code Blocks**:
  - Use fenced code blocks (` ``` `) and always specify a language
    identifier (e.g., `bash`, `yaml`, `json`)
  - Do not use indented code blocks
- **Spacing**: Do not use hard tabs or trailing whitespace. Add blank lines
  around headings for readability.

### Target Configuration (Google-Aligned)

For new content or migrated files, use `.markdownlint-strict.json`:

- **Line Length**: 80 characters for prose
- **List Indentation**: 4 spaces for nested lists
- All other rules as specified above

## Tooling and Workflow

We use `markdownlint-cli2` to standardize Markdown style across the
repository.

- **Configuration**: `.markdownlint.json` (repository root)
- **Ignore File**: `.markdownlintignore` (repository root)
- **Rule Documentation**:
    [sdd-rules-documentation-markdownlint.md](./sdd-rules-documentation-markdownlint.md)

### Local Development (Non-blocking)

1. **Check for Issues**: Run `scripts/sdd/check-markdown.sh` to see a report
   of style violations without failing the build.

2. **Auto-fix Issues**: Run `scripts/sdd/fix-markdown.sh` to automatically
   correct common formatting problems.

### CI/CD (Blocking)

The `scripts/ci/run-markdown-style.sh` script runs in our CI pipeline and
pre-commit hooks. It will fail if any Markdown files violate the configured
style, ensuring all committed documentation is compliant.

## Scope

### Included Files

- `dev-docs/**/*.md`
- `specs/**/*.md`
- `sdd-rules/**/*.md`
- Top-level documents: `README.md`, `CONTRIBUTING.md`, `WARP.md`,
    `AGENTS.md`, `CLAUDE.md`
- Any new `*.md` files unless explicitly excluded

### Excluded Files

The following are excluded via `.markdownlintignore`:

- Build artifacts (`node_modules/`, `target/`, `dist/`, `build/`)
- Git and tool directories (`.worktrees/`, `.codeql-db/`)
- Python virtual environments (`.venv/`, `venv/`)
- External specifications with canonical formatting
    (`sdd-rules/rules/changelog/semver.md`)

## Exceptions and Special Cases

To preserve specific formatting (e.g., for external content or intentional
numbering), you can disable rules for a section of a file using inline
comments:

```markdown
<!-- markdownlint-disable MD029 -->
1) First item with intentional numbering
2) Second item
<!-- markdownlint-enable MD029 -->
```

Only disable rules when absolutely necessary, such as to maintain the
original formatting of a canonical specification.

---

specification_version: 1.0.6 | sdd-rules-documentation-style.md Format: 1.3
| Last Updated: 2025-09-12
