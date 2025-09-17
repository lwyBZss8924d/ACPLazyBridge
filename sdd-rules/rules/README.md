# SDD Rules Index

This index links to the baseline SDD rules for the Developer Team AI Engineers.

## Categories

### changelog

- sdd-rules/rules/changelog/keep-a-changelog-index.html.haml
- sdd-rules/rules/changelog/semver.md
- sdd-rules/rules/changelog/sdd-rules-changelog.md

### ci

- sdd-rules/rules/ci/sdd-rules-ci.md

### code-analysis

- sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md
- **AST-grep Rules**:
    - **Go**:
        - sdd-rules/rules/code-analysis/ast-grep/go/no-fmt-println.yml
    - **JavaScript**:
        - sdd-rules/rules/code-analysis/ast-grep/js/no-console-log.yml
        - sdd-rules/rules/code-analysis/ast-grep/js/no-only-in-tests.yml
    - **Python**:
        - sdd-rules/rules/code-analysis/ast-grep/python/no-pdb.yml
        - sdd-rules/rules/code-analysis/ast-grep/python/no-print.yml
    - **Rust**:
        - sdd-rules/rules/code-analysis/ast-grep/rust/no-dbg.yml
        - sdd-rules/rules/code-analysis/ast-grep/rust/no-unwrap.yml
        - sdd-rules/rules/code-analysis/ast-grep/rust/rust-mutex-lock.yml
        - sdd-rules/rules/code-analysis/ast-grep/rust/todo-comment.yml

### documentation-style

- sdd-rules/rules/documentation-style/Google-developer-documentation-style-guide.md
- sdd-rules/rules/documentation-style/sdd-rules-documentation-markdownlint.md
- sdd-rules/rules/documentation-style/sdd-rules-documentation-style.md

### git

- sdd-rules/rules/git/comments/sdd-rules-comments.md
- sdd-rules/rules/git/issues/sdd-rules-issues.md
- sdd-rules/rules/git/pr/sdd-rules-pr.md
- sdd-rules/rules/git/worktree/sdd-rules-worktrees.md

### research

- sdd-rules/rules/research/sdd-rules-research.md

### tests

- sdd-rules/rules/tests/sdd-rules-tests.md

### tools-cli

- sdd-rules/rules/tools-cli/sdd-rules-tools-cli-list.md
- sdd-rules/rules/tools-cli/sdd-rules-tools-cli-document-search-and-parsing.md
- sdd-rules/rules/tools-cli/sdd-rules-tools-cli-astgrep.md
- sdd-rules/rules/tools-cli/ast-grep.llms.txt

### tools-mcp

- sdd-rules/rules/tools-mcp/sdd-rules-tools-mcp.md

## References

- .specify/memory/constitution.md (project SDD Constitution v1.0.1; authoritative governance)
- .specify/commands/ (SDD command implementations for /specify, /plan, /tasks)
- scripts/sdd/ (SDD workflow automation scripts)
- .specify/templates/ (SDD document templates)
- sdd-rules/AGENTS.md (authoritative agent rules)
- sdd-rules/CLAUDE.md (Claude-specific rules and configuration)
- WARP.md (repository agent operations)

---

```yaml
Constitution version: 1.0.1
Document: sdd-rules/rules/README.md
Document version: 1.0.1
Last Updated: 2025-09-17
```
