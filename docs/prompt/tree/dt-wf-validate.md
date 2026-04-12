# dt-wf-validate — Workflow: Validate

Run the full validation suite before committing. All checks must pass.

## Required Checks

```bash
# All tests pass
cargo test

# See output for debugging (if needed)
cargo test -- --nocapture

# No clippy warnings (treated as errors)
cargo clippy -- -D warnings

# Formatting is clean
cargo fmt --check
```

## Targeted Checks

For specific requirements:

```bash
# Run the VV test for the requirement you just implemented
cargo test vv_req_val_001

# Run all tests in a domain
cargo test validation_tests
cargo test block_tests
cargo test parity_tests
```

## Critical Audit Checks

After tests pass, manually verify these constraints:

### No custom CLVM execution

```bash
# Search for direct run_program calls — should only appear in chia-consensus
grep -r "run_program" src/
# If found in your code: VIOLATION of Rule 2. Delegate to run_spendbundle() instead.
```

### No IO imports

```bash
# Search for forbidden imports
grep -rE "std::fs|std::net|tokio|async fn|reqwest|sqlx" src/
# If found: VIOLATION of Rule 4. Remove all IO.
```

### Re-exports resolve

```bash
# Check that a dependent crate can use your re-exports
cargo check
```

### No circular dependencies

```
codebase_graph_circular {}
```

If SocratiCode reports circular dependencies introduced by your change, fix them before committing.

## Failure Handling

- **Test failure:** Fix the implementation to match the spec, not the other way around. The test defines the contract.
- **Clippy warning:** Fix the warning. Do not `#[allow(...)]` unless the warning is a false positive and you add a comment explaining why.
- **Format failure:** Run `cargo fmt` and include the formatting changes in your commit.
- **Circular dependency:** Restructure to break the cycle. Common fix: move shared types to a lower-level module.

## Validation Passed

When all four checks are green, proceed to tracking updates.

---

Navigation: Prev < [dt-wf-implement.md](dt-wf-implement.md) | Next > [dt-wf-update-tracking.md](dt-wf-update-tracking.md)
