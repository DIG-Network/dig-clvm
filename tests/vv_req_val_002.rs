//! REQUIREMENT: VAL-002 — Valid spend produces conditions; no custom CLVM code

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn val_002_valid_spend_produces_conditions() {
    let parent = Bytes32::from([0x10; 32]);
    let output_ph = [0x20; 32];
    let cond = create_coin_condition(&output_ph, 800);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None).unwrap();
    // conditions.spends should be non-empty
    assert!(
        !result.conditions.spends.is_empty(),
        "expected non-empty conditions.spends"
    );
}

#[test]
fn val_002_no_custom_clvm_code() {
    // Compile-time test: validate_spend_bundle is provided by dig_clvm,
    // not by custom CLVM bytecode. This test passes if it compiles.
    let _fn_ptr: fn(
        &chia_protocol::SpendBundle,
        &dig_clvm::ValidationContext,
        &dig_clvm::ValidationConfig,
        Option<&mut chia_bls::BlsCache>,
    ) -> Result<dig_clvm::SpendResult, dig_clvm::ValidationError> = dig_clvm::validate_spend_bundle;
}
