//! REQUIREMENT: API-006 — ValidationError implements Error + Display
//!
//! Verifies that ValidationError implements std::error::Error and
//! std::fmt::Display, and that format!() produces meaningful strings.

use dig_clvm::{Bytes32, ValidationError};

#[test]
fn api_006_display_produces_nonempty_strings() {
    let coin_id = Bytes32::default();

    let errors: Vec<ValidationError> = vec![
        ValidationError::Clvm("execution failed".to_string()),
        ValidationError::CoinNotFound(coin_id),
        ValidationError::AlreadySpent(coin_id),
        ValidationError::DoubleSpend(coin_id),
        ValidationError::PuzzleHashMismatch(coin_id),
        ValidationError::SignatureFailed,
        ValidationError::ConservationViolation { input: 100, output: 200 },
        ValidationError::CostExceeded { limit: 1000, consumed: 2000 },
    ];

    for err in &errors {
        let display_str = format!("{}", err);
        assert!(
            !display_str.is_empty(),
            "Display for {:?} must produce a non-empty string",
            err
        );
    }
}

#[test]
fn api_006_display_contains_expected_fragments() {
    let err = ValidationError::Clvm("boom".to_string());
    let s = format!("{}", err);
    assert!(s.contains("boom"), "Display for Clvm should contain the message");

    let err = ValidationError::SignatureFailed;
    let s = format!("{}", err);
    assert!(
        s.to_lowercase().contains("signature"),
        "Display for SignatureFailed should mention 'signature'"
    );

    let err = ValidationError::ConservationViolation { input: 100, output: 200 };
    let s = format!("{}", err);
    assert!(s.contains("100"), "Display should contain input value");
    assert!(s.contains("200"), "Display should contain output value");

    let err = ValidationError::CostExceeded { limit: 10, consumed: 20 };
    let s = format!("{}", err);
    assert!(s.contains("10"), "Display should contain limit");
    assert!(s.contains("20"), "Display should contain consumed");
}

#[test]
fn api_006_implements_std_error_trait() {
    let err = ValidationError::Clvm("test".to_string());
    // This line only compiles if ValidationError: std::error::Error.
    let _dyn_err: &dyn std::error::Error = &err;
}
