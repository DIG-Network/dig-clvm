# BLS Cache — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [BLS-001](NORMATIVE.md#BLS-001) | ❌ | Optional `BlsCache` parameter | Unit test: call `validate_spend_bundle` and `validate_block` with `Some(&mut cache)`, verify compilation and caching behavior |
| [BLS-002](NORMATIVE.md#BLS-002) | ❌ | `None` cache works correctly | Simulator test: validate a signed bundle with `bls_cache = None`, verify identical result to `Some` path with no degradation |
| [BLS-003](NORMATIVE.md#BLS-003) | ❌ | Mempool-to-block cache reuse | Benchmark test: validate bundle with warm cache from prior `validate_spend_bundle`, pass same cache to `validate_block`, verify faster execution |
| [BLS-004](NORMATIVE.md#BLS-004) | ❌ | Uses `chia-bls::BlsCache` directly | Code review: verify `BlsCache` type is imported from `chia-bls`, not custom-defined |
| [BLS-005](NORMATIVE.md#BLS-005) | ❌ | Cache doesn't affect correctness | Simulator test: validate same bundle with cold cache and warm cache, assert identical `SpendResult` |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
