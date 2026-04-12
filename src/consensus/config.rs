//! Validation configuration and cost constants.

use clvmr::cost::Cost;

/// Maximum CLVM cost per spend (matches Chia L1).
pub const L1_MAX_COST_PER_SPEND: Cost = 11_000_000_000;

/// Maximum CLVM cost per block (DIG L2: 50x L1 per-spend).
pub const L2_MAX_COST_PER_BLOCK: Cost = 550_000_000_000;

/// L2-specific validation parameters.
pub struct ValidationConfig {
    /// Maximum CLVM cost per individual spend.
    pub max_cost_per_spend: Cost,
    /// Maximum total CLVM cost per block.
    pub max_cost_per_block: Cost,
    /// Execution flags from chia-consensus (MEMPOOL_MODE, DONT_VALIDATE_SIGNATURE, etc.).
    pub flags: u32,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_cost_per_spend: L1_MAX_COST_PER_SPEND,
            max_cost_per_block: L2_MAX_COST_PER_BLOCK,
            flags: 0,
        }
    }
}
