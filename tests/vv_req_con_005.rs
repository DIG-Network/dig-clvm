//! REQUIREMENT: CON-005 — AGG_SIG additional data derivation
//!
//! Validates the cryptographic derivation of AGG_SIG_* additional data fields.
//! AGG_SIG_ME == genesis_challenge. The remaining six variants are derived as
//! sha256(genesis_challenge || opcode_byte) with opcode bytes 43..=48.
//! Mainnet and testnet must produce distinct values.

use dig_clvm::{DIG_MAINNET, DIG_TESTNET};
use sha2::{Digest, Sha256};

/// Compute sha256(genesis || opcode_byte).
fn sha256_genesis_with_opcode(genesis: &[u8; 32], opcode: u8) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(genesis);
    hasher.update([opcode]);
    hasher.finalize().into()
}

#[test]
fn con_005_agg_sig_me_equals_genesis_challenge() {
    // AGG_SIG_ME additional data is the genesis challenge itself.
    assert_eq!(
        DIG_MAINNET.agg_sig_me_additional_data(),
        DIG_MAINNET.genesis_challenge(),
        "Mainnet: AGG_SIG_ME must equal genesis_challenge"
    );
    assert_eq!(
        DIG_TESTNET.agg_sig_me_additional_data(),
        DIG_TESTNET.genesis_challenge(),
        "Testnet: AGG_SIG_ME must equal genesis_challenge"
    );
}

#[test]
fn con_005_derived_agg_sig_fields_mainnet() {
    let genesis: [u8; 32] = DIG_MAINNET.genesis_challenge().into();
    let c = DIG_MAINNET.consensus();

    // AGG_SIG_PARENT  opcode 43
    let expected_parent: [u8; 32] = sha256_genesis_with_opcode(&genesis, 43);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_parent_additional_data), expected_parent);

    // AGG_SIG_PUZZLE  opcode 44
    let expected_puzzle: [u8; 32] = sha256_genesis_with_opcode(&genesis, 44);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_puzzle_additional_data), expected_puzzle);

    // AGG_SIG_AMOUNT  opcode 45
    let expected_amount: [u8; 32] = sha256_genesis_with_opcode(&genesis, 45);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_amount_additional_data), expected_amount);

    // AGG_SIG_PUZZLE_AMOUNT  opcode 46
    let expected_puzzle_amount: [u8; 32] = sha256_genesis_with_opcode(&genesis, 46);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_puzzle_amount_additional_data), expected_puzzle_amount);

    // AGG_SIG_PARENT_AMOUNT  opcode 47
    let expected_parent_amount: [u8; 32] = sha256_genesis_with_opcode(&genesis, 47);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_parent_amount_additional_data), expected_parent_amount);

    // AGG_SIG_PARENT_PUZZLE  opcode 48
    let expected_parent_puzzle: [u8; 32] = sha256_genesis_with_opcode(&genesis, 48);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_parent_puzzle_additional_data), expected_parent_puzzle);
}

#[test]
fn con_005_derived_agg_sig_fields_testnet() {
    let genesis: [u8; 32] = DIG_TESTNET.genesis_challenge().into();
    let c = DIG_TESTNET.consensus();

    let expected_parent: [u8; 32] = sha256_genesis_with_opcode(&genesis, 43);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_parent_additional_data), expected_parent);

    let expected_puzzle: [u8; 32] = sha256_genesis_with_opcode(&genesis, 44);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_puzzle_additional_data), expected_puzzle);

    let expected_amount: [u8; 32] = sha256_genesis_with_opcode(&genesis, 45);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_amount_additional_data), expected_amount);

    let expected_puzzle_amount: [u8; 32] = sha256_genesis_with_opcode(&genesis, 46);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_puzzle_amount_additional_data), expected_puzzle_amount);

    let expected_parent_amount: [u8; 32] = sha256_genesis_with_opcode(&genesis, 47);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_parent_amount_additional_data), expected_parent_amount);

    let expected_parent_puzzle: [u8; 32] = sha256_genesis_with_opcode(&genesis, 48);
    assert_eq!(<[u8; 32]>::from(c.agg_sig_parent_puzzle_additional_data), expected_parent_puzzle);
}

#[test]
fn con_005_mainnet_and_testnet_derived_values_differ() {
    let mc = DIG_MAINNET.consensus();
    let tc = DIG_TESTNET.consensus();

    // Since genesis challenges differ, all derived values must differ.
    assert_ne!(mc.agg_sig_parent_additional_data, tc.agg_sig_parent_additional_data);
    assert_ne!(mc.agg_sig_puzzle_additional_data, tc.agg_sig_puzzle_additional_data);
    assert_ne!(mc.agg_sig_amount_additional_data, tc.agg_sig_amount_additional_data);
    assert_ne!(mc.agg_sig_puzzle_amount_additional_data, tc.agg_sig_puzzle_amount_additional_data);
    assert_ne!(mc.agg_sig_parent_amount_additional_data, tc.agg_sig_parent_amount_additional_data);
    assert_ne!(mc.agg_sig_parent_puzzle_additional_data, tc.agg_sig_parent_puzzle_additional_data);
}
