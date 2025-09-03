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
- [ ] cargo fmt --all -- --check passes
- [ ] cargo clippy --workspace --all-targets --all-features -- -D warnings passes
- [ ] cargo test --workspace --all-features --locked passes
- [ ] Protocol stdout is strict JSONL; logs go to stderr
- [ ] Evidence attached or linked
- [ ] Docs updated (CONTRIBUTING.md/CLAUDE.md/WARP.md/dev-docs)

