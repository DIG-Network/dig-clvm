# Implementation Order

Phased checklist for dig-clvm requirements. Work top-to-bottom within each phase.
After completing a requirement: update TRACKING.yaml, VERIFICATION.md, and check off here.

---

## Phase 0: Foundation

- [x] CON-001 — Separate dig-constants crate
- [x] CON-002 — NetworkConstants type
- [x] CON-003 — DIG_MAINNET and DIG_TESTNET
- [x] CON-004 — Fork heights at zero
- [ ] CON-005 — AGG_SIG additional data derivation (partial: placeholders until genesis finalized)
- [x] CON-006 — Neutral PoS/VDF fields
- [x] CON-007 — Minimal dependencies
- [x] API-008 — Module structure (scaffold src/consensus/)
- [x] API-002 — No async/IO dependencies
- [x] API-003 — Individual chia-sdk-* crates

## Phase 1: Core Validation

- [x] VAL-001 — validate_spend_bundle entry point
- [x] VAL-002 — Delegates to chia-consensus run_spendbundle
- [x] API-005 — ValidationError variants
- [x] API-006 — thiserror implementation
- [x] VAL-003 — Reject duplicate spends
- [x] VAL-004 — Reject missing coins
- [x] VAL-005 — Reject already-spent coins
- [x] VAL-006 — Reject puzzle hash mismatch
- [x] VAL-007 — Cost limit enforcement
- [x] VAL-008 — Default cost constants
- [x] VAL-009 — Condition validation delegated
- [x] VAL-011 — Conservation check
- [x] VAL-014 — SpendResult output structure
- [x] VAL-015 — No full UTXO set in memory
- [x] VAL-010 — Ephemeral coin support

## Phase 2: Signatures & Flags

- [x] VAL-012 — BLS signature verification
- [x] VAL-013 — MEMPOOL_MODE stricter rules
- [x] BLS-001 — Optional BlsCache parameter
- [x] BLS-002 — None cache behavior
- [x] BLS-004 — Uses chia-bls::BlsCache directly
- [x] BLS-003 — Mempool-to-block cache reuse
- [x] BLS-005 — Cache correctness invariant

## Phase 3: Block Generator

- [x] BLK-001 — build_block_generator entry point
- [x] BLK-002 — Cost-aware bundle iteration
- [x] BLK-003 — Uses solution_generator_backrefs
- [x] BLK-004 — BlockGeneratorResult structure
- [x] BLK-005 — Aggregated signature
- [x] BLK-006 — validate_block entry point
- [x] BLK-007 — Delegates to run_block_generator2
- [x] BLK-008 — Same checks as validate_spend_bundle
- [x] BLK-009 — Round-trip consistency

## Phase 4: Re-exports & API Surface

- [ ] API-001 — Re-exports from upstream crates
- [ ] API-004 — No storage access
- [ ] API-007 — No reimplementation

## Phase 5: Parity Verification

- [ ] PAR-001 — clvmr with ChiaDialect
- [ ] PAR-002 — chia-consensus condition parsing
- [ ] PAR-003 — clvm_utils tree hash
- [ ] PAR-004 — BLS domain separation
- [ ] PAR-005 — chia-bls aggregate_verify
- [ ] PAR-006 — Per-condition costs
- [ ] PAR-007 — Per-spend cost limit
- [ ] PAR-008 — Protocol type re-exports
- [ ] PAR-009 — Condition type re-exports
- [ ] PAR-010 — Block generator parity
- [ ] PAR-011 — Mempool flag parity
