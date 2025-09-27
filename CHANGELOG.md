# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Full support for agent_client_protocol v0.4.3 official types
- SessionNotification streaming to Zed IDE
- LastChunkGuard deduplication mechanism for streaming chunks
- Comprehensive tool call lifecycle management
- Enhanced validation for protocol compliance
- Evidence collection system for PR validation

### Changed

- Migrated from agent_client_protocol v0.4.2 to v0.4.3
- Refactored CodexStreamManager to use official ACP types
- Updated ToolCallUpdateFields to follow official schema
- Improved session/update message format compliance
- Enhanced error handling in tool call processing

### Fixed

- Streaming notification delivery to Zed IDE (T033)
- Tool call status transitions now properly mapped
- Duplicate chunk prevention in streaming responses
- Protocol version handling (integer vs string)
- AST-grep violations in test code properly suppressed

### Technical Details

#### Migration Notes for v0.4.3

1. **Type Changes**:
   - `ToolCallUpdate` now uses `ToolCallUpdateFields` wrapper
   - `SessionNotification` properly typed with official schema
   - `ContentBlock` variants aligned with protocol spec

2. **Breaking Changes**:
   - Tool call updates require `fields` wrapper object
   - Status transitions must follow official enum values
   - Raw input/output must be properly typed as `Option<Value>`

3. **Code Quality Improvements**:
   - All clippy warnings resolved
   - Consistent formatting applied
   - Test coverage comprehensive (68 tests)
   - Zero AST-grep violations

#### Evidence and Validation

- All quality gates pass (fmt, clippy, test)
- SDD document compliance validated
- Protocol JSONL scenarios verified
- E2E streaming tests successful

## [0.1.0] - 2025-09-01

### Initial Release

- Basic ACP server implementation for Codex CLI
- JSON-RPC 2.0 protocol support
- Session management capabilities
- Tool call mapping framework
- Permission system integration
