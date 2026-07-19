# dig-clvm: Modular CLVM Consensus Crate

## 1. Purpose

`dig-clvm` is a standalone Rust crate used by DIG validators to **validate spend bundles and compute the resulting coin additions and removals**. It is the module that runs CLVM programs when coins are spent, producing the set of state changes (new coins created, old coins destroyed) that the caller then commits to blockchain state.

It is built as a **thin orchestration layer on top of the Chia crate ecosystem**, not a reimplementation. The Chia crates already provide the CLVM runtime, condition types, tree hashing, currying, BLS signatures, puzzle drivers, and an in-memory simulator. `dig-clvm` composes these into a consensus-grade API with L2-specific validation rules and configurable cost limits.

### Core Contract

```
Input:  SpendBundle (coin spends + aggregated BLS signature)
Output: SpendResult { additions: Vec<Coin>, removals: Vec<Coin>, fee: u64 }
        or ValidationError
```

Blockchain state management (UTXO set persistence, Merkle roots, block assembly) is **out of scope**. This crate validates and computes; the caller commits to state.

This mirrors how Chia L1 separates execution from state: [`tx_removals_and_additions()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/generator_tools.py#L54) extracts additions and removals from `SpendBundleConditions`, then the caller ([`validate_block_body()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/block_body_validation.py#L191)) commits them to the coin store.

### Goals

1. **Maximize reuse** - Use `chia-consensus`, `chia-sdk-types`, `chia-sdk-driver`, `clvmr`, `clvm-utils`, `clvm-traits`, `chia-bls`, and `chia-sdk-test` directly. Never reimplement what they provide.
2. **1:1 Chia L1 parity** - Identical behavior to Chia full nodes because we use the same libraries they use. Parity is achieved by construction, not by reimplementation.
3. **Self-contained testing** - The crate ships with its own test suite using `chia-sdk-test::Simulator` for full spend-bundle-level tests, plus targeted unit tests for L2-specific logic.
4. **Consensus-grade API** - A well-defined public surface for DIG validators, mempools, and block builders. No UI, networking, or storage concerns leak in.

### Non-Goals

- Compiling Chialisp source to CLVM bytecode (use `clvm_tools_rs` externally).
- Blockchain state management (UTXO set persistence, Merkle roots, block storage, state commitment).
- Reimplementing anything the Chia crates already provide.

---

## 2. Chia Crate Ecosystem Inventory

This section documents what each upstream crate provides and how `dig-clvm` uses it. This is the foundation: everything listed here is **used directly, not reimplemented**.

### 2.1 `clvmr` (v0.14) - CLVM Runtime

The bytecode interpreter. Everything starts here. Chia L1 delegates all CLVM execution to this Rust crate via `chia_rs` — see the import at [`multiprocess_validation.py:20-21`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L20) where `run_block_generator` and `run_block_generator2` are pulled from `chia_rs`.

| API | Purpose |
|---|---|
| `run_program(allocator, dialect, program, env, max_cost)` | Execute CLVM bytecode with cost tracking |
| `Allocator` | Memory manager for CLVM S-expressions |
| `NodePtr`, `SExp` | Typed pointers and atom/pair discrimination |
| `ChiaDialect::new(flags)` | Chia-specific CLVM dialect with soft-fork flags |
| `Cost` | `u64` type alias for cost tracking |
| `serde::{node_from_bytes, node_to_bytes}` | Binary serialization of CLVM trees |
| `MEMPOOL_MODE`, `NO_UNKNOWN_OPS`, `LIMIT_HEAP` | Execution flag constants — Chia L1 imports these at [`mempool.py:13-14`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L13) |

**dig-clvm usage**: Direct dependency. The `run_program` function is the core execution primitive.

### 2.2 `clvm-traits` (v0.26) - Serialization Traits

| API | Purpose |
|---|---|
| `ToClvm<E>` / `FromClvm<D>` | Encode/decode Rust types to/from CLVM |
| `#[derive(ToClvm, FromClvm)]` | Derive macros with `#[clvm(list)]`, `#[clvm(curry)]` attributes |

**dig-clvm usage**: Re-exported. Callers use these to define custom puzzle arguments.

### 2.3 `clvm-utils` (v0.26) - Tree Hash & Currying

Chia L1 tests tree hash correctness in [`test_curry_and_treehash.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_curry_and_treehash.py).

| API | Purpose |
|---|---|
| `tree_hash(allocator, node)` | Canonical CLVM tree hash (atom: `sha256(0x01 \|\| data)`, pair: `sha256(0x02 \|\| left \|\| right)`) |
| `tree_hash_atom(bytes)` / `tree_hash_pair(left, right)` | Standalone hash primitives |
| `TreeHash` | 32-byte hash newtype with conversions |
| `CurriedProgram<P, A>` | `(a (q . program) args)` representation with CLVM serialization |
| `curry_tree_hash(program_hash, arg_hashes)` | Compute curried puzzle hash without an allocator |
| `ToTreeHash` | Trait for types that can compute their own tree hash |

**dig-clvm usage**: Re-exported. Used for puzzle hash computation and currying throughout.

### 2.4 `chia-protocol` (v0.26) - Core Protocol Types

These are the canonical wire types used by Chia full nodes. `SpendBundle` is imported from `chia_rs` at [`mempool_manager.py:6`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/mempool_item.py#L6), and `CoinSpend` at [`coin_spend.py:6`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/coin_spend.py#L6).

| API | Purpose |
|---|---|
| `Coin` | `(parent_coin_info, puzzle_hash, amount)` with `coin_id()` method |
| `CoinSpend` | `(coin, puzzle_reveal, solution)` |
| `SpendBundle` | `(coin_spends, aggregated_signature)` with `aggregate()`, `additions()`, `name()` |
| `Program` | Serialized CLVM bytecode wrapper |
| `CoinState` | Coin with creation/spend height tracking |
| `Bytes`, `Bytes32` | Byte array types |

**dig-clvm usage**: Re-exported. These are the canonical wire types.

### 2.5 `chia-consensus` (v0.26) - Consensus Validation Engine

This is the critical crate. It provides the **actual consensus logic** used by Chia full nodes. On L1, the mempool calls [`validate_clvm_and_signature()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool_manager.py#L445) at `mempool_manager.py:445`, and block validation calls [`_run_block()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L62) at `multiprocess_validation.py:62` which selects between `run_block_generator` (pre-hard-fork) and `run_block_generator2` (post-hard-fork) at [lines 69-71](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L69).

| API | Purpose |
|---|---|
| `run_spendbundle(allocator, bundle, max_cost, height, constants, flags)` | Execute all spends in a bundle, extract and validate conditions |
| `validate_clvm_and_signature(allocator, bundle, max_cost, constants, height)` | Full validation: CLVM execution + BLS signature verification |
| `get_conditions_from_spendbundle(allocator, bundle, max_cost, constants, height, flags)` | Conditions without signature validation |
| `SpendBundleConditions` / `OwnedSpendBundleConditions` | Aggregated conditions across all spends — imported at [`multiprocess_validation.py:17`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L17) |
| `SpendConditions` / `OwnedSpendConditions` | Per-spend conditions (coin_id, puzzle_hash, create_coin, agg_sig_*, etc.) |
| `SpendVisitor` trait | Hook into condition processing (e.g., `MempoolVisitor`) |
| `ConsensusConstants` | All blockchain parameters (costs, fork heights, additional_data) — instantiated at [`default_constants.py:13`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/default_constants.py#L13) |
| `get_flags_for_height_and_constants(height, constants)` | Compute execution flags for a given block height — imported at [`multiprocess_validation.py:19`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L19) |
| `make_allocator(flags)` | Factory for properly-configured allocators |
| Opcode constants | `AGG_SIG_ME`, `CREATE_COIN`, etc. — defined at [`condition_opcodes.py:7-73`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L7) |
| Cost constants | `AGG_SIG_COST=1_200_000` ([`condition_costs.py:8`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_costs.py#L8)), `CREATE_COIN_COST=1_800_000` ([`:9`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_costs.py#L9)), `GENERIC_CONDITION_COST=500` ([`:13`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_costs.py#L13)) |

**dig-clvm usage**: The validation backbone. `run_spendbundle()` replaces the hand-rolled `run_puzzle()` + `parse_conditions()` loop. `OwnedSpendBundleConditions` replaces the custom `ClvmResult` + `Condition` enum for consensus paths.

### 2.6 `chia-bls` (v0.26) - BLS12-381 Signatures

Chia L1 constructs signature messages via [`make_aggsig_final_message()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_tools.py#L74) at `condition_tools.py:74`, and collects (pubkey, message) pairs via [`pkm_pairs()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_tools.py#L100) at `:100`. The domain separation lookup table is at [`:87-97`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_tools.py#L87):

```python
COIN_TO_ADDENDUM_F_LOOKUP = {
    AGG_SIG_PARENT:         lambda coin: coin.parent_coin_info,
    AGG_SIG_PUZZLE:         lambda coin: coin.puzzle_hash,
    AGG_SIG_AMOUNT:         lambda coin: int_to_bytes(coin.amount),
    AGG_SIG_PUZZLE_AMOUNT:  lambda coin: coin.puzzle_hash + int_to_bytes(coin.amount),
    AGG_SIG_PARENT_AMOUNT:  lambda coin: coin.parent_coin_info + int_to_bytes(coin.amount),
    AGG_SIG_PARENT_PUZZLE:  lambda coin: coin.parent_coin_info + coin.puzzle_hash,
    AGG_SIG_ME:             lambda coin: coin.name(),
}
final_message = msg + addendum + additional_data[opcode]
```

| API | Purpose |
|---|---|
| `PublicKey`, `SecretKey`, `Signature` | Key and signature types |
| `sign(sk, msg)` | Sign a message |
| `verify(pk, msg, sig)` | Verify a single signature |
| `aggregate_verify(pks, msgs, sig)` | Verify aggregated BLS signature |
| `aggregate(signatures)` | Aggregate multiple signatures |
| `BlsCache` | Cache for verified signature pairings |

**dig-clvm usage**: Re-exported. Used by the validation layer for signature verification.

### 2.7 `chia-sdk-types` (v0.30) - SDK Type Definitions

| API | Purpose |
|---|---|
| `Condition<T>` enum | All condition opcodes with typed fields, generic over node representation. Covers all opcodes from [`condition_opcodes.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L7) including [`SEND_MESSAGE(66)` / `RECEIVE_MESSAGE(67)`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L37) (Chia 2.3+) |
| `Conditions<T>` | Builder for condition lists with `.with()` chaining |
| `run_puzzle(allocator, puzzle, solution)` | Convenience wrapper: `ChiaDialect(0)` + 11B cost limit (mirrors L1's [`MAX_BLOCK_COST_CLVM=11_000_000_000`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/default_constants.py#L68)) |
| `announcement_id(source, message)` | Compute announcement hash |
| `Mod` trait | `mod_reveal()`, `mod_hash()`, `curry_tree_hash()` for puzzle modules |
| `MAINNET_CONSTANTS`, `TESTNET11_CONSTANTS` | Network-specific constants |
| `MerkleTree`, `MerkleProof` | Merkle proof construction and verification |

**dig-clvm usage**: `Condition<T>` and `Conditions<T>` are the primary condition types for spend construction and inspection. `run_puzzle()` is the high-level execution function. `Mod` is the trait for defining puzzle modules.

### 2.8 `chia-sdk-driver` (v0.30) - Puzzle Drivers & Spend Building

| API | Purpose |
|---|---|
| `SpendContext` | Allocator wrapper with puzzle caching, currying, CLVM execution, and spend collection |
| `Layer` trait | Composable puzzle abstraction: `parse_puzzle`, `construct_puzzle`, `construct_solution`, `construct_spend` |
| `Spend { puzzle, solution }` | Puzzle + solution pair as NodePtrs |
| `SpendWithConditions` trait | Build spends from condition lists |
| `Puzzle` enum | `Curried(CurriedPuzzle)` or `Raw(RawPuzzle)` parsed puzzles |
| `DriverError` | Comprehensive error type for CLVM/driver operations |
| Primitives: `Launcher`, `Cat`, `Did`, `Nft`, `Singleton`, `Vault`, etc. | High-level asset type drivers — corresponding to puzzles tested at [`test_puzzle_drivers.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_puzzle_drivers.py) and [`test_singletons.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_singletons.py) |
| `StandardLayer` | Standard XCH transaction puzzle (p2_delegated_puzzle_or_hidden_puzzle) |
| Action system | CHIP-0050 action-based spending pattern |

**dig-clvm usage**: `SpendContext` is the primary interface for test fixture construction and any puzzle-building operations. `Layer` implementations for standard puzzles are used in tests and by callers building transactions.

### 2.9 `chia-sdk-test` (v0.30) - Testing Infrastructure

The Simulator uses `chia_consensus::run_spendbundle()` internally — the same code path that Chia L1 uses at [`_run_block()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L62) for block validation and [`pre_validate_spendbundle()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool_manager.py#L428) for mempool admission.

| API | Purpose |
|---|---|
| `Simulator` | In-memory blockchain with coin state tracking |
| `Simulator::new_transaction(bundle)` | Validate and apply a spend bundle |
| `Simulator::spend_coins(spends, keys)` | Convenience for spending with auto-signing |
| `BlsPair` / `BlsPairWithCoin` | Test key pairs with pre-funded coins |
| `to_program()`, `to_puzzle()` | Convert ToClvm values to Program/puzzle_hash |
| `expect_spend(result, should_pass)` | Assert spend success/failure |

**dig-clvm usage**: Primary testing tool. Gives us L1 parity in tests by construction.

### 2.10 `chia-puzzles` (v0.20) - Puzzle Bytecode

150+ hardcoded Chialisp puzzle constants including:
- `P2_DELEGATED_PUZZLE_OR_HIDDEN_PUZZLE` (standard transaction)
- `SINGLETON_TOP_LAYER_V1_1`, `SINGLETON_LAUNCHER`
- `CAT_PUZZLE`, `DID_INNERPUZ`, `NFT_STATE_LAYER`
- `SETTLEMENT_PAYMENT`, `VAULT`, `CLAWBACK`
- All with pre-computed `_HASH` variants

**dig-clvm usage**: Re-exported for callers that need puzzle bytecodes. Used in parity tests.

### 2.11 `chia-sdk-coinset` (v0.30) - Coin State

Mirrors the L1 `CoinRecord` used throughout [`check_time_locks()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/check_time_locks.py#L15) and [`add_spend_bundle()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool_manager.py#L480).

| API | Purpose |
|---|---|
| `CoinRecord` | `{ coin, coinbase, confirmed_block_index, spent, spent_block_index, timestamp }` |

**dig-clvm usage**: Re-exported. Used in `ValidationContext` for tracking coin state.

---

## 3. What dig-clvm Adds (L2 Consensus Orchestration)

The Chia crates provide all the primitives. `dig-clvm` adds the **L2-specific orchestration** that composes them into a consensus API.

On Chia L1, this orchestration role is split between Python code in [`mempool_manager.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool_manager.py#L428) (mempool path) and [`block_body_validation.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/block_body_validation.py#L191) (block path), with the heavy lifting delegated to Rust via `chia_rs`. `dig-clvm` does the same thing as a single Rust crate.

### 3.1 What dig-clvm Owns

| Component | Why it can't come from Chia crates |
|---|---|
| `ValidationContext` | L2 chain state: height, timestamp, genesis challenge, unspent coin set, ephemeral tracking |
| `ValidationConfig` | L2-specific parameters: configurable cost limits (L2 block cost differs from L1's [`MAX_BLOCK_COST_CLVM=11B`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/default_constants.py#L68)), `MEMPOOL_MODE` flag support for SpendVisitor |
| `validate_spend_bundle()` | Orchestrates `chia-consensus::run_spendbundle()` with L2 rules, produces additions + removals |
| `build_block_generator()` | Assembles spend bundles into a compressed block generator via `solution_generator_backrefs()` |
| `validate_block()` | Orchestrates `chia-consensus::run_block_generator2()` for block-level validation |
| `SpendResult` | The output: `additions` (new coins), `removals` (spent coins), `fee`, and full conditions for inspection |
| `ValidationError` | L2-specific error enum wrapping `chia-consensus` errors with additional context |
| BLS cache management | `validate_spend_bundle()` and `validate_block()` accept `Option<&mut BlsCache>` to avoid re-verifying signatures across mempool → block |
| L2 cost constants | `MAX_COST_PER_BLOCK` for L2 (50x L1 per-spend limit) |

### 3.2 What dig-clvm Re-exports

Everything else comes from upstream:

```rust
// Core execution
pub use clvmr::{Allocator, NodePtr, Cost};
pub use chia_sdk_types::run_puzzle;

// Conditions
pub use chia_sdk_types::{Condition, Conditions};

// Consensus engine
pub use chia_consensus::spendbundle_conditions::run_spendbundle;
pub use chia_consensus::spendbundle_validation::validate_clvm_and_signature;
pub use chia_consensus::conditions::{
    SpendBundleConditions, OwnedSpendBundleConditions,
    SpendConditions, OwnedSpendConditions,
};
pub use chia_consensus::consensus_constants::ConsensusConstants;
pub use chia_consensus::opcodes::*;

// Types
pub use chia_protocol::{Coin, CoinSpend, SpendBundle, Program, Bytes32, CoinState};
pub use chia_bls::{PublicKey, SecretKey, Signature, aggregate_verify, BlsCache};
pub use chia_sdk_coinset::CoinRecord;

// Hashing & currying
pub use clvm_utils::{tree_hash, curry_tree_hash, CurriedProgram, TreeHash, ToTreeHash};

// Serialization
pub use clvm_traits::{ToClvm, FromClvm};

// Spend construction
pub use chia_sdk_driver::{SpendContext, Layer, Spend, SpendWithConditions, DriverError, Puzzle};

// Puzzle modules
pub use chia_sdk_types::Mod;
pub use chia_puzzles;

// Constants
pub use chia_sdk_types::{MAINNET_CONSTANTS, TESTNET11_CONSTANTS};
```

---

## 4. Crate Architecture

### 4.1 Module Layout

```
dig-clvm/
  Cargo.toml
  src/
    lib.rs                  -- Re-exports from Chia crates + dig-clvm's own API
    consensus/
      mod.rs                -- Module root
      validate.rs           -- validate_spend_bundle() orchestration
      block.rs              -- Block generator construction + validate_block()
      context.rs            -- ValidationContext
      config.rs             -- ValidationConfig, L2 cost constants
      result.rs             -- SpendResult
      cache.rs              -- BLS signature cache management
      error.rs              -- ValidationError
  tests/
    validation_tests.rs     -- Full bundle validation via Simulator
    block_tests.rs          -- Block generator construction + validation
    parity_tests.rs         -- Golden tests against known Chia L1 behavior
    condition_tests.rs      -- Condition round-trip via SpendContext + Simulator
    signature_tests.rs      -- BLS domain separation for all AGG_SIG_* variants
    cost_tests.rs           -- L2 cost limit enforcement
    cache_tests.rs          -- BLS cache hit/miss across mempool → block
    simulator_tests.rs      -- Simulator-based multi-spend scenarios
```

The crate is intentionally small. Most of the complexity lives in upstream Chia crates where it is already tested and maintained.

### 4.2 Dependencies

```toml
[dependencies]
# CLVM runtime
clvmr = "0.14"
clvm-traits = "0.26"
clvm-utils = "0.26"

# Chia protocol & consensus
chia-protocol = "0.26"
chia-consensus = "0.26"
chia-bls = "0.26"
chia-traits = "0.26"

# Chia SDK (individual crates, not the umbrella)
chia-sdk-types = "0.30"
chia-sdk-driver = { version = "0.30", features = ["action-layer"] }
chia-sdk-coinset = "0.30"
chia-puzzles = "0.20"

# DIG ecosystem
dig-constants = { path = "../dig-constants" }

# Minimal own dependencies
thiserror = "2"
hex = "0.4"

[dev-dependencies]
chia-sdk-test = "0.30"
hex-literal = "0.4"
rand = "0.8"
```

No `tokio`, `serde_json`, `rocksdb`, `reqwest`, or async/IO/storage dependencies. Pure computation. Individual SDK crates are used instead of the `chia-wallet-sdk` umbrella to avoid pulling in RPC clients and wallet utilities.

### 4.3 `dig-constants` (Sibling Crate)

`dig-constants` is a separate crate at `../dig-constants` that defines DIG's network parameters. It is imported by `dig-clvm` and any other DIG crate that needs network constants. It already exists and compiles.

```rust
/// DIG network constants. Wraps chia-consensus ConsensusConstants
/// with DIG-specific values (genesis challenge, AGG_SIG additional data, etc.)
pub struct NetworkConstants { /* wraps ConsensusConstants */ }

impl NetworkConstants {
    pub fn consensus(&self) -> &ConsensusConstants;
    pub fn genesis_challenge(&self) -> Bytes32;
    pub fn agg_sig_me_additional_data(&self) -> Bytes32;
    pub fn max_block_cost_clvm(&self) -> u64;
}

pub const DIG_MAINNET: NetworkConstants = /* ... */;
pub const DIG_TESTNET: NetworkConstants = /* ... */;
```

Key design choices:
- `hard_fork_height` and `hard_fork2_height` are set to `0` — DIG L2 starts with all consensus features enabled from block 0.
- Proof-of-space and VDF fields are set to neutral values since DIG does not use Chia's PoS consensus.
- Genesis challenge and all `agg_sig_*_additional_data` fields are placeholder zeros until mainnet launch.
- Dependencies: `chia-consensus`, `chia-protocol`, `chia-bls`, `hex-literal` only.

---

## 5. Public API

### 5.1 Validation (dig-clvm's own code)

```rust
/// Validate a spend bundle against L2 consensus rules.
///
/// Internally calls chia_consensus::run_spendbundle() for CLVM execution
/// and condition extraction, then applies L2-specific validation rules.
///
/// This mirrors the L1 flow where pre_validate_spendbundle() calls
/// validate_clvm_and_signature() (mempool_manager.py:445) and
/// validate_block_body() (block_body_validation.py:191) checks the results.
///
/// `bls_cache`: When provided, verified signature pairings are cached and
/// reused. Pass the same cache across mempool → block validation to avoid
/// re-verifying signatures. Pass None to verify from scratch.
pub fn validate_spend_bundle(
    bundle: &SpendBundle,
    context: &ValidationContext,
    config: &ValidationConfig,
    bls_cache: Option<&mut BlsCache>,
) -> Result<SpendResult, ValidationError>;

/// Build a block generator from a set of spend bundles.
///
/// Bundles are added in order until `max_cost` is reached. Bundles that
/// would exceed the limit are skipped. The caller should pre-sort bundles
/// by fee/cost ratio (highest first) to maximize fee revenue — matching
/// L1's create_block_generator() (mempool.py:505).
///
/// The output includes the compressed block program (using CLVM
/// back-references via solution_generator_backrefs() at mempool.py:529),
/// the aggregated signature, additions, removals, and total cost —
/// matching L1's NewBlockGenerator (generator_types.py:28).
pub fn build_block_generator(
    bundles: &[SpendBundle],
    context: &ValidationContext,
    max_cost: Cost,
) -> Result<BlockGeneratorResult, ValidationError>;

/// Output of build_block_generator(). Mirrors L1's NewBlockGenerator
/// (generator_types.py:28).
pub struct BlockGeneratorResult {
    /// The compressed block-level CLVM program
    pub generator: BlockGenerator,
    /// Block heights of referenced previous generators (for cross-block
    /// back-references). Mirrors L1's NewBlockGenerator.block_refs.
    pub block_refs: Vec<u32>,
    /// Aggregated BLS signature across all included bundles
    pub aggregated_signature: Signature,
    /// Coins created by all included spends
    pub additions: Vec<Coin>,
    /// Coins spent by all included spends
    pub removals: Vec<Coin>,
    /// Total CLVM cost of all included spends
    pub cost: Cost,
    /// Number of bundles included (may be less than input if cost limit hit)
    pub bundles_included: usize,
}

/// Validate a block generator and return the combined additions + removals.
///
/// Executes the block-level CLVM program (which produces all spends),
/// validates all conditions, and returns the aggregate SpendResult.
/// Mirrors L1's _run_block() (multiprocess_validation.py:62) followed
/// by validate_block_body() (block_body_validation.py:191).
pub fn validate_block(
    generator: &BlockGenerator,
    generator_refs: &[Vec<u8>],
    context: &ValidationContext,
    config: &ValidationConfig,
    bls_cache: Option<&mut BlsCache>,
) -> Result<SpendResult, ValidationError>;

/// Re-exported from chia-consensus.
pub use chia_consensus::run_block_generator::BlockGenerator;

/// L2 chain state for validation.
///
/// Analogous to the coin_records and blockchain state that Chia L1 passes
/// through check_time_locks() (check_time_locks.py:15) and
/// add_spend_bundle() (mempool_manager.py:480).
///
/// `coin_records` should contain only the coins being spent in this bundle,
/// not the full UTXO set. The caller loads these from their database and
/// passes them in. dig-clvm never touches storage directly.
pub struct ValidationContext {
    /// Current L2 block height
    pub height: u32,
    /// Current block timestamp (seconds since epoch)
    pub timestamp: u64,
    /// DIG network constants (from dig-constants crate).
    /// Contains genesis_challenge, AGG_SIG additional data, cost parameters,
    /// and fork heights. Wraps chia-consensus ConsensusConstants with
    /// DIG-specific values.
    pub constants: dig_constants::NetworkConstants,
    /// Coins being spent in this bundle (coin_id -> CoinRecord).
    /// Only the coins relevant to this validation — NOT the full UTXO set.
    pub coin_records: HashMap<Bytes32, CoinRecord>,
    /// Coins created by earlier bundles in the same block (ephemeral).
    /// Allows later bundles to spend coins that don't yet exist in the
    /// persistent UTXO set, matching Chia L1's ASSERT_EPHEMERAL behavior.
    pub ephemeral_coins: HashSet<Bytes32>,
}

/// L2-specific validation parameters.
///
/// On Chia L1, these are derived from ConsensusConstants and block height
/// via get_flags_for_height_and_constants() (multiprocess_validation.py:19).
/// L2 makes them explicitly configurable.
pub struct ValidationConfig {
    /// Maximum CLVM cost per individual spend
    pub max_cost_per_spend: Cost,
    /// Maximum total CLVM cost per block
    pub max_cost_per_block: Cost,
    /// Execution flags (from chia-consensus).
    ///
    /// Common flags:
    /// - `0` — block validation (default, most permissive)
    /// - `MEMPOOL_MODE` — stricter mempool validation (rejects unknown
    ///   opcodes, stricter cost accounting). Mirrors L1 at mempool.py:13.
    /// - `DONT_VALIDATE_SIGNATURE` — skip BLS signature verification
    ///   (for mempool pre-validation when sigs are checked separately).
    ///
    /// Flags can be combined: `MEMPOOL_MODE | DONT_VALIDATE_SIGNATURE`.
    pub flags: u32,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_cost_per_spend: L1_MAX_COST_PER_SPEND,
            max_cost_per_block: L2_MAX_COST_PER_BLOCK,
            flags: 0,
        }
    }
}

/// Result of validating a spend bundle.
///
/// This is the primary output of the crate: the set of coin state changes
/// that the caller commits to blockchain state. Mirrors what L1 extracts
/// via tx_removals_and_additions() (generator_tools.py:54).
pub struct SpendResult {
    /// Coins to add to the UTXO set (created by CREATE_COIN conditions)
    pub additions: Vec<Coin>,
    /// Coins to remove from the UTXO set (the spent coins)
    pub removals: Vec<Coin>,
    /// Total fee (sum of removals - sum of additions)
    pub fee: u64,
    /// The full parsed conditions from chia-consensus (for callers that
    /// need to inspect announcements, signatures, time locks, etc.)
    pub conditions: OwnedSpendBundleConditions,
}

/// Validation errors.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("CLVM execution failed: {0}")]
    Clvm(chia_consensus::validation_error::ValidationErr),

    #[error("Coin not found: {0}")]
    CoinNotFound(Bytes32),

    #[error("Coin already spent: {0}")]
    AlreadySpent(Bytes32),

    #[error("Double spend in bundle: {0}")]
    DoubleSpend(Bytes32),

    #[error("Puzzle hash mismatch: {0}")]
    PuzzleHashMismatch(Bytes32),

    #[error("Signature verification failed")]
    SignatureFailed,

    #[error("Conservation violation: input={input}, output={output}")]
    ConservationViolation { input: u64, output: u64 },

    #[error("Cost exceeded: limit={limit}, consumed={consumed}")]
    CostExceeded { limit: Cost, consumed: Cost },

    #[error("Driver error: {0}")]
    Driver(#[from] DriverError),
}

/// L2 cost constants.
/// L1 reference: MAX_BLOCK_COST_CLVM=11_000_000_000 (default_constants.py:68)
pub const L1_MAX_COST_PER_SPEND: Cost = 11_000_000_000;
pub const L2_MAX_COST_PER_BLOCK: Cost = 550_000_000_000; // 50 * L1 per-spend
```

### 5.2 Re-exports (everything else)

```rust
// ── CLVM Runtime ──
pub use clvmr::{self, Allocator, NodePtr, Cost};
pub use clvm_traits::{self, ToClvm, FromClvm};
pub use clvm_utils::{self, tree_hash, curry_tree_hash, CurriedProgram, TreeHash, ToTreeHash};

// ── Chia Protocol Types ──
pub use chia_protocol::{self, Bytes, Bytes32, Coin, CoinSpend, CoinState, Program, SpendBundle};

// ── Consensus Engine ──
pub use chia_consensus::{self, consensus_constants::ConsensusConstants, opcodes};
pub use chia_consensus::spendbundle_conditions::run_spendbundle;
pub use chia_consensus::spendbundle_validation::validate_clvm_and_signature;
pub use chia_consensus::conditions::{
    SpendBundleConditions, OwnedSpendBundleConditions,
    SpendConditions, OwnedSpendConditions,
};

// ── BLS Signatures ──
pub use chia_bls::{self, PublicKey, SecretKey, Signature, aggregate_verify, BlsCache};

// ── SDK Types & Conditions ──
pub use chia_sdk_types::{self, Condition, Conditions, Mod};

// ── DIG Network Constants ──
pub use dig_constants::{self, DIG_MAINNET, DIG_TESTNET, NetworkConstants};

// ── Spend Construction ──
pub use chia_sdk_driver::{self, SpendContext, Layer, Spend, SpendWithConditions, Puzzle, DriverError};

// ── Coin State ──
pub use chia_sdk_coinset::{self, CoinRecord};

// ── Puzzles ──
pub use chia_puzzles;

// ── Block Generator ──
pub use chia_consensus::run_block_generator::BlockGenerator;
```

---

## 6. Validation Flow

### 6.1 Internal Implementation

`validate_spend_bundle()` orchestrates the upstream crates. This mirrors the L1 flow traced through the codebase:

```
L1 transaction flow:
  full_node.py:2755     add_transaction()
  mempool_manager.py:428  pre_validate_spendbundle()
  mempool_manager.py:445  validate_clvm_and_signature()  ← Rust
  mempool_manager.py:480  add_spend_bundle()

L1 block validation flow:
  blockchain.py:286            add_block()
  multiprocess_validation.py:62  _run_block()
  multiprocess_validation.py:69  run_block_generator2()  ← Rust
  block_body_validation.py:191   validate_block_body()
  check_time_locks.py:15         check_time_locks()
  generator_tools.py:54          tx_removals_and_additions()
```

dig-clvm provides two entry points, mirroring L1's mempool and block paths:

**Path A: Per-bundle validation (mempool admission)**

```
validate_spend_bundle(bundle, context, config, bls_cache)
  |
  v
[1. Structural checks]                     ← dig-clvm's own code
  |- Check for duplicate spends in bundle
  |- Verify all spent coins exist in context.coin_records
  |- Verify no coin is already spent
  |
  v
[2. CLVM execution + condition extraction]  ← chia-consensus::run_spendbundle()
  |- Creates allocator via make_allocator(config.flags)
  |- If config.flags includes MEMPOOL_MODE, uses MempoolVisitor
  |  for stricter validation (reject unknown opcodes, etc.)
  |- Runs each puzzle+solution through clvmr
  |- Parses all conditions from CLVM output
  |- Tracks cost per spend and total
  |- Returns SpendBundleConditions
  |
  v
[3. Cost enforcement]                       ← dig-clvm checks L2 limits
  |- conditions.cost <= config.max_cost_per_block
  |
  v
[4. Condition validation]                   ← chia-consensus handles internally
  |- Announcement matching (create vs assert)
  |- Concurrent spend/puzzle assertions
  |- Identity assertions (MY_COIN_ID, etc.)
  |- Time/height locks (relative + absolute)
  |- Ephemeral coin assertions
  |
  v
[5. BLS signature verification]             ← chia-consensus + chia-bls
  |- Unless config.flags includes DONT_VALIDATE_SIGNATURE:
  |    Collect (pubkey, message) pairs with domain separation
  |    Check bls_cache for already-verified pairings
  |    aggregate_verify remaining against bundle.aggregated_signature
  |    Store verified pairings in bls_cache
  |
  v
[6. Conservation check]                     ← dig-clvm's own code
  |- sum(removals) >= sum(additions) + fee
  |
  v
SpendResult { additions, removals, fee, conditions }
```

**Path B: Block-level validation (block building + block validation)**

```
build_block_generator(bundles, context, max_cost)
  |
  v
[1. Assemble spends]                        ← dig-clvm
  |- Iterate bundles in order (caller pre-sorts by fee/cost ratio)
  |- For each bundle: run CLVM to compute cost, skip if exceeds remaining
  |- Collect (coin, puzzle_reveal, solution) from included bundles
  |- Aggregate BLS signatures across included bundles
  |- Call solution_generator_backrefs() to create compressed
  |  block program with CLVM back-references
  |- Return BlockGeneratorResult { generator, sig, additions, removals, cost }
  |
  v
validate_block(generator, refs, context, config, bls_cache)
  |
  v
[2. Execute generator]                      ← chia-consensus::run_block_generator2()
  |- Runs the block-level CLVM program
  |- Produces all spends + conditions in one pass
  |
  v
[3-6. Same as Path A]                       ← cost, conditions, BLS, conservation
  |
  v
SpendResult { additions, removals, fee, conditions }
```

### 6.2 Consumer: Single Spend (using SDK types)

```rust
use dig_clvm::{SpendContext, Condition, Conditions, Coin, Bytes32};

let mut ctx = SpendContext::new();

// Run a puzzle+solution (uses chia-sdk-types::run_puzzle internally)
let puzzle_ptr = ctx.puzzle(puzzle_hash, &puzzle_bytes)?;
let solution_ptr = ctx.alloc(&solution_value)?;
let output = ctx.run(puzzle_ptr, solution_ptr)?;

// Extract conditions using SDK types
let conditions: Vec<Condition<NodePtr>> = ctx.extract(output)?;
```

### 6.3 Consumer: Full Bundle Validation

```rust
use dig_clvm::{
    validate_spend_bundle, ValidationContext, ValidationConfig,
    CoinRecord, BlsCache,
};
use dig_constants::DIG_MAINNET;

let mut ctx = ValidationContext {
    height: current_height,
    timestamp: current_timestamp,
    constants: DIG_MAINNET, // from dig-constants crate
    coin_records: coins_being_spent, // only the coins in this bundle
    ephemeral_coins: HashSet::new(),
};

// BLS cache persists across calls — mempool-verified sigs are reused in block validation
let mut bls_cache = BlsCache::default();

let config = ValidationConfig::default(); // L2 cost limits
let result = validate_spend_bundle(&bundle, &ctx, &config, Some(&mut bls_cache))?;

// Caller commits the additions and removals to blockchain state
for coin in &result.additions {
    state.add_coin(coin);       // new coins enter the UTXO set
}
for coin in &result.removals {
    state.remove_coin(&coin);   // spent coins leave the UTXO set
}
// result.fee is available for block reward accounting
```

### 6.4 Consumer: Block Generator Construction + Validation

```rust
use dig_clvm::{
    build_block_generator, validate_block, ValidationContext,
    ValidationConfig, BlsCache, L2_MAX_COST_PER_BLOCK,
};

// Sort mempool bundles by fee/cost ratio (highest first), then build.
// build_block_generator adds bundles until the cost limit is reached.
mempool_bundles.sort_by(|a, b| fee_rate(b).cmp(&fee_rate(a)));
let block = build_block_generator(
    &mempool_bundles,
    &ctx,
    L2_MAX_COST_PER_BLOCK,
)?;
// block.bundles_included tells you how many fit

// Validate the full block (reuses BLS cache from mempool validation)
let result = validate_block(
    &block.generator,
    &[], // generator_refs from previous blocks (if any)
    &ctx,
    &ValidationConfig::default(),
    Some(&mut bls_cache),
)?;

// result.additions and result.removals cover all spends in the block
// block.aggregated_signature goes into the block header
```

### 6.5 Consumer: Spend Construction (using SDK driver)

```rust
use dig_clvm::{
    SpendContext, StandardLayer, Conditions, Condition, Spend,
    SpendBundle, Coin, Bytes32, SecretKey,
};

let mut ctx = SpendContext::new();

// Use the standard puzzle layer
let layer = StandardLayer::new(synthetic_key);
let conditions = Conditions::new()
    .with(Condition::CreateCoin(CreateCoin {
        puzzle_hash: recipient_ph,
        amount: 1000,
        memos: vec![],
    }));

let spend = layer.spend_with_conditions(&mut ctx, conditions)?;
ctx.spend(coin, spend)?;

let coin_spends = ctx.take();
let bundle = SpendBundle::new(coin_spends, aggregated_sig);
```

---

## 7. Chia L1 Parity

Parity is achieved **by construction**: we use the same Rust crates that Chia full nodes use.

| Concern | How parity is achieved | L1 reference |
|---|---|---|
| CLVM execution | Same `clvmr` runtime, same `ChiaDialect` | [`multiprocess_validation.py:62`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L62) — `_run_block()` |
| Condition parsing | Same `chia-consensus` condition processing with `SpendVisitor` | [`condition_opcodes.py:7-73`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L7) — all opcodes |
| Tree hash | Same `clvm_utils::tree_hash()` | [`test_curry_and_treehash.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_curry_and_treehash.py) — parity tests |
| BLS signatures | Same `chia-bls` with identical domain separation | [`condition_tools.py:74-97`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_tools.py#L74) — `make_aggsig_final_message()` |
| Cost model | Same opcode costs from `chia-consensus::opcodes` | [`condition_costs.py:8-13`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_costs.py#L8) — AGG_SIG, CREATE_COIN, GENERIC |
| Cost limits | Same per-spend limit, configurable per-block | [`default_constants.py:68`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/default_constants.py#L68) — `MAX_BLOCK_COST_CLVM=11B` |
| Puzzle bytecodes | Same `chia-puzzles` constants | [`test_puzzle_drivers.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_puzzle_drivers.py) |
| Condition types | Same `chia-sdk-types::Condition<T>` enum | Covers all opcodes including [`SEND_MESSAGE/RECEIVE_MESSAGE`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L37) |
| Spend construction | Same `chia-sdk-driver::SpendContext` and `Layer` trait | Tested via [`test_singletons.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_singletons.py) |
| Mempool flags | Same `MEMPOOL_MODE`, `DONT_VALIDATE_SIGNATURE` | [`mempool.py:13-14`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L13) |
| Block generators | Same `solution_generator_backrefs()` + `run_block_generator2()` | [`mempool.py:529`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L529), [`multiprocess_validation.py:69`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L69) |
| BLS cache | Same `BlsCache` for mempool→block sig reuse | `chia-bls::BlsCache` |

The only L2-specific divergence is block-level cost limits (`L2_MAX_COST_PER_BLOCK`), which is explicitly configurable via `ValidationConfig`.

---

## 8. Testing Strategy

### 8.1 Simulator-Based Tests (Primary)

Use `chia-sdk-test::Simulator` for end-to-end validation. The Simulator internally uses `chia_consensus::run_spendbundle()` — the same code path as Chia full nodes at [`_run_block()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L62) and [`pre_validate_spendbundle()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool_manager.py#L428).

```rust
use chia_sdk_test::{Simulator, BlsPair};
use dig_clvm::{validate_spend_bundle, ValidationContext, ValidationConfig};

#[test]
fn test_standard_spend() {
    let mut sim = Simulator::new();
    let alice = sim.bls(1_000_000);

    // Build and validate a spend using the Simulator
    let spend_bundle = /* build bundle using SpendContext */;
    let result = sim.new_transaction(spend_bundle.clone());
    assert!(result.is_ok());

    // Also validate through dig-clvm's API for L2 rules
    let ctx = ValidationContext::from_simulator(&sim);
    let result = validate_spend_bundle(&spend_bundle, &ctx, &ValidationConfig::default());
    assert!(result.is_ok());
}
```

### 8.2 Test Scenarios

**Validation tests** (`tests/validation_tests.rs`) — mirrors scenarios from [`test_conditions.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/core/full_node/test_conditions.py):
- Happy path: standard spend with CREATE_COIN + AGG_SIG_ME
- Double spend within a bundle
- Puzzle hash mismatch
- Conservation violation (outputs > inputs)
- Announcement graph (create + assert across coins)
- Missing announcement assertion
- Height/time lock enforcement (absolute and relative) — as validated by [`check_time_locks()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/check_time_locks.py#L15)
- Before-height/before-time lock enforcement
- Ephemeral coins (created and spent in same block)
- Concurrent spend assertions
- Multi-spend bundles with fee

**Parity tests** (`tests/parity_tests.rs`):
- Known standard transaction puzzle hashes from `chia-puzzles`
- Round-trip: build spend via `SpendContext` -> validate via both Simulator and `validate_spend_bundle()`
- All 8 `AGG_SIG_*` variants produce correct domain-separated messages — verified against the lookup table at [`condition_tools.py:87-97`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/condition_tools.py#L87)
- `tree_hash` of known puzzles matches published Chia puzzle hashes — see [`test_curry_and_treehash.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_curry_and_treehash.py)

**Cost tests** (`tests/cost_tests.rs`):
- L2 block cost limit enforcement
- Per-spend cost limit (same as L1's [`MAX_BLOCK_COST_CLVM=11B`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/default_constants.py#L68))
- Cost boundary: program at exactly max_cost succeeds
- Cost boundary: program at max_cost + 1 fails

**Block generator tests** (`tests/block_tests.rs`) — mirrors [`create_block_generator()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L505) and [`simple_solution_generator_backrefs()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/bundle_tools.py#L15):
- Build generator from single bundle, validate, compare additions/removals to per-bundle validation
- Build generator from multiple bundles, verify all spends included
- Back-reference deduplication: bundles sharing the same puzzle produce a smaller generator than naive serialization
- Generator with `generator_refs` from previous blocks
- Round-trip: `build_block_generator()` output validates identically via `validate_block()`

**BLS cache tests** (`tests/cache_tests.rs`):
- Validate bundle with empty cache, verify cache is populated
- Re-validate same bundle, verify cache hit (no re-verification)
- Validate bundle in mempool (with cache), then validate same bundle in block (cache reused)
- Cache miss: modified signature invalidates cache entry

**Condition tests** (`tests/condition_tests.rs`) — covers all opcodes from [`condition_opcodes.py:7-73`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L7):
- Build conditions via `Conditions<T>` builder
- Execute via SpendContext
- Verify parsed output matches for each opcode
- Unknown opcode passthrough
- `SEND_MESSAGE(66)` / `RECEIVE_MESSAGE(67)` — tested on L1 at [`test_message_conditions.py`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/_tests/clvm/test_message_conditions.py)
- `MEMPOOL_MODE` flag rejects unknown opcodes that block validation accepts

### 8.3 Parity by Construction

Because tests use `chia-sdk-test::Simulator` which internally runs `chia_consensus::run_spendbundle()`, any test that passes in the Simulator is **by definition** Chia L1 compatible. We don't need separate "parity" testing infrastructure — it's built into the test framework.

---

## 9. Design Decisions

### 9.1 Why Maximize Chia Crate Reuse

The previous approach (in `l2_driver_state_channel`) reimplemented condition parsing, tree hashing, currying helpers, and validation logic. This created:
- Maintenance burden: upstream changes required manual porting
- Parity risk: subtle differences in condition parsing or cost accounting
- Test duplication: testing behavior already tested upstream

By building on the Chia crates directly, we get:
- Parity by construction (same code = same behavior)
- Upstream bug fixes for free
- Access to the full SDK (SpendContext, Layer trait, puzzle drivers, Simulator)
- Smaller codebase to maintain (~200 lines of L2-specific code vs ~3000 lines of reimplemented CLVM logic)

### 9.2 Why `chia-consensus` for Validation Instead of Hand-Rolled

`chia-consensus::run_spendbundle()` handles the entire CLVM execution + condition extraction + announcement matching + time lock validation pipeline. This is the same engine Chia L1 uses at [`validate_clvm_and_signature()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool_manager.py#L445) (`mempool_manager.py:445`). Using it directly:
- Eliminates the custom condition parser (640+ lines in the old `clvm.rs`)
- Eliminates the custom validation logic (500+ lines in the old `validation.rs`)
- Automatically supports [`SEND_MESSAGE(66)` / `RECEIVE_MESSAGE(67)`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/types/condition_opcodes.py#L37) and future opcodes
- Uses the same `SpendVisitor` pattern Chia uses for mempool vs block validation

### 9.3 Why `chia-sdk-types::Condition<T>` Instead of Custom Enum

The SDK's `Condition<T>` is generic over the node representation, supports all opcodes (including NFT/CAT-specific ones), and is directly serializable to/from CLVM via `ToClvm`/`FromClvm`. The previous custom enum:
- Missed `SEND_MESSAGE` and `RECEIVE_MESSAGE`
- Required manual synchronization with upstream opcode changes
- Couldn't be used for spend construction (only parsing)

The SDK type works for both construction (via `Conditions<T>` builder) and parsing (via `FromClvm`), eliminating the need for separate types.

### 9.4 Why `SpendContext` Instead of Raw Allocator Wrappers

The spec previously proposed custom `allocator.rs`, `node_codec.rs`, and `clvm_codec.rs` wrappers. `SpendContext` already provides all of this plus puzzle caching, currying, and spend collection. It's the standard way to interact with CLVM in the Chia Rust ecosystem.

### 9.5 Why `ValidationConfig` with Configurable Cost Limits

The only L2-specific parameter that differs from Chia L1 is the block-level cost limit. L1 uses [`MAX_BLOCK_COST_CLVM=11_000_000_000`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/default_constants.py#L68). Rather than forking the consensus code, we wrap it with configurable limits. The per-spend limit matches L1 exactly.

### 9.6 Soft Fork Flags

`chia-consensus::get_flags_for_height_and_constants()` (imported at [`multiprocess_validation.py:19`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L19)) computes the correct execution flags for any block height, handling hard fork transitions automatically. `dig-clvm` exposes this via `ValidationConfig::flags` so L2 can track upstream fork activations.

---

## 10. Resolved Decisions

1. **Block-level generator: Yes.** `dig-clvm` supports block-level generator construction and execution, matching Chia L1's [`run_block_generator()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/consensus/multiprocess_validation.py#L71) and [`solution_generator_backrefs()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L529) (`mempool.py:529`). This enables compact block encoding where repeated puzzle bytecodes are deduplicated via CLVM back-references. The crate provides both per-bundle validation (for mempool admission) and block-generator-level validation (for block building and block validation). See [`simple_solution_generator_backrefs()`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/bundle_tools.py#L15) (`bundle_tools.py:15`) for L1's approach.

2. **BLS cache: dig-clvm owns it.** `validate_spend_bundle()` and `validate_block()` accept an optional `&mut BlsCache` parameter. When provided, verified signature pairings are cached and reused across calls — avoiding re-verification when a mempool-validated bundle later appears in a block. This matches Chia L1's pattern where `BLSCache` is passed through the validation pipeline. When `None`, signatures are verified from scratch every time.

3. **SpendVisitor customization: Yes.** `dig-clvm` exposes the `SpendVisitor` hook from `chia-consensus`, enabling different validation strictness for mempool admission vs block validation. The `ValidationConfig::flags` field accepts `MEMPOOL_MODE` ([`mempool.py:13`](https://github.com/Chia-Network/chia-blockchain/blob/main/chia/full_node/mempool.py#L13)) for stricter mempool rules (reject unknown opcodes, stricter cost accounting). This allows L2-specific mempool policies (e.g., minimum fee thresholds, puzzle blacklists) to be enforced at the CLVM validation layer rather than requiring external filtering.

4. **Individual crates.** `dig-clvm` depends on `chia-sdk-types`, `chia-sdk-driver`, `chia-sdk-coinset`, etc. individually rather than the `chia-wallet-sdk` umbrella. This avoids pulling in RPC client code, wallet utilities, and other modules that a consensus crate doesn't need. Each sub-crate is pinned to the same version for consistency.
