//! dig-clvm: DIG L2 CLVM Consensus Engine
//!
//! Validates spend bundles and computes coin additions/removals for DIG validators.
//! Built as a thin orchestration layer on top of the Chia crate ecosystem.

// ── CLVM Runtime ──
pub use clvm_traits::{self, FromClvm, ToClvm};
pub use clvm_utils::{self, tree_hash, CurriedProgram, ToTreeHash, TreeHash};
pub use clvmr::{self, cost::Cost, Allocator, NodePtr};

// ── Chia Protocol Types ──
pub use chia_protocol::{self, Bytes, Bytes32, Coin, CoinSpend, CoinState, Program, SpendBundle};

// ── Consensus Engine ──
pub use chia_consensus::{self, consensus_constants::ConsensusConstants, opcodes};
// Re-export all opcode constants and costs at the top level for convenience.
// Includes: AGG_SIG_ME, CREATE_COIN, ASSERT_HEIGHT_ABSOLUTE, AGG_SIG_COST,
// CREATE_COIN_COST, ConditionOpcode, etc.
pub use chia_consensus::opcodes::*;

// ── BLS Signatures ──
pub use chia_bls::{self, aggregate_verify, BlsCache, PublicKey, SecretKey, Signature};

// ── SDK Types & Conditions ──
pub use chia_sdk_types::{self, Condition, Conditions, Mod};

// ── DIG Network Constants ──
pub use dig_constants::{self, NetworkConstants, DIG_MAINNET, DIG_TESTNET};

// ── Spend Construction ──
pub use chia_sdk_driver::{
    self, DriverError, Layer, Puzzle, Spend, SpendContext, SpendWithConditions,
};

// ── Coin State ──
pub use chia_sdk_coinset::{self, CoinRecord};

// ── Puzzles ──
pub use chia_puzzles;

// ── dig-clvm's own consensus orchestration ──
pub mod consensus;

pub use consensus::{
    build_block_generator, validate_block, validate_spend_bundle, BlockGeneratorResult,
    SpendResult, ValidationConfig, ValidationContext, ValidationError, L1_MAX_COST_PER_SPEND,
    L2_MAX_COST_PER_BLOCK,
};
