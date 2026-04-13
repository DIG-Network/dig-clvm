//! Shared test utilities for dig-clvm requirement verification tests.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use chia_protocol::{Bytes32, Coin, CoinSpend, Program};
use chia_sdk_coinset::CoinRecord;
use clvm_utils::tree_hash_atom;
use dig_clvm::{ValidationContext, DIG_TESTNET};

/// Create a simple coin spend with puzzle `(q)` (quote — returns solution as output).
///
/// puzzle = 0x01 (the atom `1`, which is the `q` operator)
/// The solution IS the output: pass conditions directly as the solution.
pub fn make_simple_spend(parent: Bytes32, amount: u64, solution: &[u8]) -> CoinSpend {
    let puzzle_bytes = [1u8];
    let puzzle_hash: Bytes32 = tree_hash_atom(&puzzle_bytes).into();
    let coin = Coin::new(parent, puzzle_hash, amount);
    CoinSpend::new(
        coin,
        Program::new(puzzle_bytes.to_vec().into()),
        Program::new(solution.to_vec().into()),
    )
}

/// Build a validation context containing the given coins (all unspent).
pub fn make_context(coins: &[Coin]) -> ValidationContext {
    let mut coin_records = HashMap::new();
    for coin in coins {
        coin_records.insert(
            coin.coin_id(),
            CoinRecord {
                coin: *coin,
                confirmed_block_index: 0,
                spent_block_index: 0,
                coinbase: false,
                timestamp: 0,
                spent: false,
            },
        );
    }
    ValidationContext {
        height: 1,
        timestamp: 1000,
        constants: DIG_TESTNET.clone(),
        coin_records,
        ephemeral_coins: HashSet::new(),
    }
}

/// Build a CLVM CREATE_COIN condition: `(51 puzzle_hash amount)`
pub fn create_coin_condition(puzzle_hash: &[u8; 32], amount: u64) -> Vec<u8> {
    let mut buf = vec![0xff, 0x33, 0xff, 0xa0];
    buf.extend_from_slice(puzzle_hash);
    buf.push(0xff);
    if amount == 0 {
        buf.push(0x80);
    } else {
        let amount_bytes = amount.to_be_bytes();
        let first_nonzero = amount_bytes.iter().position(|&b| b != 0).unwrap_or(7);
        let significant = &amount_bytes[first_nonzero..];
        if significant[0] & 0x80 != 0 {
            buf.push((significant.len() + 1) as u8 | 0x80);
            buf.push(0x00);
            buf.extend_from_slice(significant);
        } else {
            buf.push(significant.len() as u8 | 0x80);
            buf.extend_from_slice(significant);
        }
    }
    buf.push(0x80);
    buf
}

/// Wrap condition bytes into a CLVM list: `(cond1 cond2 ...)`
pub fn wrap_conditions(conditions: &[Vec<u8>]) -> Vec<u8> {
    let mut buf = Vec::new();
    for cond in conditions {
        buf.push(0xff);
        buf.extend_from_slice(cond);
    }
    buf.push(0x80);
    buf
}

/// Default test config with signature validation disabled.
pub fn test_config() -> dig_clvm::ValidationConfig {
    dig_clvm::ValidationConfig {
        flags: chia_consensus::flags::DONT_VALIDATE_SIGNATURE,
        ..dig_clvm::ValidationConfig::default()
    }
}
