## Contributing to ACPLazyBridge

We're thrilled that you'd (all Human developers and other AI engineers) like to contribute to ACPLazyBridge. Contributions to the public under the [project's open source license](LICENSE).

## Prerequisites for running and testing code

- Rust stable toolchain (rustup) with `cargo`, `rustfmt`, and `clippy`.
- Workspace uses a pinned toolchain (`rust-toolchain.toml`).
- For docs/style checks: Node.js + `markdownlint-cli2` (or use `npx`).
- Optional: `ast-grep` for code scanning; `cargo-tarpaulin` for coverage.
- macOS/Linux shell environment; zsh/bash supported.

Quick checks:

- Build: `cargo build --workspace --all-features`
- Tests: `cargo test --workspace --all-features --locked`
- Format: `cargo fmt --all -- --check`
- Lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- Local CI (structure + language + markdown + semantic): `./scripts/ci/run-local-ci.sh`

## Submitting a pull request

>[!NOTE]
>If your pull request introduces a large change that materially impacts the work of the API / CLI or the rest of the repository (e.g., you're introducing new templates, arguments, or otherwise major changes), make sure that it was **discussed and agreed upon** by the project maintainers. Pull requests with large changes that did not have a prior conversation and agreement will be closed.

Before you open a PR, ensure the following:

- Branch/worktree: do not work on `main`.
    - Create a branch from `origin/main` using one of: `feature|fix|perf|chore|docs`.
    - Prefer worktree flow (example path shown in docs): see `.specify/memory/constitution.md` and AGENTS guidance.
- SDD artifacts exist under `specs/<NNN>-<slug>/` with required metadata block:
    - `spec.md` (WHAT/WHY, requirements, acceptance)
    - `plan.md` (design, architecture, trade‑offs)
    - `tasks.md` (subtasks, responsibilities, test plan)
    - Include Issue‑URI, Spec/Plan/Tasks URIs, and Evidence‑URIs.
- Evidence is produced and linked under `dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/`.
- Quality gates pass locally:
    - `cargo fmt --all -- --check`
    - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
    - `cargo test --workspace --all-features --locked`
    - `./scripts/ci/run-local-ci.sh`
- PR description includes:
    - Links to `specs/<NNN>-<slug>/spec.md`, `plan.md`, `tasks.md`.
    - Links to evidence outputs (tests/logs/jq/reports) and summary of results.
    - Risks, rollback plan, and CI summary.

Commit hygiene:

- Use conventional, focused commits following `sdd-rules/rules/git/commit/sdd-rules-commit-message.md`.
- Keep changes minimal and scoped to the task. Avoid unrelated refactors.

Here are a few things you can do that will increase the likelihood of your pull request being accepted:

- Follow the project's SDD-CONSTITUTION (.specify/memory/constitution.md) and (.specify/memory/lifecycle.md) and (sdd-rules/rules/README.md) and coding conventions.
- Write tests for new functionality.
- Update any SDD documentations (.specify/) and (sdd-rules/) if your changes affect features. Then run (.specify/memory/constitution_update_checklist.md) constitution_update_checklist job!
- Keep your change as focused as possible. If there are multiple changes you would like to make that are not dependent upon each other, consider submitting them as separate pull requests.
- Write a [good commit message](sdd-rules/rules/git/commit/sdd-rules-commit-message.md).
- Test your changes with the Spec-Driven Development workflow to ensure compatibility.

## Development workflow

When working on "ACPLazyBridge" Specification‑Driven Development (SDD) team:

- Context gathering
    - Inspect repo and relevant SDD docs: `.specify/`, `sdd-rules/`, `dev-docs/`.
    - Review open issues under `dev-docs/plan/issues/` when applicable.
- Plan tasks (SDD flow)
    - Use `/specify`, `/plan`, `/tasks` commands per `.specify/commands/*.md` to create `specs/<NNN>-<slug>/` artifacts.
    - Ensure links and metadata blocks are present and consistent.
- Implement & verify
    - Develop on a feature/fix branch in a dedicated worktree.
    - Run: `cargo fmt`, `cargo clippy -D warnings`, `cargo test --locked`.
    - Optionally run adapter smoke tests or JSONL playback if relevant to changes.
    - Keep stdout JSON‑only for protocol paths; route logs to stderr.
- Evidence
    - Store test logs and artifacts under `dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/`.
    - Summarize pass/fail and reference in PR.
- Pre‑PR checks
    - Run `./scripts/ci/run-local-ci.sh` or individual scripts:
        - `scripts/sdd/check_language.sh`
        - `scripts/sdd/lint_docs.sh`
        - `scripts/sdd/run_semantic_checks.sh`
    - Re‑run `.specify/memory/constitution_update_checklist.md` when SDD docs change.


## Contributions in ACPLazyBridge fllow ACP (Agent Client Protocol)

We welcome and encourage the includes Human developers and other AI engineers to help improve ACPLazyBridge! Master AI engineers valuable contributions have been enhanced with AI engineers or human developers for code generation, issue detection, and feature definition.

### What we're looking for

When submitting any contributions, please ensure they include:

- **Master AI engineer understanding and testing** - You've personally tested the changes and understand what they do
- **Clear rationale** - You can explain why the change is needed and how it fits within ACPLazyBridge's goals
- **Concrete evidence** - Include test cases, scenarios, or examples that demonstrate the improvement
- **Your own analysis** - Share your thoughts on the end-to-end developer experience

### What we'll close

We reserve the right to close contributions that appear to be:

- Untested changes submitted without verification
- Generic suggestions that don't address specific ACPLazyBridge needs
- Bulk submissions that show no Master AI engineer review or understanding

### Guidelines for success

The key is demonstrating that you (all Human developers and other AI engineers) understand and have validated your proposed changes. If a PR not testing, it likely needs more work before submission.

## Resources

- [constitution](.specify/memory/constitution.md)
- [constitution-lifecycle](.specify/memory/lifecycle.md)
- [SDD documentations Index](.specify/README.md)
- [SDD Rules Index](sdd-rules/rules/README.md)
- [sdd-task SOP rules](.claude/commands/sdd-task.md)
- [sdd Tasks specify.md SOP rules](.specify/commands/specify.md)
- [sdd Tasks specify.md template](.specify/templates/spec-template.md)
- [sdd Tasks plan.md SOP rules](.specify/commands/plan.md)
- [sdd Tasks plan.md template](.specify/templates/plan-template.md)
- [sdd Tasks tasks.md SOP rules](.specify/commands/tasks.md)
- [sdd Tasks tasks.md template](.specify/templates/tasks-template.md)
- [ACP and outher Dev docs references](dev-docs/references/)
- [Spec-Driven Development Methodology](./.specify/spec-driven.md)
- [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/)
- [Using Pull Requests](https://help.github.com/articles/about-pull-requests/)
- [GitHub Help](https://help.github.com)
