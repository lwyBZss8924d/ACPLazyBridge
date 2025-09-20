# ast-grep Enforcement Activation Procedure

## Purpose

Step-by-step procedure to enable enforcement mode for ast-grep scanning after Issue #31 resolution.

## Prerequisites Checklist

- [ ] Issue #31 is closed and merged
- [ ] No critical PRs pending merge
- [ ] Team has been notified (see transition.md)
- [ ] Suppression documentation is available

## Activation Steps

### 1. Prepare Enforcement Branch

```bash
# Update from main
git checkout main
git pull origin main

# Create enforcement branch
git checkout -b feature/enable-ast-grep-enforcement
```

### 2. Modify CI Workflow

Edit `.github/workflows/ci.yml`:

```diff
  ast-grep-scan:
    runs-on: ubuntu-latest
-   continue-on-error: true  # Report-only mode - won't block PR (remove to enforce)
+   # Enforcement mode enabled - violations will block PR
```

### 3. Validate Changes Locally

```bash
# Run validation test
bash _artifacts/035-ci-add-sdd/tests/test-report-only.sh

# Should now show enforcement mode active
```

### 4. Test with Intentional Violation

```bash
# Create test file with violation
cat > test-violation.rs <<'EOF'
fn main() {
    let x = Some(42);
    println!("Value: {}", x.unwrap()); // Should trigger rust-no-unwrap
}
EOF

# Commit and push
git add .
git commit -m "test: Verify enforcement blocks PR"
git push origin feature/enable-ast-grep-enforcement
```

### 5. Verify CI Behavior

1. Open PR from enforcement branch
2. Verify CI fails on ast-grep-scan job
3. Check that PR shows failing status
4. Confirm merge is blocked

### 6. Clean Test and Finalize

```bash
# Remove test file
git rm test-violation.rs
git commit -m "chore: Remove test violation"
git push
```

### 7. Create Production PR

```bash
gh pr create \
  --title "[TASK-035] Enable ast-grep enforcement mode" \
  --body "## Summary
Enables enforcement mode for ast-grep scanning following Issue #31 resolution.

## Changes
- Removed \`continue-on-error: true\` from ast-grep-scan job
- Violations will now block PR merging

## Prerequisites
- [x] Issue #31 resolved
- [x] Team notified
- [x] Documentation updated

## Testing
- Verified enforcement blocks PRs with violations
- Confirmed SARIF upload still works
- Tested suppression comments

## Rollback
If issues arise, re-add \`continue-on-error: true\` to restore report-only mode.

Refs: #31, #32"
```

### 8. Merge and Monitor

1. Get PR approval from team lead
2. Merge PR
3. Monitor next 5 PRs for issues
4. Be ready to rollback if needed

## Verification Commands

```bash
# Check current mode
grep -A2 "ast-grep-scan:" .github/workflows/ci.yml | grep continue-on-error

# If output shows continue-on-error: Report-only mode
# If no output: Enforcement mode

# Test with ast-grep locally
ast-grep scan -c sgconfig.yml .

# Count current violations
ast-grep scan -c sgconfig.yml --json . | jq '. | length'
```

## Rollback Procedure

If enforcement causes blocking issues:

### 1. Create Hotfix Branch

```bash
git checkout main
git pull
git checkout -b hotfix/restore-report-only
```

### 2. Restore Report-Only Mode

```diff
  ast-grep-scan:
    runs-on: ubuntu-latest
+   continue-on-error: true  # Temporary rollback to report-only mode
```

### 3. Fast-Track Merge

```bash
gh pr create \
  --title "HOTFIX: Restore ast-grep report-only mode" \
  --body "Temporary rollback due to [describe issue]" \
  --label urgent
```

## Follow-up Actions

After successful enforcement:

1. **Week 1**:
   - Monitor violation frequency
   - Track fix times
   - Collect team feedback

2. **Week 2**:
   - Review suppression usage
   - Identify common patterns
   - Update rules if needed

3. **Month 1**:
   - Analyze impact on code quality
   - Document lessons learned
   - Plan rule improvements

## Success Criteria

- [ ] No emergency rollbacks in first week
- [ ] < 5 suppression comments per week
- [ ] No complaints about false positives
- [ ] Average fix time < 10 minutes

## Troubleshooting

### Issue: Too many violations blocking work

**Solution**: Temporarily suppress with comments, create tech debt ticket

```rust
// ast-grep-ignore: rust-no-unwrap
// TODO: Refactor to handle error properly (TECH-DEBT-123)
value.unwrap()
```

### Issue: SARIF upload fails

**Solution**: Check permissions, ensure security-events: write is set

### Issue: Rules too strict

**Solution**: Adjust sgconfig.yml, but require team consensus

## Contact

- **Primary**: CI/CD Team Lead
- **Backup**: Platform Engineering
- **Emergency**: #dev-ops-emergency (Slack)

---

⚠️ _This procedure follows SDD principles from `.specify/memory/constitution.md`_

```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/ci-sdd-gates
feature_branch: 035-ci-add-sdd
created: 2025-09-20
last_updated: 2025-09-20
status: complete
spec_uri: specs/035-ci-add-sdd/spec.md
plan_uri: specs/035-ci-add-sdd/plan.md
tasks_uri: specs/035-ci-add-sdd/tasks.md
issue_url: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/32
evidence_uris: _artifacts/035-ci-add-sdd/
```
