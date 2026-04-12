# Chia L1 Parity — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [PAR-001](NORMATIVE.md#PAR-001) | ❌ | `clvmr` with `ChiaDialect` | Code review: verify CLVM execution uses `clvmr` with `ChiaDialect`; grep for any custom interpreter and confirm none exists |
| [PAR-002](NORMATIVE.md#PAR-002) | ❌ | `chia-consensus` condition parsing | Code review: verify condition parsing uses `SpendVisitor` from `chia-consensus`; spot-check opcodes including `SEND_MESSAGE(66)` and `RECEIVE_MESSAGE(67)` |
| [PAR-003](NORMATIVE.md#PAR-003) | ❌ | `clvm_utils` tree hash | Cross-validation test: compute tree hash of known programs, compare output to Chia L1 reference values from `test_curry_and_treehash.py` |
| [PAR-004](NORMATIVE.md#PAR-004) | ❌ | BLS domain separation matches L1 | Cross-validation test: for each `AGG_SIG_*` variant, compute final message and compare to Chia L1's `make_aggsig_final_message()` output |
| [PAR-005](NORMATIVE.md#PAR-005) | ❌ | `chia-bls` `aggregate_verify` | Code review: verify BLS verification calls `chia-bls::aggregate_verify`, not a custom implementation |
| [PAR-006](NORMATIVE.md#PAR-006) | ❌ | Per-condition costs match L1 | Unit test: assert `AGG_SIG_COST == 1_200_000`, `CREATE_COIN_COST == 1_800_000`, `GENERIC_CONDITION_COST == 500` from chia-consensus |
| [PAR-007](NORMATIVE.md#PAR-007) | ❌ | Per-spend cost limit matches L1 | Unit test: assert default per-spend limit is `11_000_000_000`; verify per-block limit is configurable via `ValidationConfig` |
| [PAR-008](NORMATIVE.md#PAR-008) | ❌ | Protocol types re-exported from `chia-protocol` | Compilation test: verify `dig_clvm::Coin`, `CoinSpend`, `SpendBundle`, `Program` are same types as `chia_protocol::*` |
| [PAR-009](NORMATIVE.md#PAR-009) | ❌ | Condition types from `chia-sdk-types` | Code review: verify `Condition` type is imported from `chia-sdk-types`, not custom-defined |
| [PAR-010](NORMATIVE.md#PAR-010) | ❌ | Block generators match L1 | Code review: verify `solution_generator_backrefs` and `run_block_generator2` are used from `chia-consensus` |
| [PAR-011](NORMATIVE.md#PAR-011) | ❌ | Mempool flags match L1 | Simulator test: validate bundle with `MEMPOOL_MODE` flag, confirm stricter rules; test `DONT_VALIDATE_SIGNATURE` skips BLS check |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
