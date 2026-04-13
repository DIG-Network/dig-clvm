//! REQUIREMENT: API-001 — Public API surface re-exports
//!
//! Verifies that every type listed in the public API is importable through
//! the dig_clvm facade crate. Compilation alone is the primary assertion.

use dig_clvm::{
    Allocator, Cost, TreeHash,
    Coin, Program, Bytes32,
    BlsCache, PublicKey, SecretKey, Signature,
    SpendContext, ConsensusConstants,
    DIG_MAINNET,
};

// Prove these are importable even if unused in runtime tests.
#[allow(unused_imports)]
use dig_clvm::{tree_hash, CoinSpend, SpendBundle};

#[test]
fn api_001_clvm_runtime_types_importable() {
    let _allocator = Allocator::new();
    let _cost: Cost = 0;
    let _tree_hash: TreeHash = TreeHash::new([0u8; 32]);
}

#[test]
fn api_001_protocol_types_importable() {
    // Prove Coin, CoinSpend, SpendBundle, Program, Bytes32 are available.
    let _b32 = Bytes32::default();
    let _coin = Coin::new(Bytes32::default(), Bytes32::default(), 0);
    let _program = Program::default();
}

#[test]
fn api_001_bls_types_importable() {
    let _cache = BlsCache::default();
    let _pk = PublicKey::default();
    let _sk = SecretKey::from_seed(&[0u8; 32]);
    let _sig = Signature::default();
}

#[test]
fn api_001_sdk_and_consensus_types_importable() {
    // SpendContext from chia-sdk-driver
    let _spend_ctx = SpendContext::new();

    // ConsensusConstants accessible via DIG_MAINNET
    let _cc: &ConsensusConstants = DIG_MAINNET.consensus();
}
