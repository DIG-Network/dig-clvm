//! REQUIREMENT: VAL-011 — Fee computed correctly

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{make_simple_spend, make_context, create_coin_condition, wrap_conditions, test_config};

#[test]
fn val_011_fee_computed_correctly() {
    // 1000 input, 700 output -> fee = 300
    let parent = Bytes32::from([0x11; 32]);
    let output_ph = [0x12; 32];
    let cond = create_coin_condition(&output_ph, 700);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None).unwrap();
    assert_eq!(result.fee, 300);
}

#[test]
fn val_011_zero_fee() {
    // input == output -> fee = 0
    let parent = Bytes32::from([0x11; 32]);
    let output_ph = [0x12; 32];
    let cond = create_coin_condition(&output_ph, 1000);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None).unwrap();
    assert_eq!(result.fee, 0);
}
