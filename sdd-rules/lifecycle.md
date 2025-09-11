# Lifecycle (Authoritative)

This lifecycle defines the end-to-end workflow for every task (human or AI engineer).

Nodes and expected artifacts

1) Requirements analysis
   - Artifact: specs/<NNN>-<slug>/spec.md (WHAT/WHY; acceptance criteria)
   - Includes metadata (Issue-URI, Spec-URI, Evidence-URIs)

2) Product/architecture design
   - Artifact: specs/<NNN>-<slug>/design.md or data-model.md (optional)
   - May include contracts/diagrams/research

3) Technical plan
   - Artifact: specs/<NNN>-<slug>/plan.md (approach, components, trade-offs)

4) Issues
   - Artifact: GitHub Issue with links to Spec/Plan/Tasks/Evidence
   - Mirrors the metadata block from spec.md

5) Development (worktree-first)
   - Branch: feature|fix|perf|chore|docs/<kebab-slug>
   - Worktree from origin/main; follow quality gates

6) Acceptance & submit
   - Evidence placed under dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/
   - PR description links Spec/Plan/Tasks and evidence; includes risks/rollback and CI summary

7) CI (GitHub Actions + Code Scanning)
   - Rust gates (fmt/clippy/test)
   - JSONL replay (if scenarios exist)
   - SDD structure + language checks

8) PR review & merge
   - CODEOWNERS review; squash merge into main

9) Task end
   - Update specs/<NNN>-<slug>/tasks.md status
   - Close Issue; run drift checks if needed

References

- CONTRIBUTING.md
- sdd-rules/spec-template.md, sdd-rules/plan-template.md, sdd-rules/tasks-template.md
