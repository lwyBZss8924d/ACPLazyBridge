# Feature Specification: CI with SDD Gates and ast-grep Scanning

```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/ci-sdd-gates
feature_branch: 035-ci-add-sdd
created: 2025-09-19
last_updated: 2025-09-20
status: completed
input: User description: "CI: Add SDD gates and ast-grep scanning (report→enforce) - Integrate SDD quality gates and ast-grep into CI. This is complementary to Issue #31 and will start in report-only mode, then flip to gating after #31 lands."
issue_url: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/32
pr_url: https://github.com/lwyBZss8924d/ACPLazyBridge/pull/36
merged_commit: 7a273a2703555242d9858602a70d07298e361eca
specs:
    constitution: 1.0.1
    type: spec
    feature_number: 035
```

## Execution Flow (main)

```text
1. Parse user description from Input
   → ✓ CI integration with SDD gates and ast-grep scanning
2. Extract key concepts from description
   → Actors: developers, CI system, reviewers
   → Actions: validate, scan, report, enforce
   → Data: code, test results, scan findings
   → Constraints: two-stage rollout, dependency on Issue #31
3. For each unclear aspect:
   → No critical ambiguities identified
4. Fill User Scenarios & Testing section
   → ✓ Clear user flow: PR submission triggers CI checks
5. Generate Functional Requirements
   → ✓ Requirements are testable and measurable
6. Identify Key Entities (if data involved)
   → CI workflow, scan results, quality reports
7. Run Review Checklist
   → No [NEEDS CLARIFICATION] markers present
   → No implementation details in requirements
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines

- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

---

## User Scenarios & Testing _(mandatory)_

### Primary User Story

As a developer submitting a pull request, I need automated validation of my code against project quality standards and security rules, so that issues are caught early and the codebase maintains consistent quality.

### Acceptance Scenarios

1. **Given** a PR is opened with code changes, **When** the CI pipeline runs, **Then** SDD structure validation, code analysis, and quality checks execute automatically
2. **Given** ast-grep finds code issues in report-only mode, **When** the scan completes, **Then** findings are visible in GitHub Security tab but do not block PR merging
3. **Given** Issue #31 is resolved and enforcement is enabled, **When** ast-grep finds violations, **Then** the CI pipeline fails and PR cannot merge
4. **Given** all quality checks pass, **When** CI completes, **Then** a success status is reported and PR can proceed to review

### Edge Cases

- What happens when CI jobs timeout? System must handle gracefully and report timeout status
- How does system handle when ast-grep configuration is invalid? CI must report configuration error without blocking other checks
- What happens during transition from report to enforce mode? Clear communication and grace period for teams to address existing issues

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST run SDD structure validation on every pull request to ensure compliance with project documentation standards
- **FR-002**: System MUST execute ast-grep code analysis using project's sgconfig.yml configuration on all code changes
- **FR-003**: System MUST generate SARIF-formatted reports from ast-grep findings for integration with GitHub Code Scanning
- **FR-004**: System MUST run language policy checks to ensure normative artifacts are in English
- **FR-005**: System MUST execute Rust quality gates including rustfmt, clippy with warnings as errors, and typos checking
- **FR-006**: System MUST run cargo tests across multiple operating systems (Linux, macOS, Windows) to ensure cross-platform compatibility
- **FR-007**: System MUST support two operational modes: report-only (non-blocking) and enforcement (blocking) for ast-grep findings
- **FR-008**: System MUST upload scan results to GitHub Security tab for visibility and tracking
- **FR-009**: System MUST execute markdown style checks on documentation files
- **FR-010**: System MUST run semantic checks to validate cross-references and links in documentation

### Non-Functional Requirements

- **NFR-001**: CI pipeline MUST complete within reasonable time limits (under 30 minutes for full suite)
- **NFR-002**: System MUST cache dependencies where appropriate to optimize build times
- **NFR-003**: Report-only mode MUST NOT block PR merging regardless of findings
- **NFR-004**: System MUST provide clear, actionable feedback for any validation failures
- **NFR-005**: Transition from report to enforcement mode MUST be configurable without code changes

### Key Entities _(include if feature involves data)_

- **CI Workflow**: The GitHub Actions workflow configuration that orchestrates all quality checks
- **Scan Results**: Code analysis findings from ast-grep in SARIF format for GitHub integration
- **Quality Report**: Aggregated results from all CI jobs including pass/fail status and details
- **Configuration State**: The current mode (report/enforce) and associated settings

---

## Dependencies and Assumptions

### Dependencies

- **Issue #31**: Must be resolved before enabling enforcement mode for ast-grep
- **GitHub Code Scanning**: Must be enabled on repository for SARIF upload
- **Existing Scripts**: Relies on scripts in scripts/ci/ and scripts/sdd/ directories

### Assumptions

- Repository has sgconfig.yml properly configured for ast-grep
- GitHub Actions runners have necessary permissions for security scanning
- Development team is aware of two-stage rollout plan

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

---
