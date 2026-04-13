//! REQUIREMENT: BLK-009 — Round-trip consistency

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;
use dig_clvm::{build_block_generator, validate_block, validate_spend_bundle};

#[test]
fn blk_009_build_then_validate_matches_per_bundle() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);
    let config = test_config();

    let bundle = SpendBundle::new(vec![spend], Signature::default());

    // Path A: validate the spend bundle directly
    let bundle_result = validate_spend_bundle(&bundle, &ctx, &config, None).unwrap();

    // Path B: build a block generator, then validate the block
    let gen_result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();
    let block_result = validate_block(
        &gen_result.generator,
        &[],
        &ctx,
        &config,
        None,
        &gen_result.aggregated_signature,
    )
    .unwrap();

    // Additions should match
    assert_eq!(
        bundle_result.additions.len(),
        block_result.additions.len(),
        "additions count should match between bundle and block validation"
    );
    for (i, (ba, bla)) in bundle_result
        .additions
        .iter()
        .zip(block_result.additions.iter())
        .enumerate()
    {
        assert_eq!(
            ba.amount, bla.amount,
            "addition[{}] amount mismatch: bundle={} vs block={}",
            i, ba.amount, bla.amount
        );
        assert_eq!(
            ba.puzzle_hash, bla.puzzle_hash,
            "addition[{}] puzzle_hash mismatch",
            i
        );
    }

    // Removals should match
    assert_eq!(
        bundle_result.removals.len(),
        block_result.removals.len(),
        "removals count should match between bundle and block validation"
    );
    for (i, (br, blr)) in bundle_result
        .removals
        .iter()
        .zip(block_result.removals.iter())
        .enumerate()
    {
        assert_eq!(
            br.coin_id(),
            blr.coin_id(),
            "removal[{}] coin_id mismatch",
            i
        );
    }

    // Fee should match
    assert_eq!(
        bundle_result.fee, block_result.fee,
        "fee should match between bundle and block validation"
    );
}

#[test]
fn blk_009_round_trip_with_multiple_outputs() {
    let parent = Bytes32::from([0x44; 32]);
    let dest1 = [0x22; 32];
    let dest2 = [0x33; 32];
    let conditions = vec![
        create_coin_condition(&dest1, 500),
        create_coin_condition(&dest2, 300),
    ];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);
    let config = test_config();

    let bundle = SpendBundle::new(vec![spend], Signature::default());

    let bundle_result = validate_spend_bundle(&bundle, &ctx, &config, None).unwrap();

    let gen_result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();
    let block_result = validate_block(
        &gen_result.generator,
        &[],
        &ctx,
        &config,
        None,
        &gen_result.aggregated_signature,
    )
    .unwrap();

    assert_eq!(bundle_result.additions.len(), 2);
    assert_eq!(block_result.additions.len(), 2);
    assert_eq!(bundle_result.fee, 200, "fee should be 1000 - 800 = 200");
    assert_eq!(block_result.fee, 200);
}

#[test]
fn blk_009_round_trip_removals_are_consistent() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let spent_coin = spend.coin;
    let ctx = make_context(&[spent_coin]);
    let config = test_config();

    let bundle = SpendBundle::new(vec![spend], Signature::default());

    let bundle_result = validate_spend_bundle(&bundle, &ctx, &config, None).unwrap();

    let gen_result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();
    let block_result = validate_block(
        &gen_result.generator,
        &[],
        &ctx,
        &config,
        None,
        &gen_result.aggregated_signature,
    )
    .unwrap();

    // Both paths should identify the same coin as removed
    assert_eq!(bundle_result.removals.len(), 1);
    assert_eq!(block_result.removals.len(), 1);
    assert_eq!(
        bundle_result.removals[0].coin_id(),
        spent_coin.coin_id(),
        "bundle validation removal should match the spent coin"
    );
    assert_eq!(
        block_result.removals[0].coin_id(),
        spent_coin.coin_id(),
        "block validation removal should match the spent coin"
    );
}
