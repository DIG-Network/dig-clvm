//! REQUIREMENT: BLK-002 — Cost-aware bundle iteration

mod common;

use common::{create_coin_condition, make_context, make_simple_spend, wrap_conditions};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::build_block_generator;
use dig_clvm::consensus::config::L2_MAX_COST_PER_BLOCK;

#[test]
fn blk_002_bundles_within_budget_all_included() {
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

    let bundle1 = SpendBundle::new(vec![spend1], Signature::default());
    let bundle2 = SpendBundle::new(vec![spend2], Signature::default());

    let result = build_block_generator(&[bundle1, bundle2], &ctx, L2_MAX_COST_PER_BLOCK).unwrap();

    assert_eq!(
        result.bundles_included, 2,
        "both bundles should be included within the default max cost budget"
    );
}

#[test]
fn blk_002_bundle_exceeding_budget_skipped() {
    let parent = Bytes32::from([0x44; 32]);
    let dest_ph = [0x22; 32];
    let conditions = vec![create_coin_condition(&dest_ph, 900)];
    let solution = wrap_conditions(&conditions);
    let spend = make_simple_spend(parent, 1000, &solution);
    let ctx = make_context(&[spend.coin]);

    let bundle = SpendBundle::new(vec![spend], Signature::default());

    // Use an extremely low max_cost so the bundle cannot fit
    let result = build_block_generator(&[bundle], &ctx, 1).unwrap();

    assert_eq!(
        result.bundles_included, 0,
        "bundle should be skipped when max_cost is too low"
    );
}

#[test]
fn blk_002_partial_inclusion_when_budget_tight() {
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

    let bundle1 = SpendBundle::new(vec![spend1], Signature::default());
    let bundle2 = SpendBundle::new(vec![spend2], Signature::default());

    // First, find the cost of a single bundle to set a tight budget
    let single_result =
        build_block_generator(std::slice::from_ref(&bundle1), &ctx, L2_MAX_COST_PER_BLOCK).unwrap();
    let tight_budget = single_result.cost + 1; // just enough for one bundle

    let result = build_block_generator(&[bundle1, bundle2], &ctx, tight_budget).unwrap();

    // At most 1 bundle should fit in the tight budget
    assert!(
        result.bundles_included <= 1,
        "with a tight budget only 0 or 1 bundles should be included, got {}",
        result.bundles_included
    );
}
