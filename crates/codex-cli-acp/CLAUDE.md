# CLAUDE.md (crates/codex-cli-acp)

Authority

- See ../../sdd-rules/CLAUDE.md and ../../sdd-rules/AGENTS.md

What to do here

- Focus on ACP server/adapter behavior; keep stdout strictly JSONL in examples/tests
- Build/test as part of workspace gates:
  - cargo fmt --all -- --check
  - cargo clippy --workspace --all-targets --all-features -- -D warnings
  - cargo test --workspace --all-features --locked
- Place JSONL replay scenarios under dev-docs/review/_artifacts/tests and logs under dev-docs/review/_artifacts/logs

Notes

- Use sdd-rules/commands/* to create/maintain spec/plan/tasks artifacts for new features.

---

Specification Version: 1.0.3 | CLAUDE.md (crates/codex-cli-acp) Format: 1.0 | Last Updated: 2025-09-11
