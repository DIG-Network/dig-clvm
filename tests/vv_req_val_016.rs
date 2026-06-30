//! REQUIREMENT: VAL-016 — Full-validation branch (no cache) verifies signatures.
//!
//! With `flags = 0` and `None` cache, `validate_spend_bundle` delegates to
//! `validate_clvm_and_signature`, which performs the BLS aggregate verification
//! itself. A spend that requires a signature, validated against the default
//! (identity) signature, must therefore be rejected. This exercises the
//! `else` (full-validation) branch's failure outcome, complementing the
//! cache-branch coverage in vv_req_bls_006.

mod common;

use chia_bls::{PublicKey, SecretKey, Signature};
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::{validate_spend_bundle, ValidationConfig};

use common::{
    agg_sig_condition, create_coin_condition, make_context, make_simple_spend, wrap_conditions,
};

#[test]
fn val_016_full_validation_no_sig_requirement_succeeds() {
    // flags = 0, None cache -> validate_clvm_and_signature path.
    let parent = Bytes32::from([0x70; 32]);
    let output_ph = [0x71; 32];
    let cond = create_coin_condition(&output_ph, 850);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = ValidationConfig::default();

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    assert!(
        result.is_ok(),
        "no-AGG_SIG spend via full-validation should pass, got {:?}",
        result.as_ref().err()
    );
    assert_eq!(result.unwrap().fee, 150);
}

#[test]
fn val_016_full_validation_rejects_bad_signature() {
    let parent = Bytes32::from([0x72; 32]);
    let sk: SecretKey = SecretKey::from_seed(&[0x16; 32]);
    let pk: PublicKey = sk.public_key();
    let pk_bytes: [u8; 48] = pk.to_bytes();

    let sig_cond = agg_sig_condition(49, &pk_bytes, b"dig-full-path");
    let solution = wrap_conditions(&[sig_cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = ValidationConfig::default(); // flags = 0, no cache

    let result = validate_spend_bundle(&bundle, &context, &config, None);
    // validate_clvm_and_signature surfaces the failed BLS check as a CLVM/
    // consensus error, mapped to ValidationError::Clvm.
    assert!(
        result.is_err(),
        "full-validation must reject a spend with a wrong signature, got Ok"
    );
}
