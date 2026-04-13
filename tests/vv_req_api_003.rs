//! REQUIREMENT: API-003 — SpendContext and CoinRecord availability
//!
//! Verifies that SpendContext (from chia-sdk-driver) and CoinRecord
//! (from chia-sdk-coinset) are usable through the dig_clvm re-exports.

use dig_clvm::{CoinRecord, SpendContext};

#[test]
fn api_003_spend_context_new() {
    let _ctx = SpendContext::new();
}

#[test]
fn api_003_coin_record_has_nonzero_size() {
    let size = std::mem::size_of::<CoinRecord>();
    assert!(size > 0, "CoinRecord must have a non-zero size");
}

#[test]
fn api_003_coin_record_fields_accessible() {
    use dig_clvm::{Bytes32, Coin};

    let record = CoinRecord {
        coin: Coin::new(Bytes32::default(), Bytes32::default(), 1000),
        confirmed_block_index: 5,
        spent_block_index: 0,
        coinbase: false,
        timestamp: 1234567890,
        spent: false,
    };
    assert_eq!(record.coin.amount, 1000);
    assert_eq!(record.confirmed_block_index, 5);
    assert!(!record.spent);
}
