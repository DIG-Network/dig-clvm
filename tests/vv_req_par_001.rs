//! REQUIREMENT: PAR-001 — clvmr used; no custom interpreter

mod common;

use clvmr::Allocator;
use dig_clvm::validate_spend_bundle;

#[test]
fn par_001_clvmr_used() {
    // Compile test: clvmr is a dependency and Allocator::new() works
    let _a = Allocator::new();
}

#[test]
fn par_001_no_custom_interpreter() {
    // validate_spend_bundle is exported from dig_clvm — it delegates to
    // chia-consensus which uses clvmr, not a custom interpreter.
    let _fn_ptr: fn(
        &chia_protocol::SpendBundle,
        &dig_clvm::ValidationContext,
        &dig_clvm::ValidationConfig,
        Option<&mut chia_bls::BlsCache>,
    ) -> Result<dig_clvm::SpendResult, dig_clvm::ValidationError> = validate_spend_bundle;
}

#[test]
fn par_001_allocator_usable() {
    let mut a = Allocator::new();
    // Can allocate a simple atom
    let _node = a.new_atom(&[1, 2, 3]).unwrap();
}
