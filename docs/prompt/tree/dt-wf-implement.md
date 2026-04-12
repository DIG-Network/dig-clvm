# dt-wf-implement — Workflow: Implement Against Spec

## Step 0: SocratiCode + GitNexus Checks

Before writing any code:

```
codebase_search { query: "function or type being implemented" }
codebase_graph_query { filePath: "file to modify" }
```

If renaming or moving symbols:
```bash
npx gitnexus analyze
```
Then check impact via `gitnexus_impact { symbol: "..." }` before proceeding.

## Step 1: Use Chia Crates First

Check crates in this order before writing custom code:

| Priority | Crate | Provides |
|----------|-------|----------|
| 1 | `chia-consensus` | `run_spendbundle()`, `validate_clvm_and_signature()`, `run_block_generator2()`, `SpendVisitor` |
| 2 | `chia-sdk-types` | `Condition`, `Conditions`, `run_puzzle`, `Mod` |
| 3 | `chia-sdk-driver` | `SpendContext`, `Layer`, `Spend` |
| 4 | `chia-bls` | `aggregate_verify`, `BlsCache` |
| 5 | `clvm-utils` | `tree_hash`, `curry_tree_hash`, `CurriedProgram` |
| 6 | `clvmr` | `Allocator`, `run_program`, `ChiaDialect` |

Only write custom logic in `src/consensus/` when no upstream crate provides the needed functionality.

## Step 2: Implementation by Module

| Module | File | What Goes Here |
|--------|------|----------------|
| validate | `src/consensus/validate.rs` | `validate_spend_bundle()` orchestration |
| block | `src/consensus/block.rs` | `build_block_generator()`, `validate_block()` |
| context | `src/consensus/context.rs` | `ValidationContext` struct |
| config | `src/consensus/config.rs` | `ValidationConfig`, cost constants |
| result | `src/consensus/result.rs` | `SpendResult`, `BlockGeneratorResult` |
| cache | `src/consensus/cache.rs` | `BlsCache` integration helpers |
| error | `src/consensus/error.rs` | `ValidationError` enum |
| lib | `src/lib.rs` | Re-exports ONLY |

### Module Rules

- `lib.rs` contains `pub mod consensus;` and re-exports. No logic.
- Each `src/consensus/*.rs` file has a single responsibility.
- New modules require `pub mod` in `src/consensus/mod.rs` and re-export in `src/lib.rs`.

## Step 3: Smallest Change Principle

- **Match the spec exactly.** Implement what the dedicated spec says, nothing more.
- **No features beyond the requirement.** If VAL-001 says "entry point", you build the entry point. You do not add caching, logging, or metrics.
- **No speculative abstractions.** Do not create traits "for future use." Do not add generic parameters unless the spec requires them.
- **Three similar lines > premature abstraction.** Wait for the pattern to appear three times before extracting a helper.

## Implementation Checklist

Before moving to validation, verify:

- [ ] Code matches the dedicated spec's acceptance criteria
- [ ] Uses chia crate functions where available (Rule 1)
- [ ] No custom CLVM execution (Rule 2)
- [ ] No custom condition parsing (Rule 3)
- [ ] No async/IO/storage (Rule 4)
- [ ] Re-exports upstream types, does not redefine them (Rule 5)
- [ ] New public API is re-exported in `src/lib.rs`
- [ ] Tests from dt-wf-test now pass

---

Navigation: Prev < [dt-wf-test.md](dt-wf-test.md) | Next > [dt-wf-validate.md](dt-wf-validate.md)
