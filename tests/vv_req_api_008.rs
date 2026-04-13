//! REQUIREMENT: API-008 — Consensus sub-module and cost constants
//!
//! Verifies that dig_clvm::consensus::* is importable and that the cost
//! constants L1_MAX_COST_PER_SPEND and L2_MAX_COST_PER_BLOCK have the
//! expected values.

use dig_clvm::consensus::config::{L1_MAX_COST_PER_SPEND, L2_MAX_COST_PER_BLOCK};

#[test]
fn api_008_l1_max_cost_per_spend_is_11_billion() {
    assert_eq!(
        L1_MAX_COST_PER_SPEND, 11_000_000_000,
        "L1_MAX_COST_PER_SPEND must be 11 billion"
    );
}

#[test]
fn api_008_l2_max_cost_per_block_is_550_billion() {
    assert_eq!(
        L2_MAX_COST_PER_BLOCK, 550_000_000_000,
        "L2_MAX_COST_PER_BLOCK must be 550 billion"
    );
}

#[test]
fn api_008_cost_constants_also_reexported_at_crate_root() {
    // These are re-exported from dig_clvm::consensus::mod -> dig_clvm root.
    use dig_clvm::{L1_MAX_COST_PER_SPEND as ROOT_L1, L2_MAX_COST_PER_BLOCK as ROOT_L2};
    assert_eq!(ROOT_L1, L1_MAX_COST_PER_SPEND);
    assert_eq!(ROOT_L2, L2_MAX_COST_PER_BLOCK);
}
