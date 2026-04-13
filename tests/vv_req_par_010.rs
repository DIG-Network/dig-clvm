//! REQUIREMENT: PAR-010 — Block generator functions exist

mod common;

use dig_clvm::{build_block_generator, validate_block};

#[test]
fn par_010_build_block_generator_exists() {
    // build_block_generator is importable from dig_clvm
    let _fn_ptr = build_block_generator;
}

#[test]
fn par_010_validate_block_exists() {
    // validate_block is importable from dig_clvm
    let _fn_ptr = validate_block;
}

#[test]
fn par_010_block_generator_result_type() {
    // BlockGeneratorResult is importable
    let _size = std::mem::size_of::<dig_clvm::BlockGeneratorResult>();
}
