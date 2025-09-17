# Implementation Plan: Align Issue Templates with SDD Constitution v1.0.1

```yaml
Issue-URI: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/29
Spec-URI: specs/002-issue-templates-align-sdd/spec.md
Plan-URI: specs/002-issue-templates-align-sdd/plan.md
Tasks-URI: specs/002-issue-templates-align-sdd/tasks.md
Evidence-URIs: _artifacts/issue-templates-sdd-29/
```

Based on Constitution: 1.0.1 | Last Amended: 2025-09-15

## Overview

This plan details the technical approach for aligning GitHub Issue Templates with SDD Constitution v1.0.1. The implementation focuses on enhancing existing YAML forms with SDD-specific fields while maintaining GitHub Issue Forms compatibility.

## Architecture

### Template Structure

```
.github/
└── ISSUE_TEMPLATE/
    ├── bug_report.yml       # Bug reporting with SDD compliance
    ├── feature_request.yml  # Feature requests with acceptance criteria
    ├── engineering_task.yml # SDD task tracking
    └── config.yml          # Repository-wide issue configuration
```

### Field Categories

1. **Constitutional Fields** (all templates)
   - Banner with constitution version
   - English-only confirmation
   - No-secrets acknowledgement

2. **Tracking Fields** (bug_report)
   - Severity levels
   - Reproducibility metrics
   - Version/commit tracking
   - Component categorization

3. **Planning Fields** (feature_request)
   - Formal acceptance criteria
   - Non-goals definition
   - SDD impact assessment

4. **Governance Fields** (engineering_task)
   - Category taxonomy
   - SDD artifact links
   - Quality gates checklist
   - Risk assessment

## Technical Design

### bug_report.yml Structure

```yaml
name: Bug report
description: Create a report to help us improve (SDD v1.0.1 compliant)
title: "[BUG] "
labels: [bug]
body:
  - type: markdown
    attributes:
      value: "Based on Constitution: 1.0.1 | Last Amended: 2025-09-15"

  - type: dropdown
    id: severity
    attributes:
      label: Severity
      options:
        - critical (production blocker)
        - high (major functionality broken)
        - medium (minor functionality affected)
        - low (cosmetic/edge case)
    validations:
      required: true

  - type: dropdown
    id: reproducibility
    attributes:
      label: Reproducibility
      options:
        - always (100%)
        - sometimes (>50%)
        - rarely (<50%)
        - unable to reproduce
    validations:
      required: true

  # ... additional fields ...

  - type: checkboxes
    id: compliance
    attributes:
      label: Compliance Confirmation
      options:
        - label: This issue is written in English (normative artifacts requirement)
          required: true
        - label: I have not included any secrets, tokens, or sensitive data
          required: true
```

### feature_request.yml Structure

```yaml
name: Feature request
description: Suggest an idea for this project (SDD v1.0.1 compliant)
title: "[FEATURE] "
labels: [enhancement]
body:
  - type: markdown
    attributes:
      value: "Based on Constitution: 1.0.1 | Last Amended: 2025-09-15"

  - type: textarea
    id: acceptance_criteria
    attributes:
      label: Acceptance Criteria
      description: Define specific, measurable criteria for feature completion
      placeholder: |
        - [ ] Criterion 1: ...
        - [ ] Criterion 2: ...
        - [ ] Criterion 3: ...
    validations:
      required: true

  - type: dropdown
    id: sdd_impact
    attributes:
      label: SDD Impact (Spec/Plan/Tasks Required?)
      options:
        - "Yes - requires full SDD workflow"
        - "No - minor change only"
        - "TBD - needs assessment"
    validations:
      required: true
```

### engineering_task.yml Complete Restructure

```yaml
name: Engineering task
description: Implementation task with SDD governance (v1.0.1)
title: "[TASK-XXX] "
labels: [task, sdd-tracked]
body:
  - type: markdown
    attributes:
      value: "Based on Constitution: 1.0.1 | Last Amended: 2025-09-15"

  - type: dropdown
    id: category
    attributes:
      label: Task Category
      options:
        - feature (new functionality)
        - fix (bug fix)
        - perf (performance improvement)
        - chore (maintenance/refactoring)
        - docs (documentation only)
    validations:
      required: true

  - type: input
    id: spec_uri
    attributes:
      label: Specification URI
      placeholder: specs/XXX-feature-name/spec.md
    validations:
      required: true

  - type: checkboxes
    id: quality_gates
    attributes:
      label: Quality Gates Checklist
      options:
        - label: cargo fmt --all -- --check passes
        - label: cargo clippy --workspace --all-targets --all-features -- -D warnings passes
        - label: cargo test --workspace --all-features --locked passes
        - label: JSONL scenarios replay without errors
        - label: scripts/ci/run-local-ci.sh passes
        - label: Evidence collected in _artifacts/
```

### config.yml Enhancement

```yaml
blank_issues_enabled: false
contact_links:
  - name: Security Advisories
    url: https://github.com/lwyBZss8924d/ACPLazyBridge/security/advisories
    about: Report security vulnerabilities privately
  - name: Contributing Guide
    url: https://github.com/lwyBZss8924d/ACPLazyBridge/blob/main/CONTRIBUTING.md
    about: Learn how to contribute to ACPLazyBridge
  - name: SDD Documentation
    url: https://github.com/lwyBZss8924d/ACPLazyBridge/tree/main/.specify
    about: Specification-Driven Development guides and templates
  - name: Questions / Discussions
    url: https://github.com/lwyBZss8924d/ACPLazyBridge/discussions
    about: Ask questions and discuss ideas here
```

## Implementation Strategy

### Phase 1: Template Updates
1. Add constitutional banner to all templates
2. Implement compliance checkboxes
3. Add SDD-specific fields

### Phase 2: Field Enhancements
1. Bug report: severity, reproducibility, version tracking
2. Feature request: acceptance criteria, non-goals, SDD impact
3. Engineering task: complete restructure with quality gates

### Phase 3: Configuration
1. Update config.yml with new contact links
2. Ensure blank issues remain disabled

### Phase 4: Validation
1. YAML syntax validation
2. GitHub rendering preview
3. Local CI checks

## Validation Approach

### YAML Validation
```bash
# Validate YAML syntax
for file in .github/ISSUE_TEMPLATE/*.yml; do
  python -c "import yaml; yaml.safe_load(open('$file'))"
done
```

### GitHub Preview
- Push to feature branch
- Create draft issue to test each template
- Verify all fields render correctly

### CI Integration
```bash
# Run full local CI suite
scripts/ci/run-local-ci.sh
```

## Rollback Strategy

If issues arise:
1. Revert to previous templates via git
2. Templates are backward compatible (additive changes only)
3. Existing issues unaffected

## Success Metrics

- All templates pass YAML validation
- GitHub correctly renders all fields
- Users can create SDD-compliant issues
- Quality gate checkboxes functional
- Constitutional compliance enforced

## References

- [GitHub Issue Forms Syntax](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)
- [YAML Schema for Issue Forms](https://json.schemastore.org/github-issue-forms)
- [SDD Constitution](.specify/memory/constitution.md)

---

```yaml
metadata:
    constitution: "1.0.1"
    document_type: "plan"
    feature_number: "002"
    created: "2025-09-17"
    status: "draft"
```