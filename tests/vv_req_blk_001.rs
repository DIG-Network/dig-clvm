//! REQUIREMENT: BLK-001 — build_block_generator entry point

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;
use dig_clvm::build_block_generator;

#[test]
fn blk_001_single_bundle_produces_result() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    assert_eq!(result.bundles_included, 1, "expected exactly 1 bundle included");
}

#[test]
fn blk_001_empty_bundles_produces_result() {
    let ctx = make_context(&[]);
    let result = build_block_generator(&[], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    assert_eq!(result.bundles_included, 0, "expected 0 bundles included for empty input");
}

#[test]
fn blk_001_result_is_ok() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK);

    assert!(result.is_ok(), "build_block_generator should return Ok, got {:?}", result.err());
}
