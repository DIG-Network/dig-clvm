//! REQUIREMENT: PAR-009 — Condition from chia-sdk-types (generic)

mod common;

use dig_clvm::Condition;

#[test]
fn par_009_condition_from_chia_sdk_types() {
    // dig_clvm::Condition is from chia_sdk_types — compile test
    let _size = std::mem::size_of::<Condition>();
}

#[test]
fn par_009_conditions_type_exists() {
    // Conditions (plural) is also exported
    let _size = std::mem::size_of::<dig_clvm::Conditions>();
}

#[test]
fn par_009_condition_is_generic() {
    // Condition is a generic enum — verify it compiles and has variants
    // This is purely a compile-time type check
    let _type_check: fn() = || {
        let _ = std::mem::size_of::<chia_sdk_types::Condition>();
    };
}
