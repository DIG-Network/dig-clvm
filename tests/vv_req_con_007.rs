//! REQUIREMENT: CON-007 — No circular dependency; consensus() accessible
//!
//! This file compiling successfully proves that dig_clvm can be imported
//! without circular dependency issues. We also verify that
//! DIG_MAINNET.consensus() returns a usable ConsensusConstants reference.

use dig_clvm::{DIG_MAINNET, DIG_TESTNET, ConsensusConstants};

#[test]
fn con_007_compilation_proves_no_circular_dependency() {
    // If this test file compiles, there is no circular dependency between
    // dig-clvm and dig-constants.
    let _mainnet = &DIG_MAINNET;
    let _testnet = &DIG_TESTNET;
}

#[test]
fn con_007_consensus_returns_consensus_constants_ref() {
    let consensus: &ConsensusConstants = DIG_MAINNET.consensus();
    // Accessing a field proves the reference is valid.
    let _ = consensus.genesis_challenge;
}

#[test]
fn con_007_consensus_usable_across_networks() {
    // Both networks yield valid ConsensusConstants references.
    let mc: &ConsensusConstants = DIG_MAINNET.consensus();
    let tc: &ConsensusConstants = DIG_TESTNET.consensus();
    // They should have different genesis challenges but both be valid.
    assert_ne!(mc.genesis_challenge, tc.genesis_challenge);
}
