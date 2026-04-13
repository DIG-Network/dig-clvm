//! REQUIREMENT: PAR-007 — Per-spend cost is 11 billion

mod common;

use dig_clvm::consensus::config::L1_MAX_COST_PER_SPEND;

#[test]
fn par_007_per_spend_cost_11b() {
    assert_eq!(L1_MAX_COST_PER_SPEND, 11_000_000_000);
}

#[test]
fn par_007_config_default_uses_11b() {
    let config = dig_clvm::ValidationConfig::default();
    assert_eq!(config.max_cost_per_spend, 11_000_000_000);
}

#[test]
fn par_007_constant_is_cost_type() {
    let _cost: clvmr::cost::Cost = L1_MAX_COST_PER_SPEND;
}
