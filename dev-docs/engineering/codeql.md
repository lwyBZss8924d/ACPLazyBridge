# CodeQL Security Analysis for ACPLazyBridge

## Overview

This document describes how to use CodeQL for security and quality analysis of the ACPLazyBridge Rust codebase.

## Quick Start

### Prerequisites

- CodeQL CLI installed (`brew install --cask codeql` on macOS)
- Rust toolchain installed with rustfmt and clippy
- 6GB+ RAM available for analysis

### Running Analysis Locally

```bash
# Create database (one-time or after significant changes)
codeql database create .codeql-db/rust \
  --language=rust \
  --source-root . \
  --command="cargo build --workspace --all-features"

# Run standard security analysis
codeql database analyze .codeql-db/rust \
  codeql/rust-queries \
  --format=sarifv2.1.0 \
  --output dev-docs/review/_artifacts/sarif/codeql-rust-baseline.sarif

# Run custom ACPLazyBridge queries
codeql database analyze .codeql-db/rust \
  queries/codeql/acp-rust/queries \
  --format=sarifv2.1.0 \
  --output dev-docs/review/_artifacts/sarif/codeql-acp-custom.sarif

# View results summary
jq '.runs[].results | length' dev-docs/review/_artifacts/sarif/*.sarif
```

## Custom Queries

We maintain custom CodeQL queries in `queries/codeql/acp-rust/` to enforce ACPLazyBridge-specific rules:

### 1. No stdout logging (`no-stdout-logging.ql`)

- **Rule**: `println!` and `print!` macros are forbidden in production code
- **Reason**: stdout is reserved for JSON-RPC protocol messages
- **Fix**: Use `eprintln!` or structured logging (tracing) to stderr

### 2. No panics in protocol code (`no-panics-in-protocol.ql`)

- **Rule**: No `panic!`, `unwrap()`, or `expect()` in protocol handling files
- **Reason**: Protocol code must return structured JSON-RPC errors
- **Fix**: Use proper error handling with `Result` and JSON-RPC error responses

### 3. Subprocess stdio safety (`subprocess-stdio-safety.ql`)

- **Rule**: Subprocess spawning must explicitly configure stdio
- **Reason**: Enforce strict I/O separation per WARP logging rules
- **Fix**: Use `Stdio::piped()` for stdin/stdout/stderr

### 4. No secret logging (`no-secret-logging.ql`)

- **Rule**: Detect potential secrets in log statements
- **Reason**: Prevent accidental exposure of sensitive data
- **Fix**: Remove or redact sensitive information before logging

## CI/CD Integration

CodeQL is configured using GitHub's default setup:

- Runs automatically on every push and pull request
- Managed through GitHub repository settings (Settings > Security > Code scanning)
- Results available in the GitHub Security tab
- Uses GitHub's standard security queries for Rust

Note: Custom queries in `queries/codeql/acp-rust/` are available for local testing only.

## Triaging Results

### Priority Levels

1. **Critical** (Fix immediately):
   - Secrets in logs
   - Panics in protocol code

2. **High** (Fix before merge):
   - stdout logging violations
   - Missing error handling

3. **Medium** (Fix in next sprint):
   - Subprocess configuration issues
   - Code quality improvements

### False Positives

If a finding is a false positive:

1. Document the reasoning
2. Consider refining the query
3. Add inline suppression comment if appropriate

## Extending Queries

To add new custom queries:

1. Create `.ql` file in `queries/codeql/acp-rust/queries/`
2. Include proper metadata:

   ```ql
   /**
    * @name Query name
    * @description What this detects
    * @kind problem
    * @problem.severity error|warning|recommendation
    * @id rust/acplazybridge/unique-id
    * @tags category tags
    */
   ```

3. Test locally before committing
4. Update this documentation

## Troubleshooting

### Database creation fails

- Ensure clean build: `cargo clean`
- Check Rust toolchain: `rustup show`
- Verify no interactive prompts in build

### Query compilation errors

- Check imports match installed packs
- Verify API usage against CodeQL Rust documentation
- Use `codeql query compile` to debug

### No results found

- Verify database was created successfully
- Check query patterns match actual code patterns
- Review file path filters in queries

## Resources

- [CodeQL documentation](https://codeql.github.com/docs/)
- [CodeQL Rust language reference](https://codeql.github.com/docs/codeql-language-guides/codeql-for-rust/)
- [SARIF format specification](https://sarifweb.azurewebsites.net/)
- [ACPLazyBridge WARP rules](../../WARP.md)
