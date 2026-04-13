//! Spend bundle validation.
//!
//! Orchestrates `chia-consensus` with L2-specific structural checks
//! and cost enforcement.

use std::collections::HashSet;

use chia_bls::BlsCache;
use chia_consensus::allocator::make_allocator;
use chia_consensus::flags::DONT_VALIDATE_SIGNATURE;
use chia_consensus::owned_conditions::OwnedSpendBundleConditions;
use chia_consensus::spendbundle_conditions::run_spendbundle;
use chia_consensus::spendbundle_validation::validate_clvm_and_signature;
use chia_protocol::{Bytes, Coin, SpendBundle};
use clvmr::LIMIT_HEAP;

use super::config::ValidationConfig;
use super::context::ValidationContext;
use super::error::ValidationError;
use super::result::SpendResult;

/// Validate a spend bundle against L2 consensus rules.
///
/// 1. Structural checks (duplicates, coin existence)
/// 2. CLVM execution + condition extraction + BLS sig verification
///    (delegated to chia-consensus)
/// 3. Cost enforcement against L2 limits
/// 4. Conservation check (inputs >= outputs + fee)
/// 5. Extract additions/removals into SpendResult
pub fn validate_spend_bundle(
    bundle: &SpendBundle,
    context: &ValidationContext,
    config: &ValidationConfig,
    _bls_cache: Option<&mut BlsCache>,
) -> Result<SpendResult, ValidationError> {
    // ── Step 1: Structural checks ──

    // Check for duplicate spends
    let mut seen_coin_ids = HashSet::new();
    for spend in &bundle.coin_spends {
        let coin_id = spend.coin.coin_id();
        if !seen_coin_ids.insert(coin_id) {
            return Err(ValidationError::DoubleSpend(coin_id));
        }
    }

    // Check all coins exist and are unspent
    for spend in &bundle.coin_spends {
        let coin_id = spend.coin.coin_id();
        match context.coin_records.get(&coin_id) {
            Some(record) => {
                if record.spent {
                    return Err(ValidationError::AlreadySpent(coin_id));
                }
            }
            None => {
                if !context.ephemeral_coins.contains(&coin_id) {
                    return Err(ValidationError::CoinNotFound(coin_id));
                }
            }
        }
    }

    // ── Step 2: CLVM execution + conditions + BLS verification ──
    //
    // Two paths depending on whether signature verification is requested:
    // - With signatures: validate_clvm_and_signature() handles everything
    //   including allocator creation, CLVM execution, condition parsing,
    //   puzzle hash verification, and BLS aggregate verify.
    // - Without signatures (DONT_VALIDATE_SIGNATURE flag): run_spendbundle()
    //   directly, which skips the BLS pairing check.

    let max_cost = config.max_cost_per_block;
    let consensus = context.constants.consensus();

    let conditions: OwnedSpendBundleConditions =
        if config.flags & DONT_VALIDATE_SIGNATURE != 0 {
            // Skip BLS verification — use run_spendbundle with the flag
            let mut a = make_allocator(LIMIT_HEAP);
            let (sbc, _pkm_pairs) = run_spendbundle(
                &mut a,
                bundle,
                max_cost,
                context.height,
                config.flags,
                consensus,
            )
            .map_err(|e| ValidationError::Clvm(format!("{:?}", e)))?;
            OwnedSpendBundleConditions::from(&a, sbc)
        } else if let Some(cache) = _bls_cache {
            // BLS verification WITH cache — run CLVM first, then use
            // BlsCache::aggregate_verify() which checks cache before
            // computing expensive pairings.
            let mut a = make_allocator(LIMIT_HEAP);
            let (sbc, pkm_pairs) = run_spendbundle(
                &mut a,
                bundle,
                max_cost,
                context.height,
                config.flags,
                consensus,
            )
            .map_err(|e| ValidationError::Clvm(format!("{:?}", e)))?;

            // Use BlsCache for aggregate verification — cached pairings
            // are reused, new pairings are stored for future calls.
            let pks_msgs: Vec<(chia_bls::PublicKey, Bytes)> = pkm_pairs;
            let sig_valid = cache.aggregate_verify(
                pks_msgs.iter().map(|(pk, msg)| (pk, msg.as_ref())),
                &bundle.aggregated_signature,
            );
            if !sig_valid {
                return Err(ValidationError::SignatureFailed);
            }

            OwnedSpendBundleConditions::from(&a, sbc)
        } else {
            // Full validation without cache — validate_clvm_and_signature
            // handles everything including BLS aggregate verify.
            let (owned_conditions, _validation_pairs, _duration) =
                validate_clvm_and_signature(
                    bundle,
                    max_cost,
                    consensus,
                    context.height,
                )
                .map_err(|e| ValidationError::Clvm(format!("{:?}", e)))?;
            owned_conditions
        };

    // ── Step 3: Cost enforcement ──
    if conditions.cost > config.max_cost_per_block {
        return Err(ValidationError::CostExceeded {
            limit: config.max_cost_per_block,
            consumed: conditions.cost,
        });
    }

    // ── Step 4: Extract additions and removals ──
    let removals: Vec<Coin> = bundle.coin_spends.iter().map(|cs| cs.coin).collect();

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

    // ── Step 5: Conservation check ──
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
