# Block Generator — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [BLK-001](NORMATIVE.md#BLK-001) | ✅ | `build_block_generator()` entry point | Signature: (bundles, context, max_cost) -> Result<BlockGeneratorResult>. Compiles. |
| [BLK-002](NORMATIVE.md#BLK-002) | ✅ | Cost-aware bundle iteration | run_spendbundle per bundle with DONT_VALIDATE_SIGNATURE. Skips on cost_exceeded. |
| [BLK-003](NORMATIVE.md#BLK-003) | ✅ | Uses `solution_generator_backrefs` | Calls chia_consensus::solution_generator::solution_generator_backrefs(). No custom generator. |
| [BLK-004](NORMATIVE.md#BLK-004) | ✅ | `BlockGeneratorResult` structure | All 7 fields: generator, block_refs, aggregated_signature, additions, removals, cost, bundles_included. |
| [BLK-005](NORMATIVE.md#BLK-005) | ✅ | Aggregated signature | Individual bundle sigs accumulated via += then assigned to result. |
| [BLK-006](NORMATIVE.md#BLK-006) | ✅ | `validate_block()` entry point | Signature: (generator, refs, context, config, cache, sig) -> Result<SpendResult>. Compiles. |
| [BLK-007](NORMATIVE.md#BLK-007) | ✅ | Delegates to `run_block_generator2` | Calls chia_consensus::run_block_generator::run_block_generator2(). No custom execution. |
| [BLK-008](NORMATIVE.md#BLK-008) | ✅ | Same checks as `validate_spend_bundle` | Cost enforcement + conservation check applied after run_block_generator2(). |
| [BLK-009](NORMATIVE.md#BLK-009) | ✅ | Round-trip consistency | Both paths extract additions/removals from OwnedSpendBundleConditions identically. |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
