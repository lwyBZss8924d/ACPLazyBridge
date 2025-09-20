# Pull Request Template

## Summary

- What does this change do and why?

## Linked Issues

- Closes: <link to dev-docs/plan/issues item or GitHub issue>

## Design & Changes

- Key design points / trade-offs
- Affected modules / crates

## Tests & Evidence

- Commands run:
    - cargo fmt / clippy / test results
    - JSONL scenarios executed (list files)
- Evidence:
    - Output logs / snapshots (link to _artifacts or CI Artifacts)

## Risk & Rollback

- Risks and mitigations
- Rollback plan (revert/switch off)

## Checklist

- [ ] Worktree-first (developed in dedicated worktree/branch from origin/main)
- [ ] Branch uses canonical category (feature|fix|perf|chore|docs)
- [ ] Spec/Plan/Tasks links included (specs/<NNN>-<slug>/...)
- [ ] Evidence uses dev-docs/review/_artifacts/{tests,logs,jq,reports}/
- [ ] cargo fmt --all -- --check passes
- [ ] cargo clippy --workspace --all-targets --all-features -- -D warnings passes
- [ ] cargo test --workspace --all-features --locked passes
- [ ] Protocol stdout is strict JSONL; logs go to stderr
- [ ] ACP examples (if any) use protocolVersion: 1
- [ ] Risks & rollback section completed
- [ ] Docs updated (CONTRIBUTING.md/CLAUDE.md/WARP.md/dev-docs)

---

Based on Constitution: 1.0.1 | (.github/PULL_REQUEST_TEMPLATE.md) : 1.0.1 | Last Updated: 2025-09-16
