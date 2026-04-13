# BLS Cache — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [BLS-001](NORMATIVE.md#BLS-001) | ✅ | Optional `BlsCache` parameter | `Option<&mut BlsCache>` on validate_spend_bundle. Three code paths: skip (flag), cache (Some), no-cache (None). |
| [BLS-002](NORMATIVE.md#BLS-002) | ✅ | `None` cache works correctly | None path falls through to validate_clvm_and_signature(). No behavioral change. |
| [BLS-003](NORMATIVE.md#BLS-003) | ✅ | Mempool-to-block cache reuse | BlsCache::aggregate_verify() stores pairings. Same instance reusable across calls. |
| [BLS-004](NORMATIVE.md#BLS-004) | ✅ | Uses `chia-bls::BlsCache` directly | `use chia_bls::BlsCache`. BlsCache::aggregate_verify() called directly. |
| [BLS-005](NORMATIVE.md#BLS-005) | ✅ | Cache doesn't affect correctness | All three paths use same run_spendbundle(). Cache only affects BLS pairing performance. |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
