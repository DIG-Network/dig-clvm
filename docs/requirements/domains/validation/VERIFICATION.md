# Spend Validation — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [VAL-001](NORMATIVE.md#VAL-001) | ❌ | `validate_spend_bundle()` entry point signature | Unit test: call `validate_spend_bundle` with valid args, confirm it compiles and returns `Result<SpendResult, ValidationError>` |
| [VAL-002](NORMATIVE.md#VAL-002) | ❌ | Delegates to `chia_consensus::run_spendbundle` | Code review: grep for `run_spendbundle` call inside `validate_spend_bundle`; no hand-rolled CLVM execution |
| [VAL-003](NORMATIVE.md#VAL-003) | ❌ | Reject duplicate spends | Simulator test: build bundle with same coin spent twice, assert `ValidationError::DoubleSpend` |
| [VAL-004](NORMATIVE.md#VAL-004) | ❌ | Reject missing coins | Simulator test: build bundle referencing coin absent from `coin_records` and `ephemeral_coins`, assert `ValidationError::CoinNotFound` |
| [VAL-005](NORMATIVE.md#VAL-005) | ❌ | Reject already-spent coins | Simulator test: mark coin as spent in `coin_records`, attempt spend, assert `ValidationError::AlreadySpent` |
| [VAL-006](NORMATIVE.md#VAL-006) | ❌ | Reject puzzle hash mismatch | Simulator test: build bundle with wrong `puzzle_reveal`, assert `ValidationError::PuzzleHashMismatch` |
| [VAL-007](NORMATIVE.md#VAL-007) | ❌ | Cost limit enforcement | Simulator test: build expensive CLVM program exceeding `max_cost_per_block`, assert `ValidationError::CostExceeded` |
| [VAL-008](NORMATIVE.md#VAL-008) | ❌ | Default cost constants | Unit test: assert `ValidationConfig::default().max_cost_per_spend == 11_000_000_000` and `max_cost_per_block == 550_000_000_000` |
| [VAL-009](NORMATIVE.md#VAL-009) | ❌ | Condition validation delegated to chia-consensus | Code review: verify no hand-rolled announcement/assertion/timelock logic; all delegated to `chia-consensus` |
| [VAL-010](NORMATIVE.md#VAL-010) | ❌ | Ephemeral coin support | Simulator test: create coin and spend it in same bundle via `ephemeral_coins`, assert success |
| [VAL-011](NORMATIVE.md#VAL-011) | ❌ | Conservation check | Simulator test: build bundle where additions exceed removals, assert `ValidationError::ConservationViolation` |
| [VAL-012](NORMATIVE.md#VAL-012) | ❌ | BLS signature verification with flags | Simulator test: build bundle with invalid BLS signature, assert `ValidationError::SignatureFailed`; also test `DONT_VALIDATE_SIGNATURE` skips check |
| [VAL-013](NORMATIVE.md#VAL-013) | ❌ | `MEMPOOL_MODE` stricter rules | Simulator test: build bundle with unknown opcode, validate with `MEMPOOL_MODE` flag, assert rejection; validate without flag, assert acceptance |
| [VAL-014](NORMATIVE.md#VAL-014) | ❌ | `SpendResult` output structure | Simulator test: build standard spend, validate, check `SpendResult` contains `additions`, `removals`, `fee`, and `conditions` fields |
| [VAL-015](NORMATIVE.md#VAL-015) | ❌ | `coin_records` contains only relevant coins | Code review: verify `ValidationContext` does not perform DB lookups; unit test confirming only passed-in coins are referenced |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
