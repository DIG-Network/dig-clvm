# dig-clvm

DIG L2 CLVM consensus engine. Validates spend bundles and computes coin additions/removals for DIG validators.

Built as a thin orchestration layer on top of the Chia crate ecosystem. Never reimplements what `chia-consensus`, `chia-sdk-types`, `chia-sdk-driver`, `clvmr`, `clvm-utils`, or `chia-bls` already provide.

## Core Contract

```
Input:  SpendBundle (coin spends + aggregated BLS signature)
Output: SpendResult { additions: Vec<Coin>, removals: Vec<Coin>, fee: u64 }
        or ValidationError
```

Blockchain state management (UTXO persistence, Merkle roots, block storage) is out of scope. This crate validates and computes; the caller commits to state.

## Public API

### Entry Points

#### `validate_spend_bundle` — Validate a single spend bundle

```rust
pub fn validate_spend_bundle(
    bundle: &SpendBundle,
    context: &ValidationContext,
    config: &ValidationConfig,
    bls_cache: Option<&mut BlsCache>,
) -> Result<SpendResult, ValidationError>;
```

Validates a spend bundle for mempool admission or block inclusion. Internally delegates CLVM execution, condition parsing, and BLS signature verification to `chia-consensus`.

**Validation pipeline:**
1. Structural checks (duplicate spends, coin existence, already-spent)
2. CLVM execution + condition extraction via `chia-consensus::run_spendbundle()`
3. Cost enforcement against `config.max_cost_per_block`
4. BLS signature verification (skipped if `DONT_VALIDATE_SIGNATURE` flag set)
5. Conservation check (`sum(removals) >= sum(additions) + fee`)

#### `build_block_generator` — Assemble bundles into a block

```rust
pub fn build_block_generator(
    bundles: &[SpendBundle],
    context: &ValidationContext,
    max_cost: Cost,
) -> Result<BlockGeneratorResult, ValidationError>;
```

Iterates bundles in order, adds each until `max_cost` is reached. Caller pre-sorts by fee/cost ratio. Produces a compressed CLVM block program via `solution_generator_backrefs()`.

#### `validate_block` — Validate a block generator

```rust
pub fn validate_block(
    generator: &[u8],
    generator_refs: &[Vec<u8>],
    context: &ValidationContext,
    config: &ValidationConfig,
    bls_cache: Option<&mut BlsCache>,
    aggregated_signature: &Signature,
) -> Result<SpendResult, ValidationError>;
```

Executes a block-level CLVM program via `chia-consensus::run_block_generator2()`, validates conditions, and returns additions/removals.

### Input Types

#### `ValidationContext` — Chain state for validation

```rust
pub struct ValidationContext {
    pub height: u32,                              // Current L2 block height
    pub timestamp: u64,                           // Seconds since epoch
    pub constants: NetworkConstants,               // From dig-constants crate
    pub coin_records: HashMap<Bytes32, CoinRecord>, // Only coins being spent (NOT full UTXO set)
    pub ephemeral_coins: HashSet<Bytes32>,         // Coins from earlier bundles in same block
}
```

`coin_records` contains **only the coins relevant to this validation**. The caller loads them from their database. This crate never touches storage.

`ephemeral_coins` tracks coins created by earlier bundles in the same block, enabling same-block create+spend (Chia L1's `ASSERT_EPHEMERAL` behavior).

#### `ValidationConfig` — Validation parameters

```rust
pub struct ValidationConfig {
    pub max_cost_per_spend: Cost,  // Default: 11_000_000_000 (matches Chia L1)
    pub max_cost_per_block: Cost,  // Default: 550_000_000_000 (50x L1)
    pub flags: u32,                // 0 = block validation, MEMPOOL_MODE = stricter,
                                   // DONT_VALIDATE_SIGNATURE = skip BLS
}
```

Flags can be combined: `MEMPOOL_MODE | DONT_VALIDATE_SIGNATURE`.

### Output Types

#### `SpendResult` — Validation output

```rust
pub struct SpendResult {
    pub additions: Vec<Coin>,                        // Coins to add to UTXO set
    pub removals: Vec<Coin>,                         // Coins to remove from UTXO set
    pub fee: u64,                                    // sum(removals) - sum(additions)
    pub conditions: OwnedSpendBundleConditions,      // Full conditions from chia-consensus
}
```

`additions` and `removals` are what the caller commits to blockchain state. `conditions` is the raw output from `chia-consensus` for callers that need to inspect announcements, signatures, or time locks.

#### `BlockGeneratorResult` — Block construction output

```rust
pub struct BlockGeneratorResult {
    pub generator: Vec<u8>,              // Compressed CLVM block program
    pub block_refs: Vec<u32>,            // Referenced previous block heights
    pub aggregated_signature: Signature, // Combined BLS signature
    pub additions: Vec<Coin>,            // All coins created
    pub removals: Vec<Coin>,             // All coins spent
    pub cost: Cost,                      // Total CLVM cost
    pub bundles_included: usize,         // How many bundles fit in the block
}
```

#### `ValidationError` — Error types

```rust
pub enum ValidationError {
    Clvm(String),                                    // CLVM execution failed
    CoinNotFound(Bytes32),                           // Coin not in coin_records or ephemeral_coins
    AlreadySpent(Bytes32),                           // Coin marked spent in coin_records
    DoubleSpend(Bytes32),                            // Same coin spent twice in one bundle
    PuzzleHashMismatch(Bytes32),                     // tree_hash(puzzle_reveal) != coin.puzzle_hash
    SignatureFailed,                                 // BLS aggregate signature invalid
    ConservationViolation { input: u64, output: u64 }, // Outputs exceed inputs
    CostExceeded { limit: Cost, consumed: Cost },    // CLVM cost over budget
    Driver(DriverError),                             // Spend driver error
}
```

### Constants

```rust
pub const L1_MAX_COST_PER_SPEND: Cost = 11_000_000_000;   // Matches Chia L1
pub const L2_MAX_COST_PER_BLOCK: Cost = 550_000_000_000;  // 50x L1 per-spend
```

### Re-exports

All Chia ecosystem types are re-exported so callers need only depend on `dig-clvm`:

| Source Crate | Re-exported Types |
|---|---|
| `clvmr` | `Allocator`, `NodePtr`, `Cost` |
| `clvm-traits` | `ToClvm`, `FromClvm` |
| `clvm-utils` | `tree_hash`, `CurriedProgram`, `TreeHash`, `ToTreeHash` |
| `chia-protocol` | `Coin`, `CoinSpend`, `SpendBundle`, `Program`, `Bytes32`, `CoinState` |
| `chia-bls` | `PublicKey`, `SecretKey`, `Signature`, `BlsCache`, `aggregate_verify` |
| `chia-consensus` | `ConsensusConstants`, `opcodes` |
| `chia-sdk-types` | `Condition`, `Conditions`, `Mod` |
| `chia-sdk-driver` | `SpendContext`, `Layer`, `Spend`, `SpendWithConditions`, `Puzzle`, `DriverError` |
| `chia-sdk-coinset` | `CoinRecord` |
| `chia-puzzles` | All puzzle bytecodes |
| `dig-constants` | `NetworkConstants`, `DIG_MAINNET`, `DIG_TESTNET` |

## Usage

### Validate a Spend Bundle

```rust
use std::collections::{HashMap, HashSet};
use dig_clvm::{
    validate_spend_bundle, ValidationContext, ValidationConfig,
    BlsCache, DIG_MAINNET, CoinRecord,
};

let ctx = ValidationContext {
    height: current_height,
    timestamp: current_timestamp,
    constants: DIG_MAINNET.clone(),
    coin_records: coins_being_spent, // HashMap<Bytes32, CoinRecord>
    ephemeral_coins: HashSet::new(),
};

let mut cache = BlsCache::default();
let config = ValidationConfig::default();
let result = validate_spend_bundle(&bundle, &ctx, &config, Some(&mut cache))?;

// Commit to state
for coin in &result.additions { state.add_coin(coin); }
for coin in &result.removals { state.remove_coin(coin); }
```

### Build and Validate a Block

```rust
use dig_clvm::{
    build_block_generator, validate_block, ValidationConfig,
    BlsCache, L2_MAX_COST_PER_BLOCK,
};

// Sort bundles by fee/cost ratio, then build
let block = build_block_generator(&bundles, &ctx, L2_MAX_COST_PER_BLOCK)?;

// Validate the block
let result = validate_block(
    &block.generator,
    &[],
    &ctx,
    &ValidationConfig::default(),
    Some(&mut cache),
    &block.aggregated_signature,
)?;
```

### Skip Signature Verification (Mempool Pre-validation)

```rust
use chia_consensus::flags::DONT_VALIDATE_SIGNATURE;

let config = ValidationConfig {
    flags: DONT_VALIDATE_SIGNATURE,
    ..ValidationConfig::default()
};
let result = validate_spend_bundle(&bundle, &ctx, &config, None)?;
```

## Architecture

```
src/
  lib.rs              -- Re-exports only
  consensus/
    mod.rs            -- Module root
    validate.rs       -- validate_spend_bundle()
    block.rs          -- build_block_generator(), validate_block()
    context.rs        -- ValidationContext
    config.rs         -- ValidationConfig, cost constants
    result.rs         -- SpendResult, BlockGeneratorResult
    cache.rs          -- BLS cache management
    error.rs          -- ValidationError
```

No async, no IO, no storage. Pure computation. All CLVM execution, condition parsing, tree hashing, BLS verification, and cost accounting delegated to the Chia crate ecosystem.

## Dependencies

```toml
[dependencies]
clvmr = "0.14"
clvm-traits = "0.26"
clvm-utils = "0.26"
chia-protocol = "0.26"
chia-consensus = "0.26"
chia-bls = "0.26"
chia-traits = "0.26"
chia-sdk-types = "0.30"
chia-sdk-driver = { version = "0.30", features = ["action-layer"] }
chia-sdk-coinset = "0.30"
chia-puzzles = "0.20"
dig-constants = { path = "../dig-constants" }
thiserror = "2"
hex = "0.4"

[dev-dependencies]
chia-sdk-test = "0.30"
```

## Testing

55 dedicated test files, 154 tests. One file per requirement (`tests/vv_req_{prefix}_{nnn}.rs`).

```bash
cargo test              # Run all 154 tests
cargo test vv_req_val   # Run all validation tests
cargo test vv_req_blk   # Run all block generator tests
cargo test vv_req_con   # Run all constants tests
```

## Documentation

| Document | Path |
|---|---|
| Specification | `docs/resources/SPEC.md` |
| Requirements (55) | `docs/requirements/` |
| Workflow | `docs/prompt/start.md` |
