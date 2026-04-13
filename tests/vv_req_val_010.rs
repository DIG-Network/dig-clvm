//! REQUIREMENT: VAL-010 — Ephemeral coin passes existence check; non-ephemeral non-existent fails

mod common;

use std::collections::{HashMap, HashSet};

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::{validate_spend_bundle, ValidationContext, ValidationError, DIG_TESTNET};

use common::{make_simple_spend, create_coin_condition, wrap_conditions, test_config};

#[test]
fn val_010_ephemeral_coin_passes_existence_check() {
    let parent = Bytes32::from([0x10; 32]);
    let output_ph = [0x11; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;
    let coin_id = coin.coin_id();

    // Coin NOT in coin_records but IS in ephemeral_coins
    let mut ephemeral = HashSet::new();
    ephemeral.insert(coin_id);

    let context = ValidationContext {
        height: 1,
        timestamp: 1000,
        constants: DIG_TESTNET.clone(),
        coin_records: HashMap::new(),
        ephemeral_coins: ephemeral,
    };
    let config = test_config();
    let bundle = SpendBundle::new(vec![spend], Signature::default());

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    // Should NOT be CoinNotFound — the ephemeral check should pass it through
    match &result {
        Err(ValidationError::CoinNotFound(_)) => {
            panic!("ephemeral coin should not produce CoinNotFound");
        }
        _ => {} // any other result (Ok or different error) is fine
    }
}

#[test]
fn val_010_non_ephemeral_non_existent_fails() {
    let parent = Bytes32::from([0x10; 32]);
    let output_ph = [0x11; 32];
    let cond = create_coin_condition(&output_ph, 500);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);

    // Neither in coin_records nor ephemeral_coins
    let context = ValidationContext {
        height: 1,
        timestamp: 1000,
        constants: DIG_TESTNET.clone(),
        coin_records: HashMap::new(),
        ephemeral_coins: HashSet::new(),
    };
    let config = test_config();
    let bundle = SpendBundle::new(vec![spend], Signature::default());

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_err());
    match result {
        Err(ValidationError::CoinNotFound(_)) => {} // expected
        Err(other) => panic!("expected CoinNotFound, got {:?}", other),
        Ok(_) => panic!("expected error, got Ok"),
    }
}
