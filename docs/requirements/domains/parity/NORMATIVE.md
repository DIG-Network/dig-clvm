# Chia L1 Parity — Normative Requirements

> **Master spec:** [SPEC.md](../../../resources/SPEC.md) — Section 7

---

## &sect;1 Execution Parity

<a id="PAR-001"></a>**PAR-001** CLVM execution MUST use `clvmr` with `ChiaDialect`, the same runtime used by Chia L1 at [`multiprocess_validation.py:62`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L62). The crate MUST NOT contain a custom CLVM interpreter.
> **Spec:** [`PAR-001.md`](specs/PAR-001.md)

<a id="PAR-002"></a>**PAR-002** Condition parsing MUST use `chia-consensus` condition processing with `SpendVisitor`, covering all 40+ opcodes defined at [`condition_opcodes.py:7-73`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L7) including `SEND_MESSAGE(66)` and `RECEIVE_MESSAGE(67)` from [`:37-38`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L37).
> **Spec:** [`PAR-002.md`](specs/PAR-002.md)

<a id="PAR-003"></a>**PAR-003** Tree hash computation MUST use `clvm_utils::tree_hash()`, producing identical results to Chia L1 as tested in [`test_curry_and_treehash.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_curry_and_treehash.py).
> **Spec:** [`PAR-003.md`](specs/PAR-003.md)

---

## &sect;2 Signature Parity

<a id="PAR-004"></a>**PAR-004** BLS signature domain separation MUST match Chia L1's [`make_aggsig_final_message()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_tools.py#L74) at `condition_tools.py:74` for all 8 `AGG_SIG_*` variants. The message format MUST be `msg + addendum + additional_data[opcode]` per the lookup table at [`:87-97`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_tools.py#L87).
> **Spec:** [`PAR-004.md`](specs/PAR-004.md)

<a id="PAR-005"></a>**PAR-005** Aggregate signature verification MUST use `chia-bls::aggregate_verify()`, the same BLS12-381 implementation used by Chia L1.
> **Spec:** [`PAR-005.md`](specs/PAR-005.md)

---

## &sect;3 Cost Parity

<a id="PAR-006"></a>**PAR-006** Per-condition costs MUST match Chia L1: `AGG_SIG_COST=1_200_000` ([`condition_costs.py:8`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_costs.py#L8)), `CREATE_COIN_COST=1_800_000` ([`:9`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_costs.py#L9)), `GENERIC_CONDITION_COST=500` ([`:13`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_costs.py#L13)). This is achieved by using `chia-consensus` directly.
> **Spec:** [`PAR-006.md`](specs/PAR-006.md)

<a id="PAR-007"></a>**PAR-007** The per-spend cost limit MUST be `11_000_000_000`, matching Chia L1's [`MAX_BLOCK_COST_CLVM`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/default_constants.py#L68). The only permitted divergence is the per-block cost limit, which is configurable via `ValidationConfig`.
> **Spec:** [`PAR-007.md`](specs/PAR-007.md)

---

## &sect;4 Type Parity

<a id="PAR-008"></a>**PAR-008** `Coin`, `CoinSpend`, `SpendBundle`, and `Program` MUST be re-exported from `chia-protocol`, not redefined. This ensures wire-format compatibility with Chia peers.
> **Spec:** [`PAR-008.md`](specs/PAR-008.md)

<a id="PAR-009"></a>**PAR-009** Condition types MUST use `chia-sdk-types::Condition<T>`, not a custom enum. This ensures all opcodes are supported including future additions.
> **Spec:** [`PAR-009.md`](specs/PAR-009.md)

---

## &sect;5 Block Generator Parity

<a id="PAR-010"></a>**PAR-010** Block generators MUST be constructed using `solution_generator_backrefs()` from `chia-consensus` and validated using `run_block_generator2()`, matching Chia L1's [`mempool.py:529`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L529) and [`multiprocess_validation.py:69`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L69).
> **Spec:** [`PAR-010.md`](specs/PAR-010.md)

---

## &sect;6 Mempool Flag Parity

<a id="PAR-011"></a>**PAR-011** `MEMPOOL_MODE` and `DONT_VALIDATE_SIGNATURE` flags MUST produce identical behavior to Chia L1's usage at [`mempool.py:13-14`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L13). `MEMPOOL_MODE` MUST enable stricter validation; `DONT_VALIDATE_SIGNATURE` MUST skip BLS verification.
> **Spec:** [`PAR-011.md`](specs/PAR-011.md)
