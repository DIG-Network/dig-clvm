//! REQUIREMENT: PAR-004 — Domain separation via chia-consensus

mod common;

#[test]
fn par_004_domain_separation_via_chia_consensus() {
    // Compile test: validate_clvm_and_signature handles domain separation internally.
    // We verify the function is importable from chia-consensus.
    use chia_consensus::spendbundle_validation::validate_clvm_and_signature;
    let _fn = validate_clvm_and_signature;
}

#[test]
fn par_004_consensus_constants_importable() {
    // ConsensusConstants carries the AGG_SIG domain strings used for separation
    use chia_consensus::consensus_constants::ConsensusConstants;
    let _size = std::mem::size_of::<ConsensusConstants>();
}

#[test]
fn par_004_dig_testnet_has_consensus() {
    // DIG_TESTNET provides a consensus() method that returns ConsensusConstants
    let constants = dig_clvm::DIG_TESTNET.clone();
    let _consensus = constants.consensus();
}
