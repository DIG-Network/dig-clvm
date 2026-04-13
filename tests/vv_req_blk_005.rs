//! REQUIREMENT: BLK-005 — Aggregated signature

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::build_block_generator;
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;

#[test]
fn blk_005_single_bundle_sig_matches() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle_sig = Signature::default();
    let bundle = SpendBundle::new(vec![spend], bundle_sig.clone());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    // With a single bundle, the aggregated signature should equal the bundle's signature
    assert_eq!(
        result.aggregated_signature, bundle_sig,
        "aggregated signature should match the single bundle's signature"
    );
}

#[test]
fn blk_005_empty_bundles_default_sig() {
    let ctx = make_context(&[]);
    let result = build_block_generator(&[], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    // With no bundles, aggregated_signature should be the identity (default)
    assert_eq!(
        result.aggregated_signature,
        Signature::default(),
        "aggregated signature should be the identity element when no bundles are included"
    );
}

#[test]
fn blk_005_multiple_bundles_aggregates() {
    let parent1 = Bytes32::from([0x44; 32]);
    let parent2 = Bytes32::from([0x55; 32]);
    let dest_ph = [0x22; 32];

    let cond1 = vec![create_coin_condition(&dest_ph, 900)];
    let sol1 = wrap_conditions(&cond1);
    let spend1 = make_simple_spend(parent1, 1000, &sol1);

    let cond2 = vec![create_coin_condition(&dest_ph, 400)];
    let sol2 = wrap_conditions(&cond2);
    let spend2 = make_simple_spend(parent2, 500, &sol2);

    let ctx = make_context(&[spend1.coin, spend2.coin]);

    // Both bundles use default (identity) signatures, so aggregation should
    // also yield the identity signature.
    let bundle1 = SpendBundle::new(vec![spend1], Signature::default());
    let bundle2 = SpendBundle::new(vec![spend2], Signature::default());

    let result = build_block_generator(&[bundle1, bundle2], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    assert_eq!(result.bundles_included, 2);
    // identity + identity = identity
    assert_eq!(
        result.aggregated_signature,
        Signature::default(),
        "aggregation of identity signatures should be identity"
    );
}
