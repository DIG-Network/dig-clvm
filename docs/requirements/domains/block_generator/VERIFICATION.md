# Block Generator — Verification

| ID | Status | Summary | Verification Approach |
|----|--------|---------|----------------------|
| [BLK-001](NORMATIVE.md#BLK-001) | ❌ | `build_block_generator()` entry point | Unit test: call `build_block_generator` with valid args, confirm it compiles and returns `Result<BlockGeneratorResult, ValidationError>` |
| [BLK-002](NORMATIVE.md#BLK-002) | ❌ | Cost-aware bundle iteration | Simulator test: pass multiple bundles where total cost exceeds `max_cost`, verify bundles beyond budget are skipped and included count is correct |
| [BLK-003](NORMATIVE.md#BLK-003) | ❌ | Uses `solution_generator_backrefs` | Code review: grep for `solution_generator_backrefs` call; verify no hand-rolled generator construction |
| [BLK-004](NORMATIVE.md#BLK-004) | ❌ | `BlockGeneratorResult` structure | Unit test: build block from bundles, verify result contains `generator`, `block_refs`, `aggregated_signature`, `additions`, `removals`, `cost`, `bundles_included` |
| [BLK-005](NORMATIVE.md#BLK-005) | ❌ | Aggregated signature across bundles | Simulator test: build block from two signed bundles, verify `aggregated_signature` equals BLS aggregation of individual signatures |
| [BLK-006](NORMATIVE.md#BLK-006) | ❌ | `validate_block()` entry point | Unit test: call `validate_block` with valid args, confirm it compiles and returns `Result<SpendResult, ValidationError>` |
| [BLK-007](NORMATIVE.md#BLK-007) | ❌ | Delegates to `run_block_generator2` | Code review: grep for `run_block_generator2` call inside `validate_block`; no hand-rolled generator execution |
| [BLK-008](NORMATIVE.md#BLK-008) | ❌ | Same checks as `validate_spend_bundle` | Simulator test: build invalid block (e.g., conservation violation), verify same error types as `validate_spend_bundle` |
| [BLK-009](NORMATIVE.md#BLK-009) | ❌ | Round-trip consistency | Integration test: build block from bundles, validate block, compare additions/removals to union of individual bundle validations |

**Status legend:** ✅ verified · ⚠️ partial · ❌ gap
