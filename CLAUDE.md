---

# dig-clvm — Project Context

## What This Is

`dig-clvm` is a standalone Rust crate used by DIG validators to validate spend bundles and compute coin additions/removals. It is a thin orchestration layer on top of the Chia crate ecosystem — it never reimplements what `chia-consensus`, `chia-sdk-types`, `chia-sdk-driver`, `clvmr`, `clvm-utils`, or `chia-bls` already provide.

## Key Documents

| Document | Path | Purpose |
|----------|------|---------|
| Master Spec | `SPEC.md` | Complete specification |
| Prompt System | `docs/prompt/start.md` | Workflow entry point |
| Requirements | `docs/requirements/README.md` | 56 requirements across 6 domains |
| Implementation Order | `docs/requirements/IMPLEMENTATION_ORDER.md` | Phased checklist |
| Network Constants | `../dig-constants/src/lib.rs` | DIG_MAINNET / DIG_TESTNET |

## Hard Rules

1. **Use chia crates first** — never reimplement upstream functionality
2. **No custom CLVM execution** — delegate to `run_spendbundle()` / `run_block_generator2()`
3. **No async/IO/storage** — pure computation, all state passed via parameters
4. **Re-export, don't redefine** — `Coin`, `CoinSpend`, `SpendBundle`, `Condition<T>` from upstream
5. **Tests use `chia-sdk-test::Simulator`** — same validation path as Chia L1
6. **One requirement per commit**
7. **SocratiCode before file reads** — search semantically first
8. **Repomix before implementation** — pack context for LLM
9. **GitNexus before refactoring** — check dependency impact

## Tool Usage

| Tool | When | Command |
|------|------|---------|
| SocratiCode | Before reading files | `codebase_search { query: "..." }` |
| GitNexus | Before refactoring | `gitnexus_impact({target: "symbol"})` |
| Repomix | Before implementing | `npx repomix@latest src/consensus -o .repomix/pack-consensus.xml` |

## Architecture

```
src/
  lib.rs              — Re-exports only
  consensus/
    validate.rs       — validate_spend_bundle()
    block.rs          — build_block_generator(), validate_block()
    context.rs        — ValidationContext
    config.rs         — ValidationConfig, cost constants
    result.rs         — SpendResult, BlockGeneratorResult
    cache.rs          — BlsCache integration
    error.rs          — ValidationError
```

## Requirement Domains

| Domain | Prefix | Count |
|--------|--------|-------|
| Spend Validation | VAL | 15 |
| Block Generator | BLK | 9 |
| BLS Cache | BLS | 5 |
| Chia L1 Parity | PAR | 12 |
| Crate API | API | 8 |
| Network Constants | CON | 7 |
