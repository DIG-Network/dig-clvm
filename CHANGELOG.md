# Changelog

All notable changes to this project are documented here.
This project adheres to [Semantic Versioning](https://semver.org) and
[Conventional Commits](https://www.conventionalcommits.org).

## [0.2.0] - 2026-08-05

### Features
- Bump dig-constants to 0.9 (fixes all-zeros DIG_MAINNET genesis challenge) (#4)

## [0.1.4] - 2026-07-23

### Documentation
- **spec:** Rewrite SPEC.md as normative §4.2 specification (#3)

## [0.1.3] - 2026-07-19

### Documentation
- **dig-clvm:** Move normative SPEC.md to repo root (§4.2) (#2)

## [0.1.2] - 2026-07-12

### Testing
- Cover BLS signature-validation branches in validate_spend_bundle

### CI
- Gate line coverage at >=80% with cargo-llvm-cov- Enforce version increment in PRs (package.json / Cargo.toml)- Enforce Conventional Commits with commitlint on PRs- Enforce Conventional Commits with commitlint on PRs- Release automation (git-cliff changelog + tag on merge); publish is manual workflow_dispatch (#230)- Re-arm crates.io auto-publish on version tag (token in org secrets; auto-publish-everything #230)- Add flaky-test management (#489) (#1)

### Chores
- **changelog:** Add git-cliff config for Conventional-Commit changelog

## [0.1.1] - 2026-04-13

### Features
- **constants:** Verify CON-001 — separate dig-constants crate- **constants:** Verify CON-002 — NetworkConstants type- **constants:** Verify CON-003, CON-004, CON-006, CON-007; partial CON-005- **api:** Implement API-008 — scaffold crate module structure- **api:** Verify API-002, API-003 — no async/IO deps, individual SDK crates- **validation:** Implement validate_spend_bundle — Phase 1 complete- **validation:** Implement VAL-012 — BLS signature verification- **validation:** Verify VAL-013 — MEMPOOL_MODE stricter rules- **cache:** Implement BLS-001 through BLS-005 — BlsCache integration- **block:** Implement build_block_generator + validate_block — Phase 3 complete- **api:** Verify API-001, API-004, API-007 — Phase 4 complete- **parity:** Verify PAR-001 through PAR-011 — Phase 5 complete- **constants:** Implement CON-005 — AGG_SIG additional data derivation- Re-export all chia-consensus opcode constants at top level

### Bug Fixes
- **ci:** Remove dig-constants checkout and dep swap — now uses crates.io

### Documentation
- Mark all 55 requirements verified — 154 tests passing- Add comprehensive README with full public API reference

### Testing
- Add 154 comprehensive tests across 55 requirement files

### Chores
- Set up GitNexus, Repomix, and SocratiCode tooling- Reset all 55 requirements to gap — tests required before completion- Add crates.io publish workflow- Switch dig-constants to crates.io version 0.1.0- Cargo fmt + fix all clippy warnings

### Initial
- Scaffold dig-clvm crate with spec, requirements, and prompt system


