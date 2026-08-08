//! Validation context — L2 chain state passed into validation.

use std::collections::{HashMap, HashSet};

use chia_consensus::flags::MEMPOOL_MODE;
use chia_consensus::spendbundle_validation::get_flags_for_height_and_constants;
use chia_protocol::Bytes32;
use chia_sdk_coinset::CoinRecord;
use dig_constants::NetworkConstants;

/// L2 chain state for validation.
///
/// `coin_records` should contain only the coins being spent in this bundle,
/// not the full UTXO set. The caller loads these from their database and
/// passes them in. dig-clvm never touches storage directly.
pub struct ValidationContext {
    /// Current L2 block height.
    pub height: u32,
    /// Current block timestamp (seconds since epoch).
    pub timestamp: u64,
    /// DIG network constants (from dig-constants crate).
    pub constants: NetworkConstants,
    /// Coins being spent in this bundle (coin_id -> CoinRecord).
    /// Only the coins relevant to this validation — NOT the full UTXO set.
    pub coin_records: HashMap<Bytes32, CoinRecord>,
    /// Coins created by earlier bundles in the same block (ephemeral).
    pub ephemeral_coins: HashSet<Bytes32>,
}

impl ValidationContext {
    /// Execution flags for running a spend bundle at this context's height.
    ///
    /// Combines the height-activated hard-fork flags, mempool-mode strictness,
    /// and the caller's `extra` flags.
    ///
    /// This derivation must be done by the caller: `chia-consensus` 0.26 took a
    /// `height` argument and computed
    /// `get_flags_for_height_and_constants(height, constants) | flags | MEMPOOL_MODE`
    /// internally, but 0.36 dropped both the argument and the derivation. Passing
    /// the caller's flags alone would silently execute spends under pre-hard-fork
    /// rules and outside mempool mode — a consensus divergence that still compiles.
    pub fn spend_flags(&self, extra: u32) -> u32 {
        get_flags_for_height_and_constants(self.height, self.constants.consensus())
            | extra
            | MEMPOOL_MODE
    }
}
