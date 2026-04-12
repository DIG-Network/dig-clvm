//! Spend bundle validation.

use chia_bls::BlsCache;
use chia_protocol::SpendBundle;

use super::config::ValidationConfig;
use super::context::ValidationContext;
use super::error::ValidationError;
use super::result::SpendResult;

/// Validate a spend bundle against L2 consensus rules.
///
/// Internally calls `chia_consensus::run_spendbundle()` for CLVM execution
/// and condition extraction, then applies L2-specific validation rules.
pub fn validate_spend_bundle(
    _bundle: &SpendBundle,
    _context: &ValidationContext,
    _config: &ValidationConfig,
    _bls_cache: Option<&mut BlsCache>,
) -> Result<SpendResult, ValidationError> {
    todo!("VAL-001 through VAL-015")
}
