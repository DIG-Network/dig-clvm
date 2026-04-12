# Crate API — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [API-001](NORMATIVE.md#API-001) | ❌ | Re-exports from upstream crates | Compilation test: `use dig_clvm::*` and reference each listed type (`Allocator`, `Coin`, `SpendBundle`, `BlsCache`, etc.); verify all resolve |
| [API-002](NORMATIVE.md#API-002) | ❌ | No async/IO/storage dependencies | Cargo.toml inspection: verify no `tokio`, `serde_json`, `rocksdb`, `reqwest` in `[dependencies]`; `cargo tree` check for transitive pulls |
| [API-003](NORMATIVE.md#API-003) | ❌ | Individual `chia-sdk-*` crates | Cargo.toml inspection: verify dependencies list `chia-sdk-types`, `chia-sdk-driver`, `chia-sdk-coinset` individually, not `chia-wallet-sdk` |
| [API-004](NORMATIVE.md#API-004) | ❌ | No database/network/filesystem access | Code review: grep for `std::fs`, `std::net`, `tokio`, `reqwest`, database crate usage; verify none found in crate source |
| [API-005](NORMATIVE.md#API-005) | ❌ | `ValidationError` variants | Unit test: pattern-match on all `ValidationError` variants (`Clvm`, `CoinNotFound`, `AlreadySpent`, `DoubleSpend`, `PuzzleHashMismatch`, `SignatureFailed`, `ConservationViolation`, `CostExceeded`, `Driver`) |
| [API-006](NORMATIVE.md#API-006) | ❌ | `thiserror` implementation | Compilation test: verify `ValidationError` implements `std::error::Error` and `Display`; check `#[from]` wraps `ValidationErr` and `DriverError` |
| [API-007](NORMATIVE.md#API-007) | ❌ | No reimplementation of upstream | Code review: grep for custom CLVM eval loops, custom tree hash, custom BLS verify, custom opcode constants; verify none exist |
| [API-008](NORMATIVE.md#API-008) | ❌ | Module structure | Filesystem inspection: verify `consensus/` module contains `validate.rs`, `block.rs`, `context.rs`, `config.rs`, `result.rs`, `cache.rs`, `error.rs`; verify `lib.rs` has only re-exports |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
