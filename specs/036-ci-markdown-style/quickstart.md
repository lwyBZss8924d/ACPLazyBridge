# Quickstart: Markdown Style Checking

## Local Development Workflow

### Running Markdown Checks Locally

```bash
# Run full local CI including markdown checks
./scripts/ci/run-local-ci.sh

# Run only markdown style checks
./scripts/ci/run-markdown-style.sh

# Check specific files
npx markdownlint-cli2 "path/to/file.md" --config .markdownlint.json
```

### Fixing Violations

#### Automatic Fixes

```bash
# Fix auto-fixable issues (spacing, trailing whitespace)
./scripts/sdd/fix-markdown.sh

# This fixes:
# - MD009: Trailing spaces
# - MD022: Blank lines around headings
# - MD032: Blank lines around lists
# - MD047: Files should end with single newline
```

#### Manual Fixes

Most violations require manual intervention:

**MD007: Unordered list indentation**

```markdown
# Wrong (2 spaces)
- Item 1
  - Subitem

# Correct (4 spaces)
- Item 1
    - Subitem
```

**MD003: Heading style**

```markdown
# Wrong (mixed styles)
# Heading 1
## Heading 2
### Heading 3

# Correct (ATX style only)
# Heading 1
## Heading 2
### Heading 3
```

## Understanding CI Failures

### Reading CI Output

When the CI job fails, look for:

1. **Job Name**: "Markdown Style Verification"
2. **Error Message**: Lists specific files and line numbers
3. **Fix Instructions**: "Run './scripts/ci/run-local-ci.sh' locally"

### Example CI Output

```bash
[DOC-STYLE] checking markdown files...
specs/036-ci-markdown-style/spec.md:25 MD007/ul-indent Unordered list indentation [Expected: 4; Actual: 2]
README.md:45 MD022/blanks-around-headings Headings should be surrounded by blank lines

Error: Markdown violations detected. Please run './scripts/ci/run-local-ci.sh' locally and fix issues before pushing.
```

### Common Issues and Solutions

| Rule | Issue | Solution |
|------|-------|----------|
| MD007 | Wrong list indentation | Use 4 spaces for nested lists |
| MD022 | Missing blank lines | Add blank line before/after headings |
| MD032 | Missing blank lines | Add blank line before/after lists |
| MD013 | Line too long | Disabled - no action needed |

## CI Workflow Behavior

### Report-Only Mode (Initial)

- Violations are reported but don't block PR
- Check appears with warning icon
- Artifacts uploaded for review

### Enforcement Mode (Future)

- Violations block PR merge
- Check appears with red X
- Must fix locally and push updates

## Best Practices

1. **Before Creating PR**:
   - Run `./scripts/ci/run-local-ci.sh`
   - Fix any markdown issues
   - Run auto-fix script if needed

2. **After CI Failure**:
   - Don't fix in GitHub web UI
   - Pull changes locally
   - Run fix script
   - Verify with local checks
   - Push fixes

3. **For New Documentation**:
   - Follow existing patterns
   - Use 4-space indentation for lists
   - Add blank lines around headings/lists
   - Check locally before committing

## Configuration Reference

The project uses `.markdownlint.json` with:

- ATX-style headings only (`#`, `##`, etc.)
- 4-space list indentation
- No line length limit (MD013 disabled)
- Inline HTML allowed
- See full config: `.markdownlint.json`

Files excluded via `.markdownlintignore`:

- External specifications (semver.md)
- Google style guides
- Build directories
- CLAUDE.md files (special formatting)
