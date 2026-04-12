# Network Constants — Normative Requirements

> **Master spec:** [SPEC.md](../../../resources/SPEC.md) — Section 4.3

---

## &sect;1 Sibling Crate

<a id="CON-001"></a>**CON-001** DIG network parameters MUST be defined in a separate `dig-constants` crate so that any DIG crate can import them without pulling in the full CLVM engine.
> **Spec:** [`CON-001.md`](specs/CON-001.md)

<a id="CON-002"></a>**CON-002** `dig-constants` MUST export a `NetworkConstants` type that wraps `chia_consensus::ConsensusConstants` and provides accessor methods for `genesis_challenge()`, `agg_sig_me_additional_data()`, `max_block_cost_clvm()`, `cost_per_byte()`, and `consensus()`.
> **Spec:** [`CON-002.md`](specs/CON-002.md)

<a id="CON-003"></a>**CON-003** `dig-constants` MUST export `DIG_MAINNET` and `DIG_TESTNET` as `const NetworkConstants` values with distinct genesis challenges.
> **Spec:** [`CON-003.md`](specs/CON-003.md)

---

## &sect;2 Consensus Compatibility

<a id="CON-004"></a>**CON-004** `hard_fork_height` and `hard_fork2_height` in DIG constants MUST be set to `0`, enabling all Chia consensus features from block 0. DIG L2 does not have phased fork activations.
> **Spec:** [`CON-004.md`](specs/CON-004.md)

<a id="CON-005"></a>**CON-005** The `agg_sig_me_additional_data` field MUST equal the DIG genesis challenge. All other `agg_sig_*_additional_data` fields MUST be derived as `sha256(genesis_challenge || opcode_byte)`, matching the Chia L1 derivation at [`condition_tools.py:58-71`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_tools.py#L58).
> **Spec:** [`CON-005.md`](specs/CON-005.md)

<a id="CON-006"></a>**CON-006** Proof-of-space, VDF, plot filter, and weight proof fields in `ConsensusConstants` MUST be set to neutral/valid values. DIG does not use Chia's PoS consensus, but these fields must be valid since `ConsensusConstants` is passed to `chia-consensus` functions.
> **Spec:** [`CON-006.md`](specs/CON-006.md)

---

## &sect;3 Dependencies

<a id="CON-007"></a>**CON-007** `dig-constants` MUST depend only on `chia-consensus`, `chia-protocol`, `chia-bls`, and `hex-literal`. It MUST NOT depend on `dig-clvm` or any other DIG crate to avoid circular dependencies.
> **Spec:** [`CON-007.md`](specs/CON-007.md)
