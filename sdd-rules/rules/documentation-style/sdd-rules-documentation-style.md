# SDD Documentation Style

SDD Documentation Style follows the [Google developer documentation style guide](https://developers.google.com/style).

## Text Style

- Use an imperative style. "Run a prompt using the API."
- Use sentence case in titles/headings.
- Use short titles/headings: "Download the data", "Call the API", "Process the results".
- Use the [Google developer documentation style guide](https://developers.google.com/style).
- Use second person: "you" rather than "we".
- When using links between notebooks, use relative ones as they'll work better in IDEs and Colab. Use absolute ones to link to folders or markdown files.

## Markdown — markdownlint configuration

This repository standardizes Markdown style using markdownlint with a project-wide configuration.

- Config file: `.markdownlint.json` (repository root)
- Base rules: aligned with the Google Developer Documentation Style Guide
  - Headings: ATX style, sentence case (MD003 atx)
  - Lists: dash bullets (MD004 dash), ordered lists ordered (MD029 ordered)
  - Indentation: 2 spaces for lists (MD007 indent=2)
  - Code blocks: fenced style, backticks (MD046 fenced, MD048 backtick)
  - Punctuation: allow common sentence punctuation (MD026 . , ; : !)
  - Headings duplication limited to siblings (MD024 siblings_only=true, no-duplicate-heading siblings_only=true)
  - Line-length disabled for readability in diffs (MD013=false)
  - Bare URLs allowed in some docs (no-bare-urls=false)
  - Trailing spaces/hard tabs disallowed (no-trailing-spaces=true, no-hard-tabs=true)

Recommended tools and workflow:

- Local non-blocking lint (developer convenience): `scripts/sdd/lint_docs.sh`
  - Suggests installing `markdownlint-cli` locally and can generate `.markdownlint.json` if missing
  - Produces a documentation report and warns but does not fail
- CI/blocking lint (quality gate): `scripts/ci/run-markdown-style.sh`
  - Runs markdownlint over the repository’s documentation scope using `.markdownlint.json`
  - Exits non‑zero on violations (intended for CI and pre‑PR checks)
- Aggregated local CI: `scripts/ci/run-local-ci.sh`
  - Runs structure, language, markdown style, and semantic checks in one pass

Scope (lint targets):

- `dev-docs/**/*.md`, `specs/**/*.md`, `sdd-rules/**/*.md`
- Top-level docs: `README.md`, `CONTRIBUTING.md`, `WARP.md`, `AGENTS.md`, `CLAUDE.md`

Exclusions (ignored by search or via git ls-files):

- `node_modules/`, `.git/`, `target/`, `dist/`, `build/`, `.venv/`

---

specification_version: 1.0.3 | sdd-rules-documentation-style.md Format: 1.0 | Last Updated: 2025-09-11
