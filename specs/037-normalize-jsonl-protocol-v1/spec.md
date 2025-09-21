# Feature Specification: Normalize JSONL fixtures to ACP v1 protocolVersion

```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/037-normalize-jsonl-protocol-v1
feature_branch: chore/037-normalize-jsonl-protocol-v1
created: 2025-09-21T19:30:00Z
last_updated: 2025-09-21T21:48:12Z
status: completed
input: GitHub Issue #14
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/14
spec_uri: specs/037-normalize-jsonl-protocol-v1/spec.md
plan_uri: specs/037-normalize-jsonl-protocol-v1/plan.md
tasks_uri: specs/037-normalize-jsonl-protocol-v1/tasks.md
evidence_uris: _artifacts/037-normalize-jsonl-protocol-v1/
specs:
    constitution: 1.0.1
    type: spec
    feature_number: 037
```

## Execution Flow (main)

```text
1. Parse issue requirements from Issue #14
   → Normalize protocolVersion in JSONL fixtures
2. Extract key requirements:
   → All fixtures must use integer protocolVersion: 1
   → Preserve existing clientCapabilities/capabilities
   → Update test scripts that generate JSONL
3. Identify scope:
   → dev-docs/review/_artifacts/tests/*.jsonl
   → test_streaming.sh script
4. Generate acceptance criteria
   → All fixtures use integer protocolVersion
   → Tests pass with normalized fixtures
   → CI replay runner compatible
5. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines

- ✅ Focus on WHAT: Normalize protocolVersion to integer format
- ✅ Focus on WHY: Align with ACP v1 specification for CI replay runner
- ❌ Avoid HOW: Implementation details in plan.md

---

## User Scenarios & Testing

### Primary User Story

As a developer working with the ACPLazyBridge ACP adapter, I need all JSONL test fixtures to use the correct integer protocolVersion format so that tests accurately reflect the ACP v1 protocol specification and work with the upstream agent-client-protocol crate.

### Acceptance Scenarios

1. **Given** a JSONL fixture with string protocolVersion "2024-11-05", **When** the normalization is applied, **Then** it should have integer protocolVersion 1
2. **Given** a JSONL fixture with string protocolVersion "1", **When** the normalization is applied, **Then** it should have integer protocolVersion 1
3. **Given** a test script that generates JSONL with string protocolVersion, **When** updated, **Then** it should generate integer protocolVersion 1
4. **Given** normalized JSONL fixtures, **When** playback tests are run, **Then** all tests should pass without protocol version errors

### Edge Cases

- What happens when a fixture has no initialize request? → Skip file, no changes needed
- How does system handle fixtures with existing integer protocolVersion? → Leave unchanged
- What if clientCapabilities vs capabilities naming differs? → Preserve as-is, only fix protocolVersion

### Acceptance Criteria

- [x] All initialize.params.protocolVersion in dev-docs/review/_artifacts/tests/*.jsonl are integer 1 (FR-001/FR-002)
- [x] clientCapabilities/capabilities in fixtures remain unmodified (FR-003)
- [x] test_streaming.sh outputs only integer protocol versions (FR-004)
- [x] All JSONL files are valid JSON (FR-005)
- [x] All replays using codex-cli-acp pass (FR-006)

## Requirements

### Functional Requirements

- **FR-001**: System MUST update all JSONL fixtures in dev-docs/review/_artifacts/tests/ to use integer protocolVersion 1
- **FR-002**: System MUST replace string protocolVersion values ("2024-11-05", "1") with integer 1
- **FR-003**: System MUST preserve existing clientCapabilities or capabilities blocks exactly as they are
- **FR-004**: System MUST update test_streaming.sh to generate integer protocolVersion in its output
- **FR-005**: System MUST validate that all updated fixtures parse as valid JSON
- **FR-006**: Updated fixtures MUST pass playback tests with codex-cli-acp binary

### Key Entities

- **JSONL Fixture**: Test file containing JSON-RPC requests/responses, one per line
- **Initialize Request**: First request in fixture with method "initialize" containing protocolVersion
- **ProtocolVersion**: Integer field (uint16) per ACP specification, should be 1 for v1

---

## Review & Acceptance Checklist

_GATE: Automated checks run during main() execution_

### Content Quality

- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness

- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

---

## Execution Status

_Updated by main() during processing_

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
