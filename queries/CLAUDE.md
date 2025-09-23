# CLAUDE.md (queries/)

## Authority

- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md
- Security guidelines: ../CONTRIBUTING.md

## Purpose

Custom CodeQL queries for static analysis and security scanning of the ACPLazyBridge codebase. These queries enforce project-specific security and quality requirements.

## Query Structure

```tree
queries/
└── codeql/
    └── acp-rust/
        ├── qlpack.yml              # Query pack configuration
        ├── acp-custom.qls          # Query suite
        ├── codeql-pack.lock.yml    # Dependencies
        └── queries/
            ├── no-secret-logging.ql
            ├── no-stdout-logging.ql
            ├── no-panics-in-protocol.ql
            └── subprocess-stdio-safety.ql
```

## Current Queries

### Security Queries

| Query | Purpose | Severity |
|-------|---------|----------|
| `no-secret-logging.ql` | Prevent logging of secrets | Error |
| `subprocess-stdio-safety.ql` | Validate subprocess I/O | Warning |

### Protocol Queries

| Query | Purpose | Severity |
|-------|---------|----------|
| `no-stdout-logging.ql` | Keep stdout clean for JSONL | Error |
| `no-panics-in-protocol.ql` | Prevent panics in protocol code | Error |

## Writing New Queries

### Query Template

```ql
/**
 * @name Query Name
 * @description What this query detects
 * @kind problem
 * @problem.severity error|warning|recommendation
 * @id acp/query-id
 * @tags security|correctness|maintainability
 */

import rust

from Element e
where /* condition */
select e, "Message about the problem"
```

### Testing Queries

```bash
# Run specific query
codeql query run queries/codeql/acp-rust/queries/no-secret-logging.ql \
  --database codeql-db

# Run query suite
codeql database analyze codeql-db \
  queries/codeql/acp-rust/acp-custom.qls \
  --format csv \
  --output results.csv
```

## Integration with CI

### GitHub Actions

```yaml
- name: Run CodeQL Analysis
  uses: github/codeql-action/analyze@v2
  with:
    queries: queries/codeql/acp-rust/acp-custom.qls
```

### Local Validation

```bash
# Build database
codeql database create codeql-db --language=rust

# Run analysis
codeql database analyze codeql-db queries/codeql/acp-rust/

# Generate SARIF
codeql database analyze codeql-db \
  --format=sarif-latest \
  --output=results.sarif
```

## Query Development Guidelines

### Focus Areas

1. **Security**: Prevent common vulnerabilities
2. **Protocol Compliance**: Ensure ACP protocol rules
3. **Quality**: Maintain code standards
4. **Performance**: Detect inefficiencies

### Best Practices

- Use descriptive query names
- Include clear problem messages
- Set appropriate severity levels
- Add test cases for queries
- Document false positive patterns

## Common Patterns

### Detecting Logging

```ql
from MacroCall mc
where mc.getMacroName() = "println" or
      mc.getMacroName() = "print" or
      mc.getMacroName() = "eprintln"
select mc, "Logging detected"
```

### Finding Sensitive Data

```ql
from Variable v
where v.getName().toLowerCase().matches("%token%") or
      v.getName().toLowerCase().matches("%secret%") or
      v.getName().toLowerCase().matches("%password%")
select v, "Sensitive variable found"
```

### Protocol Validation

```ql
from Function f
where f.getName().matches("handle_%") and
      not exists(ErrorHandling eh | eh.getFunction() = f)
select f, "Protocol handler lacks error handling"
```

## Results Interpretation

### Severity Levels

- **Error**: Must fix before merge
- **Warning**: Should address
- **Recommendation**: Consider improving

### Suppression

```rust
// Suppress specific query
#[allow(clippy::query_name)]

// Suppress with justification
// codeql[acp/no-stdout-logging]: Intentional protocol output
println!("{}", protocol_message);
```

## Maintenance

### Updating Queries

1. Modify query in `queries/` directory
2. Test against known cases
3. Update documentation
4. Run full analysis
5. Review results

### Version Management

```yaml
# qlpack.yml
name: acp-rust-queries
version: 1.0.0
dependencies:
  codeql/rust-all: ^0.1.0
```

## Quick Reference

### Run All Queries

```bash
# From repo root
codeql database create codeql-db --language=rust
codeql database analyze codeql-db queries/codeql/acp-rust/
```

### Check Specific File

```bash
codeql query run queries/codeql/acp-rust/queries/no-secret-logging.ql \
  --database codeql-db \
  -- crates/codex-cli-acp/src/main.rs
```

### Generate Report

```bash
codeql database analyze codeql-db \
  --format=csv \
  --output=_artifacts/reports/legacy/codeql_$(date +%Y%m%d).csv
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "claude-memory"
    path: "./queries/CLAUDE.md"
    version: "1.0.1"
    last_updated: "2025-09-17T08:26:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - "./CLAUDE.md"
```
