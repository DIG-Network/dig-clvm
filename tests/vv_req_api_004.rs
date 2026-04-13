//! REQUIREMENT: API-004 — ValidationContext uses HashMap, not a DB handle
//!
//! Verifies that ValidationContext stores coin records in a HashMap (in-memory),
//! and does not require a database connection or file handle.

use std::collections::{HashMap, HashSet};

use dig_clvm::{ValidationContext, DIG_TESTNET};

#[test]
fn api_004_validation_context_uses_hashmap() {
    let ctx = ValidationContext {
        height: 0,
        timestamp: 0,
        constants: DIG_TESTNET.clone(),
        coin_records: HashMap::new(),
        ephemeral_coins: HashSet::new(),
    };
    assert!(ctx.coin_records.is_empty());
}

#[test]
fn api_004_coin_records_is_empty_on_new_context() {
    let ctx = ValidationContext {
        height: 1,
        timestamp: 1000,
        constants: DIG_TESTNET.clone(),
        coin_records: HashMap::new(),
        ephemeral_coins: HashSet::new(),
    };
    assert!(ctx.coin_records.is_empty(), "Fresh context must have empty coin_records");
    assert!(ctx.ephemeral_coins.is_empty(), "Fresh context must have empty ephemeral_coins");
}

#[test]
fn api_004_validation_context_fields_are_public() {
    let mut ctx = ValidationContext {
        height: 10,
        timestamp: 5000,
        constants: DIG_TESTNET.clone(),
        coin_records: HashMap::new(),
        ephemeral_coins: HashSet::new(),
    };
    // All fields are pub and directly mutable.
    ctx.height = 20;
    ctx.timestamp = 9000;
    assert_eq!(ctx.height, 20);
    assert_eq!(ctx.timestamp, 9000);
}
