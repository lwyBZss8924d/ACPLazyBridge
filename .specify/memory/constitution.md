# "ACPLazyBridge" SDD CONSTITUTION

## CORE PRINCIPLES

### PRINCIPLE I. Library-First

Every feature begins as a standalone library or crate. No application-first implementations.
Libraries must be self-contained, testable independently, and documented.
Where applicable, expose a minimal public API with clear semantics.

### PRINCIPLE II. CLI Interface

Each library exposes functionality via a CLI entrypoint when meaningful.
Text I/O contract: accept input via stdin/args/files; produce output on stdout;
diagnostics go to stderr.
Support JSON for structured IO when applicable;
provide human-readable output as needed.

### PRINCIPLE III. Test-First (Non-Negotiable)

Tests are written and validated before implementation (RED → GREEN → REFACTOR).
Contract and integration tests must exist prior to implementation of behavior.
No code is committed without failing tests first demonstrating intent.

### PRINCIPLE IV. Integration Testing

Integration tests are required for:

- New library contracts and any change to existing contracts
- Cross-process or inter-crate workflows
- Protocol/JSONL behaviors and error handling
Prefer contract tests and realistic environments over mocks where practical.

### PRINCIPLE V. Observability

stdout is reserved exclusively for protocol/JSONL or tool output;
all logs go to stderr.
Use structured logging where practical and capture logs as evidence during
CI/local runs.

### PRINCIPLE VI. Versioning & Breaking Changes

Follow semantic versioning for published artifacts and document breaking changes.
Provide migration notes and clear deprecation paths for breaking changes.

### PRINCIPLE VII. Simplicity

Favor the simplest implementation that satisfies requirements.
≤ 3 projects for initial implementation; additional projects require
justification and approval.
No speculative features or premature abstractions.

### PRINCIPLE VIII. Anti-Abstraction

Prefer framework/library features directly; avoid unnecessary wrappers and indirection.
Maintain a single model representation; avoid parallel model hierarchies unless justified.

### PRINCIPLE IX. Integration-First

Define contracts before implementation; write contract tests first.
Prefer real dependencies and realistic IO paths where practical during testing.

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema

## ADDITIONAL CONSTRAINTS

### AUTHORITY AND PRECEDENCE

- This Constitution is authoritative for project-wide SDD governance.
- Rule precedence: project rules with file paths override personal rules;
  subdirectory rules override parent directory rules; when multiple rules conflict,
  the later rule in the ordered list takes precedence.
- Normative artifacts (specifications/plans/tasks/issues/PRDs/commits) must be
  written in English.

### LANGUAGE POLICY

Normative artifacts must be English-only. Non-normative references may be
multilingual with clear disclaimers.

### SECURITY & COMPLIANCE

Never log secrets or print them in CI; use environment variables and GitHub secrets.
Avoid running untrusted code or scripts without review.

## DEVELOPMENT WORKFLOW, REVIEW PROCESS, QUALITY GATES

### QUALITY GATES (must pass)

- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace --all-features --locked
- Protocol JSONL scenarios (if present) replay without errors; stdout is valid
  JSONL; logs to stderr.
- Security scanning is enabled.

### PRE-PR CHECKS

- scripts/ci/run-local-ci.sh (or component scripts):
    - scripts/ci/run-sdd-structure-lint.sh
    - scripts/ci/check-language-policy.sh
    - scripts/ci/run-markdown-style.sh
    - scripts/sdd/run_semantic_checks.sh

### BRANCHING & WORKTREES

- Worktree-first: never develop on main; use feature/fix/perf/chore/docs/<kebab-slug>.
- Canonical worktree command:

  ```bash
  git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/<task-dir> origin/main -b <branch>
  ```

### EVIDENCE & LOGGING

- Store evidence under _artifacts/{tests,logs,jq,reports}/<task>/.
- Keep logs and test outputs with timestamps for reproducibility.

## GOVERNANCE

This Constitution supersedes all other practices in case of conflict.
Amendments require documentation, approval, and a migration plan.

### GOVERNANCE RULES

- All PRs/reviews must verify compliance
- Complexity must be justified

## OPERATIONAL IMPLEMENTATION

  For the detailed SDD workflow implementation of these principles, see:

- [Lifecycle](.specify/memory/lifecycle.md) - Authoritative workflow
  definition
- [Commands](.specify/commands/) - Command implementations
- [Templates](.specify/templates/) - Document templates

📌 Wen any AI-Engineer SDD-TASKs Cooking Workflow can follow the BASELINE TEMPLATES work in (specs/): [AI-Engineer-SDD-Workflow-Baseline-templates](.specify/memory/AI-Engineer-SDD-Workflow-Baseline-templates.txt)

---

```yaml
constitution:
    version: "1.0.1"
    ratified: "2025-09-15"
    last_amended: "2025-09-15"
document:
    type: "constitution"
    path: ".specify/memory/constitution.md"
    version: "1.0.1"
    last_updated: "2025-09-27T10:14:00Z"
```
