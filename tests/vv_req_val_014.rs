//! REQUIREMENT: VAL-014 — SpendResult has additions, removals, fee, conditions

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::validate_spend_bundle;

use common::{
    create_coin_condition, make_context, make_simple_spend, test_config, wrap_conditions,
};

#[test]
fn val_014_spend_result_has_additions_removals_fee_conditions() {
    // Two CREATE_COIN outputs from a single spend
    let parent = Bytes32::from([0x14; 32]);
    let ph1 = [0x15; 32];
    let ph2 = [0x16; 32];
    let cond1 = create_coin_condition(&ph1, 400);
    let cond2 = create_coin_condition(&ph2, 300);
    let solution = wrap_conditions(&[cond1, cond2]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = test_config();

    let result = validate_spend_bundle(&bundle, &context, &config, None).unwrap();

    // Additions: 2 coins
    assert_eq!(result.additions.len(), 2);
    let amounts: Vec<u64> = result.additions.iter().map(|c| c.amount).collect();
    assert!(amounts.contains(&400));
    assert!(amounts.contains(&300));

    // Removals: 1 coin (the spent coin)
    assert_eq!(result.removals.len(), 1);
    assert_eq!(result.removals[0].coin_id(), coin.coin_id());

    // Fee: 1000 - 400 - 300 = 300
    assert_eq!(result.fee, 300);

    // Conditions: spends is non-empty
    assert!(!result.conditions.spends.is_empty());
}
