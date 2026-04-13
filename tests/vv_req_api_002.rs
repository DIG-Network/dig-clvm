//! REQUIREMENT: API-002 — ValidationConfig default in synchronous context
//!
//! Verifies that ValidationConfig::default() works in a plain synchronous
//! context without requiring async runtime or other special setup.

use dig_clvm::consensus::config::{L1_MAX_COST_PER_SPEND, L2_MAX_COST_PER_BLOCK};
use dig_clvm::ValidationConfig;

#[test]
fn api_002_validation_config_default_is_sync() {
    // This must compile and run without any async runtime.
    let config = ValidationConfig::default();
    assert_eq!(config.max_cost_per_spend, L1_MAX_COST_PER_SPEND);
    assert_eq!(config.max_cost_per_block, L2_MAX_COST_PER_BLOCK);
    assert_eq!(config.flags, 0);
}

#[test]
fn api_002_validation_config_fields_are_public() {
    // All fields are pub — verify by constructing directly.
    let config = ValidationConfig {
        max_cost_per_spend: 100,
        max_cost_per_block: 200,
        flags: 42,
    };
    assert_eq!(config.max_cost_per_spend, 100);
    assert_eq!(config.max_cost_per_block, 200);
    assert_eq!(config.flags, 42);
}

#[test]
fn api_002_default_flags_are_zero() {
    let config = ValidationConfig::default();
    assert_eq!(
        config.flags, 0,
        "Default flags must be 0 (no special modes)"
    );
}
