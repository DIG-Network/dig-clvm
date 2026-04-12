# Spend Validation — Normative Requirements

> **Master spec:** [SPEC.md](../../../resources/SPEC.md) — Sections 5.1, 6.1

---

## &sect;1 Entry Point

<a id="VAL-001"></a>**VAL-001** `validate_spend_bundle()` MUST accept a `SpendBundle`, `ValidationContext`, `ValidationConfig`, and optional `BlsCache`, and MUST return `Result<SpendResult, ValidationError>`.
> **Spec:** [`VAL-001.md`](specs/VAL-001.md)

<a id="VAL-002"></a>**VAL-002** `validate_spend_bundle()` MUST internally delegate CLVM execution and condition extraction to `chia_consensus::run_spendbundle()`, not a hand-rolled implementation.
> **Spec:** [`VAL-002.md`](specs/VAL-002.md)

---

## &sect;2 Structural Checks

<a id="VAL-003"></a>**VAL-003** `validate_spend_bundle()` MUST reject bundles containing duplicate spends (same coin spent twice) with `ValidationError::DoubleSpend`.
> **Spec:** [`VAL-003.md`](specs/VAL-003.md)

<a id="VAL-004"></a>**VAL-004** `validate_spend_bundle()` MUST reject bundles where a spent coin does not exist in `context.coin_records` and is not in `context.ephemeral_coins`, with `ValidationError::CoinNotFound`.
> **Spec:** [`VAL-004.md`](specs/VAL-004.md)

<a id="VAL-005"></a>**VAL-005** `validate_spend_bundle()` MUST reject bundles where a spent coin is already marked as spent in `context.coin_records`, with `ValidationError::AlreadySpent`.
> **Spec:** [`VAL-005.md`](specs/VAL-005.md)

<a id="VAL-006"></a>**VAL-006** `validate_spend_bundle()` MUST reject bundles where the tree hash of `puzzle_reveal` does not equal the coin's `puzzle_hash`, with `ValidationError::PuzzleHashMismatch`.
> **Spec:** [`VAL-006.md`](specs/VAL-006.md)

---

## &sect;3 Cost Enforcement

<a id="VAL-007"></a>**VAL-007** `validate_spend_bundle()` MUST reject bundles whose total CLVM execution cost exceeds `config.max_cost_per_block`, with `ValidationError::CostExceeded`.
> **Spec:** [`VAL-007.md`](specs/VAL-007.md)

<a id="VAL-008"></a>**VAL-008** The default `max_cost_per_spend` MUST be `11_000_000_000` (matching Chia L1's [`MAX_BLOCK_COST_CLVM`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/default_constants.py#L68)). The default `max_cost_per_block` MUST be `550_000_000_000` (50x L1 per-spend).
> **Spec:** [`VAL-008.md`](specs/VAL-008.md)

---

## &sect;4 Condition Validation

<a id="VAL-009"></a>**VAL-009** All condition validation (announcement matching, concurrent spend assertions, identity assertions, time/height locks, ephemeral assertions) MUST be delegated to `chia-consensus`, not reimplemented.
> **Spec:** [`VAL-009.md`](specs/VAL-009.md)

<a id="VAL-010"></a>**VAL-010** Ephemeral coins (created and spent in the same block) MUST be supported. Coins in `context.ephemeral_coins` MUST be treated as valid spend targets even though they are not in `context.coin_records`.
> **Spec:** [`VAL-010.md`](specs/VAL-010.md)

---

## &sect;5 Conservation

<a id="VAL-011"></a>**VAL-011** `validate_spend_bundle()` MUST verify that `sum(removal amounts) >= sum(addition amounts) + fee`. Violations MUST produce `ValidationError::ConservationViolation`.
> **Spec:** [`VAL-011.md`](specs/VAL-011.md)

---

## &sect;6 BLS Signature Verification

<a id="VAL-012"></a>**VAL-012** Unless `config.flags` includes `DONT_VALIDATE_SIGNATURE`, `validate_spend_bundle()` MUST verify the aggregated BLS signature against all `AGG_SIG_*` conditions with correct domain separation per variant. Failures MUST produce `ValidationError::SignatureFailed`.
> **Spec:** [`VAL-012.md`](specs/VAL-012.md)

<a id="VAL-013"></a>**VAL-013** When `config.flags` includes `MEMPOOL_MODE`, validation MUST use stricter rules (reject unknown opcodes, stricter cost accounting), matching Chia L1's mempool behavior at [`mempool.py:13`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L13).
> **Spec:** [`VAL-013.md`](specs/VAL-013.md)

---

## &sect;7 Output

<a id="VAL-014"></a>**VAL-014** On success, `SpendResult` MUST contain `additions` (coins created by `CREATE_COIN` conditions), `removals` (coins consumed by the bundle), `fee` (sum of removals minus sum of additions), and `conditions` (`OwnedSpendBundleConditions` from `chia-consensus`).
> **Spec:** [`VAL-014.md`](specs/VAL-014.md)

<a id="VAL-015"></a>**VAL-015** `coin_records` in `ValidationContext` MUST contain only the coins relevant to the current validation, NOT the full UTXO set. The crate MUST NOT perform any database or storage lookups.
> **Spec:** [`VAL-015.md`](specs/VAL-015.md)
