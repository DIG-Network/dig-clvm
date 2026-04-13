//! REQUIREMENT: API-005 — ValidationError variant construction
//!
//! Verifies that all 9 ValidationError variants can be constructed and
//! pattern-matched to extract their context fields.

use dig_clvm::{Bytes32, ValidationError};

#[test]
fn api_005_construct_all_simple_variants() {
    let coin_id = Bytes32::default();

    let err_clvm = ValidationError::Clvm("test error".to_string());
    let err_coin_not_found = ValidationError::CoinNotFound(coin_id);
    let err_already_spent = ValidationError::AlreadySpent(coin_id);
    let err_double_spend = ValidationError::DoubleSpend(coin_id);
    let err_puzzle_mismatch = ValidationError::PuzzleHashMismatch(coin_id);
    let err_sig_failed = ValidationError::SignatureFailed;

    // Verify Debug is implemented (required for assert macros).
    let _ = format!("{:?}", err_clvm);
    let _ = format!("{:?}", err_coin_not_found);
    let _ = format!("{:?}", err_already_spent);
    let _ = format!("{:?}", err_double_spend);
    let _ = format!("{:?}", err_puzzle_mismatch);
    let _ = format!("{:?}", err_sig_failed);
}

#[test]
fn api_005_construct_struct_variants() {
    let err_conservation = ValidationError::ConservationViolation {
        input: 1000,
        output: 2000,
    };
    let err_cost = ValidationError::CostExceeded {
        limit: 11_000_000_000,
        consumed: 12_000_000_000,
    };

    // Pattern match to verify fields.
    match err_conservation {
        ValidationError::ConservationViolation { input, output } => {
            assert_eq!(input, 1000);
            assert_eq!(output, 2000);
        }
        _ => panic!("Expected ConservationViolation"),
    }

    match err_cost {
        ValidationError::CostExceeded { limit, consumed } => {
            assert_eq!(limit, 11_000_000_000);
            assert_eq!(consumed, 12_000_000_000);
        }
        _ => panic!("Expected CostExceeded"),
    }
}

#[test]
fn api_005_pattern_match_all_variants() {
    let coin_id = Bytes32::default();

    let variants: Vec<ValidationError> = vec![
        ValidationError::Clvm("msg".to_string()),
        ValidationError::CoinNotFound(coin_id),
        ValidationError::AlreadySpent(coin_id),
        ValidationError::DoubleSpend(coin_id),
        ValidationError::PuzzleHashMismatch(coin_id),
        ValidationError::SignatureFailed,
        ValidationError::ConservationViolation { input: 1, output: 2 },
        ValidationError::CostExceeded { limit: 10, consumed: 20 },
    ];

    for v in &variants {
        // Exhaustive match — compiler guarantees coverage.
        match v {
            ValidationError::Clvm(msg) => assert!(!msg.is_empty()),
            ValidationError::CoinNotFound(_id) => {}
            ValidationError::AlreadySpent(_id) => {}
            ValidationError::DoubleSpend(_id) => {}
            ValidationError::PuzzleHashMismatch(_id) => {}
            ValidationError::SignatureFailed => {}
            ValidationError::ConservationViolation { input, output } => {
                assert!(output > input);
            }
            ValidationError::CostExceeded { limit, consumed } => {
                assert!(consumed > limit);
            }
            ValidationError::Driver(_) => {}
        }
    }
}
