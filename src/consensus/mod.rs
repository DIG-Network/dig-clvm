//! L2 consensus orchestration.
//!
//! This module contains dig-clvm's own code — the thin orchestration layer
//! that composes chia-consensus, chia-bls, and chia-sdk into a consensus API.

pub mod block;
pub mod cache;
pub mod config;
pub mod context;
pub mod error;
pub mod result;
pub mod validate;

pub use block::{build_block_generator, validate_block};
pub use result::BlockGeneratorResult;
pub use config::{ValidationConfig, L1_MAX_COST_PER_SPEND, L2_MAX_COST_PER_BLOCK};
pub use context::ValidationContext;
pub use error::ValidationError;
pub use result::SpendResult;
pub use validate::validate_spend_bundle;
