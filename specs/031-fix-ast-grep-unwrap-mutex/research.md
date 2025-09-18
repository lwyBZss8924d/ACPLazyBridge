# Research: Fix ast-grep Rust Warnings

## AST-grep Pattern Research

### Exclusion Pattern Syntax

**Decision**: Use `not:` with `inside:` to exclude code within specific AST nodes

**Rationale**:

- ast-grep supports negative patterns via `not:` operator
- `inside:` matches code within specific parent nodes
- Combination allows excluding test-specific contexts

**Pattern Structure**:

```yaml
rule:
  all:
    - any:
        - pattern: $EXPR.unwrap()
        - pattern: $EXPR.expect($MSG)
    - not:
        inside:
          any:
            - kind: attribute_item
              has:
                kind: meta_item
                pattern: cfg(test)
            - kind: attribute_item
              has:
                kind: identifier
                pattern: test
```

### AST Node Types for Rust

**Findings**:

- `#[cfg(test)]` creates an `attribute_item` node
- The `cfg(test)` part is a `meta_item` node
- `#[test]` creates an `attribute_item` with `identifier` node containing "test"
- These patterns work for both module-level and function-level attributes

## Error Handling Patterns

### When to Use `?` Operator

**Decision**: Use `?` for all fallible operations in functions that return Result

**Rationale**:

- Cleaner than unwrap()
- Proper error propagation
- Works with anyhow for context

**Example**:

```rust
// Before
let value = operation.unwrap();

// After
let value = operation?;
```

### When to Use anyhow::Context

**Decision**: Add context when the error alone doesn't provide enough information

**Rationale**:

- Helps debugging in production
- No performance overhead when no error occurs
- Standard practice in Rust ecosystem

**Example**:

```rust
// Before
let file = File::open(path).unwrap();

// After
let file = File::open(path)
    .with_context(|| format!("Failed to open config file: {}", path))?;
```

### Mutex Lock Handling

**Decision**: Use `.expect()` with descriptive message for Mutex poisoning

**Rationale**:

- Mutex poisoning is typically unrecoverable
- `expect()` with context is acceptable for this case
- Alternative: Use parking_lot::Mutex which doesn't poison (future consideration)

**Example**:

```rust
// Before
let guard = mutex.lock().unwrap();

// After
let guard = mutex.lock().expect("mutex poisoned");
// OR if more context needed:
let guard = mutex.lock().expect("transport state mutex poisoned");
```

### Cases Where unwrap() May Remain

**Decision**: Only in these specific cases with justification:

1. Tests (already excluded by rule update)
2. Infallible operations that return Result for API consistency
3. After explicit validation that guarantees success

**Example**:

```rust
// Acceptable in production if preceded by validation
if string.chars().all(|c| c.is_ascii_digit()) {
    let num = string.parse::<u32>().unwrap(); // Safe: validated above
}
```

## Implementation Strategy

### Order of Operations

1. **Update rules first** - Reduces noise, shows real issues
2. **Fix high-impact files** - transport.rs has most warnings
3. **Validate incrementally** - Run ast-grep after each major change

### Testing Approach

- Existing test suite validates behavior preservation
- No new tests needed (bug fix, not new feature)
- Run full test suite after each file modification

### Evidence Collection

**Commands for evidence**:

```bash
# Before changes
ast-grep scan -c ./sgconfig.yml --json > before.json
ast-grep scan -c ./sgconfig.yml > before.log

# After rule changes
ast-grep scan -c ./sgconfig.yml > after-rules.log

# After code changes
ast-grep scan -c ./sgconfig.yml > after-code.log

# Quality gates
cargo fmt --all -- --check > fmt.log 2>&1
cargo clippy --workspace --all-targets --all-features -- -D warnings > clippy.log 2>&1
cargo test --workspace --all-features --locked > test.log 2>&1
```

## Alternatives Considered

### Alternative: File-based Exclusion

**What**: Exclude entire test files via file patterns
**Why Rejected**: Doesn't solve inline test problem, those are in source files

### Alternative: Disable Rules for Test Code

**What**: Turn off rules entirely for test code
**Why Rejected**: Still want some checks in tests, just not unwrap() restrictions

### Alternative: Custom Macro

**What**: Create safe_unwrap!() macro for tests
**Why Rejected**: Adds complexity, doesn't solve existing code, non-idiomatic

## Risks and Mitigations

### Risk: Breaking Existing Functionality

**Mitigation**:

- Comprehensive test suite must pass
- Changes are mechanical (unwrap → ?)
- Git history allows easy reversion

### Risk: Performance Impact

**Mitigation**:

- `?` operator has no runtime overhead vs unwrap
- anyhow::Context only allocates on error path
- Benchmark critical paths if concerns arise

### Risk: Incomplete Pattern Matching

**Mitigation**:

- Test rules on sample code first
- Verify both module and function-level exclusions
- Check nested test modules work correctly
