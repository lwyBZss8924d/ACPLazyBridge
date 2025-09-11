# Tasks Template

Metadata
- Issue-URI: <https://github.com/<owner>/<repo>/issues/<id>>
- Spec-URI: specs/<NNN>-<slug>/spec.md
- Plan-URI: specs/<NNN>-<slug>/plan.md
- Tasks-URI: specs/<NNN>-<slug>/tasks.md
- Evidence-URIs: dev-docs/review/_artifacts/{tests,logs,jq,reports}/<NNN>-<slug>/

1. Task Breakdown
- [ ] T1: ... (owner, estimate)
- [ ] T2: ...

2. Validation Checklist
- [ ] CI (fmt/clippy/test) green
- [ ] JSONL scenarios replay (if present)
- [ ] Evidence files saved under _artifacts
- [ ] PR description includes Spec/Plan/Tasks/Evidence links and risk/rollback

3. Post-merge
- [ ] Clean up worktree
- [ ] Update tasks.md status to Done
- [ ] Close Issue

