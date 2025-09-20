# Known Issues and Fix Plan

## Issue Date: 2025-09-20

## Summary

Total markdown violations: 525
- Auto-fixable: 26 (but automation currently not working)
- Manual fixes required: 499
- Primary issue: MD007 (list indentation) - 498 violations

## Known Issues

### 1. fix-markdown.sh Script Bug

**Issue**: The script reports "0 issues found" when auto-fixable issues exist.

**Evidence**:
```bash
$ ./scripts/sdd/fix-markdown.sh
Checking for auto-fixable markdown issues...
0 issues found - all markdown files are properly formatted
```

**Actual state**: 26 auto-fixable issues exist (verified with markdownlint-cli2)

**Workaround**: Use markdownlint-cli2 directly:
```bash
npx --yes markdownlint-cli2@latest "**/*.md" --config .markdownlint.json --fix
```

**Root cause**: Script logic error in parsing markdownlint output

### 2. markdownlint-cli2 --fix Not Applying Changes

**Issue**: Running `markdownlint-cli2 --fix` reports "Linted" but doesn't actually fix violations.

**Evidence**:
- Before fix: 26 auto-fixable issues
- After running --fix: Still 26 auto-fixable issues
- Files show as "Linted" but violations remain

**Investigation needed**:
- Check file permissions
- Verify markdownlint-cli2 version compatibility
- Test with individual files vs glob patterns

### 3. MD007 List Indentation (498 violations)

**Issue**: Large number of manual fixes required for list indentation.

**Current state**: Lists use 2-space indentation
**Required state**: Lists must use 4-space indentation

**Example**:
```markdown
# Wrong (current)
- Item
  - Subitem

# Correct (required)
- Item
    - Subitem
```

## Fix Plan

### Phase 1: Automated Fixes (Immediate)

1. Debug and fix the `fix-markdown.sh` script
2. Investigate markdownlint-cli2 --fix behavior
3. Apply all auto-fixable violations (26 issues)
4. Verify fixes with comprehensive testing

### Phase 2: High-Priority Manual Fixes (Session 1)

Target: Critical path documentation
- `specs/` directory (affects SDD workflow)
- `.specify/` directory (SDD meta-documentation)
- Root documentation files (README.md, CONTRIBUTING.md, CLAUDE.md)

Estimated violations: ~150
Time estimate: 2-3 hours

### Phase 3: SDD Rules Documentation (Session 2)

Target: SDD rules and guidance
- `sdd-rules/` directory
- Focus on rule documentation consistency

Estimated violations: ~200
Time estimate: 3-4 hours

### Phase 4: Developer Documentation (Session 3)

Target: Engineering and review documentation
- `dev-docs/` directory
- `_artifacts/` documentation

Estimated violations: ~149
Time estimate: 2-3 hours

## Implementation Strategy

### Using the sdd-doc-validator Sub-agent

The `.claude/agents/sdd-doc-validator.md` sub-agent has been created to manage this process:

1. **Automatic fixing**:
   ```bash
   # Bypass buggy fix-markdown.sh
   npx --yes markdownlint-cli2@latest "**/*.md" --config .markdownlint.json --fix
   ```

2. **Progress tracking**:
   - Use TodoWrite tool for systematic progress
   - Track by file and rule type
   - Maintain fix history in `_artifacts/`

3. **Manual fix approach**:
   - Group files by directory
   - Use MultiEdit for batch changes per file
   - Focus on MD007 (4-space indentation)
   - Verify each file after fixing

## Testing Evidence

All testing logs stored in: `_artifacts/036-ci-markdown-style/tests/`

- `markdown-check-initial.log` - Initial state (525 violations)
- `markdown-fix-attempt.log` - Fix attempt results
- `markdown-check-after-fix.log` - Post-fix verification
- `markdown-violations-by-file.json` - Detailed violation breakdown

## Success Criteria

1. All auto-fixable issues resolved
2. Critical path documentation (specs/, .specify/) compliant
3. CI workflow passing in report-only mode
4. Fix plan documented for remaining violations
5. Sub-agent capable of multi-session fix management

## Current Status

- [x] CI workflow created (report-only mode)
- [x] Sub-agent created for documentation validation
- [x] Known issues documented
- [ ] fix-markdown.sh bug resolved
- [ ] Auto-fixes successfully applied
- [ ] Manual fixes started

## Next Steps

1. Create PR with current implementation (report-only mode)
2. Debug automation issues in parallel
3. Begin systematic manual fixes using sub-agent
4. Track progress across multiple sessions
5. Transition to enforcement mode after compliance achieved