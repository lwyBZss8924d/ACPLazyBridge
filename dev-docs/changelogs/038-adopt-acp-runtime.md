# Changelog

All notable changes to this project will be documented in this file. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Integrated the official ACP runtime for the Codex adapter, including the shared `RuntimeServer`, `ProviderAdapter`, and contract tests that cover initialize/new-session/prompt flows (Issue #44 / Spec 038).
- Captured runtime evidence logs and regression/performance outputs under `_artifacts/038-adopt-acp-runtime/` to support SDD verification.

### Changed

- Updated `acp-lazy-core` module documentation and added a runtime README describing configuration, adapter lifecycle, and testing guidance.
