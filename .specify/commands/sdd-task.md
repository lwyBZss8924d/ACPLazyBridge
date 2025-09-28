---
name: sdd-task
description: Initialize SDD task from GitHub Issue (supports issue number or full URL)
argument-hint: <issue-number|issue-url> [additional-context]
allowed-tools:
  Bash(gh issue view:*), Bash(git worktree:*), Bash(git checkout:*), Bash(git branch:*), Bash(git
  status:*), Bash(scripts/sdd/*:*), Read, Write, Edit, MultiEdit, Glob, Grep, Task, TodoWrite
---

📌 When any AI-Engineer SDD-TASKs Cooking Workflow can follow the BASELINE TEMPLATES work in (specs/): [AI-Engineer-SDD-Workflow-Baseline-templates](.specify/memory/AI-Engineer-SDD-Workflow-Baseline-templates.txt)

## SDD Task Initialization

You are initializing an SDD task from GitHub issue: $1

Additional context provided: $ARGUMENTS

### Your Task

**Step 1: Fetch Issue Details** First, fetch the GitHub issue details by running:

```bash
gh issue view "$1" --json title,body,number,url,state,labels
```

Note: The `$1` argument accepts both issue numbers (e.g., "28") and full GitHub URLs.

**Step 2: Check Repository State** Run these commands to understand the current state:

- `git branch --show-current` - to see current branch
- `git status --short` - to check for uncommitted changes

**Step 3: Analyze the Issue** Based on the issue details:

- Identify the issue type from labels (enhancement, bug, documentation, etc.)
- Determine the appropriate branch prefix (feature, fix, docs, chore, perf)
- Extract the issue number for the branch name

**Step 4: Create Worktree and Branch** Create a new worktree following the naming convention:

```bash
git worktree add ../acplb-worktrees/<NNN-slug> origin/main -b <branch-type>/<NNN-slug>
```

Where:

- NNN = issue number (e.g., 028 for issue #28)
- slug = short descriptive name from issue title
- branch-type = feature | fix | perf | chore | docs

**Step 5: Initialize SDD Workflow** Follow the Specification-Driven Development workflow. You MUST
**deeply reason** about how to advance the SDD tasks:

### Phase 1: Specification (/specify)

Generate specification using `/specify` command:

- Extract requirements from issue body
- Mark unclear aspects with [NEEDS CLARIFICATION]
- Focus on WHAT and WHY, not HOW
- Use @.specify/templates/spec-template.md structure
- Store in `specs/[NNN-slug]/spec.md`

### Phase 2: Planning (/plan)

Create implementation plan using `/plan` command:

- Technical approach and architecture
- Constitutional compliance check
- Generate: research.md, data-model.md, contracts/, quickstart.md
- Use @.specify/templates/plan-template.md structure
- Store in `specs/[NNN-slug]/plan.md`

### Phase 3: Tasks (/tasks)

Generate executable tasks using `/tasks` command:

- TDD-ordered task list
- Parallel execution markers [P]
- Dependency tracking
- Use @.specify/templates/tasks-template.md structure
- Store in `specs/[NNN-slug]/tasks.md`

### Important References

You MUST follow these SDD documents:

- Constitution: @.specify/memory/constitution.md
- Lifecycle: @.specify/memory/lifecycle.md
- SDD Rules: @sdd-rules/rules/README.md

### Metadata Template

Create this metadata block for all SDD artifacts:

```yaml
worktree: [WORKTREE-PATH]
feature_branch: [###-feature-name]
created: [UTC-DATE-TIME]
last_updated: [UTC-DATE-TIME]
status: [STATUS]
input: User description: "$ARGUMENTS"
issue_uri: [ISSUE-LINK]
specs:
    constitution: [CONSTITUTION-VERSION]
    type: [SPECS-TYPE]
    feature_number: [FEATURE-NUMBER]
```

### Execution Guidelines

1. **Use TodoWrite tool** to track your progress through each phase
2. **Validate against SDD rules** at each step
3. **Ensure constitutional compliance** (Test-First, Library-First, etc.)
4. **Create evidence artifacts** in `_artifacts/[NNN-slug]/`
5. **Run validation scripts** after creating each SDD document

### Usage Examples

This command accepts:

- Issue number: `/sdd-task 036`
- GitHub URL: `/sdd-task https://github.com/lwyBZss8924d/ACPLazyBridge/issues/28`
- With context: `/sdd-task 036 "focus on performance optimization"`

---

// ⚠️ _Whether initializing, modifying, or updating this SDD TASKs file (specs/xxxx-xxxx/tasks.md), YOU MUST ULTRATHINK Analyze ISSUES [input] then edit the template file for the task first!_
// ⚠️ MUST follow and get UTC time NOW! `{{YYYY-MM-DD}}T{{HH:MM:SS}}Z` (date -u '+%Y-%m-%dT%H:%M:%SZ') first for specs file in the header's metadata-date yaml code block.
// ⚠️ MUST Update worktree with last local main as the tree directory for ISSUES(#xx) --> NEW SDD-TASKS(#xx)
// ⚠️ Always use local main to create the SDD TASKs initialization image task tree for the worktree, and create a symbolic link with the main worktree directory at: (/dev-space/ACPLazyBridge/.worktrees)
