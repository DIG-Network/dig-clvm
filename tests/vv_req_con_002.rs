//! REQUIREMENT: CON-002 — NetworkConstants accessor correctness
//!
//! Tests all six public accessors on NetworkConstants: consensus(),
//! genesis_challenge(), agg_sig_me_additional_data(), max_block_cost_clvm(),
//! cost_per_byte(), and max_coin_amount(). Validates expected constant values.

use dig_clvm::{NetworkConstants, DIG_MAINNET, DIG_TESTNET};

#[test]
fn con_002_max_block_cost_clvm_is_11_billion() {
    assert_eq!(DIG_MAINNET.max_block_cost_clvm(), 11_000_000_000);
    assert_eq!(DIG_TESTNET.max_block_cost_clvm(), 11_000_000_000);
}

#[test]
fn con_002_cost_per_byte_is_12000() {
    assert_eq!(DIG_MAINNET.cost_per_byte(), 12_000);
    assert_eq!(DIG_TESTNET.cost_per_byte(), 12_000);
}

#[test]
fn con_002_max_coin_amount_is_u64_max() {
    assert_eq!(DIG_MAINNET.max_coin_amount(), u64::MAX);
    assert_eq!(DIG_TESTNET.max_coin_amount(), u64::MAX);
}

#[test]
fn con_002_consensus_returns_same_genesis() {
    // consensus().genesis_challenge must agree with the dedicated accessor.
    let mainnet_via_accessor = DIG_MAINNET.genesis_challenge();
    let mainnet_via_consensus = DIG_MAINNET.consensus().genesis_challenge;
    assert_eq!(mainnet_via_accessor, mainnet_via_consensus);

    let testnet_via_accessor = DIG_TESTNET.genesis_challenge();
    let testnet_via_consensus = DIG_TESTNET.consensus().genesis_challenge;
    assert_eq!(testnet_via_accessor, testnet_via_consensus);
}

#[test]
fn con_002_agg_sig_me_additional_data_accessible() {
    // Must return a Bytes32 without panicking.
    let _mainnet_agg = DIG_MAINNET.agg_sig_me_additional_data();
    let _testnet_agg = DIG_TESTNET.agg_sig_me_additional_data();
}

#[test]
fn con_002_all_six_accessors_callable() {
    // Ensure every accessor compiles and executes on both networks.
    fn exercise_accessors(nc: &NetworkConstants) {
        let _ = nc.consensus();
        let _ = nc.genesis_challenge();
        let _ = nc.agg_sig_me_additional_data();
        let _ = nc.max_block_cost_clvm();
        let _ = nc.cost_per_byte();
        let _ = nc.max_coin_amount();
    }
    exercise_accessors(&DIG_MAINNET);
    exercise_accessors(&DIG_TESTNET);
}
