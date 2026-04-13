# Spend Validation — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [VAL-001](NORMATIVE.md#VAL-001) | ❌ | Entry point signature | Signature matches spec. Compiles clean. |
| [VAL-002](NORMATIVE.md#VAL-002) | ❌ | Delegates to chia-consensus | Calls validate_clvm_and_signature(). No custom CLVM. |
| [VAL-003](NORMATIVE.md#VAL-003) | ❌ | Reject duplicate spends | HashSet dedup; DoubleSpend error. |
| [VAL-004](NORMATIVE.md#VAL-004) | ❌ | Reject missing coins | coin_records + ephemeral fallback; CoinNotFound error. |
| [VAL-005](NORMATIVE.md#VAL-005) | ❌ | Reject already-spent | coin_record.spent check; AlreadySpent error. |
| [VAL-006](NORMATIVE.md#VAL-006) | ❌ | Puzzle hash mismatch | Handled by validate_clvm_and_signature() internally. |
| [VAL-007](NORMATIVE.md#VAL-007) | ❌ | Cost enforcement | conditions.cost vs config.max_cost_per_block; CostExceeded error. |
| [VAL-008](NORMATIVE.md#VAL-008) | ❌ | Default cost constants | L1=11B, L2=550B, flags=0 in Default impl. |
| [VAL-009](NORMATIVE.md#VAL-009) | ❌ | Conditions delegated | All inside chia-consensus. No custom condition code. |
| [VAL-010](NORMATIVE.md#VAL-010) | ❌ | Ephemeral coins | ephemeral_coins fallback in coin existence check. |
| [VAL-011](NORMATIVE.md#VAL-011) | ❌ | Conservation check | input >= output; ConservationViolation with totals. |
| [VAL-012](NORMATIVE.md#VAL-012) | ❌ | BLS signatures | validate_clvm_and_signature() handles; flags pass-through. |
| [VAL-013](NORMATIVE.md#VAL-013) | ❌ | MEMPOOL_MODE | run_spendbundle() adds MEMPOOL_MODE internally. |
| [VAL-014](NORMATIVE.md#VAL-014) | ❌ | SpendResult | additions, removals, fee, conditions. |
| [VAL-015](NORMATIVE.md#VAL-015) | ❌ | No full UTXO | HashMap only, no storage access. |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
