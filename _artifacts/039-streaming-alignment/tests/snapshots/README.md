# Snapshot Evidence

- Inline insta snapshots are stored directly in the Rust test sources (see `crates/codex-cli-acp/tests/streaming_snapshots_test.rs`).
- `cargo test -p codex-cli-acp --tests` ran successfully on 2025-09-26T04:54:05Z; logs recorded in `../cargo-test-codex-cli-acp.log`.
- `cargo insta` subcommand is not available on the host; attempt captured in `../cargo-insta-test.log`.
