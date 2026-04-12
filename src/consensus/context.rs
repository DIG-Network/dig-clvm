//! Validation context — L2 chain state passed into validation.

use std::collections::{HashMap, HashSet};

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
