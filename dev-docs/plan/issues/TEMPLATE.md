# ISSUE Template (Humans and AI Developers should submit tasks to the repository using this template)

Title: <module> - <serial number> - <sentence objective>

## Metadata

---

- Updated at: <updated_at>
- Issue status: "waiting" / "open" / "closed"
- Issue number: [#<issue_number>] is open on Github's issue # Number tracker
- Issue title: <sentence objective>
- Issue URL: example: [Normalize JSONL fixtures to ACP v1 protocolVersion (1)](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/14)
- Issue type: "Engineering task" / "Feature request" / "docs" / "ci" / "Bug report" etc.
- Issue owner: "github_user_name", "claude", "claude[bot]", "warp-agent", "example_ai_developer_agent_name"
- Task Worktree directory: <task_worktree_directory>
- Task Feature branch: <task_feature_branch>
- Linked plan issue file: <linked_plan_issue_file>
- Implementation commit: <implementation_commit>

---

## Background / Requirement
- Explain the problem and scope (reference REQ/ARC/SPEC)

⚠️ ACPLazyBridge related interface design & implementation must strictly follow ACP specifications & check Codex CLI parameters!
**ACP-DocsAndSourceCodeReference**: [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)

## Technical Solution
- Design and implementation points (interface/data structure/concurrency/error handling/logging)

## Refs
- Repo url: <repo_url>
- Docs url/path: <docs_url>
- Code url/path: <code_url>
- Examples url/path: <examples_url>

## Corresponding dev-docs/review items
- SPEC: ...
- REQ: ...
- ARC: ...
- CODEX: ...
- ZED: ...

## Acceptance Criteria
- Test cases: dev-docs/review/_artifacts/tests/<file>.jsonl
- Log evidence: dev-docs/review/_artifacts/logs/<run_yyyymmdd_hhmmss>.log + jq filter script

## Worktree-first
- Branch: feature/<module>-<serial number>
- Initialize: git worktree add ../<module>-<serial number> origin/main -b feature/<module>-<serial number>
- Merge: via PR to main repo; ensure traceability.csv is updated to Verified/Partial, no orphan entries
