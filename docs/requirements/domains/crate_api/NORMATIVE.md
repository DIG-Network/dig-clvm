# Crate API — Normative Requirements

> **Master spec:** [SPEC.md](../../../resources/SPEC.md) — Sections 3, 4, 5.2

---

## &sect;1 Re-exports

<a id="API-001"></a>**API-001** `dig-clvm` MUST re-export the following from upstream crates so callers need only `use dig_clvm::*`: `Allocator`, `NodePtr`, `Cost` (clvmr); `ToClvm`, `FromClvm` (clvm-traits); `tree_hash`, `curry_tree_hash`, `CurriedProgram`, `TreeHash` (clvm-utils); `Coin`, `CoinSpend`, `SpendBundle`, `Program`, `Bytes32` (chia-protocol); `PublicKey`, `SecretKey`, `Signature`, `aggregate_verify`, `BlsCache` (chia-bls); `Condition`, `Conditions`, `Mod` (chia-sdk-types); `SpendContext`, `Layer`, `Spend`, `SpendWithConditions`, `Puzzle`, `DriverError` (chia-sdk-driver); `CoinRecord` (chia-sdk-coinset); `ConsensusConstants`, `opcodes` (chia-consensus); `BlockGenerator` (chia-consensus); `DIG_MAINNET`, `DIG_TESTNET`, `NetworkConstants` (dig-constants).
> **Spec:** [`API-001.md`](specs/API-001.md)

---

## &sect;2 Dependency Boundaries

<a id="API-002"></a>**API-002** `dig-clvm` MUST NOT depend on `tokio`, `serde_json`, `rocksdb`, `reqwest`, or any async/IO/storage crate. The crate MUST be pure computation with no side effects.
> **Spec:** [`API-002.md`](specs/API-002.md)

<a id="API-003"></a>**API-003** `dig-clvm` MUST depend on individual `chia-sdk-*` crates (`chia-sdk-types`, `chia-sdk-driver`, `chia-sdk-coinset`), NOT the `chia-wallet-sdk` umbrella crate. This avoids pulling in RPC clients and wallet utilities.
> **Spec:** [`API-003.md`](specs/API-003.md)

<a id="API-004"></a>**API-004** `dig-clvm` MUST NOT perform any database lookups, network requests, or filesystem access. All external state MUST be passed in via function parameters.
> **Spec:** [`API-004.md`](specs/API-004.md)

---

## &sect;3 Error Handling

<a id="API-005"></a>**API-005** `ValidationError` MUST be an enum with variants: `Clvm`, `CoinNotFound`, `AlreadySpent`, `DoubleSpend`, `PuzzleHashMismatch`, `SignatureFailed`, `ConservationViolation`, `CostExceeded`, `Driver`. Each variant MUST carry enough context to identify the failing coin or condition.
> **Spec:** [`API-005.md`](specs/API-005.md)

<a id="API-006"></a>**API-006** `ValidationError` MUST implement `std::error::Error` and `Display` via `thiserror`. It MUST wrap `chia_consensus::validation_error::ValidationErr` for CLVM errors and `DriverError` for driver errors.
> **Spec:** [`API-006.md`](specs/API-006.md)

---

## &sect;4 No Reimplementation

<a id="API-007"></a>**API-007** The crate MUST NOT contain custom implementations of CLVM execution, condition parsing, tree hashing, currying, BLS signature verification, or opcode/cost constants. All of these MUST come from the upstream Chia crate ecosystem.
> **Spec:** [`API-007.md`](specs/API-007.md)

---

## &sect;5 Module Structure

<a id="API-008"></a>**API-008** The crate's own code MUST be confined to the `consensus/` module containing: `validate.rs`, `block.rs`, `context.rs`, `config.rs`, `result.rs`, `cache.rs`, `error.rs`. The `lib.rs` MUST contain only re-exports and the public API surface.
> **Spec:** [`API-008.md`](specs/API-008.md)
