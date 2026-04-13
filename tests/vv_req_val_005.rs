//! REQUIREMENT: VAL-005 — Already spent coin rejected; unspent coin passes

mod common;

use std::collections::{HashMap, HashSet};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use chia_sdk_coinset::CoinRecord;
use dig_clvm::{validate_spend_bundle, ValidationContext, ValidationError, DIG_TESTNET};

use common::{make_simple_spend, make_context, create_coin_condition, wrap_conditions, test_config};

#[test]
fn val_005_already_spent_rejected() {
    let parent = Bytes32::from([0x05; 32]);
    let output_ph = [0x06; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    // Mark coin as already spent
    let mut coin_records = HashMap::new();
    coin_records.insert(
        coin.coin_id(),
        CoinRecord {
            coin,
            confirmed_block_index: 0,
            spent_block_index: 1,
            coinbase: false,
            timestamp: 0,
            spent: true,
        },
    );

    let context = ValidationContext {
        height: 2,
        timestamp: 1000,
        constants: DIG_TESTNET.clone(),
        coin_records,
        ephemeral_coins: HashSet::new(),
    };
    let config = test_config();
    let bundle = SpendBundle::new(vec![spend], Signature::default());

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_err());
    match result {
        Err(ValidationError::AlreadySpent(_)) => {} // expected
        Err(other) => panic!("expected AlreadySpent, got {:?}", other),
        Ok(_) => panic!("expected error, got Ok"),
    }
}

#[test]
fn val_005_unspent_passes() {
    let parent = Bytes32::from([0x05; 32]);
    let output_ph = [0x06; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]); // unspent by default
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_ok(), "expected Ok, got {:?}", result.as_ref().err());
}
