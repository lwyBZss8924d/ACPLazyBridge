# Specification: Align Issue Templates with SDD Constitution v1.0.1

```yaml
worktree: specs/002-issue-templates-align-sdd
feature_branch: 002-issue-templates-align-sdd
created: 2025-09-17
last_updated: 2025-09-18
status: completed
input: User description: "Align Issue Templates with SDD Constitution v1.0.1"
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/29
spec_uri: specs/002-issue-templates-align-sdd/spec.md
plan_uri: specs/002-issue-templates-align-sdd/plan.md
tasks_uri: specs/002-issue-templates-align-sdd/tasks.md
evidence_uris: _artifacts/issue-templates-sdd-29/
specs:
    constitution: 1.0.1
    type: spec
    feature_number: 002
last_commit_hash: "49c59fa599f80af64cc5340d383ac7fd09da45b3"
```

## Summary

Align all GitHub Issue Templates under `.github/ISSUE_TEMPLATE/` with the SDD Constitution v1.0.1 to enforce SDD governance at issue creation time, improve triage quality, and ensure English-only normative artifacts.

## Motivation

Following the adoption of SDD Constitution v1.0.1, the issue templates must be updated to:

- Enforce constitutional principles from the start of each task
- Ensure all normative artifacts are English-only
- Improve traceability through SDD artifact linking
- Standardize quality gates and acceptance criteria
- Prevent accidental exposure of secrets

## Requirements

### Functional Requirements

#### FR1: Bug Report Template

- Display Constitution baseline (1.0.1 | 2025-09-15)
- Severity field (critical/high/medium/low)
- Reproducibility field (always/sometimes/rarely/unable)
- Affected version/commit field
- Component/area dropdown
- Protocol version field (optional)
- SDD links section (optional)
- English-only confirmation checkbox
- No-secrets acknowledgement checkbox

#### FR2: Feature Request Template

- Display Constitution baseline
- Acceptance criteria field (required, formal)
- Non-goals field
- SDD impact field (requires spec/plan/tasks: yes/no/tbd)
- English-only confirmation checkbox
- No-secrets acknowledgement checkbox

#### FR3: Engineering Task Template

- Display Constitution baseline
- Category dropdown (feature/fix/perf/chore/docs)
- Spec-URI, Plan-URI, Tasks-URI fields
- Evidence-URIs field
- Acceptance Criteria field (required)
- Risks & Rollback field
- Quality Gates checklist:
    - [ ] cargo fmt passes
    - [ ] cargo clippy passes
    - [ ] cargo test passes
    - [ ] JSONL scenarios replay OK
    - [ ] scripts/ci/run-local-ci.sh passes
    - [ ] Evidence stored
- English-only confirmation
- No-secrets acknowledgement

#### FR4: Configuration File

- Security advisories link
- Contributing guide link
- SDD documentation link (.specify/)

### Non-Functional Requirements

#### NFR1: Usability

- Forms must be intuitive and guide users
- Required fields clearly marked
- Helpful descriptions for each field

#### NFR2: Compliance

- All templates enforce English-only policy
- Security reminders prevent secret exposure
- Constitution version visible

#### NFR3: Maintainability

- YAML structure follows GitHub best practices
- Comments explain complex fields
- Consistent naming conventions

## Scope

### In Scope

- Update bug_report.yml
- Update feature_request.yml
- Update engineering_task.yml
- Update config.yml
- Validate templates locally
- Test rendering on GitHub

### Out of Scope

- PR templates (already updated)
- Automated CI depending on new fields
- Custom GitHub Actions
- External integrations

## Acceptance Criteria

### AC1: Constitution Compliance

- [ ] All templates display "Based on Constitution: 1.0.1 | Last Amended: 2025-09-15"
- [ ] English-only confirmation present in all templates
- [ ] No-secrets acknowledgement in all templates

### AC2: Bug Report Enhancements

- [ ] Severity dropdown functional
- [ ] Reproducibility dropdown functional
- [ ] Version/commit tracking fields present
- [ ] Component/area categorization available
- [ ] Optional protocol version field

### AC3: Feature Request Formalization

- [ ] Acceptance criteria field is required
- [ ] Non-goals field available
- [ ] SDD impact assessment dropdown functional

### AC4: Engineering Task Structure

- [ ] Category dropdown with all 5 options
- [ ] All SDD URI fields present
- [ ] Evidence-URIs field available
- [ ] Quality Gates checklist complete
- [ ] Acceptance criteria required
- [ ] Risks & Rollback field present

### AC5: Configuration Links

- [ ] Security advisories link active
- [ ] Contributing guide link active
- [ ] SDD documentation link active

### AC6: Quality Validation

- [ ] YAML syntax valid
- [ ] Templates render correctly on GitHub
- [ ] Local CI checks pass
- [ ] Evidence collected and stored

## Non-Goals

- Modifying PR templates
- Creating custom validators
- Implementing auto-labeling
- Changing issue workflow

## Dependencies

- GitHub Issue Forms feature
- YAML syntax support
- SDD Constitution v1.0.1
- Local CI scripts

## Risks

### R1: Breaking Existing Issues

- **Mitigation**: Changes are additive, not breaking
- **Impact**: Low

### R2: User Confusion

- **Mitigation**: Clear field descriptions and examples
- **Impact**: Medium

### R3: Template Complexity

- **Mitigation**: Progressive disclosure, optional advanced fields
- **Impact**: Low

## References

- [SDD Constitution v1.0.1](.specify/memory/constitution.md)
- [GitHub Issue Forms Documentation](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)
- [SDD Rules](sdd-rules/rules/README.md)

---
