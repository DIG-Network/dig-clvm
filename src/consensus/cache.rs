//! BLS signature cache management.
//!
//! Wraps `chia-bls::BlsCache` for use across mempool and block validation.
//! The cache is owned by the caller and passed into validation functions.
