//! Validation result types.

use chia_bls::Signature;
use chia_consensus::owned_conditions::OwnedSpendBundleConditions;
use chia_protocol::Coin;
use clvmr::cost::Cost;

/// Result of validating a spend bundle or block.
///
/// This is the primary output: the set of coin state changes that the caller
/// commits to blockchain state.
pub struct SpendResult {
    /// Coins to add to the UTXO set (created by CREATE_COIN conditions).
    pub additions: Vec<Coin>,
    /// Coins to remove from the UTXO set (the spent coins).
    pub removals: Vec<Coin>,
    /// Total fee (sum of removals - sum of additions).
    pub fee: u64,
    /// Full parsed conditions from chia-consensus.
    pub conditions: OwnedSpendBundleConditions,
}

/// Output of build_block_generator().
pub struct BlockGeneratorResult {
    /// The compressed block-level CLVM program.
    pub generator: Vec<u8>,
    /// Block heights of referenced previous generators.
    pub block_refs: Vec<u32>,
    /// Aggregated BLS signature across all included bundles.
    pub aggregated_signature: Signature,
    /// Coins created by all included spends.
    pub additions: Vec<Coin>,
    /// Coins spent by all included spends.
    pub removals: Vec<Coin>,
    /// Total CLVM cost of all included spends.
    pub cost: Cost,
    /// Number of bundles included.
    pub bundles_included: usize,
}
