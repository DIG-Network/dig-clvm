//! REQUIREMENT: BLS-003 — Cache reusable across calls

mod common;

use chia_bls::{BlsCache, Signature};
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn bls_003_cache_reusable_across_calls() {
    let mut cache = BlsCache::default();

    // First validation
    let parent1 = Bytes32::from([0xC1; 32]);
    let output_ph = [0xC2; 32];
    let cond1 = create_coin_condition(&output_ph, 800);
    let sol1 = wrap_conditions(&[cond1]);
    let spend1 = make_simple_spend(parent1, 1000, &sol1);
    let coin1 = spend1.coin;

    let bundle1 = SpendBundle::new(vec![spend1], Signature::default());
    let context1 = make_context(&[coin1]);
    let config = test_config();

    let r1 = validate_spend_bundle(&bundle1, &context1, &config, Some(&mut cache));
    assert!(
        r1.is_ok(),
        "first call with cache failed: {:?}",
        r1.as_ref().err()
    );

    // Second validation — same cache reused
    let parent2 = Bytes32::from([0xC3; 32]);
    let cond2 = create_coin_condition(&output_ph, 600);
    let sol2 = wrap_conditions(&[cond2]);
    let spend2 = make_simple_spend(parent2, 1000, &sol2);
    let coin2 = spend2.coin;

    let bundle2 = SpendBundle::new(vec![spend2], Signature::default());
    let context2 = make_context(&[coin2]);

    let r2 = validate_spend_bundle(&bundle2, &context2, &config, Some(&mut cache));
    assert!(
        r2.is_ok(),
        "second call with reused cache failed: {:?}",
        r2.as_ref().err()
    );
}

#[test]
fn bls_003_cache_persists_state() {
    // Just verify the cache object lives across multiple uses without error
    let mut cache = BlsCache::default();

    for i in 0u8..3 {
        let parent = Bytes32::from([i + 0xD0; 32]);
        let output_ph = [i + 0xE0; 32];
        let cond = create_coin_condition(&output_ph, 500);
        let sol = wrap_conditions(&[cond]);
        let spend = make_simple_spend(parent, 1000, &sol);
        let coin = spend.coin;

        let bundle = SpendBundle::new(vec![spend], Signature::default());
        let context = make_context(&[coin]);
        let config = test_config();

        let result = validate_spend_bundle(&bundle, &context, &config, Some(&mut cache));
        assert!(
            result.is_ok(),
            "iteration {} failed: {:?}",
            i,
            result.as_ref().err()
        );
    }
}
