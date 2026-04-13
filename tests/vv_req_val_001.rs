//! REQUIREMENT: VAL-001 — Valid spend returns Ok; invalid spend returns error

mod common;

use std::collections::{HashMap, HashSet};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::{validate_spend_bundle, ValidationContext, ValidationError, DIG_TESTNET};

use common::{make_simple_spend, make_context, create_coin_condition, wrap_conditions, test_config};

#[test]
fn val_001_valid_spend_returns_ok() {
    // 1000 mojo coin -> 900 output -> fee = 100
    let parent = Bytes32::from([0x01; 32]);
    let output_ph = [0x02; 32];
    let cond = create_coin_condition(&output_ph, 900);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_ok(), "expected Ok, got {:?}", result.as_ref().err());

    let sr = result.unwrap();
    assert_eq!(sr.fee, 100);
    assert_eq!(sr.additions.len(), 1);
    assert_eq!(sr.additions[0].amount, 900);
}

#[test]
fn val_001_returns_validation_error_on_invalid() {
    // Nonexistent coin -> CoinNotFound
    let parent = Bytes32::from([0xAA; 32]);
    let output_ph = [0x02; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    // Empty context — coin not present
    let context = ValidationContext {
        height: 1,
        timestamp: 1000,
        constants: DIG_TESTNET.clone(),
        coin_records: HashMap::new(),
        ephemeral_coins: HashSet::new(),
    };
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_err());
    match result {
        Err(ValidationError::CoinNotFound(_)) => {} // expected
        Err(other) => panic!("expected CoinNotFound, got {:?}", other),
        Ok(_) => panic!("expected error, got Ok"),
    }
}
