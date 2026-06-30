//! REQUIREMENT: BLS-006 — Signature-validating cache branch (run_spendbundle +
//! BlsCache::aggregate_verify), covering both the success and failure outcomes.
//!
//! The other BLS tests pass `Some(&mut cache)` but with the
//! `DONT_VALIDATE_SIGNATURE` flag set, which takes the flag branch and never
//! exercises `BlsCache::aggregate_verify`. This file runs with signature
//! validation ENABLED (flags = 0) and a cache present, which is the branch in
//! `validate_spend_bundle` that calls `cache.aggregate_verify(...)` and maps a
//! `false` result to `ValidationError::SignatureFailed`.

mod common;

use chia_bls::{BlsCache, PublicKey, SecretKey, Signature};
use chia_protocol::{Bytes32, SpendBundle};
use dig_clvm::{validate_spend_bundle, ValidationConfig, ValidationError};

use common::{
    agg_sig_condition, create_coin_condition, make_context, make_simple_spend, wrap_conditions,
};

/// A spend with NO signature requirement, validated with sig-validation ENABLED
/// and a cache present, must succeed: `aggregate_verify` over an empty
/// public-key/message set with the identity signature is valid.
#[test]
fn bls_006_cache_branch_succeeds_without_sig_requirement() {
    let parent = Bytes32::from([0x60; 32]);
    let output_ph = [0x61; 32];
    let cond = create_coin_condition(&output_ph, 900);
    let solution = wrap_conditions(&[cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);

    // flags = 0 -> signature validation ON; Some(cache) -> the cache branch.
    let config = ValidationConfig::default();
    let mut cache = BlsCache::default();

    let result = validate_spend_bundle(&bundle, &context, &config, Some(&mut cache));
    assert!(
        result.is_ok(),
        "no-AGG_SIG spend via the cache branch should verify, got {:?}",
        result.as_ref().err()
    );
    let sr = result.unwrap();
    assert_eq!(sr.fee, 100);
    assert_eq!(sr.additions.len(), 1);
}

/// A spend that REQUIRES a signature (AGG_SIG_UNSAFE), validated against a
/// default (identity) aggregated signature, must fail with `SignatureFailed`
/// when taking the cache branch.
#[test]
fn bls_006_cache_branch_signature_failed() {
    let parent = Bytes32::from([0x62; 32]);
    let sk = SecretKey::from_seed(&[0x06; 32]);
    let pk: PublicKey = sk.public_key();
    let pk_bytes: [u8; 48] = pk.to_bytes();

    // 49 = AGG_SIG_UNSAFE — requires a valid sig over the message.
    let sig_cond = agg_sig_condition(49, &pk_bytes, b"dig-l2-message");
    let solution = wrap_conditions(&[sig_cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    // Default (wrong) aggregated signature — verification must fail.
    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);

    let config = ValidationConfig::default(); // flags = 0
    let mut cache = BlsCache::default();

    let result = validate_spend_bundle(&bundle, &context, &config, Some(&mut cache));
    match result {
        Err(ValidationError::SignatureFailed) => {} // expected
        Err(other) => panic!("expected SignatureFailed, got {:?}", other),
        Ok(_) => panic!("expected SignatureFailed, got Ok"),
    }
}

/// AGG_SIG_ME (opcode 50) drives the same failure path via the cache branch.
#[test]
fn bls_006_cache_branch_agg_sig_me_signature_failed() {
    let parent = Bytes32::from([0x63; 32]);
    let sk = SecretKey::from_seed(&[0x07; 32]);
    let pk_bytes: [u8; 48] = sk.public_key().to_bytes();

    let sig_cond = agg_sig_condition(50, &pk_bytes, b"x");
    let solution = wrap_conditions(&[sig_cond]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);

    let config = ValidationConfig::default();
    let mut cache = BlsCache::default();

    let result = validate_spend_bundle(&bundle, &context, &config, Some(&mut cache));
    assert!(
        matches!(result, Err(ValidationError::SignatureFailed)),
        "AGG_SIG_ME with a wrong signature should fail via the cache branch, got {:?}",
        result.as_ref().err()
    );
}
