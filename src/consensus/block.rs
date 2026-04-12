//! Block generator construction and validation.

use chia_bls::BlsCache;
use chia_protocol::SpendBundle;
use clvmr::cost::Cost;

use super::config::ValidationConfig;
use super::context::ValidationContext;
use super::error::ValidationError;
use super::result::{BlockGeneratorResult, SpendResult};

/// Build a block generator from a set of spend bundles.
///
/// Bundles are added in order until `max_cost` is reached.
pub fn build_block_generator(
    _bundles: &[SpendBundle],
    _context: &ValidationContext,
    _max_cost: Cost,
) -> Result<BlockGeneratorResult, ValidationError> {
    todo!("BLK-001 through BLK-005")
}

/// Validate a block generator and return the combined additions + removals.
pub fn validate_block(
    _generator: &[u8],
    _generator_refs: &[Vec<u8>],
    _context: &ValidationContext,
    _config: &ValidationConfig,
    _bls_cache: Option<&mut BlsCache>,
) -> Result<SpendResult, ValidationError> {
    todo!("BLK-006 through BLK-009")
}
