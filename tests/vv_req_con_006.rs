//! REQUIREMENT: CON-006 — Proof-of-Space fields and plot filter heights
//!
//! Validates that PoS fields carry sensible neutral values (slot_blocks_target > 0,
//! max_sub_slot_blocks > 0, etc.) and that plot filter transition heights are set
//! to 0xFFFFFFFF (effectively disabled for DIG L2).

use dig_clvm::{DIG_MAINNET, DIG_TESTNET};

#[test]
fn con_006_pos_fields_are_valid_mainnet() {
    let c = DIG_MAINNET.consensus();
    assert!(c.slot_blocks_target > 0, "slot_blocks_target must be > 0");
    assert!(c.max_sub_slot_blocks > 0, "max_sub_slot_blocks must be > 0");
    assert!(c.num_sps_sub_slot > 0, "num_sps_sub_slot must be > 0");
    assert!(c.sub_slot_iters_starting > 0, "sub_slot_iters_starting must be > 0");
    assert!(c.sub_slot_time_target > 0, "sub_slot_time_target must be > 0");
    assert!(c.epoch_blocks > 0, "epoch_blocks must be > 0");
}

#[test]
fn con_006_pos_fields_are_valid_testnet() {
    let c = DIG_TESTNET.consensus();
    assert!(c.slot_blocks_target > 0, "slot_blocks_target must be > 0");
    assert!(c.max_sub_slot_blocks > 0, "max_sub_slot_blocks must be > 0");
    assert!(c.num_sps_sub_slot > 0, "num_sps_sub_slot must be > 0");
    assert!(c.sub_slot_iters_starting > 0, "sub_slot_iters_starting must be > 0");
    assert!(c.sub_slot_time_target > 0, "sub_slot_time_target must be > 0");
    assert!(c.epoch_blocks > 0, "epoch_blocks must be > 0");
}

#[test]
fn con_006_plot_filter_heights_are_max_u32() {
    let sentinel = 0xffff_ffff_u32;

    // Mainnet
    let mc = DIG_MAINNET.consensus();
    assert_eq!(mc.plot_filter_128_height, sentinel, "plot_filter_128_height must be 0xFFFFFFFF");
    assert_eq!(mc.plot_filter_64_height, sentinel, "plot_filter_64_height must be 0xFFFFFFFF");
    assert_eq!(mc.plot_filter_32_height, sentinel, "plot_filter_32_height must be 0xFFFFFFFF");

    // Testnet
    let tc = DIG_TESTNET.consensus();
    assert_eq!(tc.plot_filter_128_height, sentinel, "plot_filter_128_height must be 0xFFFFFFFF");
    assert_eq!(tc.plot_filter_64_height, sentinel, "plot_filter_64_height must be 0xFFFFFFFF");
    assert_eq!(tc.plot_filter_32_height, sentinel, "plot_filter_32_height must be 0xFFFFFFFF");
}
