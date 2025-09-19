# Quickstart: CI with SDD Gates and ast-grep Scanning

## Overview

Quick validation guide for the CI enhancement with SDD gates and ast-grep scanning.

## Prerequisites

- GitHub repository with Actions enabled
- GitHub Code Scanning enabled (Settings → Security → Code scanning)
- Write access to create pull requests

## Validation Scenarios

### Scenario 1: Report-Only Mode Validation

**Goal**: Verify ast-grep findings don't block PR merging

1. **Create test branch**:

   ```bash
   git checkout -b test/ci-report-only
   ```

2. **Add intentional violation**:

   ```rust
   // Add unwrap() in non-test code
   fn example() {
       let result = Some(42);
       let value = result.unwrap(); // Should trigger ast-grep
   }
   ```

3. **Push and create PR**:

   ```bash
   git add .
   git commit -m "test: Add unwrap for CI validation"
   git push origin test/ci-report-only
   gh pr create --title "Test: CI report-only mode"
   ```

4. **Expected Results**:
   - ✅ CI runs all checks
   - ✅ ast-grep findings appear in Security tab
   - ✅ PR can still be merged (green status)
   - ✅ SARIF results visible in Code Scanning

### Scenario 2: SDD Structure Validation

**Goal**: Verify SDD checks run correctly

1. **Create branch with SDD violation**:

   ```bash
   git checkout -b test/sdd-structure
   ```

2. **Create invalid structure**:

   ```bash
   # Create spec without required sections
   mkdir specs/999-test
   echo "Invalid spec" > specs/999-test/spec.md
   ```

3. **Push and observe**:

   ```bash
   git add .
   git commit -m "test: Invalid SDD structure"
   git push origin test/sdd-structure
   ```

4. **Expected Results**:
   - ❌ SDD structure check fails
   - ✅ Clear error message in CI logs
   - ✅ Other checks still run

### Scenario 3: Cross-Platform Testing

**Goal**: Verify tests run on all platforms

1. **Check CI logs for matrix jobs**:

   ```bash
   gh run list --workflow=ci.yml
   gh run view <run-id>
   ```

2. **Expected Results**:
   - ✅ Ubuntu job completes
   - ✅ macOS job completes
   - ✅ Windows job completes
   - ✅ All use cached dependencies (after first run)

### Scenario 4: Enforcement Mode Preparation

**Goal**: Test enforcement readiness (DO NOT MERGE)

1. **Temporarily edit workflow**:

   ```yaml
   ast-grep-scan:
     continue-on-error: false  # Changed from true
   ```

2. **Create PR with violation**:

   ```bash
   # Add unwrap() as before
   ```

3. **Expected Results**:
   - ❌ CI fails on ast-grep findings
   - ❌ PR shows failing status
   - ✅ Cannot merge without fixing

4. **Revert changes** (important!):

   ```bash
   git checkout .github/workflows/ci.yml
   ```

## Local Testing

### Run CI Checks Locally

```bash
# SDD checks
./scripts/ci/run-local-ci.sh

# ast-grep scan
ast-grep scan -c sgconfig.yml --json

# Rust checks
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features --locked

# Typos check
typos
```

### Validate Workflow Syntax

```bash
# Use GitHub CLI
gh workflow list
gh workflow view ci.yml

# Or use act for local testing
act -n  # Dry run
```

## Monitoring

### Check SARIF Upload

1. Navigate to: Repository → Security → Code scanning
2. Look for "ast-grep" in the tool filter
3. Verify findings are listed with:
   - File location
   - Line number
   - Rule ID
   - Severity

### Check CI Performance

```bash
# Get recent run times
gh run list --workflow=ci.yml --json databaseId,displayTitle,conclusion,startedAt,updatedAt \
  | jq '.[] | {title: .displayTitle, duration: (.updatedAt - .startedAt), conclusion}'
```

### Monitor Cache Hit Rate

Check "Cache cargo" step in CI logs for:

```bash
Cache restored from key: <cache-key>
```

## Troubleshooting

### Issue: SARIF upload fails

**Solution**: Check permissions in workflow:

```yaml
permissions:
  contents: read
  security-events: write
```

### Issue: ast-grep not finding sgconfig.yml

**Solution**: Ensure working directory is correct:

```yaml
- name: Run ast-grep
  run: ast-grep scan -c ./sgconfig.yml --json
  working-directory: ${{ github.workspace }}
```

### Issue: CI taking too long

**Solutions**:

1. Check cache is working
2. Review parallel job configuration
3. Add timeout-minutes to jobs

### Issue: Different results locally vs CI

**Solutions**:

1. Ensure same tool versions
2. Check for uncommitted sgconfig.yml changes
3. Verify working directory in CI

## Success Criteria Checklist

- [ ] All CI jobs visible in Actions tab
- [ ] ast-grep findings in Security tab
- [ ] Report-only mode doesn't block PRs
- [ ] SDD validation catches structure issues
- [ ] Cross-platform tests all pass
- [ ] Cache improves build times
- [ ] Clear error messages on failures
- [ ] SARIF format correctly parsed

## Next Steps

After validation:

1. Monitor for 1-2 weeks in report-only mode
2. Address any critical findings
3. Wait for Issue #31 resolution
4. Switch to enforcement mode
5. Communicate change to team

---

⚠️ _Based on validation approach from `.specify/memory/lifecycle.md`_
