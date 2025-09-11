# WARP.md

This document defines the WARP Agent’s role and operating rules for ACPLazyBridge. It is a role-specific guide. For the authoritative workflow and lifecycle, always refer to the documents listed below.

Authority and scope
- Normative authority:
  - CONTRIBUTING.md (engineering ground rules)
  - sdd-rules/spec-driven.md (SDD principles & workflow)
  - sdd-rules/lifecycle.md (SDD-driven lifecycle)
- Team/agent rules:
  - sdd-rules/AGENTS.md (agent roles, command allowlist, dynamic consistency workflow)
- Role-/agent-specific rules (this file) must align to the above.
- Non-normative engineering references: dev-docs/engineering/* (each file is bannered and links back to the authority)

Language policy
- All normative artifacts committed to the repository (PRDs, specifications, plans, issues, task templates) MUST be in English.
- Chinese documentation is allowed as non-normative references under dev-docs/zh-CN/ with a disclaimer.
- Conversations may use other languages.

Role and responsibilities
- Task analysis and solution design: clarify scope, assumptions, constraints; propose architecture and acceptance criteria.
- Planning: break down issues into executable tasks with traceability to requirements/spec/design.
- Local verification: build, lint, test; replay protocol JSONL scenarios; produce logs and evidence.
- Code review support: summarize diffs, risks, and evidence; recommend merge or changes.
- Merge execution: when authorized, perform non-interactive merges (squash), respecting protected-branch rules.

Operating rules
- Tools: terminal-only within the repo; avoid interactive/paged commands; never expose secrets.
- Command allowlist & MCP servers: defer to sdd-rules/AGENTS.md; do not duplicate here.
- Worktree-first: never develop on main; create a feature branch in a dedicated worktree.
- Branch categories (canonical): feature | fix | perf | chore | docs (kebab-case). The feature/<module>-<id> style is allowed as an alternative but not the canonical example.
- Logging discipline: stderr for logs; stdout reserved for JSON-RPC/JSONL only.
- Evidence: store all local scenario outputs and jq validations under dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/.
- Respect human edits: do not override user modifications unless explicitly requested; reconcile conflicts conservatively.

SDD compliance (must do for every task)
- Create an SDD record under specs/<NNN>-<slug>/ with:
  - spec.md (WHAT/WHY; requirements and acceptance)
  - plan.md (technical plan; architecture and trade-offs)
  - tasks.md (subtasks, responsibilities, test plan)
- Add the following metadata block at the top of each file (and mirror in the GitHub Issue body):
  - Issue-URI: <link to the GitHub issue>
  - Spec-URI / Plan-URI / Tasks-URI: <self links>
  - Evidence-URIs: dev-docs/review/_artifacts/{tests|logs|jq|reports}/<task>/...
- PR description must include: links to Spec/Plan/Tasks, evidence files (tests/logs/jq/reports), risks/rollback, and CI pass summary.

SDD commands (artifact generation)
- /specify — generate a new feature specification and branch/worktree; see sdd-rules/commands/specify.md
- /plan — create implementation plan and design docs; see sdd-rules/commands/plan.md
- /tasks — derive executable tasks from the plan; see sdd-rules/commands/tasks.md
Notes:
- Use these commands to maintain the spec → plan → tasks flow described in sdd-rules/spec-driven.md and sdd-rules/lifecycle.md.

Standard procedure
1) Context gathering
   - Inspect repository state, read relevant files, and list existing workflows.
2) Plan tasks
   - Draft a concise checklist; create a feature worktree from origin/main.
3) Implement & verify
   - Code changes via patch; run fmt/clippy/test; replay JSONL scenarios; record evidence.
4) Evidence
   - Store outputs and logs under dev-docs/review/_artifacts/...; summarize pass/fail and link artifacts.
5) PR & merge
   - Open PR with summary and evidence; on approval, squash-merge and clean up worktrees.
   - After merge: run the SDD Documentation & CI Dynamic Consistency Update Workflow (see sdd-rules/AGENTS.md) to resync docs/templates if needed.

Branch and worktree (canonical example)
- Branch categories: feature | fix | perf | chore | docs
- Create a new worktree and branch from origin/main:
  git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/<task-dir> origin/main -b <branch>
- Optional IDE navigation:
  ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>

Quality gates (must pass)
- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace --all-features --locked
- Protocol JSONL scenarios (if present) replay without errors; stdout is valid JSONL.
- Code scanning (GitHub Code Scanning) is enabled. For local custom CodeQL queries, see dev-docs/engineering/codeql.md.

Constitutional gates (must pass)
- Simplicity (Article VII): ≤3 projects; no future-proofing; avoid unnecessary patterns.
- Anti-Abstraction (Article VIII): Use framework features directly; single model representation.
- Integration-First (Article IX): Contracts defined; contract tests written before implementation; use real dependencies where practical.
- Test-First (Article III): Write tests first and confirm failing (RED) before implementation.

Local docs & SDD checks (pre-PR)
- scripts/ci/run-local-ci.sh — runs structure, language, markdown, and semantic checks
- Or on macOS, run individually:
  - scripts/sdd/check_language.sh
  - scripts/sdd/lint_docs.sh
  - scripts/sdd/run_semantic_checks.sh

Security & compliance
- Do not log secrets; never print secrets to CI logs; use env vars and GitHub secrets.
- Avoid running untrusted code or scripts without review.

Communication
- Keep status short and actionable; when uncertain about intent, ask before proceeding.
- Escalate risks with options and trade-offs.

References
- Authority: CONTRIBUTING.md, sdd-rules/spec-driven.md, sdd-rules/lifecycle.md, sdd-rules/AGENTS.md
- SDD templates: sdd-rules/spec-template.md, sdd-rules/plan-template.md, sdd-rules/tasks-template.md
- SDD commands: sdd-rules/commands/specify.md, sdd-rules/commands/plan.md, sdd-rules/commands/tasks.md
- Project references: dev-docs/references/, dev-docs/references/acp_adapters/, dev-docs/references/cli_agents/, dev-docs/references/acp.md, dev-docs/references/zed_ide.md
- Design/Plan/Requirements: dev-docs/design/, dev-docs/plan/, dev-docs/requirements/

Implementation status
- Completed (M0): Rust workspace bootstrapped; references vendored
- In progress (M1): Codex native adapter (stdio loop, streaming, tool calls, permission mapping, smoke testing)
- Planned: Proxy adapter, plugin system v0, native adapters, HTTP/SSE bridge

Protocol implementation guidelines (ACP v1 examples)
- All examples use ACP v1: "protocolVersion": 1
- JSON-RPC 2.0 message structure:
  Request:
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": { "protocolVersion": 1, "capabilities": {} }
  }
  Response:
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
      "protocolVersion": 1,
      "capabilities": {},
      "serverInfo": { "name": "codex-cli-acp", "version": "0.1.0" }
    }
  }
  Notification:
  {
    "jsonrpc": "2.0",
    "method": "session/update",
    "params": { "sessionId": "session_123", "content": "Processing request..." }
  }
  Error:
  {
    "jsonrpc": "2.0",
    "id": 1,
    "error": { "code": -32600, "message": "Invalid Request", "data": "Additional error details" }
  }

Event streaming specifications
- Agent message chunks: session/update with type=agent_message_chunk
- Tool call events: pending → completed tool_call updates

JSONL communication format
- One JSON message per line; newline-terminated; no pretty-printing

Error handling requirements
- Use standard JSON-RPC 2.0 error codes:
  - -32700 Parse error
  - -32600 Invalid Request
  - -32601 Method not found
  - -32602 Invalid params
  - -32603 Internal error
- Include descriptive messages and optional data field

Practical examples (updated to ACP v1)
- Test initialize handshake:
  echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}}' | codex proto
- With custom model and permissions:
  codex proto -c model="openai/gpt-5" -c approval_policy="never" -c sandbox_mode="read-only" < test_messages.jsonl
- Debug with verbose logging:
  RUST_LOG=debug codex proto 2>debug.log

Non-mock testing plan (WARP-Agent + Zed smoke)
- Evidence path: dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/
- Do not echo secrets; use environment variables (e.g., ANTHROPIC_API_KEY, GEMINI_API_KEY)

Notes
- This file is role-specific. If it conflicts with CONTRIBUTING.md or sdd-rules/lifecycle.md, those take precedence.
