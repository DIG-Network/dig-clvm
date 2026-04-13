//! Block generator construction and validation.
//!
//! `build_block_generator()` assembles spend bundles into a compressed block
//! generator using CLVM back-references. `validate_block()` executes a block
//! generator and validates all conditions.

use chia_bls::{BlsCache, Signature};
use chia_consensus::allocator::make_allocator;
use chia_consensus::flags::DONT_VALIDATE_SIGNATURE;
use chia_consensus::owned_conditions::OwnedSpendBundleConditions;
use chia_consensus::run_block_generator::run_block_generator2;
use chia_consensus::solution_generator::solution_generator_backrefs;
use chia_consensus::spendbundle_conditions::run_spendbundle;
use chia_protocol::{Coin, SpendBundle};
use clvmr::cost::Cost;
use clvmr::LIMIT_HEAP;

use super::config::ValidationConfig;
use super::context::ValidationContext;
use super::error::ValidationError;
use super::result::{BlockGeneratorResult, SpendResult};

/// Build a block generator from a set of spend bundles.
///
/// Bundles are added in order until `max_cost` is reached. The caller should
/// pre-sort bundles by fee/cost ratio (highest first) to maximize fee revenue.
///
/// Mirrors L1's `create_block_generator()` at `mempool.py:505`.
pub fn build_block_generator(
    bundles: &[SpendBundle],
    context: &ValidationContext,
    max_cost: Cost,
) -> Result<BlockGeneratorResult, ValidationError> {
    let consensus = context.constants.consensus();
    let mut cost_remaining = max_cost;
    let mut included_spends: Vec<(Coin, Vec<u8>, Vec<u8>)> = Vec::new();
    let mut all_additions: Vec<Coin> = Vec::new();
    let mut all_removals: Vec<Coin> = Vec::new();
    let mut signatures: Vec<Signature> = Vec::new();
    let mut total_cost: Cost = 0;
    let mut bundles_included: usize = 0;

    for bundle in bundles {
        // Run CLVM to compute cost for this bundle (skip sig verification)
        let mut a = make_allocator(LIMIT_HEAP);
        let result = run_spendbundle(
            &mut a,
            bundle,
            cost_remaining,
            context.height,
            DONT_VALIDATE_SIGNATURE,
            consensus,
        );

        let (sbc, _pkm_pairs) = match result {
            Ok(r) => r,
            Err(_) => continue, // Skip bundles that fail (cost exceeded, invalid, etc.)
        };

        let conditions = OwnedSpendBundleConditions::from(&a, sbc);
        let bundle_cost = conditions.cost;

        if bundle_cost > cost_remaining {
            continue; // Skip if this bundle would exceed remaining budget
        }

        // Collect spends for generator construction
        for cs in &bundle.coin_spends {
            included_spends.push((
                cs.coin,
                cs.puzzle_reveal.to_vec(),
                cs.solution.to_vec(),
            ));
            all_removals.push(cs.coin);
        }

        // Extract additions from conditions
        for spend in &conditions.spends {
            let parent_id = spend.coin_id;
            for cc in &spend.create_coin {
                all_additions.push(Coin::new(parent_id, cc.0, cc.1));
            }
        }

        signatures.push(bundle.aggregated_signature.clone());
        total_cost += bundle_cost;
        cost_remaining -= bundle_cost;
        bundles_included += 1;
    }

    // Build the compressed generator using CLVM back-references
    let spends_iter = included_spends
        .iter()
        .map(|(coin, puz, sol)| (*coin, puz.as_slice(), sol.as_slice()));

    let generator = solution_generator_backrefs(spends_iter)
        .map_err(|e| ValidationError::Clvm(format!("solution_generator_backrefs: {}", e)))?;

    // Aggregate all signatures
    let aggregated_signature = if signatures.is_empty() {
        Signature::default()
    } else {
        let mut agg = signatures[0].clone();
        for sig in &signatures[1..] {
            agg += sig;
        }
        agg
    };

    Ok(BlockGeneratorResult {
        generator,
        block_refs: Vec::new(), // No cross-block refs for now
        aggregated_signature,
        additions: all_additions,
        removals: all_removals,
        cost: total_cost,
        bundles_included,
    })
}

/// Validate a block generator and return the combined additions + removals.
///
/// Executes the block-level CLVM program, validates all conditions, and
/// returns the aggregate SpendResult.
///
/// Mirrors L1's `_run_block()` at `multiprocess_validation.py:62`.
pub fn validate_block(
    generator: &[u8],
    generator_refs: &[Vec<u8>],
    context: &ValidationContext,
    config: &ValidationConfig,
    bls_cache: Option<&mut BlsCache>,
    aggregated_signature: &Signature,
) -> Result<SpendResult, ValidationError> {
    let consensus = context.constants.consensus();
    let mut a = make_allocator(LIMIT_HEAP);

    // Execute the block generator via run_block_generator2
    // This runs the CLVM program that produces all spends + conditions
    let sbc = run_block_generator2(
        &mut a,
        generator,
        generator_refs.iter().map(|r| r.as_slice()),
        config.max_cost_per_block,
        config.flags,
        aggregated_signature,
        bls_cache.map(|c| &*c),
        consensus,
    )
    .map_err(|e| ValidationError::Clvm(format!("{:?}", e)))?;

    let conditions = OwnedSpendBundleConditions::from(&a, sbc);

    // Cost enforcement
    if conditions.cost > config.max_cost_per_block {
        return Err(ValidationError::CostExceeded {
            limit: config.max_cost_per_block,
            consumed: conditions.cost,
        });
    }

    // Extract additions from conditions
    let additions: Vec<Coin> = conditions
        .spends
        .iter()
        .flat_map(|spend| {
            let parent_id = spend.coin_id;
            spend
                .create_coin
                .iter()
                .map(move |cc| Coin::new(parent_id, cc.0, cc.1))
        })
        .collect();

    // Extract removals from conditions (spent coin IDs)
    let removals: Vec<Coin> = conditions
        .spends
        .iter()
        .map(|spend| {
            Coin::new(
                spend.parent_id,
                spend.puzzle_hash,
                spend.coin_amount,
            )
        })
        .collect();

    // Conservation check
    let total_input: u64 = removals.iter().map(|c| c.amount).sum();
    let total_output: u64 = additions.iter().map(|c| c.amount).sum();

    if total_input < total_output {
        return Err(ValidationError::ConservationViolation {
            input: total_input,
            output: total_output,
        });
    }

    let fee = total_input - total_output;

    Ok(SpendResult {
        additions,
        removals,
        fee,
        conditions,
    })
}
