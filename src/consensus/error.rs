//! Validation error types.

use chia_protocol::Bytes32;
use chia_sdk_driver::DriverError;
use clvmr::cost::Cost;

/// Errors produced by spend bundle and block validation.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("CLVM execution failed: {0}")]
    Clvm(String),

    #[error("Coin not found: {0}")]
    CoinNotFound(Bytes32),

    #[error("Coin already spent: {0}")]
    AlreadySpent(Bytes32),

    #[error("Double spend in bundle: {0}")]
    DoubleSpend(Bytes32),

    #[error("Puzzle hash mismatch: {0}")]
    PuzzleHashMismatch(Bytes32),

    #[error("Signature verification failed")]
    SignatureFailed,

    #[error("Conservation violation: input={input}, output={output}")]
    ConservationViolation { input: u64, output: u64 },

    #[error("Cost exceeded: limit={limit}, consumed={consumed}")]
    CostExceeded { limit: Cost, consumed: Cost },

    #[error("Driver error: {0}")]
    Driver(#[from] DriverError),
}
