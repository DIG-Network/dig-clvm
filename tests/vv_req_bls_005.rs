//! REQUIREMENT: BLS-005 — Cache does not affect correctness

mod common;

use chia_bls::{BlsCache, Signature};
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn bls_005_cache_does_not_affect_correctness() {
    let parent = Bytes32::from([0xF1; 32]);
    let output_ph = [0xF2; 32];
    let cond = create_coin_condition(&output_ph, 600);
    let solution = wrap_conditions(&[cond]);

    // Without cache
    let spend1 = make_simple_spend(parent, 1000, &solution);
    let coin = spend1.coin;
    let bundle1 = SpendBundle::new(vec![spend1], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let r_no_cache = validate_spend_bundle(&bundle1, &context, &config, None).unwrap();

    // With cache
    let spend2 = make_simple_spend(parent, 1000, &solution);
    let bundle2 = SpendBundle::new(vec![spend2], Signature::default());
    let mut cache = BlsCache::default();

    let r_with_cache =
        validate_spend_bundle(&bundle2, &context, &config, Some(&mut cache)).unwrap();

    // Same results
    assert_eq!(r_no_cache.fee, r_with_cache.fee);
    assert_eq!(r_no_cache.additions.len(), r_with_cache.additions.len());
    assert_eq!(r_no_cache.removals.len(), r_with_cache.removals.len());

    for (a, b) in r_no_cache
        .additions
        .iter()
        .zip(r_with_cache.additions.iter())
    {
        assert_eq!(a.amount, b.amount);
        assert_eq!(a.puzzle_hash, b.puzzle_hash);
    }
}

#[test]
fn bls_005_additions_match_without_cache() {
    let parent = Bytes32::from([0xF3; 32]);
    let ph1 = [0xF4; 32];
    let ph2 = [0xF5; 32];
    let cond1 = create_coin_condition(&ph1, 300);
    let cond2 = create_coin_condition(&ph2, 400);
    let solution = wrap_conditions(&[cond1, cond2]);

    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;
    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None).unwrap();
    assert_eq!(result.additions.len(), 2);
    assert_eq!(result.fee, 300); // 1000 - 300 - 400
}
