# Quickstart: Validation Steps for ast-grep Fix

## Prerequisites

```bash
# Ensure you're in the worktree
cd /Users/arthur/dev-space/acplb-worktrees/fix-ast-grep-unwrap-mutex

# Verify ast-grep is installed
ast-grep --version  # Should be 0.30.0 or higher
```

## Step 1: Capture Initial State

```bash
# Create evidence directory
mkdir -p _artifacts/reports/fix-ast-grep-unwrap-mutex

# Capture current warnings (verbose)
ast-grep scan -c ./sgconfig.yml > _artifacts/reports/fix-ast-grep-unwrap-mutex/before-verbose.log 2>&1

# Capture current warnings (JSON for analysis)
ast-grep scan -c ./sgconfig.yml --json > _artifacts/reports/fix-ast-grep-unwrap-mutex/before.json 2>&1

# Count warnings by rule
echo "=== Warning Summary ===" > _artifacts/reports/fix-ast-grep-unwrap-mutex/before-summary.txt
echo "rust-no-unwrap: $(grep -c 'rust-no-unwrap' _artifacts/reports/fix-ast-grep-unwrap-mutex/before-verbose.log)" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/before-summary.txt
echo "rust-mutex-lock: $(grep -c 'rust-mutex-lock' _artifacts/reports/fix-ast-grep-unwrap-mutex/before-verbose.log)" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/before-summary.txt
echo "Total warnings: $(grep -c 'warning\[' _artifacts/reports/fix-ast-grep-unwrap-mutex/before-verbose.log)" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/before-summary.txt
```

## Step 2: Validate Rule Updates

After updating the rule files:

```bash
# Test rules on sample inline test code
cat > /tmp/test-sample.rs << 'EOF'
// Production code - should trigger warning
fn process_data() -> Result<String, Error> {
    let data = fetch_data().unwrap();  // BAD: should warn
    Ok(data)
}

// Inline test - should NOT trigger warning
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        let result = compute().unwrap();  // OK: in test
        assert_eq!(result, 42);
    }
}

// Test function - should NOT trigger warning
#[test]
fn test_another() {
    let value = get_value().unwrap();  // OK: test function
}
EOF

# Run ast-grep on sample
ast-grep scan -c ./sgconfig.yml /tmp/test-sample.rs

# Capture rule validation
ast-grep scan -c ./sgconfig.yml > _artifacts/reports/fix-ast-grep-unwrap-mutex/after-rules.log 2>&1
```

## Step 3: Validate Code Changes

After refactoring production code:

```bash
# Run quality gates
echo "=== Cargo Format ===" | tee _artifacts/reports/fix-ast-grep-unwrap-mutex/quality-gates.log
cargo fmt --all -- --check 2>&1 | tee -a _artifacts/reports/fix-ast-grep-unwrap-mutex/quality-gates.log

echo -e "\n=== Cargo Clippy ===" | tee -a _artifacts/reports/fix-ast-grep-unwrap-mutex/quality-gates.log
cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1 | tee -a _artifacts/reports/fix-ast-grep-unwrap-mutex/quality-gates.log

echo -e "\n=== Cargo Test ===" | tee -a _artifacts/reports/fix-ast-grep-unwrap-mutex/quality-gates.log
cargo test --workspace --all-features --locked 2>&1 | tee -a _artifacts/reports/fix-ast-grep-unwrap-mutex/quality-gates.log
```

## Step 4: Capture Final State

```bash
# Final ast-grep scan
ast-grep scan -c ./sgconfig.yml > _artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.log 2>&1
ast-grep scan -c ./sgconfig.yml --json > _artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.json 2>&1

# Generate final summary
echo "=== Final Warning Summary ===" > _artifacts/reports/fix-ast-grep-unwrap-mutex/after-summary.txt
echo "rust-no-unwrap: $(grep -c 'rust-no-unwrap' _artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.log)" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/after-summary.txt
echo "rust-mutex-lock: $(grep -c 'rust-mutex-lock' _artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.log)" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/after-summary.txt
echo "Total warnings: $(grep -c 'warning\[' _artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.log)" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/after-summary.txt

# Compare before and after
echo -e "\n=== Comparison ===" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/after-summary.txt
echo "Before: $(grep -c 'warning\[' _artifacts/reports/fix-ast-grep-unwrap-mutex/before-verbose.log) warnings" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/after-summary.txt
echo "After: $(grep -c 'warning\[' _artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.log) warnings" >> _artifacts/reports/fix-ast-grep-unwrap-mutex/after-summary.txt
```

## Step 5: Verify Success Criteria

Run this checklist:

```bash
# All checks should pass
echo "=== Success Criteria Check ==="

# 1. No warnings in test files
echo -n "✓ No warnings in test files: "
if grep -q "tests/" _artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.log; then
  echo "FAIL - still has test file warnings"
else
  echo "PASS"
fi

# 2. Quality gates pass
echo -n "✓ Cargo fmt passes: "
if cargo fmt --all -- --check > /dev/null 2>&1; then
  echo "PASS"
else
  echo "FAIL"
fi

echo -n "✓ Cargo clippy passes: "
if cargo clippy --workspace --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
  echo "PASS"
else
  echo "FAIL"
fi

echo -n "✓ Cargo test passes: "
if cargo test --workspace --all-features --locked > /dev/null 2>&1; then
  echo "PASS"
else
  echo "FAIL"
fi

# 3. Evidence collected
echo -n "✓ Evidence collected: "
if [ -f "_artifacts/reports/fix-ast-grep-unwrap-mutex/before.json" ] && [ -f "_artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.json" ]; then
  echo "PASS"
else
  echo "FAIL"
fi
```

## Quick Commands Reference

```bash
# Check current warnings
ast-grep scan -c ./sgconfig.yml

# Run specific rule
ast-grep scan -c ./sgconfig.yml --filter "rust-no-unwrap"

# Check specific file
ast-grep scan -c ./sgconfig.yml crates/acp-lazy-core/src/transport.rs

# Run all quality checks
cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace --all-features --locked
```

## Troubleshooting

If rules don't work as expected:

1. **Check AST structure**:

   ```bash
   ast-grep --lang rust --debug-query=ast 'fn test_example() {}'
   ```

2. **Test pattern matching**:

   ```bash
   echo '#[test] fn test() { x.unwrap(); }' | ast-grep --lang rust -p '$EXPR.unwrap()'
   ```

3. **Validate YAML syntax**:

   ```bash
   ast-grep test  # Runs rule tests if configured
   ```
