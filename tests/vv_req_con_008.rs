//! REQUIREMENT: CON-008 — The re-exported DIG network constants carry the
//! CANONICAL genesis challenges and correctly-derived AGG_SIG domain values.
//!
//! Re-exporting `DIG_MAINNET`/`DIG_TESTNET` (CON-001) proves only that the names
//! resolve. This requirement pins their VALUES, because a consensus constant that
//! resolves but holds the wrong bytes produces signatures the network rejects — a
//! silent, total break rather than a compile error.
//!
//! # Why the assertions derive from pinned literals, not from the crate
//!
//! Every `agg_sig_*_additional_data` value is `sha256(genesis_challenge || opcode)`.
//! A naive test would read the genesis back out of `DIG_MAINNET` and check the
//! derivation against it — but that assertion holds for ANY self-consistent set of
//! constants, including the placeholder all-zeros genesis this crate shipped with
//! through v0.1.4 (whose AGG_SIG values were genuinely `sha256(0x00…00 || opcode)`).
//! Such a test is green on both the right and the wrong constants and therefore
//! proves nothing.
//!
//! So the genesis challenges below are hard-pinned LITERALS, independent of the
//! dependency, and the AGG_SIG values are derived from those literals. That
//! distinguishes the canonical constants from the two nearest wrong ones: a stale
//! placeholder genesis, and a corrected genesis whose derived domain values were
//! not recomputed alongside it.

use dig_clvm::{NetworkConstants, DIG_MAINNET, DIG_TESTNET};
use hex_literal::hex;
use sha2::{Digest, Sha256};

/// The canonical DIG **L2 mainnet** genesis challenge: the Chia mainnet header
/// hash at height 9,021,277, pinned 2026-07-17.
///
/// Pinned here as a literal so this test is an independent statement of the
/// contract rather than a restatement of whatever the dependency happens to hold.
const CANONICAL_MAINNET_GENESIS: [u8; 32] =
    hex!("0af981862a4df51f51ec59c312315d959931d917c375730b89b9e2b0854d1abf");

/// The canonical DIG **L2 testnet** genesis challenge.
const CANONICAL_TESTNET_GENESIS: [u8; 32] =
    hex!("088c18d6b7859d885dc2f03166e862c958f74b63b6353c3df71d103b9b806c3b");

/// Chia condition opcodes for the six domain-separated AGG_SIG variants, paired
/// with the accessor name each one's additional-data field carries.
///
/// `AGG_SIG_ME` (opcode 49) is deliberately absent: its additional data is the
/// genesis challenge itself, not a hash of it, and is asserted separately.
const AGG_SIG_OPCODES: [(&str, u8); 6] = [
    ("agg_sig_parent", 43),
    ("agg_sig_puzzle", 44),
    ("agg_sig_amount", 45),
    ("agg_sig_puzzle_amount", 46),
    ("agg_sig_parent_amount", 47),
    ("agg_sig_parent_puzzle", 48),
];

/// The Chia L1 derivation for a domain-separated AGG_SIG variant's additional
/// data: `sha256(genesis_challenge || opcode_byte)`.
///
/// See `chia/consensus/condition_tools.py:58-71` in chia-blockchain.
fn derive_agg_sig_domain(genesis: &[u8; 32], opcode: u8) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(genesis);
    hasher.update([opcode]);
    hasher.finalize().into()
}

/// The six domain-separated AGG_SIG additional-data values a `NetworkConstants`
/// actually holds, in `AGG_SIG_OPCODES` order.
fn actual_agg_sig_domains(constants: &NetworkConstants) -> [[u8; 32]; 6] {
    let consensus = constants.consensus();
    [
        consensus.agg_sig_parent_additional_data.to_bytes(),
        consensus.agg_sig_puzzle_additional_data.to_bytes(),
        consensus.agg_sig_amount_additional_data.to_bytes(),
        consensus.agg_sig_puzzle_amount_additional_data.to_bytes(),
        consensus.agg_sig_parent_amount_additional_data.to_bytes(),
        consensus.agg_sig_parent_puzzle_additional_data.to_bytes(),
    ]
}

/// Asserts one network's genesis challenge, its `AGG_SIG_ME` additional data, and
/// all six derived AGG_SIG domain values against a pinned genesis literal.
fn assert_network_matches_genesis(
    network: &str,
    constants: &NetworkConstants,
    expected_genesis: &[u8; 32],
) {
    assert_eq!(
        constants.genesis_challenge().to_bytes(),
        *expected_genesis,
        "{network} genesis_challenge must be the canonical value {}, got {}",
        hex::encode(expected_genesis),
        hex::encode(constants.genesis_challenge()),
    );

    // AGG_SIG_ME's additional data is the genesis challenge verbatim.
    assert_eq!(
        constants.agg_sig_me_additional_data().to_bytes(),
        *expected_genesis,
        "{network} agg_sig_me_additional_data must equal the genesis challenge",
    );

    let actual = actual_agg_sig_domains(constants);
    for (index, (name, opcode)) in AGG_SIG_OPCODES.iter().enumerate() {
        let expected = derive_agg_sig_domain(expected_genesis, *opcode);
        assert_eq!(
            actual[index],
            expected,
            "{network} {name}_additional_data must be sha256(canonical_genesis || {opcode}) = {}, got {}",
            hex::encode(expected),
            hex::encode(actual[index]),
        );
    }
}

#[test]
fn con_008_mainnet_genesis_and_agg_sig_domains_are_canonical() {
    assert_network_matches_genesis("DIG_MAINNET", &DIG_MAINNET, &CANONICAL_MAINNET_GENESIS);
}

#[test]
fn con_008_testnet_genesis_and_agg_sig_domains_are_canonical() {
    assert_network_matches_genesis("DIG_TESTNET", &DIG_TESTNET, &CANONICAL_TESTNET_GENESIS);
}

#[test]
fn con_008_mainnet_and_testnet_genesis_challenges_differ() {
    // Domain separation between the two networks is the whole point of having a
    // distinct testnet genesis: if they collide, a testnet signature is valid on
    // mainnet. Cheap to assert, catastrophic to get wrong.
    assert_ne!(
        DIG_MAINNET.genesis_challenge(),
        DIG_TESTNET.genesis_challenge(),
        "mainnet and testnet genesis challenges must differ",
    );
}

#[test]
fn con_008_no_genesis_challenge_is_a_placeholder() {
    // The pre-0.2.0 constants used all-zeros (mainnet) and 0x00…01 (testnet)
    // placeholders. A low-entropy genesis is never a real header hash, so reject
    // the whole shape rather than the two specific literals that were shipped.
    for (network, genesis) in [
        ("DIG_MAINNET", DIG_MAINNET.genesis_challenge()),
        ("DIG_TESTNET", DIG_TESTNET.genesis_challenge()),
    ] {
        let nonzero_bytes = genesis.iter().filter(|byte| **byte != 0).count();
        assert!(
            nonzero_bytes >= 16,
            "{network} genesis_challenge {} looks like a placeholder, not a real \
             header hash ({nonzero_bytes} of 32 bytes non-zero)",
            hex::encode(genesis),
        );
    }
}
