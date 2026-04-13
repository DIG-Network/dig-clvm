# Implementation Order

Phased checklist for dig-clvm requirements. Work top-to-bottom within each phase.
After completing a requirement: write tests, verify they pass, update TRACKING.yaml, VERIFICATION.md, and check off here.

**A requirement is NOT complete until comprehensive tests verify it.**

---

## Phase 0: Foundation

- [ ] CON-001 — Separate dig-constants crate
- [ ] CON-002 — NetworkConstants type
- [ ] CON-003 — DIG_MAINNET and DIG_TESTNET
- [ ] CON-004 — Fork heights at zero
- [ ] CON-005 — AGG_SIG additional data derivation
- [ ] CON-006 — Neutral PoS/VDF fields
- [ ] CON-007 — Minimal dependencies
- [ ] API-008 — Module structure (scaffold src/consensus/)
- [ ] API-002 — No async/IO dependencies
- [ ] API-003 — Individual chia-sdk-* crates

## Phase 1: Core Validation

- [ ] VAL-001 — validate_spend_bundle entry point
- [ ] VAL-002 — Delegates to chia-consensus run_spendbundle
- [ ] API-005 — ValidationError variants
- [ ] API-006 — thiserror implementation
- [ ] VAL-003 — Reject duplicate spends
- [ ] VAL-004 — Reject missing coins
- [ ] VAL-005 — Reject already-spent coins
- [ ] VAL-006 — Reject puzzle hash mismatch
- [ ] VAL-007 — Cost limit enforcement
- [ ] VAL-008 — Default cost constants
- [ ] VAL-009 — Condition validation delegated
- [ ] VAL-011 — Conservation check
- [ ] VAL-014 — SpendResult output structure
- [ ] VAL-015 — No full UTXO set in memory
- [ ] VAL-010 — Ephemeral coin support

## Phase 2: Signatures & Flags

- [ ] VAL-012 — BLS signature verification
- [ ] VAL-013 — MEMPOOL_MODE stricter rules
- [ ] BLS-001 — Optional BlsCache parameter
- [ ] BLS-002 — None cache behavior
- [ ] BLS-004 — Uses chia-bls::BlsCache directly
- [ ] BLS-003 — Mempool-to-block cache reuse
- [ ] BLS-005 — Cache correctness invariant

## Phase 3: Block Generator

- [ ] BLK-001 — build_block_generator entry point
- [ ] BLK-002 — Cost-aware bundle iteration
- [ ] BLK-003 — Uses solution_generator_backrefs
- [ ] BLK-004 — BlockGeneratorResult structure
- [ ] BLK-005 — Aggregated signature
- [ ] BLK-006 — validate_block entry point
- [ ] BLK-007 — Delegates to run_block_generator2
- [ ] BLK-008 — Same checks as validate_spend_bundle
- [ ] BLK-009 — Round-trip consistency

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
