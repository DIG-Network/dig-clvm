# dig-clvm — Normative Specification

This document is the authoritative contract for the `dig-clvm` crate: the DIG L2 CLVM
consensus engine. It specifies the public API surface, the exact validation semantics,
invariants, error behavior, configuration defaults, and the conformance obligations that
bind this crate to the rest of the DIG ecosystem and to Chia L1 consensus.

The key words **MUST**, **MUST NOT**, **SHALL**, **SHOULD**, **SHOULD NOT**, and **MAY**
are to be interpreted as described in RFC 2119.

Non-normative design rationale and the upstream Chia crate inventory live in
`docs/resources/SPEC.md`; where the two disagree, this document is authoritative.

---

## 1. Scope

### 1.1 What this crate is

`dig-clvm` validates Chia-format spend bundles and block generators under DIG L2
consensus rules and computes the resulting coin state changes (additions, removals, fee).
It is a **thin orchestration layer** over the Chia consensus crate ecosystem
(`chia-consensus`, `clvmr`, `chia-bls`, `chia-protocol`, `chia-sdk-*`).

The core contract:

```
Input:  SpendBundle (coin spends + aggregated BLS signature)
        — or a serialized block generator + refs —
Output: SpendResult { additions: Vec<Coin>, removals: Vec<Coin>, fee: u64, conditions }
        or ValidationError
```

### 1.2 What this crate is not

The crate MUST NOT:

- persist state, perform I/O, or open network connections — all chain state is passed
  in via `ValidationContext` and results are returned to the caller, who commits them;
- expose `async` APIs — every public function is a pure, synchronous computation;
- reimplement CLVM execution, condition parsing, cost accounting, tree hashing, or BLS
  verification — these MUST be delegated to the upstream Chia crates
  (`run_spendbundle`, `run_block_generator2`, `validate_clvm_and_signature`,
  `BlsCache::aggregate_verify`);
- compile Chialisp source (callers use external tooling for that);
- manage the UTXO set, Merkle state roots, or block storage (see `dig-coinstore` /
  `dig-blockstore` in the DIG ecosystem).

### 1.3 Position in the DIG ecosystem

`dig-clvm` is the consensus-execution building block of the DIG L2 chain stack. Its
consumers (mempool admission, block building, block validation in DIG validators) rely
on the guarantee in §11.1 that its CLVM semantics are byte-for-byte identical to Chia
L1 full-node behavior, because it executes the same upstream code. The L2 chain layer
is described for end users in the DIG Protocol documentation at https://docs.dig.net
(Protocol section).

---

## 2. Crate identity and dependencies

- Crate name: `dig-clvm` (import as `dig_clvm`). License: MIT.
  Published to crates.io on `v*` tags (§13.2).
- `rust-version = 1.75.0` — the crate MUST build on Rust 1.75 stable.
- Pinned upstream stack (a consumer of `dig-clvm` inherits these consensus semantics):
  `clvmr 0.14`, `clvm-traits 0.26`, `clvm-utils 0.26`, `chia-protocol 0.26`,
  `chia-consensus 0.26`, `chia-bls 0.26`, `chia-traits 0.26`, `chia-sdk-types 0.30`,
  `chia-sdk-driver 0.30` (feature `action-layer`), `chia-sdk-coinset 0.30`,
  `chia-puzzles 0.20`, `dig-constants 0.9`.
- The only non-Chia runtime dependencies are `thiserror` and `hex`. The crate MUST NOT
  add async runtimes, storage engines, serializers, or network clients.

---

## 3. Public API surface

The public surface consists of (a) three validation entry points plus their input/output
types, all defined in this crate, and (b) a re-export layer that makes `dig-clvm` a
single-dependency facade over the Chia stack.

### 3.1 Entry points (this crate's own code)

```rust
pub fn validate_spend_bundle(
    bundle: &SpendBundle,
    context: &ValidationContext,
    config: &ValidationConfig,
    bls_cache: Option<&mut BlsCache>,
) -> Result<SpendResult, ValidationError>;          // §5

pub fn build_block_generator(
    bundles: &[SpendBundle],
    context: &ValidationContext,
    max_cost: Cost,
) -> Result<BlockGeneratorResult, ValidationError>; // §6

pub fn validate_block(
    generator: &[u8],
    generator_refs: &[Vec<u8>],
    context: &ValidationContext,
    config: &ValidationConfig,
    bls_cache: Option<&mut BlsCache>,
    aggregated_signature: &Signature,
) -> Result<SpendResult, ValidationError>;          // §7
```

All three, together with `ValidationContext`, `ValidationConfig`, `ValidationError`,
`SpendResult`, `BlockGeneratorResult`, `L1_MAX_COST_PER_SPEND`, and
`L2_MAX_COST_PER_BLOCK`, are exported both at the crate root and under
`dig_clvm::consensus`. These signatures are the crate's API contract; changing them is
a semver-breaking change.

### 3.2 Re-export layer

`dig-clvm` re-exports the following so that consumers need only depend on `dig-clvm`
(each source crate module is also re-exported wholesale, e.g. `dig_clvm::chia_consensus`):

| Source crate | Re-exported at the `dig_clvm` root |
|---|---|
| `clvmr` | `Allocator`, `NodePtr`, `Cost` (from `clvmr::cost`) |
| `clvm-traits` | `ToClvm`, `FromClvm` |
| `clvm-utils` | `tree_hash`, `CurriedProgram`, `ToTreeHash`, `TreeHash` |
| `chia-protocol` | `Bytes`, `Bytes32`, `Coin`, `CoinSpend`, `CoinState`, `Program`, `SpendBundle` |
| `chia-consensus` | `ConsensusConstants`, the `opcodes` module, **and every item of `chia_consensus::opcodes` flattened to the root** (`AGG_SIG_ME`, `CREATE_COIN`, `ASSERT_HEIGHT_ABSOLUTE`, `AGG_SIG_COST`, `CREATE_COIN_COST`, `ConditionOpcode`, …) |
| `chia-bls` | `aggregate_verify`, `BlsCache`, `PublicKey`, `SecretKey`, `Signature` |
| `chia-sdk-types` | `Condition`, `Conditions`, `Mod` |
| `chia-sdk-driver` | `DriverError`, `Layer`, `Puzzle`, `Spend`, `SpendContext`, `SpendWithConditions` |
| `chia-sdk-coinset` | `CoinRecord` |
| `chia-puzzles` | the whole crate (puzzle bytecode + hash constants) |
| `dig-constants` | `NetworkConstants`, `DIG_MAINNET`, `DIG_TESTNET` |

The re-export layer is **append-only in spirit**: removing a re-export is a breaking
change for consumers and MUST be treated as semver-major.

A re-export's VALUE is part of the contract, not only its name. `DIG_MAINNET` and
`DIG_TESTNET` MUST carry these genesis challenges:

| Re-export | `genesis_challenge` |
|---|---|
| `DIG_MAINNET` | `0af981862a4df51f51ec59c312315d959931d917c375730b89b9e2b0854d1abf` |
| `DIG_TESTNET` | `088c18d6b7859d885dc2f03166e862c958f74b63b6353c3df71d103b9b806c3b` |

`agg_sig_me_additional_data` MUST equal the genesis challenge verbatim; each of the
six domain-separated variants MUST equal `sha256(genesis_challenge || opcode_byte)`
for its opcode (`AGG_SIG_PARENT` 43, `PUZZLE` 44, `AMOUNT` 45, `PUZZLE_AMOUNT` 46,
`PARENT_AMOUNT` 47, `PARENT_PUZZLE` 48). Changing a genesis challenge changes every
signature the network accepts, so it is a **breaking** change for every consumer
(§13) even though the re-exported names are unchanged.

---

## 4. Input types

### 4.1 `ValidationContext` — chain state supplied by the caller

```rust
pub struct ValidationContext {
    pub height: u32,                                 // current L2 block height
    pub timestamp: u64,                              // current block timestamp (seconds since epoch)
    pub constants: NetworkConstants,                 // DIG network constants (dig-constants)
    pub coin_records: HashMap<Bytes32, CoinRecord>,  // coin_id -> record, ONLY the coins being spent
    pub ephemeral_coins: HashSet<Bytes32>,           // coin_ids created earlier in the same block
}
```

- `coin_records` MUST contain the records of the coins the bundle under validation
  spends — it is NOT the full UTXO set. The caller loads them from its own store;
  `dig-clvm` never touches storage.
- `ephemeral_coins` lists coins created by earlier bundles within the same block, so
  that same-block create-and-spend chains validate (§5.2).
- `constants` provides the `ConsensusConstants` (via `NetworkConstants::consensus()`)
  passed verbatim to `chia-consensus`. All consensus-critical parameters — genesis
  challenge, `agg_sig_*_additional_data` domain-separation values, fork heights, cost
  model — come from this value. Implementations MUST source them from the
  `dig-constants` crate (`DIG_MAINNET` / `DIG_TESTNET`), never hardcode them.
- `height` is forwarded to `chia-consensus` for height-dependent condition/flag
  semantics.
- `timestamp` is carried for callers/context completeness; the current validation
  pipeline does not read it directly (time-lock conditions are evaluated by
  `chia-consensus` from the constants + height inputs).

### 4.2 `ValidationConfig` — validation parameters

```rust
pub struct ValidationConfig {
    pub max_cost_per_spend: Cost,   // default: L1_MAX_COST_PER_SPEND
    pub max_cost_per_block: Cost,   // default: L2_MAX_COST_PER_BLOCK
    pub flags: u32,                 // chia-consensus execution flags; default: 0
}
```

- `Default` yields `{ max_cost_per_spend: 11_000_000_000, max_cost_per_block:
  550_000_000_000, flags: 0 }`.
- `flags` accepts `chia-consensus` flag bits and is forwarded verbatim to the
  underlying engine. Recognized values with dig-clvm-level semantics:
  - `0` — full validation (block-validation strictness, signatures verified);
  - `DONT_VALIDATE_SIGNATURE` (`chia_consensus::flags`) — skip BLS aggregate
    verification (§5.3);
  - `MEMPOOL_MODE` — stricter mempool-admission semantics, enforced inside
    `chia-consensus` (rejects unknown opcodes etc.). Flags MAY be OR-combined.
- `max_cost_per_block` is the cost budget enforced by both `validate_spend_bundle`
  and `validate_block` (§5.4, §7.2).
- `max_cost_per_spend` is a carried configuration value; the per-spend limit is
  enforced by `chia-consensus` through the cost budget it is given. dig-clvm's own
  pipeline does not apply `max_cost_per_spend` as an additional independent check.

---

## 5. `validate_spend_bundle` — semantics

Validates one spend bundle (mempool admission or per-bundle block inclusion) and
computes its coin state delta. The pipeline below is normative and MUST execute in
this order; the first failing step returns its error and later steps are not reached.

### 5.1 Step 1 — structural checks (duplicate spends)

Each `coin_spend.coin.coin_id()` in the bundle MUST be unique. A repeated coin id
fails with `ValidationError::DoubleSpend(coin_id)` (detected on the second
occurrence, in bundle order).

### 5.2 Step 2 — coin existence and spent-ness

For every spend, the coin id MUST resolve, in this order:

1. If present in `context.coin_records`: the record's `spent` flag MUST be `false`,
   otherwise `ValidationError::AlreadySpent(coin_id)`.
2. Otherwise, the coin id MUST be present in `context.ephemeral_coins`
   (created earlier in the same block); otherwise
   `ValidationError::CoinNotFound(coin_id)`.

### 5.3 Step 3 — CLVM execution, condition extraction, BLS verification

CLVM execution and condition parsing are delegated to `chia-consensus` with
`max_cost = config.max_cost_per_block`, `context.height`, `config.flags`, and
`context.constants.consensus()`. The allocator is created with `clvmr::LIMIT_HEAP`.
Any failure from the engine (cost exhaustion during execution, invalid conditions,
malformed CLVM, failed announcement/time-lock/identity assertions, or — on the
full-validation path — a bad signature detected inside the engine) maps to
`ValidationError::Clvm(String)` carrying the upstream error's debug rendering.

Exactly one of three verification paths is taken:

| Condition | Path | BLS verification |
|---|---|---|
| `config.flags & DONT_VALIDATE_SIGNATURE != 0` | `run_spendbundle` with the caller's flags | **Skipped** |
| flag clear, `bls_cache = Some(cache)` | `run_spendbundle`, then `BlsCache::aggregate_verify` over the returned (pubkey, message) pairs against `bundle.aggregated_signature` | Verified, with cached pairings reused and new pairings stored |
| flag clear, `bls_cache = None` | `chia_consensus::validate_clvm_and_signature` | Verified from scratch inside the engine |

On the cached path, an invalid aggregate signature fails with
`ValidationError::SignatureFailed`. The signature messages carry Chia's standard
`AGG_SIG_*` domain separation: per-variant coin addenda plus the
`agg_sig_*_additional_data` from `context.constants` — this is computed entirely
inside `chia-consensus`/`chia-bls` and MUST NOT be reimplemented.

When the same `BlsCache` is passed across mempool admission and later block
validation, previously verified pairings are not re-verified. Cache reuse MUST NOT
change the accept/reject outcome, only the cost of reaching it.

### 5.4 Step 4 — cost enforcement

`conditions.cost` (the total CLVM + condition cost computed by `chia-consensus`)
MUST satisfy `cost <= config.max_cost_per_block`; otherwise
`ValidationError::CostExceeded { limit, consumed }`. (The engine already enforces
the same budget during execution; this check is a defensive re-assertion.)

### 5.5 Step 5 — additions and removals

- `removals` = the spent coins, i.e. `bundle.coin_spends[i].coin`, in bundle order.
- `additions` = one `Coin { parent_coin_info: spend.coin_id, puzzle_hash, amount }`
  per `CREATE_COIN` condition, in engine spend/condition order.

### 5.6 Step 6 — value conservation

Let `input = Σ removals.amount` and `output = Σ additions.amount` (u64 sums).
The bundle MUST satisfy `input >= output`; otherwise
`ValidationError::ConservationViolation { input, output }`. The fee is defined as
`fee = input − output`.

### 5.7 Result

On success, returns `SpendResult { additions, removals, fee, conditions }`, where
`conditions: OwnedSpendBundleConditions` is the full parsed condition set from
`chia-consensus` (announcements, agg-sig pairs, time locks, per-spend detail) for
callers that need deeper inspection. The caller — not this crate — commits
`additions`/`removals` to chain state.

---

## 6. `build_block_generator` — semantics

Assembles spend bundles into a compressed block generator (block building).

### 6.1 Selection

- Bundles are considered strictly **in the caller-supplied order** (greedy,
  first-fit). Callers SHOULD pre-sort by fee/cost ratio, highest first, to maximize
  fee revenue; `build_block_generator` performs no sorting of its own.
- For each bundle, CLVM cost is computed via `run_spendbundle` with
  `DONT_VALIDATE_SIGNATURE` (block building does not verify signatures per bundle)
  and a budget of the **remaining** block cost.
- A bundle is **skipped, not fatal**, when it fails execution (invalid, or exceeds
  the remaining budget during execution) or when its computed cost exceeds the
  remaining budget. Selection continues with subsequent bundles.
- For every included bundle: its spends' `(coin, puzzle_reveal, solution)` triples
  are appended, its removals and `CREATE_COIN` additions are accumulated, its
  `aggregated_signature` is aggregated (BLS sum), and its cost is deducted from the
  remaining budget.

### 6.2 Output

Returns `BlockGeneratorResult`:

```rust
pub struct BlockGeneratorResult {
    pub generator: Vec<u8>,              // compressed block-level CLVM program
    pub block_refs: Vec<u32>,            // ALWAYS empty in the current protocol version
    pub aggregated_signature: Signature, // BLS aggregate over included bundles
    pub additions: Vec<Coin>,
    pub removals: Vec<Coin>,
    pub cost: Cost,                      // total cost of included bundles
    pub bundles_included: usize,         // may be < bundles.len()
}
```

- `generator` MUST be produced by `chia_consensus::solution_generator_backrefs`
  (CLVM back-reference compression — the same encoding Chia L1 block builders emit
  and `run_block_generator2` consumes). A serialization failure maps to
  `ValidationError::Clvm`.
- `block_refs` is reserved for cross-block generator references; this version never
  emits them, and `validate_block` correspondingly accepts an empty
  `generator_refs`.
- With zero included bundles the result is a valid empty generator with the default
  (identity) `Signature`, empty additions/removals, `cost = 0`.

### 6.3 Round-trip invariant

A generator produced by `build_block_generator` MUST validate under
`validate_block` (same context/constants, budget ≥ `cost`, and the returned
`aggregated_signature`), yielding exactly the accumulated `additions` and
`removals`.

---

## 7. `validate_block` — semantics

Validates a serialized block generator (block validation).

### 7.1 Execution

The generator plus `generator_refs` are executed via
`chia_consensus::run_block_generator2` with `config.max_cost_per_block`,
`config.flags`, `context.constants.consensus()`, the supplied
`aggregated_signature`, and the optional `BlsCache`. Signature verification is
performed **inside the engine** against all `AGG_SIG_*` conditions produced by the
block, unless `config.flags` contains `DONT_VALIDATE_SIGNATURE`. Any engine failure
(execution error, invalid conditions, cost exhaustion, bad signature) maps to
`ValidationError::Clvm(String)`.

### 7.2 Post-execution checks (same rules as §5.4–§5.6)

1. `conditions.cost <= config.max_cost_per_block`, else `CostExceeded`.
2. `additions` from `CREATE_COIN` conditions;
   `removals` reconstructed per executed spend as
   `Coin { parent_id, puzzle_hash, coin_amount }` from the engine's spend records.
3. Conservation: `Σ removals >= Σ additions`, else `ConservationViolation`;
   `fee = input − output`.

Returns the block-aggregate `SpendResult`.

Note: `validate_block` derives coin existence from the generator itself; the §5.1–§5.2
structural checks against `coin_records` apply only to the per-bundle path. Callers
performing full block validation remain responsible for checking the block's removals
against their UTXO set when committing state.

---

## 8. Error behavior

`ValidationError` (`thiserror`-derived, `Debug + Display`) is the single error type
for all entry points:

| Variant | Emitted when | Emitting path |
|---|---|---|
| `Clvm(String)` | Any `chia-consensus` engine failure (execution, conditions, cost-during-run, in-engine signature failure, generator serialization) | §5.3, §6.2, §7.1 |
| `CoinNotFound(Bytes32)` | Spent coin neither in `coin_records` nor `ephemeral_coins` | §5.2 |
| `AlreadySpent(Bytes32)` | Coin record has `spent = true` | §5.2 |
| `DoubleSpend(Bytes32)` | Same coin id spent twice within one bundle | §5.1 |
| `SignatureFailed` | Cached-path BLS aggregate verification fails | §5.3 |
| `ConservationViolation { input, output }` | `Σ outputs > Σ inputs` | §5.6, §7.2 |
| `CostExceeded { limit, consumed }` | Post-execution cost above `max_cost_per_block` | §5.4, §7.2 |
| `PuzzleHashMismatch(Bytes32)` | Reserved. Defined in the API; puzzle-reveal/puzzle-hash equality is currently detected inside `chia-consensus` and surfaces as `Clvm` | — |
| `Driver(DriverError)` | Reserved (`From<DriverError>` conversion for spend-construction call sites) | — |

Errors are **deterministic**: the same inputs MUST produce the same variant. The
first failing pipeline step wins. `Clvm`'s payload string is diagnostic text and is
NOT part of the stable contract; match on the variant, not the message.

---

## 9. Constants and configuration defaults

| Constant | Value | Meaning |
|---|---|---|
| `L1_MAX_COST_PER_SPEND` | `11_000_000_000` | Chia L1's `MAX_BLOCK_COST_CLVM`; default per-spend budget |
| `L2_MAX_COST_PER_BLOCK` | `550_000_000_000` | DIG L2 block cost budget (50 × the L1 per-spend limit); default block budget |

These are `clvmr::cost::Cost` (`u64`). The L2 block budget being 50× L1's limit is
the **only intentional consensus divergence from Chia L1** in this crate; everything
else (cost model, condition semantics, signature scheme) follows L1 by construction
(§11.1). Network-level constants (genesis challenge, AGG_SIG additional data, fork
heights) are NOT defined here — they come exclusively from `dig-constants`.

---

## 10. Determinism and purity invariants

- Every public function is a **pure function of its arguments** (plus the mutation
  of an explicitly passed `BlsCache`). No global state, no I/O, no clock, no
  randomness. Identical inputs MUST yield identical outputs on every platform.
- CLVM allocators are created per call (`make_allocator(LIMIT_HEAP)`) and never
  shared or leaked across calls.
- The presence/absence/contents of a `BlsCache` MUST NOT change any accept/reject
  decision — only performance.
- The crate performs no logging.

---

## 11. Security properties

1. **BLS aggregate signatures.** Unless `DONT_VALIDATE_SIGNATURE` is explicitly
   requested, every accepted bundle/block has a valid BLS12-381 aggregate signature
   over all `AGG_SIG_*` (pubkey, message) pairs, with Chia's per-opcode domain
   separation (coin-derived addendum + network `additional_data` from the DIG
   constants). Validators MUST NOT set `DONT_VALIDATE_SIGNATURE` on the
   consensus-accept path; it exists for cost estimation and staged mempool
   pipelines where signatures are verified separately.
2. **Value conservation.** No accepted bundle or block can create value:
   `Σ created ≤ Σ spent`, with the difference being the fee (§5.6, §7.2).
3. **Cost bounding.** CLVM execution is budget-limited both during execution (inside
   `chia-consensus`) and by a post-execution re-check, bounding CPU per
   bundle/block. Heap use is bounded via `LIMIT_HEAP`.
4. **Double-spend resistance (bundle-local).** A bundle cannot spend the same coin
   twice, spend a coin marked spent, or spend a nonexistent coin (given a truthful
   `ValidationContext`). Cross-bundle/global double-spend prevention is the
   caller's UTXO-set responsibility.
5. **No key material.** The crate handles public keys and signatures only; it never
   holds or derives secret keys (signing lives in wallet crates such as
   `dig-l1-wallet`).

---

## 12. Conformance requirements

### 12.1 Chia L1 parity — by construction

CLVM execution, condition opcodes and semantics, cost accounting, tree hashing,
puzzle bytecodes, and BLS message construction MUST be byte-for-byte identical to
Chia L1 full nodes. `dig-clvm` achieves this by **delegating to the same Rust crates
Chia's full node uses** (`chia-consensus`'s `run_spendbundle` /
`run_block_generator2` / `validate_clvm_and_signature`, `clvmr`, `chia-bls`,
`chia-puzzles`) rather than by reimplementation. Any change that replaces a
delegated code path with local logic is a conformance violation.

### 12.2 Cross-repo contracts

- **`dig-constants`** is the single source of the DIG network's
  `ConsensusConstants`; `dig-clvm` consumers MUST construct `ValidationContext`
  from `DIG_MAINNET` / `DIG_TESTNET` (or an equivalently sourced
  `NetworkConstants`).
- **Generator encoding** produced here (`solution_generator_backrefs` output) is
  what DIG L2 block producers embed in `dig-block` blocks and what every validator
  re-executes via `validate_block`; producer and validator MUST use compatible
  `chia-consensus` versions so the encoding round-trips.
- The DIG L2 chain stack (`dig-block`, `dig-mempool`, `dig-coinstore`) consumes
  `SpendResult.additions/removals/fee` as the canonical state delta.

### 12.3 Verification gates (CI)

The repository CI (`publish.yml`, on every push/PR to `main`) MUST pass:
`cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
the full test suite under coverage with a **≥80% line-coverage gate**
(`cargo llvm-cov nextest --all-features --fail-under-lines 80 --retries 2
--test-threads 1`; Simulator-based tests are not parallel-safe and MUST run
single-threaded), and `cargo doc --no-deps --all-features`. The test suite is
requirement-driven: one integration-test file per requirement
(`tests/vv_req_{val|blk|bls|par|api|con}_{nnn}.rs`) per the registry in
`docs/requirements/`.

---

## 13. Versioning and release

1. **Semver.** The API in §3 is the compatibility surface. Removing/renaming any
   entry point, field, error variant, constant, or re-export is semver-major;
   additive changes are semver-minor.
2. **Release trigger.** Publication to crates.io and the GitHub release are driven
   by pushing a `v*` tag (or manual dispatch); pushes to `main` run the test gate
   only.
3. **Consensus stability.** Changing `L2_MAX_COST_PER_BLOCK`, the pinned
   `chia-consensus`/`clvmr` major behavior, or any validation-pipeline rule in
   §5–§7 changes DIG L2 consensus and MUST be coordinated as a network-level
   protocol change, not shipped as a routine crate bump.

---

## 14. Conformance summary

| # | Requirement | Level | Spec |
|---|---|---|---|
| 1 | Delegate all CLVM/condition/BLS/cost logic to upstream Chia crates | MUST | §1.2, §12.1 |
| 2 | No I/O, async, storage, or global state; pure deterministic functions | MUST | §1.2, §10 |
| 3 | Validation pipeline order: structure → existence → CLVM+sig → cost → conservation | MUST | §5 |
| 4 | Duplicate coin id in a bundle → `DoubleSpend` | MUST | §5.1 |
| 5 | Unknown coin → `CoinNotFound` unless listed ephemeral; spent coin → `AlreadySpent` | MUST | §5.2 |
| 6 | BLS verified unless `DONT_VALIDATE_SIGNATURE`; cache changes cost only, never outcome | MUST | §5.3, §10 |
| 7 | Total cost ≤ `max_cost_per_block`, else `CostExceeded` | MUST | §5.4, §7.2 |
| 8 | Conservation `Σin ≥ Σout`; `fee = in − out` | MUST | §5.6, §7.2 |
| 9 | Block generator emitted via `solution_generator_backrefs`; round-trips through `validate_block` | MUST | §6.2, §6.3 |
| 10 | Bundle selection is greedy in caller order; failing bundles skipped, not fatal | MUST | §6.1 |
| 11 | Block validation executes via `run_block_generator2` with in-engine signature check | MUST | §7.1 |
| 12 | Network constants sourced from `dig-constants`, never hardcoded | MUST | §4.1, §12.2 |
| 12a | Re-exported `DIG_MAINNET`/`DIG_TESTNET` carry the specified genesis challenges and derived `agg_sig_*` domain values | MUST | §3.2 |
| 13 | Defaults: per-spend 11 G cost, per-block 550 G cost, flags 0 | MUST | §4.2, §9 |
| 14 | Callers pre-sort bundles by fee/cost before block building | SHOULD | §6.1 |
| 15 | Consensus-accept paths never set `DONT_VALIDATE_SIGNATURE` | MUST NOT (set) | §11.1 |
| 16 | CI green: fmt, clippy `-D warnings`, tests single-threaded, ≥80% line coverage, docs | MUST | §12.3 |
| 17 | Releases tag-driven (`v*`); consensus-affecting changes are protocol events | MUST | §13 |
