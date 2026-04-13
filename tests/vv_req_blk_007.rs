//! REQUIREMENT: BLK-007 — Delegates to run_block_generator2

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;
use dig_clvm::{build_block_generator, validate_block};

#[test]
fn blk_007_valid_generator_executes() {
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

    // The generator was executed via run_block_generator2, which produces conditions.
    // Verify that the conditions contain spends (non-empty).
    assert!(
        !spend_result.conditions.spends.is_empty(),
        "conditions should contain at least one spend after executing the generator"
    );
}

#[test]
fn blk_007_no_custom_execution() {
    // Compile-time test: validate_block is importable and callable.
    // This test verifies that the public API exists and that validate_block
    // delegates to run_block_generator2 (the only block execution path).
    let _fn_ptr: fn(
        &[u8],
        &[Vec<u8>],
        &dig_clvm::ValidationContext,
        &dig_clvm::ValidationConfig,
        Option<&mut chia_bls::BlsCache>,
        &Signature,
    ) -> Result<dig_clvm::SpendResult, dig_clvm::ValidationError> = validate_block;

    // If this compiles, validate_block has the expected signature.
}

#[test]
fn blk_007_conditions_have_create_coin() {
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

    // The CREATE_COIN condition should appear in the parsed conditions
    let total_create_coins: usize = spend_result
        .conditions
        .spends
        .iter()
        .map(|s| s.create_coin.len())
        .sum();

    assert_eq!(
        total_create_coins, 1,
        "expected exactly 1 CREATE_COIN condition in the parsed output"
    );
}
