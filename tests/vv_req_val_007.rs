//! REQUIREMENT: VAL-007 — Cost within limit succeeds; cost exceeded with tiny limit fails

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::{validate_spend_bundle, ValidationConfig};

use common::{create_coin_condition, make_context, make_simple_spend, wrap_conditions};

#[test]
fn val_007_cost_within_limit_succeeds() {
    let parent = Bytes32::from([0x07; 32]);
    let output_ph = [0x08; 32];
    let cond = create_coin_condition(&output_ph, 800);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = common::test_config(); // uses L2 limit

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(
        result.is_ok(),
        "expected Ok within L2 cost limit, got {:?}",
        result.as_ref().err()
    );
}

#[test]
fn val_007_cost_exceeded_with_tiny_limit() {
    let parent = Bytes32::from([0x07; 32]);
    let output_ph = [0x08; 32];
    let cond = create_coin_condition(&output_ph, 800);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    // Tiny cost limit — should fail
    let config = ValidationConfig {
        max_cost_per_block: 1,
        flags: chia_consensus::flags::DONT_VALIDATE_SIGNATURE,
        ..ValidationConfig::default()
    };

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_err(), "expected error with max_cost_per_block=1");
}
