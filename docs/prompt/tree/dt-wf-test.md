# dt-wf-test — Workflow: TDD — Write Failing Tests First

Write the test before writing the implementation. The test defines the contract.

## Test File Naming

```
tests/vv_req_{prefix}_{nnn}.rs
```

Examples:
- VAL-001 --> `tests/vv_req_val_001.rs`
- BLK-003 --> `tests/vv_req_blk_003.rs`
- PAR-002 --> `tests/vv_req_par_002.rs`

## File Structure

```rust
//! REQUIREMENT: VAL-001 — validate_spend_bundle Entry Point
//!
//! Simulator + unit tests for the primary validation entry point.

use chia_sdk_test::{Simulator, BlsPair};
use dig_clvm::{validate_spend_bundle, ValidationContext, ValidationConfig};
use dig_constants::DIG_TESTNET;

#[test]
fn vv_req_val_001_valid_standard_spend() {
    // Arrange: create simulator, mint coins, build spend bundle
    // Act: call validate_spend_bundle()
    // Assert: success with expected cost and conditions
}

#[test]
fn vv_req_val_001_returns_error_on_invalid() {
    // Arrange: create simulator, build invalid spend bundle
    // Act: call validate_spend_bundle()
    // Assert: specific error variant returned
}
```

## Required Test Types

### Simulator Tests (MUST for every requirement)

Full spend bundle lifecycle via `chia_sdk_test::Simulator`:
- Create coins with known puzzles
- Build realistic spend bundles
- Validate through the public API
- Assert on results, costs, and conditions

### Unit Tests

Individual function behavior for internal helpers:
- Input/output correctness
- Error path coverage
- Boundary conditions

### Parity Tests (for PAR-* requirements)

Prove identical results to Chia L1:
- Same input produces same output
- Same error for same invalid input
- Cost calculations match exactly

## Permutation Matrix

Cover all dimensions for each requirement:

| Dimension | Examples |
|-----------|----------|
| Valid inputs | Correct spend, proper signature, within cost limit |
| Invalid inputs | Bad signature, missing coin, cost exceeded, double spend |
| Edge cases | Zero amount, max u64 amount, empty bundle, single coin |
| Flag combinations | `MEMPOOL_MODE`, `DONT_VALIDATE_SIGNATURE`, combined flags |

## When to Skip Test-First

Only skip TDD for:
- Documentation-only changes (tracking updates, spec corrections)
- Pure configuration changes (Cargo.toml, constants)
- Tracking file updates

For everything else: test first, then implement.

## Running Tests

```bash
# Run the specific VV test
cargo test vv_req_val_001

# Run with output visible
cargo test vv_req_val_001 -- --nocapture

# Run all tests
cargo test
```

---

Navigation: Prev < [dt-wf-gather-context.md](dt-wf-gather-context.md) | Next > [dt-wf-implement.md](dt-wf-implement.md)
