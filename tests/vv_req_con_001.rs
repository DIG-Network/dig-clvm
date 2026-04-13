//! REQUIREMENT: CON-001 — Re-export of DIG network constants
//!
//! Verifies that dig_clvm re-exports DIG_MAINNET, DIG_TESTNET, and
//! NetworkConstants from the dig-constants crate. Compilation of this
//! file alone constitutes a passing test.

use dig_clvm::{DIG_MAINNET, DIG_TESTNET, NetworkConstants};

#[test]
fn con_001_dig_mainnet_is_accessible() {
    // Binding to a typed variable proves the re-export exists and the
    // type is correct.
    let _mainnet: &NetworkConstants = &DIG_MAINNET;
}

#[test]
fn con_001_dig_testnet_is_accessible() {
    let _testnet: &NetworkConstants = &DIG_TESTNET;
}

#[test]
fn con_001_network_constants_type_is_reexported() {
    // Prove we can use NetworkConstants as a type annotation through
    // the dig_clvm facade crate.
    fn accept_constants(_c: &NetworkConstants) {}
    accept_constants(&DIG_MAINNET);
    accept_constants(&DIG_TESTNET);
}
