//! REQUIREMENT: BLK-008 — Same checks as validate_spend_bundle

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;
use dig_clvm::{build_block_generator, validate_block};

#[test]
fn blk_008_conservation_checked() {
    // Build a valid block and validate it. The validate_block function enforces
    // that additions do not exceed removals (conservation of value).
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

    // Conservation: total input >= total output
    let total_input: u64 = spend_result.removals.iter().map(|c| c.amount).sum();
    let total_output: u64 = spend_result.additions.iter().map(|c| c.amount).sum();

    assert!(
        total_input >= total_output,
        "conservation violated: input {} < output {}",
        total_input,
        total_output
    );
    assert_eq!(
        spend_result.fee,
        total_input - total_output,
        "fee should equal input - output"
    );
}

#[test]
fn blk_008_cost_tracked() {
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

    assert!(
        spend_result.conditions.cost > 0,
        "conditions.cost should be positive after executing a valid generator"
    );
}

#[test]
fn blk_008_fee_matches_difference() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    // Create two outputs: 600 + 300 = 900, fee = 100
    let conditions = vec![
        create_coin_condition(&dest_ph, 600),
        create_coin_condition(&[0x33; 32], 300),
    ];
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

    assert_eq!(spend_result.fee, 100, "fee should be 1000 - (600 + 300) = 100");
    assert_eq!(spend_result.additions.len(), 2, "should have 2 additions");
}
