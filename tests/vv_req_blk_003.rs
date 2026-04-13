//! REQUIREMENT: BLK-003 — Uses solution_generator_backrefs

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;
use dig_clvm::build_block_generator;

#[test]
fn blk_003_generator_is_nonempty() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    assert!(
        !result.generator.is_empty(),
        "generator bytes should be non-empty when bundles are included"
    );
}

#[test]
fn blk_003_generator_is_valid_clvm() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    // CLVM serialised programs start with 0xff (a cons pair).
    // solution_generator_backrefs produces a valid CLVM program.
    assert_eq!(
        result.generator[0], 0xff,
        "generator should start with 0xff (CLVM cons pair prefix), got 0x{:02x}",
        result.generator[0]
    );
}

#[test]
fn blk_003_generator_length_is_reasonable() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let result = build_block_generator(&[bundle], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    // A single simple spend should produce a generator that is at least a few
    // dozen bytes but not excessively large.
    assert!(
        result.generator.len() > 10,
        "generator should be more than 10 bytes, got {}",
        result.generator.len()
    );
    assert!(
        result.generator.len() < 100_000,
        "generator should be less than 100KB for a single simple spend, got {}",
        result.generator.len()
    );
}
