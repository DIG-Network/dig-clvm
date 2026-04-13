//! REQUIREMENT: VAL-013 — Mempool mode via flags; config flags field exists

mod common;

use dig_clvm::ValidationConfig;

#[test]
fn val_013_mempool_mode_via_flags() {
    // run_spendbundle internally ORs MEMPOOL_MODE when requested via flags.
    // Verify the flag constant is importable and can be set on config.
    let config = ValidationConfig {
        flags: chia_consensus::flags::MEMPOOL_MODE,
        ..ValidationConfig::default()
    };
    assert_ne!(config.flags, 0);
    assert_eq!(config.flags & chia_consensus::flags::MEMPOOL_MODE, chia_consensus::flags::MEMPOOL_MODE);
}

#[test]
fn val_013_config_flags_field_exists() {
    let config = ValidationConfig::default();
    // flags field exists and defaults to 0
    let _flags: u32 = config.flags;
    assert_eq!(config.flags, 0);
}

#[test]
fn val_013_flags_combinable() {
    // Multiple flags can be combined
    let config = ValidationConfig {
        flags: chia_consensus::flags::MEMPOOL_MODE | chia_consensus::flags::DONT_VALIDATE_SIGNATURE,
        ..ValidationConfig::default()
    };
    assert_ne!(config.flags & chia_consensus::flags::MEMPOOL_MODE, 0);
    assert_ne!(config.flags & chia_consensus::flags::DONT_VALIDATE_SIGNATURE, 0);
}
