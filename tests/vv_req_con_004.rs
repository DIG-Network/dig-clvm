//! REQUIREMENT: CON-004 — Hard fork heights are zero
//!
//! DIG L2 starts with all consensus features enabled from block 0.
//! Both hard_fork_height and hard_fork2_height must be 0 for mainnet
//! and testnet.

use dig_clvm::{DIG_MAINNET, DIG_TESTNET};

#[test]
fn con_004_mainnet_hard_fork_height_is_zero() {
    assert_eq!(
        DIG_MAINNET.consensus().hard_fork_height,
        0,
        "Mainnet hard_fork_height must be 0"
    );
}

#[test]
fn con_004_mainnet_hard_fork2_height_is_zero() {
    assert_eq!(
        DIG_MAINNET.consensus().hard_fork2_height,
        0,
        "Mainnet hard_fork2_height must be 0"
    );
}

#[test]
fn con_004_testnet_hard_fork_heights_are_zero() {
    assert_eq!(
        DIG_TESTNET.consensus().hard_fork_height,
        0,
        "Testnet hard_fork_height must be 0"
    );
    assert_eq!(
        DIG_TESTNET.consensus().hard_fork2_height,
        0,
        "Testnet hard_fork2_height must be 0"
    );
}
