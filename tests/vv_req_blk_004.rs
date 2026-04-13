//! REQUIREMENT: BLK-004 — BlockGeneratorResult structure

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::build_block_generator;
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;

#[test]
fn blk_004_result_has_all_fields() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    // Verify all fields of BlockGeneratorResult are accessible
    let _generator: &Vec<u8> = &result.generator;
    let _block_refs: &Vec<u32> = &result.block_refs;
    let _agg_sig: &Signature = &result.aggregated_signature;
    let _additions: &Vec<chia_protocol::Coin> = &result.additions;
    let _removals: &Vec<chia_protocol::Coin> = &result.removals;
    let _cost: clvmr::cost::Cost = result.cost;
    let _bundles_included: usize = result.bundles_included;

    assert!(result.cost > 0, "cost should be positive for a valid spend");
    assert_eq!(result.bundles_included, 1);
    assert!(!result.generator.is_empty());
}

#[test]
fn blk_004_additions_match_conditions() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    // We created a single CREATE_COIN of 900 mojos to dest_ph
    assert_eq!(result.additions.len(), 1, "expected exactly 1 addition");
    assert_eq!(
        result.additions[0].amount, 900,
        "addition amount should be 900"
    );
    assert_eq!(
        result.additions[0].puzzle_hash.as_ref(),
        &dest_ph,
        "addition puzzle_hash should match the destination"
    );
}

#[test]
fn blk_004_removals_match_spent_coin() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let spent_coin = spend.coin;
    let ctx = make_context(&[spent_coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    assert_eq!(result.removals.len(), 1, "expected exactly 1 removal");
    assert_eq!(
        result.removals[0].coin_id(),
        spent_coin.coin_id(),
        "removal coin_id should match the spent coin"
    );
}
