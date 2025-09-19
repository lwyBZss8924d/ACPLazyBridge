# CI Enforcement Mode Transition Plan

## Overview

This document outlines the transition plan from report-only mode to enforcement mode for ast-grep scanning in the CI pipeline.

## Current State (Report-Only Mode)

- **Status**: Active as of 2025-09-19
- **Configuration**: `continue-on-error: true` in ast-grep-scan job
- **Behavior**:
    - ast-grep findings are collected and uploaded to GitHub Security tab
    - Violations do NOT block PR merging
    - SARIF reports are visible for review

## Target State (Enforcement Mode)

- **Configuration**: Remove `continue-on-error: true` from ast-grep-scan job
- **Behavior**:
    - ast-grep violations will fail the CI pipeline
    - PRs cannot merge with unresolved findings
    - All code must pass ast-grep rules

## Prerequisites

1. **Issue #31 Resolution**: Must be completed before enforcement
   - Fixes false positives in inline tests
   - Refactors production unwrap()/expect() usage
   - Ensures rules are properly configured

2. **Team Readiness**:
   - All developers aware of ast-grep rules
   - Documentation available for fixing common violations
   - Suppression mechanism understood

## Transition Timeline

### Week 1-2: Monitoring Phase (Current)

- [x] Deploy report-only mode
- [x] Monitor findings in Security tab
- [ ] Track false positive rate
- [ ] Document common violations

### Week 3-4: Communication Phase

- [ ] Send team announcement about upcoming enforcement
- [ ] Share violation fixing guide
- [ ] Provide suppression examples
- [ ] Schedule Q&A session if needed

### Week 5: Pre-enforcement Cleanup

- [ ] Review all open PRs for violations
- [ ] Help teams fix existing issues
- [ ] Validate suppression comments work
- [ ] Final reminder to team

### Week 6: Enforcement Activation

- [ ] Create PR to remove `continue-on-error: true`
- [ ] Verify CI fails appropriately on test branch
- [ ] Merge enforcement PR
- [ ] Monitor for issues

## Activation Steps

1. **Create enforcement branch**:

   ```bash
   git checkout -b feature/enable-ast-grep-enforcement
   ```

2. **Update workflow**:

   ```yaml
   ast-grep-scan:
     runs-on: ubuntu-latest
     # Remove this line to enable enforcement:
     # continue-on-error: true
   ```

3. **Test on branch with violations**:

   ```bash
   # Add test violation
   echo 'fn test() { Some(1).unwrap(); }' >> src/test.rs
   git add . && git commit -m "test: Verify enforcement"
   git push origin feature/enable-ast-grep-enforcement
   # Verify CI fails
   ```

4. **Create PR**:

   ```bash
   gh pr create \
     --title "Enable ast-grep enforcement mode" \
     --body "Removes report-only mode after Issue #31 resolution"
   ```

## Rollback Plan

If enforcement causes unexpected issues:

1. **Immediate rollback**:

   ```yaml
   # Re-add to ast-grep-scan job:
   continue-on-error: true
   ```

2. **Hotfix PR**:

   ```bash
   gh pr create \
     --title "HOTFIX: Restore ast-grep report-only mode" \
     --label "urgent"
   ```

3. **Communication**:
   - Notify team immediately
   - Document issues encountered
   - Plan fixes before re-enabling

## Success Metrics

- **Before Enforcement**:
    - < 5% false positive rate
    - All active PRs reviewed for violations
    - Team trained on suppression

- **After Enforcement**:
    - No emergency rollbacks needed
    - < 10 minutes average fix time for violations
    - Positive team feedback

## Communication Template

### Pre-enforcement Announcement

> **Subject**: ast-grep Enforcement Mode Starting [DATE]
>
> Team,
>
> We will be enabling enforcement mode for ast-grep scanning on [DATE]. After this date, code with ast-grep violations will not be able to merge.
>
> **What you need to know**:
>
> - Current violations are visible in the Security tab
> - Use `// ast-grep-ignore: rule-id` for legitimate exceptions
> - See our fixing guide: [link]
>
> **Timeline**:
>
> - Now - [DATE]: Report-only mode (current)
> - [DATE]: Enforcement begins
>
> Questions? Reach out in #dev-channel

### Post-enforcement Confirmation

> **Subject**: ast-grep Enforcement Mode Active
>
> Team,
>
> ast-grep enforcement is now active. All PRs must pass ast-grep checks to merge.
>
> **Resources**:
>
> - Fixing guide: [link]
> - Suppression docs: [link]
> - Support: #dev-channel
>
> Thank you for maintaining code quality!

## Related Documents

- Issue #31: Fix ast-grep false positives
- Issue #32: CI implementation (this feature)
- `specs/035-ci-add-sdd/enforcement.md`: Enforcement procedures
- `sdd-rules/rules/code-analysis/sdd-rules-code-analysis.md`: ast-grep rules

---

⚠️ _This transition plan follows SDD principles from `.specify/memory/constitution.md`_
