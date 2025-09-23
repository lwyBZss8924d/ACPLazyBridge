# Issue Draft Template (SDD-Aligned)

```yaml
Issue-URI: <https://github.com/<org>/<repo>/issues/XXX>
Spec-URI: specs/<NNN>-<slug>/spec.md
Plan-URI: specs/<NNN>-<slug>/plan.md
Tasks-URI: specs/<NNN>-<slug>/tasks.md
Evidence-URIs: _artifacts/<task>/{tests,logs,jq,reports}/
Worktree: /Users/arthur/dev-space/acplb-worktrees/<branch>
Branch: feature/<slug>
```

## 1. Summary

- **Category**: feature | fix | perf | chore | docs
- **Problem Statement**: _Briefly describe the user or system need._
- **Expected Outcome**: _Describe the measurable result once the task is done._

## 2. Background & References

- ACP spec links or local references (e.g., `references/acp.md`)
- Architecture/Runtime docs (e.g., `dev-docs/architecture/acplb-architecture.md`)
- Related issues/PRs (closed or in progress)

## 3. Requirements & Scope

- Functional requirements (map to FR-XXXX/NFR-XX identifiers when possible)
- Non-functional constraints (latency, security, observability, etc.)
- Explicit out-of-scope items to avoid scope creep

## 4. Acceptance Criteria

- [ ] Criterion 1 (e.g., “JSONL playback `handshake.jsonl` succeeds”)
- [ ] Criterion 2 (e.g., “Tool-call updates use official ACP schema”)
- [ ] Criterion 3

## 5. Implementation Plan

1. RED: write failing tests / JSONL scenarios
2. GREEN: implement minimal code to satisfy tests
3. REFACTOR: clean up, document, and harden
4. Update documentation (README, dev docs, changelog if needed)
5. Capture evidence under `_artifacts/<task>/...`

### Work Breakdown (optional table)

| Step | Owner | Notes |
| ---- | ----- | ----- |
| 1 |  | |
| 2 |  | |

## 6. Evidence & Test Plan

- Unit tests: `cargo test -p <crate> -- <filter>`
- Integration tests / JSONL replays: `_artifacts/tests/<task>/...`
- Logs & reports: `_artifacts/logs/<task>/...`, `_artifacts/reports/<task>/...`
- Additional validation (e.g., `scripts/ci/run-local-ci.sh`)

### Quality Gates Checklist

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features --locked`
- [ ] JSONL scenarios replay without errors (stdout JSONL, logs to stderr)
- [ ] `scripts/ci/run-local-ci.sh`
- [ ] `_artifacts/<task>/` populated (logs/tests/jq/reports)
- [ ] Language policy (English-only normative docs)
- [ ] Markdown style & semantic lint pass
- [ ] Security scan / secrets check (where applicable)

### Constitutional Gates (confirm before close)

- [ ] Article I (Library-First) – reuse or extend existing crates before new binaries
- [ ] Article III (Test-First) – failing test exists before implementation
- [ ] Article VII (Simplicity) – ≤3 projects touched; no unnecessary abstractions
- [ ] Article VIII (Anti-Abstraction) – framework features used directly
- [ ] Article IX (Integration-First) – contracts/tests defined prior to integration

## 7. Risks & Rollback

- Risks:
    - Risk A – mitigation
    - Risk B – mitigation
- Rollback strategy:
  1. Revert PR / feature branch
  2. Restore environment state (database, config, etc.)
  3. Verify baseline tests

## 8. Dependencies & Follow-Ups

- Depends on: Issue/PR
- Blocks: Issue/PR
- External dependencies (CLI versions, SDK updates)
- Follow-up work (future milestone or tech debt)

## 9. Notes

- Additional context, open questions, or links to design discussions

---
**Filing instructions**: When creating a GitHub issue, copy the relevant sections into the issue body or reference this draft. Ensure all URIs are replaced with real links before submission.
