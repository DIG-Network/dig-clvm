//! REQUIREMENT: BLS-002 — None cache works (produces same result as not using cache)

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn bls_002_none_cache_works() {
    let parent = Bytes32::from([0xB3; 32]);
    let output_ph = [0xB4; 32];
    let cond = create_coin_condition(&output_ph, 700);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(
        result.is_ok(),
        "None cache should work fine, got {:?}",
        result.as_ref().err()
    );

    let sr = result.unwrap();
    assert_eq!(sr.fee, 300);
    assert_eq!(sr.additions.len(), 1);
    assert_eq!(sr.removals.len(), 1);
}

#[test]
fn bls_002_none_cache_same_as_no_cache() {
    // Both None paths produce identical results
    let parent = Bytes32::from([0xB5; 32]);
    let output_ph = [0xB6; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);

    let spend1 = make_simple_spend(parent, 1000, &solution);
    let coin = spend1.coin;
    let spend2 = make_simple_spend(parent, 1000, &solution);

    let bundle1 = SpendBundle::new(vec![spend1], Signature::default());
    let bundle2 = SpendBundle::new(vec![spend2], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let r1 = validate_spend_bundle(&bundle1, &context, &config, None).unwrap();
    let r2 = validate_spend_bundle(&bundle2, &context, &config, None).unwrap();

    assert_eq!(r1.fee, r2.fee);
    assert_eq!(r1.additions.len(), r2.additions.len());
    assert_eq!(r1.removals.len(), r2.removals.len());
}
