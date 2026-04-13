//! REQUIREMENT: VAL-008 — Default cost constants (11B, 550B, flags=0)

mod common;

use dig_clvm::consensus::config::{L1_MAX_COST_PER_SPEND, L2_MAX_COST_PER_BLOCK};
use dig_clvm::ValidationConfig;

#[test]
fn val_008_default_cost_constants() {
    let config = ValidationConfig::default();
    assert_eq!(config.max_cost_per_spend, 11_000_000_000);
    assert_eq!(config.max_cost_per_block, 550_000_000_000);
    assert_eq!(config.flags, 0);
}

#[test]
fn val_008_l1_constant_value() {
    assert_eq!(L1_MAX_COST_PER_SPEND, 11_000_000_000);
}

#[test]
fn val_008_l2_constant_value() {
    assert_eq!(L2_MAX_COST_PER_BLOCK, 550_000_000_000);
}
