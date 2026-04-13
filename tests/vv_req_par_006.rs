//! REQUIREMENT: PAR-006 — Costs from chia-consensus (not reimplemented)

mod common;

#[test]
fn par_006_costs_from_chia_consensus() {
    // Compile test: cost constants come from chia-consensus, not reimplemented
    use chia_consensus::opcodes::{CREATE_COIN_COST, AGG_SIG_COST};
    let _cc = CREATE_COIN_COST;
    let _as = AGG_SIG_COST;
}

#[test]
fn par_006_cost_type_is_clvmr() {
    // Cost type comes from clvmr
    use clvmr::cost::Cost;
    let _c: Cost = 0;
}

#[test]
fn par_006_l2_costs_derived_from_l1() {
    // L2 costs are derived from L1 constants, not independently defined
    use dig_clvm::consensus::config::{L1_MAX_COST_PER_SPEND, L2_MAX_COST_PER_BLOCK};
    // L2 block limit = 50 * L1 per-spend limit
    assert_eq!(L2_MAX_COST_PER_BLOCK, L1_MAX_COST_PER_SPEND * 50);
}
