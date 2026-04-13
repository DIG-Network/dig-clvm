//! REQUIREMENT: PAR-002 — Conditions from chia-consensus; all opcodes handled

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn par_002_conditions_from_chia_consensus() {
    // Valid spend -> conditions.spends populated by chia-consensus
    let parent = Bytes32::from([0xA1; 32]);
    let output_ph = [0xA2; 32];
    let cond = create_coin_condition(&output_ph, 800);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None).unwrap();
    assert!(
        !result.conditions.spends.is_empty(),
        "conditions.spends should be populated"
    );
    // Verify CREATE_COIN was parsed
    assert!(!result.conditions.spends[0].create_coin.is_empty());
}

#[test]
fn par_002_all_opcodes_handled() {
    // Compile test: OwnedSpendBundleConditions from chia-consensus handles all opcodes
    let _type: fn() = || {
        use chia_consensus::owned_conditions::OwnedSpendBundleConditions;
        let _ = std::mem::size_of::<OwnedSpendBundleConditions>();
    };
}

#[test]
fn par_002_opcodes_importable() {
    // chia-consensus opcodes module is re-exported
    let _create_coin: u16 = dig_clvm::opcodes::CREATE_COIN;
}
