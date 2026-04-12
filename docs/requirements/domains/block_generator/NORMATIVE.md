# Block Generator — Normative Requirements

> **Master spec:** [SPEC.md](../../../resources/SPEC.md) — Sections 5.1, 6.1 (Path B)

---

## &sect;1 Block Construction

<a id="BLK-001"></a>**BLK-001** `build_block_generator()` MUST accept a slice of `SpendBundle`, a `ValidationContext`, and a `max_cost: Cost`, and MUST return `Result<BlockGeneratorResult, ValidationError>`.
> **Spec:** [`BLK-001.md`](specs/BLK-001.md)

<a id="BLK-002"></a>**BLK-002** `build_block_generator()` MUST iterate bundles in the order provided, adding each bundle until the next would exceed `max_cost`. Bundles that exceed the remaining cost budget MUST be skipped.
> **Spec:** [`BLK-002.md`](specs/BLK-002.md)

<a id="BLK-003"></a>**BLK-003** The block generator program MUST be produced using `solution_generator_backrefs()` from `chia-consensus`, matching Chia L1's approach at [`mempool.py:529`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L529). This MUST deduplicate repeated puzzle bytecodes via CLVM back-references.
> **Spec:** [`BLK-003.md`](specs/BLK-003.md)

<a id="BLK-004"></a>**BLK-004** `BlockGeneratorResult` MUST contain `generator` (the compressed CLVM program), `block_refs` (previous block heights for cross-block back-references), `aggregated_signature`, `additions`, `removals`, `cost`, and `bundles_included`.
> **Spec:** [`BLK-004.md`](specs/BLK-004.md)

<a id="BLK-005"></a>**BLK-005** The `aggregated_signature` in `BlockGeneratorResult` MUST be the BLS aggregation of all individual bundle signatures for the included bundles.
> **Spec:** [`BLK-005.md`](specs/BLK-005.md)

---

## &sect;2 Block Validation

<a id="BLK-006"></a>**BLK-006** `validate_block()` MUST accept a `BlockGenerator`, `generator_refs` (previous block generators for back-reference resolution), `ValidationContext`, `ValidationConfig`, and optional `BlsCache`, and MUST return `Result<SpendResult, ValidationError>`.
> **Spec:** [`BLK-006.md`](specs/BLK-006.md)

<a id="BLK-007"></a>**BLK-007** `validate_block()` MUST delegate generator execution to `chia_consensus::run_block_generator2()`, matching Chia L1's post-hard-fork execution path at [`multiprocess_validation.py:69`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L69).
> **Spec:** [`BLK-007.md`](specs/BLK-007.md)

<a id="BLK-008"></a>**BLK-008** `validate_block()` MUST apply the same cost enforcement, condition validation, BLS signature verification, and conservation checks as `validate_spend_bundle()` (VAL-007 through VAL-014).
> **Spec:** [`BLK-008.md`](specs/BLK-008.md)

---

## &sect;3 Round-Trip Consistency

<a id="BLK-009"></a>**BLK-009** The `additions` and `removals` produced by `validate_block()` on the output of `build_block_generator()` MUST be identical to the union of `additions` and `removals` from individually validating each included bundle via `validate_spend_bundle()`.
> **Spec:** [`BLK-009.md`](specs/BLK-009.md)
