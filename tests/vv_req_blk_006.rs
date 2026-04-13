//! REQUIREMENT: BLK-006 — validate_block entry point

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;
use dig_clvm::{build_block_generator, validate_block};

#[test]
fn blk_006_validates_generator_from_build() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    let spend_result = validate_block(
        &result.generator,
        &[],
        &ctx,
        &test_config(),
        None,
        &result.aggregated_signature,
    );

    assert!(
        spend_result.is_ok(),
        "validate_block should return Ok for a generator built from a valid bundle, got {:?}",
        spend_result.err()
    );
}

#[test]
fn blk_006_returns_spend_result() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let gen_result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    let spend_result = validate_block(
        &gen_result.generator,
        &[],
        &ctx,
        &test_config(),
        None,
        &gen_result.aggregated_signature,
    )
    .unwrap();

    // SpendResult should have additions and removals
    assert!(
        !spend_result.additions.is_empty(),
        "validate_block should produce additions"
    );
    assert!(
        !spend_result.removals.is_empty(),
        "validate_block should produce removals"
    );
    assert!(spend_result.fee > 0, "fee should be positive (1000 - 900 = 100)");
}

#[test]
fn blk_006_spend_result_fee_correct() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let gen_result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    let spend_result = validate_block(
        &gen_result.generator,
        &[],
        &ctx,
        &test_config(),
        None,
        &gen_result.aggregated_signature,
    )
    .unwrap();

    assert_eq!(spend_result.fee, 100, "fee should be 1000 - 900 = 100");
}
