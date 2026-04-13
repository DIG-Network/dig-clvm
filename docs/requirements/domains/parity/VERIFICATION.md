# Chia L1 Parity — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [PAR-001](NORMATIVE.md#PAR-001) | ❌ | `clvmr` with `ChiaDialect` | clvmr in Cargo.toml. run_spendbundle/run_block_generator2 use ChiaDialect internally. |
| [PAR-002](NORMATIVE.md#PAR-002) | ❌ | `chia-consensus` condition parsing | chia-consensus in Cargo.toml. run_spendbundle handles all condition parsing via SpendVisitor. |
| [PAR-003](NORMATIVE.md#PAR-003) | ❌ | `clvm_utils` tree hash | clvm-utils in Cargo.toml. tree_hash re-exported in lib.rs. |
| [PAR-004](NORMATIVE.md#PAR-004) | ❌ | BLS domain separation | Delegated to chia-consensus internally. Domain separation applied by run_spendbundle. |
| [PAR-005](NORMATIVE.md#PAR-005) | ❌ | `chia-bls` `aggregate_verify` | chia-bls in Cargo.toml. BlsCache::aggregate_verify used. validate_clvm_and_signature uses chia-bls. |
| [PAR-006](NORMATIVE.md#PAR-006) | ❌ | Per-condition costs | Costs from chia-consensus (AGG_SIG=1.2M, CREATE_COIN=1.8M, GENERIC=500). Not reimplemented. |
| [PAR-007](NORMATIVE.md#PAR-007) | ❌ | Per-spend cost limit | L1_MAX_COST_PER_SPEND=11B. Per-block is configurable via ValidationConfig. |
| [PAR-008](NORMATIVE.md#PAR-008) | ❌ | Protocol types re-exported | Coin, CoinSpend, SpendBundle, Program re-exported from chia-protocol in lib.rs. |
| [PAR-009](NORMATIVE.md#PAR-009) | ❌ | Condition types from SDK | Condition, Conditions re-exported from chia-sdk-types in lib.rs. Not custom-defined. |
| [PAR-010](NORMATIVE.md#PAR-010) | ❌ | Block generators match L1 | solution_generator_backrefs + run_block_generator2 from chia-consensus in block.rs. |
| [PAR-011](NORMATIVE.md#PAR-011) | ❌ | Mempool flags match L1 | DONT_VALIDATE_SIGNATURE and MEMPOOL_MODE imported from chia-consensus::flags. |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
