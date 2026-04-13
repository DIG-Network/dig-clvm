//! REQUIREMENT: BLS-001 — Accepts Some(&mut BlsCache) and accepts None

mod common;

use chia_bls::{BlsCache, Signature};
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn bls_001_accepts_some_cache() {
    let parent = Bytes32::from([0xB1; 32]);
    let output_ph = [0xB2; 32];
    let cond = create_coin_condition(&output_ph, 800);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let mut cache = BlsCache::default();
    let result = validate_spend_bundle(&bundle, &context, &config, Some(&mut cache));
    assert!(
        result.is_ok(),
        "expected Ok with Some(cache), got {:?}",
        result.as_ref().err()
    );
}

#[test]
fn bls_001_accepts_none() {
    let parent = Bytes32::from([0xB1; 32]);
    let output_ph = [0xB2; 32];
    let cond = create_coin_condition(&output_ph, 800);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(
        result.is_ok(),
        "expected Ok with None cache, got {:?}",
        result.as_ref().err()
    );
}
