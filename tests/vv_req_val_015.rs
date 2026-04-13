//! REQUIREMENT: VAL-015 — Context accepts minimal coins; context uses HashMap

mod common;

use std::collections::{HashMap, HashSet};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use chia_sdk_coinset::CoinRecord;
use dig_clvm::{validate_spend_bundle, ValidationContext, DIG_TESTNET};

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn val_015_context_accepts_minimal_coins() {
    // 1 coin in HashMap
    let parent = Bytes32::from([0x15; 32]);
    let output_ph = [0x16; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let context = make_context(&[coin]);
    assert_eq!(context.coin_records.len(), 1);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(
        result.is_ok(),
        "expected Ok with 1 coin in context, got {:?}",
        result.as_ref().err()
    );
}

#[test]
fn val_015_context_uses_hashmap() {
    // Verify coin_records is a HashMap<Bytes32, CoinRecord>
    let context = ValidationContext {
        height: 1,
        timestamp: 1000,
        constants: DIG_TESTNET.clone(),
        coin_records: HashMap::new(),
        ephemeral_coins: HashSet::new(),
    };
    let _: &HashMap<Bytes32, CoinRecord> = &context.coin_records;
    assert!(context.coin_records.is_empty());
}

#[test]
fn val_015_context_fields_accessible() {
    let context = ValidationContext {
        height: 42,
        timestamp: 9999,
        constants: DIG_TESTNET.clone(),
        coin_records: HashMap::new(),
        ephemeral_coins: HashSet::new(),
    };
    assert_eq!(context.height, 42);
    assert_eq!(context.timestamp, 9999);
}
