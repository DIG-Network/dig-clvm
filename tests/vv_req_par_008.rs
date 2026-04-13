//! REQUIREMENT: PAR-008 — Coin from chia_protocol (same type)

mod common;

use dig_clvm::Coin;
use chia_protocol::Bytes32;

#[test]
fn par_008_coin_from_chia_protocol() {
    // dig_clvm::Coin is chia_protocol::Coin — same type
    let parent = Bytes32::from([0x01; 32]);
    let puzzle_hash = Bytes32::from([0x02; 32]);
    let coin: dig_clvm::Coin = Coin::new(parent, puzzle_hash, 1000);

    // Can also be used as chia_protocol::Coin
    let _cp_coin: chia_protocol::Coin = coin;
}

#[test]
fn par_008_coin_fields_accessible() {
    let parent = Bytes32::from([0x01; 32]);
    let puzzle_hash = Bytes32::from([0x02; 32]);
    let coin = Coin::new(parent, puzzle_hash, 500);

    assert_eq!(coin.parent_coin_info, parent);
    assert_eq!(coin.puzzle_hash, puzzle_hash);
    assert_eq!(coin.amount, 500);
}

#[test]
fn par_008_coin_id_deterministic() {
    let parent = Bytes32::from([0x01; 32]);
    let puzzle_hash = Bytes32::from([0x02; 32]);
    let coin1 = Coin::new(parent, puzzle_hash, 1000);
    let coin2 = Coin::new(parent, puzzle_hash, 1000);
    assert_eq!(coin1.coin_id(), coin2.coin_id());
}
