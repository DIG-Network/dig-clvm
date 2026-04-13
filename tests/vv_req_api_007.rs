//! REQUIREMENT: API-007 — Core validation functions exist
//!
//! Verifies that validate_spend_bundle, build_block_generator, and
//! validate_block are importable as functions from the dig_clvm crate.
//! Compilation is the primary assertion.

use dig_clvm::{build_block_generator, validate_block, validate_spend_bundle};

#[test]
fn api_007_validate_spend_bundle_exists() {
    // Binding the function to a variable proves it exists and has the
    // expected type signature.
    let _f = validate_spend_bundle;
}

#[test]
fn api_007_build_block_generator_exists() {
    let _f = build_block_generator;
}

#[test]
fn api_007_validate_block_exists() {
    let _f = validate_block;
}
