#!/bin/bash
set -euo pipefail

# Configuration
REPO="/Users/arthur/dev-space/ACPLazyBridge"
TS="$(date +%Y%m%d_%H%M%S)"
LOG_ROOT="$REPO/_artifacts/logs/docs-sync-$TS"

# Create log directory
mkdir -p "$LOG_ROOT"
echo "=== Starting docs sync to worktrees at $(date) ===" | tee "$LOG_ROOT/run.log"
echo "Log directory: $LOG_ROOT" | tee -a "$LOG_ROOT/run.log"

# Part 1: Commit and push on main
echo "" | tee -a "$LOG_ROOT/run.log"
echo "=== Part 1: Commit and push changes on main ===" | tee -a "$LOG_ROOT/run.log"

# Verify we're on main
cd "$REPO"
CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "ERROR: Not on main branch (currently on $CURRENT_BRANCH)" | tee -a "$LOG_ROOT/run.log"
    exit 1
fi

# Record initial state
git remote -v > "$LOG_ROOT/remote.txt" 2>&1
git status --porcelain=v2 > "$LOG_ROOT/main-status-before.txt" 2>&1

# Fetch latest
echo "Fetching origin..." | tee -a "$LOG_ROOT/run.log"
git fetch origin --prune 2>&1 | tee -a "$LOG_ROOT/run.log"

# Stage the files
echo "Staging files..." | tee -a "$LOG_ROOT/run.log"
git add dev-docs/plan/issues/TEMPLATE.md 2>&1 | tee -a "$LOG_ROOT/run.log"
git add dev-docs/plan/issues/open/normalize-jsonl-protocol-v1.md 2>&1 | tee -a "$LOG_ROOT/run.log"
git add dev-docs/plan/issues/open/feature-codex-notify-1.md 2>&1 | tee -a "$LOG_ROOT/run.log" || true

# Show staged changes
git diff --staged --name-status > "$LOG_ROOT/main-staged-diff.txt" 2>&1

if [ -s "$LOG_ROOT/main-staged-diff.txt" ]; then
    echo "Committing changes..." | tee -a "$LOG_ROOT/run.log"
    cat "$LOG_ROOT/main-staged-diff.txt" | tee -a "$LOG_ROOT/run.log"
    
    # Commit with no-edit
    GIT_MERGE_AUTOEDIT=no git commit -m "docs(issues): update TEMPLATE to English, normalize-jsonl type to ci, add feature-codex-notify-1" 2>&1 | tee -a "$LOG_ROOT/run.log"
    git show --stat -1 > "$LOG_ROOT/main-commit.txt" 2>&1
    
    # Push to origin
    echo "Pushing to origin/main..." | tee -a "$LOG_ROOT/run.log"
    git push origin HEAD:main 2>&1 | tee -a "$LOG_ROOT/run.log"
    git fetch origin --prune 2>&1
    git rev-parse origin/main > "$LOG_ROOT/origin-main-after-push.txt" 2>&1
else
    echo "No changes to commit on main" | tee -a "$LOG_ROOT/run.log"
fi

# Part 2: Sync worktrees
echo "" | tee -a "$LOG_ROOT/run.log"
echo "=== Part 2: Sync worktrees with origin/main ===" | tee -a "$LOG_ROOT/run.log"

# Define worktree targets
WORKTREE_TARGETS="/Users/arthur/dev-space/acplb-worktrees/codex-notify-1|feature/codex-notify-1
/Users/arthur/dev-space/acplb-worktrees/codex-tools-1|pr/codex-tools-1-scope
/Users/arthur/dev-space/acplb-worktrees/docs-usage-1|docs/usage-1
/Users/arthur/dev-space/acplb-worktrees/tests-jsonl-1|chore/tests-jsonl-1"

# Create summary table
SUMMARY_TSV="$LOG_ROOT/worktree-summary.tsv"
echo -e "path\tbranch\tbefore\tafter\tmerge_result\tstash_used\tstash_restore\tstatus" > "$SUMMARY_TSV"

# Process each worktree
echo "$WORKTREE_TARGETS" | while IFS="|" read -r WT_PATH WT_BRANCH; do
    echo "" | tee -a "$LOG_ROOT/run.log"
    echo "=== Syncing $WT_PATH ($WT_BRANCH) ===" | tee -a "$LOG_ROOT/run.log"
    
    if [ ! -d "$WT_PATH" ]; then
        echo -e "$WT_PATH\t$WT_BRANCH\tNA\tNA\tmissing_worktree\tno\tno\tSKIPPED" >> "$SUMMARY_TSV"
        echo "SKIPPED: Directory does not exist" | tee -a "$LOG_ROOT/run.log"
        continue
    fi
    
    # Fetch in worktree
    git -C "$WT_PATH" fetch origin --prune 2>&1 | tee -a "$LOG_ROOT/run.log"
    
    # Ensure we're on the right branch
    CUR_BRANCH="$(git -C "$WT_PATH" rev-parse --abbrev-ref HEAD)"
    if [ "$CUR_BRANCH" != "$WT_BRANCH" ]; then
        echo "Checking out $WT_BRANCH..." | tee -a "$LOG_ROOT/run.log"
        git -C "$WT_PATH" checkout -q "$WT_BRANCH" 2>&1 | tee -a "$LOG_ROOT/run.log"
    fi
    
    # Record before state
    BEFORE="$(git -C "$WT_PATH" rev-parse --short HEAD)"
    echo "Before: $BEFORE" | tee -a "$LOG_ROOT/run.log"
    
    # Check for uncommitted changes
    DIRTY="$(git -C "$WT_PATH" status --porcelain)"
    STASH_REF=""
    STASH_USED="no"
    STASH_RESTORED="NA"
    
    if [ -n "$DIRTY" ]; then
        STASH_USED="yes"
        echo "Stashing uncommitted changes..." | tee -a "$LOG_ROOT/run.log"
        git -C "$WT_PATH" stash push -u -m "docs-sync-$TS" 2>&1 | tee -a "$LOG_ROOT/run.log"
        STASH_REF="$(git -C "$WT_PATH" stash list | head -n1 | cut -d: -f1 || true)"
    fi
    
    # Attempt merge
    MERGE_RESULT="unknown"
    echo "Merging origin/main..." | tee -a "$LOG_ROOT/run.log"
    
    set +e
    git -C "$WT_PATH" merge --no-edit -X ours origin/main 2>&1 | tee -a "$LOG_ROOT/run.log"
    MERGE_EXIT=$?
    set -e
    
    if [ $MERGE_EXIT -eq 0 ]; then
        MERGE_RESULT="merged_or_ff"
        echo "Merge successful" | tee -a "$LOG_ROOT/run.log"
    else
        MERGE_RESULT="conflict_abort"
        echo "Merge conflict, aborting..." | tee -a "$LOG_ROOT/run.log"
        git -C "$WT_PATH" merge --abort 2>&1 | tee -a "$LOG_ROOT/run.log" || true
    fi
    
    # Restore stash if needed
    if [ "$STASH_USED" = "yes" ]; then
        if [ -n "$STASH_REF" ]; then
            echo "Restoring stashed changes..." | tee -a "$LOG_ROOT/run.log"
            if git -C "$WT_PATH" stash apply -q "$STASH_REF" 2>/dev/null; then
                STASH_RESTORED="applied_and_kept"
                git -C "$WT_PATH" stash drop -q "$STASH_REF" 2>/dev/null || true
                echo "Stash restored successfully" | tee -a "$LOG_ROOT/run.log"
            else
                STASH_RESTORED="apply_conflicts_left_for_manual_resolution"
                echo "WARNING: Stash conflicts, left for manual resolution" | tee -a "$LOG_ROOT/run.log"
            fi
        else
            STASH_RESTORED="missing_ref"
        fi
    fi
    
    # Record after state
    AFTER="$(git -C "$WT_PATH" rev-parse --short HEAD || echo NA)"
    STATUS_SUMMARY="$(git -C "$WT_PATH" status --porcelain | wc -l | tr -d ' ')"
    
    echo "After: $AFTER, Changed paths: $STATUS_SUMMARY" | tee -a "$LOG_ROOT/run.log"
    echo -e "$WT_PATH\t$WT_BRANCH\t$BEFORE\t$AFTER\t$MERGE_RESULT\t$STASH_USED\t$STASH_RESTORED\t${STATUS_SUMMARY}_changed_paths" >> "$SUMMARY_TSV"
    
    # Save per-worktree evidence
    WT_SLUG="$(basename "$WT_PATH")"
    git -C "$WT_PATH" log -3 --oneline > "$LOG_ROOT/${WT_SLUG}-last3.txt" 2>&1 || true
    git -C "$WT_PATH" diff --name-status origin/main...HEAD -- dev-docs/ > "$LOG_ROOT/${WT_SLUG}-docs-diff.txt" 2>&1 || true
done

# Final summary
echo "" | tee -a "$LOG_ROOT/run.log"
echo "=== Summary ===" | tee -a "$LOG_ROOT/run.log"
cat "$SUMMARY_TSV" | column -t -s $'\t' | tee -a "$LOG_ROOT/run.log"

# Save summary for review
cat "$SUMMARY_TSV" > "$LOG_ROOT/summary-view.txt"

echo "" | tee -a "$LOG_ROOT/run.log"
echo "=== Sync completed at $(date) ===" | tee -a "$LOG_ROOT/run.log"
echo "Logs saved to: $LOG_ROOT" | tee -a "$LOG_ROOT/run.log"

# Report any issues
if grep -q "conflict_abort" "$SUMMARY_TSV"; then
    echo "" | tee -a "$LOG_ROOT/run.log"
    echo "WARNING: Some worktrees had merge conflicts and need manual resolution" | tee -a "$LOG_ROOT/run.log"
fi

if grep -q "apply_conflicts_left_for_manual_resolution" "$SUMMARY_TSV"; then
    echo "" | tee -a "$LOG_ROOT/run.log"
    echo "WARNING: Some worktrees had stash restore conflicts and need manual resolution" | tee -a "$LOG_ROOT/run.log"
fi
