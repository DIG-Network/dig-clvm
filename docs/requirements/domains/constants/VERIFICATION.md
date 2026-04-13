# Network Constants — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [CON-001](NORMATIVE.md#CON-001) | ✅ | Separate `dig-constants` crate | Crate at `../dig-constants/`, compiles independently, no dig-clvm dep, no direct clvmr dep |
| [CON-002](NORMATIVE.md#CON-002) | ✅ | `NetworkConstants` type | All 6 accessors implemented: consensus(), genesis_challenge(), agg_sig_me_additional_data(), max_block_cost_clvm(), cost_per_byte(), max_coin_amount(). Debug+Clone derived. |
| [CON-003](NORMATIVE.md#CON-003) | ✅ | `DIG_MAINNET` and `DIG_TESTNET` | Both const values compile; distinct genesis challenges (0x00..00 vs 0x00..01) |
| [CON-004](NORMATIVE.md#CON-004) | ✅ | Fork heights set to 0 | hard_fork_height=0, hard_fork2_height=0 in both mainnet and testnet |
| [CON-005](NORMATIVE.md#CON-005) | ✅ | `AGG_SIG` additional data derivation | agg_sig_me = genesis_challenge. Others = sha256(genesis \|\| opcode_byte) with L1 opcodes 43-48. Cross-checked via Python. Mainnet/testnet differ. |
| [CON-006](NORMATIVE.md#CON-006) | ✅ | Neutral PoS/VDF fields | All PoS/VDF/plot filter fields set to valid neutral values matching Chia TEST_CONSTANTS |
| [CON-007](NORMATIVE.md#CON-007) | ✅ | Minimal dependencies | Cargo.toml: chia-consensus, chia-protocol, chia-bls, hex-literal only |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
