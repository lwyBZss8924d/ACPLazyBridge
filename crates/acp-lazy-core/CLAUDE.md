# CLAUDE.md (crates/acp-lazy-core)

Authority
- See ../../sdd-rules/CLAUDE.md and ../../sdd-rules/AGENTS.md

What to do here
- Build/test this crate as part of workspace gates:
  - cargo fmt --all -- --check
  - cargo clippy --workspace --all-targets --all-features -- -D warnings
  - cargo test --workspace --all-features --locked
- Link evidence to dev-docs/review/_artifacts when adding tests/logs.

Notes
- Keep stdout clean; use stderr for logs when running protocol-related examples.

