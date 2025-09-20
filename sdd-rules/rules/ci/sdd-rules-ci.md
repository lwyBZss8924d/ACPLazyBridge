# SDD Rules - CI

## Overview

This document defines the continuous integration rules and quality gates for the ACPLazyBridge project. These rules ensure code quality, SDD compliance, and security standards are maintained throughout the development lifecycle.

## CI Pipeline Architecture

### Core Jobs

1. **build-test**: Cross-platform build and test matrix
2. **sdd-checks**: SDD structure and compliance validation
3. **ast-grep-scan**: Code quality and security scanning
4. **typos-check**: Documentation quality assurance

### Execution Strategy

```yaml
on:
  pull_request:
    branches: [main]
  push:
    branches:
      - feature/**
      - fix/**
      - perf/**
      - chore/**
      - docs/**
```

## Quality Gates

### 1. Rust Quality Gates (Mandatory)

All Rust code must pass:

```bash
cargo fmt --all -- --check          # Format validation
cargo clippy --workspace --all-targets --all-features -- -D warnings  # Lint with warnings as errors
cargo test --workspace --all-features --locked  # Test suite
```

**Rules:**

- Format check MUST pass (no formatting differences)
- Clippy MUST NOT report any warnings (treated as errors)
- All tests MUST pass with locked dependencies
- Cross-platform compatibility (Ubuntu, macOS)

### 2. SDD Validation Gates

#### Structure Validation

```bash
scripts/ci/run-sdd-structure-lint.sh
```

Validates:

- Required directories exist (`specs/`, `sdd-rules/`, `.specify/`)
- Spec artifacts present (`spec.md`, `plan.md`, `tasks.md`)
- Metadata blocks properly formatted
- Evidence paths correctly linked

#### Language Policy

```bash
scripts/ci/check-language-policy.sh
```

Enforces:

- English-only for normative artifacts
- No non-ASCII characters in specs
- Proper capitalization in documentation
- Technical terms consistency

#### Semantic Checks

```bash
scripts/sdd/run_semantic_checks.sh
```

Validates:

- Cross-references between documents
- No broken internal links
- No `[NEEDS CLARIFICATION]` markers in completed work
- Constitution version consistency

### 3. AST-grep Code Scanning

#### Configuration

Uses `sgconfig.yml` at repository root with rule directories:

- `sdd-rules/rules/code-analysis/ast-grep/rust/`
- `sdd-rules/rules/code-analysis/ast-grep/js/`
- `sdd-rules/rules/code-analysis/ast-grep/python/`
- `sdd-rules/rules/code-analysis/ast-grep/go/`

#### Active Rules

**Rust Rules:**

- `rust-no-unwrap`: Prevents unsafe `unwrap()` usage
- `rust-no-dbg`: Removes debug macros from production
- `rust-todo-comment`: Tracks TODO comments
- `rust-mutex-lock`: Validates mutex handling

**JavaScript Rules:**

- `js-no-console-log`: Prevents console logging
- `js-no-only-in-tests`: Prevents `.only` in tests

**Python Rules:**

- `python-no-print`: Prevents print statements
- `python-no-pdb`: Prevents debugger imports

**Go Rules:**

- `go-no-fmt-println`: Prevents fmt.Println usage

#### Two-Stage Rollout Strategy

**Stage 1: Report-Only Mode (Current)**

```yaml
ast-grep-scan:
  continue-on-error: true  # Won't block PR
```

- Findings uploaded to GitHub Security tab
- Developers can see issues but PRs can merge
- Allows time to address existing violations

**Stage 2: Enforcement Mode (After Issue #31)**

```yaml
ast-grep-scan:
  continue-on-error: false  # Will block PR
```

- Remove `continue-on-error` line
- PRs blocked if violations found
- Full quality gate enforcement

### 4. SARIF Integration

#### Conversion Pipeline

```bash
# 1. Run ast-grep scan
ast-grep scan -c sgconfig.yml --json > ast-grep-results.json

# 2. Convert to SARIF
jq -f scripts/ci/json-to-sarif.jq ast-grep-results.json > results.sarif

# 3. Upload to GitHub Security
github/codeql-action/upload-sarif@v3
```

#### SARIF Schema Compliance

- Version: 2.1.0
- Includes: rule metadata, location info, severity levels
- Integration: GitHub Code Scanning dashboard
- Visibility: Security tab in repository

## Evidence Collection

### Artifact Categories

1. **Protocol Outputs** (`_ci_artifacts/protocol/`)
   - JSONL replay results
   - Protocol validation logs

2. **AST-grep Results** (`ast-grep-results/`)
   - JSON findings
   - SARIF reports

3. **Test Evidence** (`_artifacts/tests/<task>/`)
   - Test execution logs
   - Coverage reports

### Upload Strategy

```yaml
- uses: actions/upload-artifact@v4
  with:
    name: <category>-<timestamp>
    path: <artifact-path>
```

## CI Scripts

### Core Scripts

| Script | Purpose | When Run |
|--------|---------|----------|
| `run-sdd-gates.sh` | Complete SDD validation suite | Every PR |
| `json-to-sarif.jq` | Convert ast-grep to SARIF | After scanning |
| `run-sdd-structure-lint.sh` | Validate SDD structure | Every commit |
| `check-language-policy.sh` | Enforce English normative | Every commit |

### Enhanced SDD Gates Script

`scripts/ci/run-sdd-gates.sh` orchestrates:

1. Structure validation
2. Language policy checks
3. Markdown linting
4. Semantic validation
5. Template drift detection
6. Error aggregation and reporting

## Caching Strategy

### Dependency Caching

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    cache-on-failure: true
```

Benefits:

- Reduced build times
- Lower CI resource usage
- Faster feedback loops

## Platform Matrix

### Supported Platforms

- `ubuntu-latest`: Primary development platform
- `macos-latest`: macOS compatibility validation
- ~~`windows-latest`~~: Temporarily disabled (Unix-specific code)

### Platform-Specific Considerations

- Path handling differences
- Line ending normalization
- Shell script compatibility

## Monitoring and Alerts

### Success Criteria

- All jobs green
- No security findings in enforce mode
- Evidence artifacts uploaded
- SARIF successfully processed

### Failure Handling

- Clear error messages in logs
- Actionable feedback for developers
- Links to relevant documentation
- Evidence preserved for debugging

## Future Enhancements

### Planned Improvements

1. Windows platform support restoration
2. Performance benchmarking integration
3. Dependency vulnerability scanning
4. License compliance checking
5. Code coverage thresholds

### Migration Path

1. Complete Issue #31 (fix existing violations)
2. Remove `continue-on-error` from ast-grep job
3. Enable branch protection with required checks
4. Add code coverage gates
5. Integrate security advisories

## Claude Code GitHub Actions

[claude-code-github-actions.md](./claude-code-github-actions.md)

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-20T16:00:00Z"
rules:
    name: "ci-quality-gates"
    category: "ci"
    version: "1.1.0"
document:
    type: "sdd-rule"
    path: "sdd-rules/rules/ci/sdd-rules-ci.md"
    version: "1.1.0"
    last_updated: "2025-09-20T16:00:00Z"
    changelog: "Added comprehensive CI rules following PR #36 implementation"
    related:
        - ".github/workflows/ci.yml"
        - "scripts/ci/run-sdd-gates.sh"
        - "scripts/ci/json-to-sarif.jq"
        - "sgconfig.yml"
        - "specs/035-ci-add-sdd/"
```
