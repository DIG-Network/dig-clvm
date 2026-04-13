//! REQUIREMENT: CON-003 — Distinct mainnet/testnet genesis challenges
//!
//! Verifies that DIG_MAINNET and DIG_TESTNET have different genesis
//! challenges, and that both NetworkConstants values are cloneable.

use dig_clvm::{DIG_MAINNET, DIG_TESTNET};

#[test]
fn con_003_mainnet_and_testnet_have_distinct_genesis_challenges() {
    let mainnet_genesis = DIG_MAINNET.genesis_challenge();
    let testnet_genesis = DIG_TESTNET.genesis_challenge();
    assert_ne!(
        mainnet_genesis, testnet_genesis,
        "Mainnet and testnet must have different genesis challenges"
    );
}

#[test]
fn con_003_network_constants_are_cloneable() {
    let mainnet_clone = DIG_MAINNET.clone();
    let testnet_clone = DIG_TESTNET.clone();

    // Cloned values must produce the same genesis challenges.
    assert_eq!(
        mainnet_clone.genesis_challenge(),
        DIG_MAINNET.genesis_challenge()
    );
    assert_eq!(
        testnet_clone.genesis_challenge(),
        DIG_TESTNET.genesis_challenge()
    );
}

#[test]
fn con_003_cloned_constants_are_independent() {
    // After cloning, modifications to one should not affect the other.
    // Since NetworkConstants fields are not pub-mut, just verify the clone
    // yields a separate instance with identical data.
    let clone1 = DIG_MAINNET.clone();
    let clone2 = DIG_MAINNET.clone();
    assert_eq!(clone1.genesis_challenge(), clone2.genesis_challenge());
    assert_eq!(clone1.max_block_cost_clvm(), clone2.max_block_cost_clvm());
}
