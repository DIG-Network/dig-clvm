//! REQUIREMENT: PAR-012 — Spend-bundle execution flags MUST be derived the way
//! Chia L1 derives them: the height-activated hard-fork flags, OR the caller's
//! flags, OR `MEMPOOL_MODE`.
//!
//! # Why this requirement exists as its own test
//!
//! `chia-consensus` 0.26's `run_spendbundle()` took a `height` argument and
//! computed `get_flags_for_height_and_constants(height, constants) | flags |
//! MEMPOOL_MODE` internally. 0.36 removed both the argument and the derivation:
//! the function now takes fully-resolved `flags` and applies them verbatim.
//!
//! That makes the obvious migration — delete the `height` argument, keep passing
//! `config.flags` — compile cleanly while silently executing every spend outside
//! mempool mode and without the hard-fork flags. It is a consensus divergence with
//! no compile error and no runtime error, and the whole pre-existing suite passes
//! under it (verified by mutation: replacing the derivation with `extra` left all
//! 163 tests green).
//!
//! PAR-011's existing tests do not discriminate it: they assert that the
//! `MEMPOOL_MODE` constant is non-zero and that a config field accepts it, never
//! that the flag reaches CLVM execution.
//!
//! So this file asserts the derivation two independent ways:
//!   1. structurally, against flag constants composed here rather than read back
//!      out of the helper under test, and
//!   2. behaviourally, by driving a spend whose acceptance depends on
//!      `NO_UNKNOWN_CONDS` (a `MEMPOOL_MODE` component) actually reaching the
//!      condition parser.

mod common;

use chia_bls::Signature;
use chia_consensus::flags::{
    COST_CONDITIONS, DONT_VALIDATE_SIGNATURE, MEMPOOL_MODE, NO_UNKNOWN_CONDS, SIMPLE_GENERATOR,
    STRICT_ARGS_COUNT,
};
use chia_protocol::{Bytes32, SpendBundle};
use clvmr::chia_dialect::ENABLE_KECCAK_OPS_OUTSIDE_GUARD;
use clvmr::MEMPOOL_MODE as CLVM_MEMPOOL_MODE;
use dig_clvm::{validate_spend_bundle, ValidationConfig};

use common::{create_coin_condition, make_context, make_simple_spend, wrap_conditions};

/// Both DIG networks set `hard_fork2_height = 0`, so hard fork 2 is active at
/// every height and these flags must appear in the derivation unconditionally.
///
/// Written out from the `chia-consensus` flag constants rather than obtained from
/// `get_flags_for_height_and_constants()`, so this test states the contract
/// instead of restating whatever the dependency computes.
const HARD_FORK_2_FLAGS: u32 = ENABLE_KECCAK_OPS_OUTSIDE_GUARD | COST_CONDITIONS | SIMPLE_GENERATOR;

/// A CLVM condition with opcode 300 and no arguments.
///
/// 300 falls in `chia-consensus`'s `256..=65535` unknown-condition range, which
/// is a hard error under `NO_UNKNOWN_CONDS` and a priced no-op without it. That
/// makes it a direct probe for whether `MEMPOOL_MODE` reached the parser.
fn unknown_condition() -> Vec<u8> {
    // cons | 2-byte atom header | 0x012C (300) | nil terminator
    vec![0xff, 0x82, 0x01, 0x2c, 0x80]
}

#[test]
fn par_012_spend_flags_include_every_mempool_mode_bit() {
    let context = make_context(&[]);
    let flags = context.spend_flags(0);

    for (name, bit) in [
        ("clvmr MEMPOOL_MODE", CLVM_MEMPOOL_MODE),
        ("NO_UNKNOWN_CONDS", NO_UNKNOWN_CONDS),
        ("STRICT_ARGS_COUNT", STRICT_ARGS_COUNT),
    ] {
        assert_eq!(
            flags & bit,
            bit,
            "spend_flags dropped {name}; spends would execute outside mempool mode"
        );
    }
}

#[test]
fn par_012_spend_flags_include_hard_fork_2_flags() {
    let context = make_context(&[]);
    let flags = context.spend_flags(0);

    assert_eq!(
        flags & HARD_FORK_2_FLAGS,
        HARD_FORK_2_FLAGS,
        "spend_flags dropped a hard-fork-2 flag; DIG activates hard fork 2 at height 0, \
         so spends would execute under pre-fork cost and operator rules"
    );
}

#[test]
fn par_012_spend_flags_preserve_caller_flags() {
    let context = make_context(&[]);

    assert_eq!(
        context.spend_flags(DONT_VALIDATE_SIGNATURE) & DONT_VALIDATE_SIGNATURE,
        DONT_VALIDATE_SIGNATURE,
        "spend_flags discarded the caller's flags"
    );
}

#[test]
fn par_012_spend_flags_are_exactly_the_l1_derivation() {
    let context = make_context(&[]);

    assert_eq!(
        context.spend_flags(DONT_VALIDATE_SIGNATURE),
        HARD_FORK_2_FLAGS | DONT_VALIDATE_SIGNATURE | MEMPOOL_MODE,
        "spend_flags is not the Chia L1 derivation \
         (hard-fork flags | caller flags | MEMPOOL_MODE)"
    );
}

/// The behavioural half: prove `MEMPOOL_MODE` reaches the condition parser rather
/// than merely appearing in a `u32` the helper returns.
#[test]
fn par_012_mempool_mode_reaches_the_condition_parser() {
    let parent = Bytes32::from([0xE1; 32]);
    let output_ph = [0xE2; 32];
    let solution = wrap_conditions(&[create_coin_condition(&output_ph, 800), unknown_condition()]);
    let spend = make_simple_spend(parent, 1000, &solution);
    let coin = spend.coin;

    let bundle = SpendBundle::new(vec![spend], Signature::default());
    let context = make_context(&[coin]);
    let config = ValidationConfig {
        flags: DONT_VALIDATE_SIGNATURE,
        ..ValidationConfig::default()
    };

    let result = validate_spend_bundle(&bundle, &context, &config, None);

    assert!(
        result.is_err(),
        "an unknown condition (opcode 300) was accepted, so NO_UNKNOWN_CONDS — and \
         therefore MEMPOOL_MODE — never reached CLVM execution"
    );
}
