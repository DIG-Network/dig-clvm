# Network Constants — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [CON-001](NORMATIVE.md#CON-001) | ✅ | Separate `dig-constants` crate | Crate at `../dig-constants/`, compiles independently, no dig-clvm dep, no direct clvmr dep |
| [CON-002](NORMATIVE.md#CON-002) | ❌ | `NetworkConstants` type | Unit test: construct `NetworkConstants`, call `genesis_challenge()`, `agg_sig_me_additional_data()`, `max_block_cost_clvm()`, `cost_per_byte()`, `consensus()` |
| [CON-003](NORMATIVE.md#CON-003) | ❌ | `DIG_MAINNET` and `DIG_TESTNET` | Unit test: assert `DIG_MAINNET` and `DIG_TESTNET` are `const NetworkConstants` with distinct `genesis_challenge()` values |
| [CON-004](NORMATIVE.md#CON-004) | ❌ | Fork heights set to 0 | Unit test: assert `DIG_MAINNET.consensus().hard_fork_height == 0` and `hard_fork2_height == 0` for both networks |
| [CON-005](NORMATIVE.md#CON-005) | ❌ | `AGG_SIG` additional data derivation | Cross-validation test: verify `agg_sig_me_additional_data == genesis_challenge`; verify other `agg_sig_*` fields equal `sha256(genesis_challenge \|\| opcode_byte)` |
| [CON-006](NORMATIVE.md#CON-006) | ❌ | Neutral PoS/VDF fields | Unit test: verify PoS/VDF/plot filter fields in `ConsensusConstants` are set to valid neutral values; pass constants to `chia-consensus` function without panic |
| [CON-007](NORMATIVE.md#CON-007) | ❌ | Minimal dependencies | Cargo.toml inspection: verify `dig-constants` depends only on `chia-consensus`, `chia-protocol`, `chia-bls`, `hex-literal`; no dependency on `dig-clvm` |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
