//! REQUIREMENT: VAL-006 — Puzzle hash mismatch rejected; correct hash passes

mod common;

use std::collections::{HashMap, HashSet};

use chia_bls::Signature;
use chia_protocol::{Bytes32, Coin, CoinSpend, Program, SpendBundle};
use chia_sdk_coinset::CoinRecord;
use dig_clvm::{validate_spend_bundle, ValidationContext, DIG_TESTNET};

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn val_006_puzzle_hash_mismatch_rejected() {
    let parent = Bytes32::from([0x06; 32]);
    let output_ph = [0x07; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);

    // Create a coin with WRONG puzzle hash (not matching the puzzle reveal)
    let puzzle_bytes = [1u8];
    let wrong_hash = Bytes32::from([0xAA; 32]); // doesn't match tree_hash([1])
    let coin = Coin::new(parent, wrong_hash, 1000);
    let spend = CoinSpend::new(
        coin,
        Program::new(puzzle_bytes.to_vec().into()),
        Program::new(solution.to_vec().into()),
    );

    // Add coin to records so it passes existence check
    let mut coin_records = HashMap::new();
    coin_records.insert(
        coin.coin_id(),
        CoinRecord {
            coin,
            confirmed_block_index: 0,
            spent_block_index: 0,
            coinbase: false,
            timestamp: 0,
            spent: false,
        },
    );

    let context = ValidationContext {
        height: 1,
        timestamp: 1000,
        constants: DIG_TESTNET.clone(),
        coin_records,
        ephemeral_coins: HashSet::new(),
    };
    let config = test_config();
    let bundle = SpendBundle::new(vec![spend], Signature::default());

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    // chia-consensus will reject with a CLVM error about wrong puzzle hash
    assert!(
        result.is_err(),
        "expected error for puzzle hash mismatch, got Ok"
    );
}

#[test]
fn val_006_correct_hash_passes() {
    let parent = Bytes32::from([0x06; 32]);
    let output_ph = [0x07; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution); // uses correct hash
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(
        result.is_ok(),
        "expected Ok, got {:?}",
        result.as_ref().err()
    );
}
