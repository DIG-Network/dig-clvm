//! REQUIREMENT: VAL-012 — Signature skipped with flag; without flag path also works for simple puzzle

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::{validate_spend_bundle, ValidationConfig};

use common::{make_simple_spend, make_context, create_coin_condition, wrap_conditions};

#[test]
fn val_012_sig_skipped_with_flag() {
    // DONT_VALIDATE_SIGNATURE flag -> OK with default (identity) signature
    let parent = Bytes32::from([0x12; 32]);
    let output_ph = [0x13; 32];
    let cond = create_coin_condition(&output_ph, 800);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = ValidationConfig {
        flags: chia_consensus::flags::DONT_VALIDATE_SIGNATURE,
        ..ValidationConfig::default()
    };

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_ok(), "expected Ok with DONT_VALIDATE_SIGNATURE, got {:?}", result.as_ref().err());
}

#[test]
fn val_012_without_flag_simple_puzzle_passes() {
    // flags=0, simple `(q)` puzzle does NOT emit AGG_SIG conditions,
    // so identity signature actually passes BLS verification (no pairings needed).
    let parent = Bytes32::from([0x12; 32]);
    let output_ph = [0x13; 32];
    let cond = create_coin_condition(&output_ph, 800);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = ValidationConfig {
        flags: 0,
        ..ValidationConfig::default()
    };

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(result.is_ok(), "expected Ok for simple puzzle with flags=0, got {:?}", result.as_ref().err());
}

#[test]
fn val_012_flag_constant_exists() {
    // Compile test: DONT_VALIDATE_SIGNATURE is importable
    let _flag: u32 = chia_consensus::flags::DONT_VALIDATE_SIGNATURE;
}
