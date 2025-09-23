# ACPLazyBridge Evidence Repository

This directory is the canonical location for Specification-Driven Development (SDD) evidence. Each task (`specs/<NNN>-<slug>/`) MUST record artefacts under the subdirectories below using the same task identifier.

## Directory Layout

- `tests/` – Test outputs such as `cargo test` logs, JSONL replays, and protocol fixtures.
- `logs/` – Runtime logs, tracing output, and shell transcripts.
- `reports/` – Generated reports (coverage, profiling, audits).
- `jq/` – JSON/JQ filters, query results, and structured analysis scripts.
- `meta/` – Shared metadata (e.g., ARC/REQ/SPEC indexes) that apply across tasks.
- `templates/` – Optional helper templates or examples for evidence capture.
- `<task>/` – Task-specific folders (e.g., `037-normalize-jsonl-protocol-v1/`) created automatically by SDD scripts.

## Usage Guidelines

1. Create task-scoped folders under each evidence type, e.g. `_artifacts/tests/041-runtime-adoption/`.
2. Name files with timestamps (`YYYYMMDD_HHMMSS`) when capturing repeated runs.
3. Do not store large binaries; compress or link to external storage if necessary.
4. Legacy artefacts have been archived under `_artifacts/<type>/legacy/` (e.g., `_artifacts/tests/legacy/`). Reference them only for historical context.

See `dev-docs/CLAUDE.md` for detailed evidence collection procedures.
