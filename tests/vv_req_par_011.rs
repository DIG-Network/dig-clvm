//! REQUIREMENT: PAR-011 — DONT_VALIDATE_SIGNATURE flag skips BLS; MEMPOOL_MODE exists

mod common;

use chia_bls::Signature;
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::{validate_spend_bundle, ValidationConfig};

use common::{create_coin_condition, make_context, make_simple_spend, wrap_conditions};

#[test]
fn par_011_dont_validate_signature_flag() {
    // DONT_VALIDATE_SIGNATURE skips BLS verification
    let parent = Bytes32::from([0xD1; 32]);
    let output_ph = [0xD2; 32];
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
    assert!(
        result.is_ok(),
        "DONT_VALIDATE_SIGNATURE should skip BLS, got {:?}",
        result.as_ref().err()
    );
}

#[test]
fn par_011_mempool_mode_exists() {
    // MEMPOOL_MODE flag exists and can be set on config
    let config = ValidationConfig {
        flags: chia_consensus::flags::MEMPOOL_MODE,
        ..ValidationConfig::default()
    };
    assert_ne!(config.flags, 0);
}

#[test]
fn par_011_flags_field_on_config() {
    let config = ValidationConfig::default();
    let _flags: u32 = config.flags;
    assert_eq!(config.flags, 0);
}
