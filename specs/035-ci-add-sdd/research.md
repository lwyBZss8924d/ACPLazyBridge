# Research: CI with SDD Gates and ast-grep Scanning

## Research Summary

This document consolidates research findings for implementing CI with SDD gates and ast-grep scanning in GitHub Actions.

## Key Decisions

### 1. ast-grep SARIF Integration

**Decision**: Use ast-grep's JSON output with conversion to SARIF format

**Rationale**:

- ast-grep doesn't natively output SARIF format
- GitHub Code Scanning requires SARIF for integration
- JSON to SARIF conversion is well-documented

**Implementation**:

```yaml
- name: Run ast-grep scan
  run: |
    ast-grep scan -c sgconfig.yml --json > ast-grep-results.json

- name: Convert to SARIF
  run: |
    # Use jq or custom script to transform JSON to SARIF
    jq -f scripts/ci/json-to-sarif.jq ast-grep-results.json > results.sarif
```

**Alternatives Considered**:

- Direct SARIF output: Not supported by ast-grep
- Custom action: Unnecessary complexity for simple transformation

### 2. Report-Only Mode Implementation

**Decision**: Use `continue-on-error: true` for non-blocking behavior

**Rationale**:

- Simple, declarative approach
- Easy to toggle for enforcement mode
- Clear in workflow definition

**Implementation**:

```yaml
ast-grep-scan:
  runs-on: ubuntu-latest
  continue-on-error: true  # Report-only mode
  steps:
    - name: Run ast-grep
      # ... scanning steps
```

**Alternatives Considered**:

- Separate workflow: Would duplicate code
- Environment variable: Less visible in workflow

### 3. GitHub Code Scanning Upload

**Decision**: Use official `github/codeql-action/upload-sarif@v3`

**Rationale**:

- Official GitHub action
- Well-maintained and documented
- Handles authentication automatically

**Implementation**:

```yaml
- name: Upload SARIF
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: results.sarif
    category: ast-grep
```

### 4. Rust Build Caching

**Decision**: Use `Swatinem/rust-cache@v2`

**Rationale**:

- Purpose-built for Rust projects
- Handles cargo registry and target caching
- Significant speed improvement

**Implementation**:

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    cache-on-failure: true
```

### 5. Cross-Platform Testing Matrix

**Decision**: Test on Ubuntu, macOS, and Windows latest

**Rationale**:

- Covers major development platforms
- GitHub-hosted runners available
- Parallel execution for speed

**Implementation**:

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
runs-on: ${{ matrix.os }}
```

### 6. Typos Integration

**Decision**: Install via cargo-binstall for speed

**Rationale**:

- Pre-compiled binaries available
- Faster than building from source
- Consistent across platforms

**Implementation**:

```yaml
- name: Install typos
  uses: taiki-e/install-action@v2
  with:
    tool: typos-cli

- name: Run typos
  run: typos
```

## SARIF Format Requirements

### Minimum SARIF Structure

```json
{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "ast-grep",
          "version": "0.x.x",
          "rules": []
        }
      },
      "results": []
    }
  ]
}
```

### Mapping ast-grep to SARIF

| ast-grep Field | SARIF Field | Notes |
|---------------|-------------|-------|
| rule_id | ruleId | Direct mapping |
| file | physicalLocation.artifactLocation.uri | Relative path |
| line | physicalLocation.region.startLine | 1-based |
| column | physicalLocation.region.startColumn | 1-based |
| severity | level | Map to: error, warning, note |
| message | message.text | Rule description |

## Performance Considerations

### CI Time Optimization

1. **Parallel Jobs**: Run independent checks concurrently
2. **Caching**: Cache Rust dependencies and build artifacts
3. **Selective Runs**: Use path filters for relevant changes
4. **Timeout Limits**: Set reasonable timeouts (30 minutes max)

### Resource Usage

- ast-grep scan: ~1-2 minutes for large codebases
- Rust build (cached): ~2-3 minutes
- Rust build (uncached): ~10-15 minutes
- Test matrix: Parallel execution across OS

## Security Considerations

### SARIF Upload Permissions

Required GitHub token permissions:

```yaml
permissions:
  contents: read
  security-events: write
```

### Workflow Security

- Use pinned action versions (`@v3` not `@main`)
- Minimal permissions principle
- No secret exposure in logs

## Transition Strategy

### Phase 1: Report-Only (Current)

```yaml
continue-on-error: true  # Non-blocking
```

### Phase 2: Enforcement (After Issue #31)

```yaml
continue-on-error: false  # Blocking (or remove the line)
```

### Communication Plan

1. Announce report-only phase in team channels
2. Monitor findings for 1-2 weeks
3. Address critical issues found
4. Announce enforcement date
5. Enable enforcement mode

## Tool Versions

### Recommended Versions

- ast-grep: Latest stable (0.x.x)
- GitHub Actions: ubuntu-latest, macos-latest, windows-latest
- upload-sarif action: v3
- rust-cache action: v2
- typos-cli: Latest stable

### Version Pinning Strategy

- Actions: Pin to major version (`@v3`)
- Tools: Use latest for security updates
- Document version changes in PR

## References

- [ast-grep Documentation](https://ast-grep.github.io/)
- [GitHub Code Scanning](https://docs.github.com/en/code-security/code-scanning)
- [SARIF Specification](https://sarifweb.azurewebsites.net/)
- [GitHub Actions Best Practices](https://docs.github.com/en/actions/guides)
- [rust-cache Action](https://github.com/Swatinem/rust-cache)
- [typos Documentation](https://github.com/crate-ci/typos)

---

⚠️ _Based on SDD research guidelines: `sdd-rules/rules/research/sdd-rules-research.md`_
