# dt-hard-rules — 14 Non-Negotiable Rules

Every rule below is a hard constraint. Violating any one of them is a blocking defect.

## Rule 1: Use chia crate ecosystem first

Before writing ANY custom code, check these crates in order:
- `chia-consensus` — run_spendbundle(), validate_clvm_and_signature(), run_block_generator2(), SpendVisitor
- `chia-sdk-types` — Condition, Conditions, run_puzzle, Mod
- `chia-sdk-driver` — SpendContext, Layer, Spend
- `chia-bls` — aggregate_verify, BlsCache
- `clvm-utils` — tree_hash, curry_tree_hash, CurriedProgram
- `clvmr` — Allocator, run_program, ChiaDialect

Only write custom logic when no upstream crate provides the needed functionality.

## Rule 2: No custom CLVM execution

Delegate to `run_spendbundle()` and `run_block_generator2()` from chia-consensus. Never call `run_program` directly for spend validation.

## Rule 3: No custom condition parsing

Use chia-consensus `SpendVisitor` trait for condition extraction. Never manually parse CLVM output into conditions.

## Rule 4: No async/IO/storage

This is a pure computation crate. No `async`, no `tokio`, no `std::fs`, no `std::net`, no database drivers. All IO belongs to the caller.

## Rule 5: Re-export, don't redefine

Re-export these types from upstream crates via `src/lib.rs`:
- `Coin`, `CoinSpend`, `SpendBundle` from chia-sdk-types
- `Condition<T>` from chia-consensus
- Never create your own `Coin` struct or similar.

## Rule 6: Tests MUST use chia-sdk-test::Simulator

All integration tests go through the Simulator. It provides realistic coin creation, spending, and mempool behavior. No hand-rolled test harnesses.

## Rule 7: One requirement per commit

Each commit implements exactly one requirement ID. No batching, no partial implementations.

## Rule 8: Update tracking after each requirement

After implementing a requirement, update:
- `TRACKING.yaml` — status, tests, notes
- `VERIFICATION.md` — status column, verification approach
- `IMPLEMENTATION_ORDER.md` — check off the `[ ]`

## Rule 9: SocratiCode search before file reads

Always use `codebase_search` before reading files. Search finds the right files; you read targeted sections. Never blindly cat entire directories.

## Rule 10: Repomix pack before implementation

Before writing implementation code:
```bash
npx repomix@latest <scope> -o .repomix/pack-<scope>.xml
```
Fresh context = better code. Pack the scope you are about to modify.

## Rule 11: GitNexus impact check before refactoring

Before renaming symbols or restructuring modules:
```bash
npx gitnexus analyze
```
Then check the dependency graph. Never rename a public symbol without understanding its dependents.

## Rule 12: dig-constants is separate

`dig-constants` is a sibling crate at `../dig-constants/`. No circular dependencies between dig-clvm and dig-constants. dig-clvm depends on dig-constants, never the reverse.

## Rule 13: DIG fork_height = 0

All Chia features are active from block 0 in DIG. There are no soft-fork activation heights. Pass `fork_height=0` wherever chia-consensus requires it.

## Rule 14: coin_records is minimal

The `coin_records` input to validation functions contains only the coins being spent in the current bundle/block. It is not a full UTXO set. The caller provides exactly the records needed.

## Post-Pull Rule

After `git pull`: treat `[x]` items in IMPLEMENTATION_ORDER.md as done. Only `[ ]` items are selectable for work. Never re-implement a checked item.

---

Navigation: Prev < [dt-role.md](dt-role.md) | Next > [dt-authoritative-sources.md](dt-authoritative-sources.md)
