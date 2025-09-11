# Git Worktrees for Parallel Development

Git worktrees enable multiple working directories from a single repository, allowing human and AI engineers to work on different features simultaneously without workspace conflicts.

## Overview

Worktrees provide isolated development environments where each feature, bug fix, or experiment has its own:
- Working directory
- Branch checkout
- File state
- Agent session
- Dependencies (when initialized)

## Basic Commands

### Creating Worktrees

```bash
# Create new worktree with new branch
git worktree add ../spec-kit-feature-auth -b 001-auth

# Create worktree from existing branch
git worktree add ../spec-kit-bugfix bugfix/security-patch

# Create worktree at specific commit
git worktree add ../spec-kit-experiment HEAD~3
```

### Managing Worktrees

```bash
# List all worktrees
git worktree list

# Show detailed worktree information
git worktree list --verbose

# Remove worktree (clean)
git worktree remove ../spec-kit-feature-auth

# Remove worktree (force)
git worktree remove --force ../spec-kit-broken

# Prune stale worktree information
git worktree prune
```

### Moving Worktrees

```bash
# Move worktree to new location
git worktree move ../spec-kit-old ../spec-kit-new

# Repair worktree references after move
git worktree repair
```

## Naming Conventions

### Directory Names
```
Pattern: ../spec-kit-<type>-<id>-<description>

Types:
- feature: New functionality
- bugfix: Bug repairs  
- hotfix: Emergency fixes
- experiment: Exploratory work
- review: PR reviews
- release: Release preparation
```

### Examples
```bash
../spec-kit-feature-001-auth         # Feature: authentication
../spec-kit-bugfix-002-validation    # Bug fix: validation error
../spec-kit-hotfix-003-security      # Hotfix: security patch
../spec-kit-experiment-ml-pipeline   # Experiment: ML pipeline
../spec-kit-review-pr-123           # Review: PR #123
../spec-kit-release-v1.2.0          # Release: version 1.2.0
```

## SDD Task Workflow

### 1. Create Task Worktree
```bash
# From main repository
cd ~/projects/spec-kit

# Create worktree for new task
git worktree add ../spec-kit-task-001-api origin/main -b task/001-api

# Navigate to worktree
cd ../spec-kit-task-001-api
```

### 2. Initialize Environment
```bash
# Python project
python -m venv venv
source venv/bin/activate
pip install -e .

# Node project
npm install

# Go project
go mod download
```

### 3. Start Agent Session
```bash
# Claude Code
code .  # Opens in VS Code with Claude

# Warp Agent
warp-preview agent run --profile dev \
  -C . \
  --prompt "Implement task 001 from spec"
```

### 4. Development Cycle
```bash
# Regular git operations work normally
git add .
git commit -m "feat: implement API endpoints [TASK-001]"
git push -u origin task/001-api
```

### 5. Clean Up
```bash
# After PR merged
cd ~/projects/spec-kit
git worktree remove ../spec-kit-task-001-api
```

## Parallel Human/AI Tasking

### Scenario: Multiple Features
```bash
# Human works on frontend
git worktree add ../spec-kit-frontend origin/main -b feature/ui
cd ../spec-kit-frontend
# Human develops...

# AI Agent 1 works on API
git worktree add ../spec-kit-api origin/main -b feature/api
cd ../spec-kit-api
claude-code implement specs/002-api/

# AI Agent 2 works on database
git worktree add ../spec-kit-database origin/main -b feature/db
cd ../spec-kit-database
warp-preview agent run --prompt "Implement database schema"

# AI Agent 3 runs tests
git worktree add ../spec-kit-testing origin/main -b feature/tests
cd ../spec-kit-testing
cursor-agent write tests/
```

### Coordination
```bash
# Check all active work
git worktree list
# /Users/dev/spec-kit         abc123 [main]
# /Users/dev/spec-kit-frontend def456 [feature/ui]
# /Users/dev/spec-kit-api      ghi789 [feature/api]
# /Users/dev/spec-kit-database jkl012 [feature/db]
# /Users/dev/spec-kit-testing  mno345 [feature/tests]

# Merge completed work
cd ~/projects/spec-kit
git merge feature/api
git merge feature/db
```

## Safety Guidelines

### Dependency Isolation
```bash
# Each worktree needs its own dependencies
# Bad: Sharing virtual environment
../spec-kit/venv/

# Good: Separate environments
../spec-kit/venv/
../spec-kit-feature/venv/
../spec-kit-bugfix/venv/
```

### State Management
```bash
# Check worktree state before removal
cd ../spec-kit-feature
git status  # Ensure no uncommitted changes
git stash   # Save work if needed
```

### Branch Protection
```bash
# Avoid same branch in multiple worktrees
# This will fail:
git worktree add ../spec-kit-dup -b main  # Error: branch already checked out

# Use different branches
git worktree add ../spec-kit-exp -b experiment/test
```

## Agent Integration

### Claude Code
```bash
# Each worktree gets its own Claude session
cd ../spec-kit-feature-auth
code .  # New VS Code window with Claude

# Claude uses worktree's git context
# Branch: feature/001-auth
# No interference with other worktrees
```

### Warp Agent
```bash
# Specify worktree path
warp-preview agent run \
  -C ../spec-kit-bugfix \
  --profile dev \
  --prompt "Fix validation bug"
```

### Multiple Agents
```bash
# Agent 1 in worktree 1
cd ../spec-kit-task-001
claude-code implement

# Agent 2 in worktree 2  
cd ../spec-kit-task-002
warp-preview agent run

# No conflicts or confusion
```

## Best Practices

### Do's
- Create worktree per task/feature
- Initialize dependencies separately
- Use descriptive worktree names
- Clean up completed worktrees
- Keep main branch in primary repo

### Don'ts
- Share worktrees between agents
- Leave stale worktrees
- Commit from wrong worktree
- Force remove with uncommitted changes
- Create deeply nested worktrees

## Troubleshooting

### Worktree Locked
```bash
# If worktree is locked
git worktree unlock ../spec-kit-feature

# Force unlock if needed
rm ../spec-kit-feature/.git/worktree.lock
```

### Missing Worktree
```bash
# If directory was deleted
git worktree prune

# Repair references
git worktree repair
```

### Branch Conflicts
```bash
# Check what's using branch
git worktree list | grep feature/auth

# Remove conflicting worktree
git worktree remove ../old-worktree
```

## Example Workflows

### Feature Development
```bash
#!/bin/bash
# create-feature-worktree.sh

FEATURE_NAME=$1
FEATURE_BRANCH="feature/$FEATURE_NAME"
WORKTREE_DIR="../spec-kit-$FEATURE_NAME"

# Create worktree
git worktree add "$WORKTREE_DIR" -b "$FEATURE_BRANCH"

# Setup environment
cd "$WORKTREE_DIR"
python -m venv venv
source venv/bin/activate
pip install -e .

# Start development
code .
```

### Parallel Testing
```bash
#!/bin/bash  
# parallel-test.sh

# Create test worktrees
git worktree add ../spec-kit-test-unit HEAD
git worktree add ../spec-kit-test-integration HEAD
git worktree add ../spec-kit-test-e2e HEAD

# Run tests in parallel
(cd ../spec-kit-test-unit && pytest tests/unit/) &
(cd ../spec-kit-test-integration && pytest tests/integration/) &
(cd ../spec-kit-test-e2e && pytest tests/e2e/) &

# Wait for completion
wait
```

### Review Workflow
```bash
#!/bin/bash
# review-pr.sh

PR_NUMBER=$1
WORKTREE_DIR="../spec-kit-review-pr-$PR_NUMBER"

# Fetch PR branch
gh pr checkout $PR_NUMBER --detach

# Create review worktree
git worktree add "$WORKTREE_DIR" HEAD

# Run review
cd "$WORKTREE_DIR"
warp-preview agent run --profile review \
  --prompt "Review changes for SDD compliance"
```

## Performance Considerations

### Disk Usage
- Each worktree has full working directory
- Shared .git repository saves space
- Consider SSD for better performance
- Clean up unused worktrees regularly

### Memory Usage
- Each agent session uses separate memory
- Dependencies loaded per worktree
- Monitor system resources with many worktrees

### Network
- Parallel pushes may hit rate limits
- Coordinate push timing if needed
- Use different remotes if necessary

---

*Version: 1.0.0 | Git Version: 2.40+ recommended | Last Updated: 2025-09-09*