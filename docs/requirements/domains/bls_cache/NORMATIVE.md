# BLS Cache — Normative Requirements

> **Master spec:** [SPEC.md](../../../resources/SPEC.md) — Sections 5.1, 10

---

## &sect;1 Cache Integration

<a id="BLS-001"></a>**BLS-001** `validate_spend_bundle()` and `validate_block()` MUST accept an `Option<&mut BlsCache>` parameter. When `Some`, verified signature pairings MUST be cached for reuse across calls.
> **Spec:** [`BLS-001.md`](specs/BLS-001.md)

<a id="BLS-002"></a>**BLS-002** When `bls_cache` is `None`, signature verification MUST proceed from scratch every time with no performance degradation or behavioral change beyond the absence of caching.
> **Spec:** [`BLS-002.md`](specs/BLS-002.md)

---

## &sect;2 Cache Behavior

<a id="BLS-003"></a>**BLS-003** A signature pairing verified during mempool validation (`validate_spend_bundle()`) MUST be reusable during subsequent block validation (`validate_block()`) when the same `BlsCache` instance is passed to both calls.
> **Spec:** [`BLS-003.md`](specs/BLS-003.md)

<a id="BLS-004"></a>**BLS-004** The `BlsCache` MUST use `chia-bls::BlsCache` directly, not a custom implementation. This ensures identical caching behavior to Chia L1.
> **Spec:** [`BLS-004.md`](specs/BLS-004.md)

---

## &sect;3 Correctness

<a id="BLS-005"></a>**BLS-005** The presence or absence of a `BlsCache` MUST NOT affect validation correctness. A bundle that passes validation with a warm cache MUST also pass with a cold cache, and vice versa.
> **Spec:** [`BLS-005.md`](specs/BLS-005.md)
