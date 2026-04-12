# Start

## Immediate Actions

1. **Sync**
   ```bash
   git fetch origin && git pull origin main
   ```

2. **Check tools**
   ```bash
   npx gitnexus status          # GitNexus index fresh?
   npx gitnexus analyze         # Update if stale
   # SocratiCode: verify Docker running, index current
   codebase_status {}            # SocratiCode MCP status
   ```

3. **Pick work** — open `docs/requirements/IMPLEMENTATION_ORDER.md`
   - Choose the first `- [ ]` item
   - Every `- [x]` is done on main — skip it

4. **Pack context**
   ```bash
   npx repomix@latest src/consensus -o .repomix/pack-consensus.xml
   npx repomix@latest tests -o .repomix/pack-tests.xml
   ```

5. **Search with SocratiCode** — find related code before reading files
   ```
   codebase_search { query: "validate_spend_bundle" }
   codebase_graph_query { filePath: "src/consensus/validate.rs" }
   ```

6. **Read spec** — follow the trace:
   - `NORMATIVE.md#PREFIX-NNN` → authoritative requirement
   - `specs/PREFIX-NNN.md` → detailed specification
   - `VERIFICATION.md` → how to verify
   - `TRACKING.yaml` → current status

7. **Continue** → [dt-wf-select.md](tree/dt-wf-select.md)

---

## Hard Requirements

1. **Use chia crate ecosystem first** — never reimplement what `chia-consensus`, `chia-sdk-types`, `chia-sdk-driver`, `clvmr`, `clvm-utils`, `chia-bls` provide.
2. **No custom CLVM execution** — delegate to `run_spendbundle()` / `run_block_generator2()`.
3. **No custom condition parsing** — use `chia-consensus` `SpendVisitor`.
4. **No async/IO/storage** — dig-clvm is pure computation. All state passed in via parameters.
5. **Re-export, don't redefine** — `Coin`, `CoinSpend`, `SpendBundle`, `Condition<T>` come from upstream.
6. **Tests MUST use `chia-sdk-test::Simulator`** — same validation path as Chia L1.
7. **One requirement per commit** — don't batch unrelated work.
8. **Update tracking after each requirement** — VERIFICATION.md, TRACKING.yaml, IMPLEMENTATION_ORDER.md.
9. **SocratiCode before file reads** — search semantically first, read targeted files second.
10. **Repomix before implementation** — pack relevant scope for context.
11. **GitNexus before refactoring** — check dependency impact before renaming or moving symbols.
12. **`dig-constants` is a separate crate** — network parameters importable without CLVM engine.
13. **DIG uses fork_height=0** — all Chia consensus features from block 0.
14. **`coin_records` is minimal** — only coins being spent, never the full UTXO set.

---

## Tech Stack

| Component | Crate | Version |
|-----------|-------|---------|
| CLVM runtime | `clvmr` | 0.14 |
| Consensus engine | `chia-consensus` | 0.26 |
| Protocol types | `chia-protocol` | 0.26 |
| BLS signatures | `chia-bls` | 0.26 |
| Condition types | `chia-sdk-types` | 0.30 |
| Spend construction | `chia-sdk-driver` | 0.30 |
| Coin state | `chia-sdk-coinset` | 0.30 |
| Puzzle bytecodes | `chia-puzzles` | 0.20 |
| CLVM traits | `clvm-traits` | 0.26 |
| Tree hash / currying | `clvm-utils` | 0.26 |
| Network constants | `dig-constants` | 0.1.0 |
| Testing | `chia-sdk-test` | 0.30 |
