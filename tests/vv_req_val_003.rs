//! REQUIREMENT: VAL-003 — Duplicate spend rejected; distinct spends pass

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::{validate_spend_bundle, ValidationError};

use common::{make_simple_spend, make_context, create_coin_condition, wrap_conditions, test_config};

#[test]
fn val_003_duplicate_spend_rejected() {
    let parent = Bytes32::from([0x03; 32]);
    let output_ph = [0x04; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    // Same CoinSpend twice
    let bundle = SpendBundle::new(vec![spend.clone(), spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_err());
    match result {
        Err(ValidationError::DoubleSpend(_)) => {} // expected
        Err(other) => panic!("expected DoubleSpend, got {:?}", other),
        Ok(_) => panic!("expected error, got Ok"),
    }
}

#[test]
fn val_003_distinct_spends_pass() {
    let output_ph = [0x04; 32];

    let parent1 = Bytes32::from([0x11; 32]);
    let cond1 = create_coin_condition(&output_ph, 400);
    let sol1 = wrap_conditions(&[cond1]);
    let spend1 = make_simple_spend(parent1, 500, &sol1);
    let coin1 = spend1.coin;

    let parent2 = Bytes32::from([0x22; 32]);
    let cond2 = create_coin_condition(&output_ph, 300);
    let sol2 = wrap_conditions(&[cond2]);
    let spend2 = make_simple_spend(parent2, 500, &sol2);
    let coin2 = spend2.coin;

    let bundle = SpendBundle::new(vec![spend1, spend2], Signature::default());
    let context = make_context(&[coin1, coin2]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_ok(), "expected Ok, got {:?}", result.as_ref().err());
}
