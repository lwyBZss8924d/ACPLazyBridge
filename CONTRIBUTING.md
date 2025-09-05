# CONTRIBUTING

This repository follows a worktree-first, trunk-based development workflow with protected main and PR-based merges. Please read this guide before contributing.

## Summary

- main is always stable and release-ready. No direct commits, PR-only.
- Root repo directory is always checked out to main and is not used for development.
- Each task uses its own Git worktree and feature branch from origin/main.
- All changes go through PR with CI checks, code review, and evidence.
- Squash merge strategy keeps a clean main history.

## Prerequisites

- Rust stable via rustup
- cargo, rustfmt, clippy
- Optional: jq (for validating JSONL outputs), Codex CLI for integration testing

## Branching & Worktrees

- Naming: feature|fix|perf|chore|docs/&lt;kebab-slug&gt;
- Create from origin/main:
  - Container path (required): /Users/arthur/dev-space/acplb-worktrees/&lt;task-dir&gt;
  - Command:
    git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/&lt;task-dir&gt; origin/main -b feature/&lt;slug&gt;
  - Optional symlink (for IDE navigation under repo root):
    ln -sfn /Users/arthur/dev-space/acplb-worktrees/&lt;task-dir&gt; /Users/arthur/dev-space/ACPLazyBridge/.worktrees/&lt;task-dir&gt;
- One worktree per feature branch. Do not checkout the same branch in multiple worktrees.
- After PR merge, remove the worktree and delete the local branch if desired.

## Commits & PRs

- Conventional Commits: feat, fix, perf, chore, docs, test, refactor, build, ci
- Keep commits focused and small; split logically distinct changes into separate PRs.
- PR description must include:
  - Motivation and design summary
  - Acceptance criteria and risk/rollback plan
  - Links to dev-docs/plan/issues and references
  - Evidence: test commands, JSONL inputs/outputs locations, logs, and jq filters
- Default merge method: Squash and merge into main

## Code Quality Gates

- Formatting: cargo fmt --all -- --check
- Lint: cargo clippy --workspace --all-targets --all-features -- -D warnings
- Tests: cargo test --workspace --all-features --locked
- Protocol scenarios (JSONL) should run cleanly and log to stderr only; stdout must be valid JSONL.

## Security & Logging

- Never log or print secrets. Use environment variables securely in CI.
- Protocol processes must write logs to stderr; stdout is reserved for JSON-RPC/JSONL output.
- CodeQL custom queries enforce security rules automatically. See `dev-docs/engineering/codeql.md` for query details.

## CI/CD

- CI runs on PRs and on push to feature/*.
- Required checks: fmt, clippy, test, protocol scenario replay (if scenarios present), optional security (deny/audit).
- CodeQL security analysis runs on all PRs and main pushes. See `dev-docs/engineering/codeql.md` for details.
- main is a protected branch requiring successful status checks and at least one review.

## Reviewers & Ownership

- CODEOWNERS define default reviewers. Use them for subsystem expertise.
- For large changes, propose design in dev-docs/plan/issues and seek early feedback.

## Evidence & Traceability

- Keep JSONL tests under `dev-docs/review/_artifacts/tests/` and outputs under `_artifacts/logs/` when running locally.
- Maintain traceability (e.g., dev-docs/review/_artifacts/traceability.csv) linking requirements/specs/implementation/tests.

## Local Quickstart (example)

- New task (worktree setup):
  git -C /Users/arthur/dev-space/ACPLazyBridge fetch origin
  git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/<task-dir> origin/main -b feature/<slug>
- Build & check:
  cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace --all-features --locked
- Run protocol scenarios (if present):
  for f in dev-docs/review/_artifacts/tests/*.jsonl; do cargo run -p codex-cli-acp < "$f" | jq -c . >/dev/null || exit 1; done

## Questions

- Open a discussion or issue with context (link issues/specs). See ISSUE_TEMPLATE for guidance.

## Non-mock testing policy (scripts + manual smoke)

- Scope: WARP-Agent scripted non-mock testing and Zed manual smoke testing (interfacing with real Provider CLI).
- Current implementation: Codex via codex-cli-acp; Claude Code / Gemini agent adapters (ACPLazyBridge) will be added later.

## Non-mock testing prerequisites

- Provider CLI installation and configuration:
  - Codex CLI installed and configured (global config: ~/.codex/config.toml).
  - Zed installed; user config: ~/.config/zed/settings.json.
  - Claude Code (future): ~/.claude/settings.json; ANTHROPIC_API_KEY via environment variable or login flow.
  - Gemini (future): GEMINI_API_KEY via environment variable.
- Build adapter binaries:
  - cargo build --release -p codex-cli-acp

## Secrets

- Never print or echo keys. Inject via environment variables and use implicitly in commands.
- In documentation/command examples, use placeholders like {{ANTHROPIC_API_KEY}} / {{GEMINI_API_KEY}}.

## WARP/CLAUDE-CODE/GEMINI Agent scripted non-mock tests (Codex/Claude Code/Gemini)

- JSONL scenario location: dev-docs/review/_artifacts/tests/
- Run example:
  - target/release/codex-cli-acp < dev-docs/review/_artifacts/tests/handshake.jsonl | tee dev-docs/review/_artifacts/logs/run_$(date +%Y%m%d_%H%M%S).log
- Notify integration test:
  - ACPLB_NOTIFY_PATH=/tmp/notify.jsonl target/release/codex-cli-acp < dev-docs/review/_artifacts/tests/notify_idle.jsonl | tee dev-docs/review/_artifacts/logs/notify_$(date +%Y%m%d_%H%M%S).log
  - Verify immediate completion on notify signal vs idle timeout fallback
- Optional validation:
  - Use jq filters from dev-docs/review/_artifacts/jq/filters.md to generate review snapshots (error and result.stopReason).
- Acceptance criteria:
  - initialize returns protocolVersion, and agentCapabilities.promptCapabilities.image=false
  - session/new returns non-empty sessionId
  - session/prompt shows multiple session/update(type=agent_message_chunk) messages and finally returns result.stopReason
  - session/cancel produces stopReason=Cancelled
  - With notify: immediate completion on agent-turn-complete signal
  - Without notify: completion after idle timeout (default 1200ms)

## Zed manual smoke (manual smoke testing)

- Configure `~/.config/zed/settings.json`:
  - Current: ACPLazyBridge (Codex) → points to absolute path of target/release/codex-cli-acp
  - Claude/Gemini entries will be enabled later when corresponding ACPLazyBridge binaries are available
- IDE side ensures adapter stdout only outputs JSONL; logs written to `stderr` and archived per specification.
- After running, save complete output to dev-docs/review/_artifacts/logs/ (see dev-docs/review/_artifacts/logs/README.md).

## Evidence retention

- Log files named with timestamps, stored in dev-docs/review/_artifacts/logs/.
- Use jq filters to generate review summaries.
- Do not record plaintext keys; sanitize when necessary.

## Notes

- This section covers repository-level policies; detailed steps for each Agent are in WARP.md and CLAUDE.md respectively.
- After Claude Code related branches are merged into main, enable Claude/Gemini non-mock smoke configurations per the supplementary M1 test environment ISSUE checklist.
