# Crate API — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [API-001](NORMATIVE.md#API-001) | ✅ | Re-exports from upstream crates | lib.rs re-exports all listed types. cargo check passes. |
| [API-002](NORMATIVE.md#API-002) | ✅ | No async/IO/storage dependencies | Cargo.toml: no tokio, serde_json, rocksdb, reqwest in [dependencies] |
| [API-003](NORMATIVE.md#API-003) | ✅ | Individual `chia-sdk-*` crates | Cargo.toml: chia-sdk-types, chia-sdk-driver, chia-sdk-coinset individually; no chia-wallet-sdk |
| [API-004](NORMATIVE.md#API-004) | ✅ | No database/network/filesystem access | grep for std::fs/std::net/rocksdb/reqwest/tokio:: in src/ — none found |
| [API-005](NORMATIVE.md#API-005) | ✅ | `ValidationError` variants | All 9 variants implemented in error.rs: Clvm, CoinNotFound, AlreadySpent, DoubleSpend, PuzzleHashMismatch, SignatureFailed, ConservationViolation, CostExceeded, Driver |
| [API-006](NORMATIVE.md#API-006) | ✅ | `thiserror` implementation | Uses thiserror v2; #[error] on all variants; #[from] DriverError; compiles clean |
| [API-007](NORMATIVE.md#API-007) | ✅ | No reimplementation of upstream | No run_program/tree_hash/aggregate_verify/ChiaDialect::new in src/. All from chia crates. |
| [API-008](NORMATIVE.md#API-008) | ✅ | Module structure | 9 files match spec exactly: lib.rs + consensus/{mod,validate,block,context,config,result,cache,error}.rs. Compiles clean. |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
