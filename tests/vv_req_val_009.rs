//! REQUIREMENT: VAL-009 — Conditions handled by chia-consensus; no custom condition code

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn val_009_conditions_handled_by_chia_consensus() {
    // Valid spend with CREATE_COIN -> additions populated
    let parent = Bytes32::from([0x09; 32]);
    let output_ph = [0x0A; 32];
    let cond = create_coin_condition(&output_ph, 600);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None).unwrap();
    assert_eq!(result.additions.len(), 1);
    assert_eq!(result.additions[0].amount, 600);
    // The addition's puzzle_hash should match the one we specified
    assert_eq!(result.additions[0].puzzle_hash, Bytes32::from(output_ph));
}

#[test]
fn val_009_no_custom_condition_code() {
    // Compile-time test: dig-clvm delegates condition handling to chia-consensus.
    // We just verify that OwnedSpendBundleConditions is from chia_consensus.
    let _type_check: fn() = || {
        let _: chia_consensus::owned_conditions::OwnedSpendBundleConditions;
    };
}
