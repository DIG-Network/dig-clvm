# Requirements Schema

This document defines the data model and conventions for all requirements in the
dig-clvm project.

---

## Three-Document Pattern

Each domain has exactly three files in `docs/requirements/domains/{domain}/`:

| File | Purpose |
|------|---------|
| `NORMATIVE.md` | Authoritative requirement statements with MUST/SHOULD/MAY keywords |
| `VERIFICATION.md` | QA approach and verification status per requirement |
| `TRACKING.yaml` | Machine-readable status, test references, and implementation notes |

Each requirement also has a dedicated specification file in
`docs/requirements/domains/{domain}/specs/{PREFIX-NNN}.md`.

---

## Requirement ID Format

**Pattern:** `{PREFIX}-{NNN}`

- **PREFIX**: 2-4 letter domain identifier (uppercase)
- **NNN**: Zero-padded numeric ID starting at 001

| Domain | Directory | Prefix | Description |
|--------|-----------|--------|-------------|
| Spend Validation | `validation/` | `VAL` | Spend bundle validation pipeline |
| Block Generator | `block_generator/` | `BLK` | Block generator construction and validation |
| BLS Cache | `bls_cache/` | `BLS` | Signature caching across mempool and block |
| Chia L1 Parity | `parity/` | `PAR` | 1:1 behavioral parity with Chia L1 |
| Crate API | `crate_api/` | `API` | Public API surface, types, re-exports |
| Network Constants | `constants/` | `CON` | DIG network parameters via dig-constants |

**Immutability:** Requirement IDs are permanent. Deprecate requirements rather
than renumbering.

---

## Requirement Keywords

Per RFC 2119:

| Keyword | Meaning | Impact |
|---------|---------|--------|
| **MUST** | Absolute requirement | Blocks "done" status if not met |
| **MUST NOT** | Absolute prohibition | Blocks "done" status if violated |
| **SHOULD** | Expected behavior; may be deferred with rationale | Phase 2+ polish items |
| **SHOULD NOT** | Discouraged behavior | Phase 2+ polish items |
| **MAY** | Optional, nice-to-have | Stretch goals |

---

## Status Values

| Status | Description |
|--------|-------------|
| `gap` | Not implemented |
| `partial` | Implementation in progress or incomplete |
| `implemented` | Code complete, awaiting verification |
| `verified` | Implemented and verified per VERIFICATION.md |
| `deferred` | Explicitly postponed with rationale |

---

## TRACKING.yaml Item Schema

```yaml
- id: PREFIX-NNN           # Requirement ID (required)
  section: "Section Name"  # Logical grouping within domain (required)
  summary: "Brief title"   # Human-readable description (required)
  status: gap              # One of: gap, partial, implemented, verified, deferred
  spec_ref: "docs/requirements/domains/{domain}/specs/{PREFIX-NNN}.md"
  tests: []                # Array of test names or ["manual"]
  notes: ""                # Implementation notes, blockers, or evidence
```

---

## Testing Requirements

All dig-clvm requirements MUST be tested using:

### 1. chia-sdk-test Simulator Tests (MUST)

All validation paths MUST be tested using the `chia-sdk-test::Simulator`:

1. **Create** a `Simulator` instance
2. **Mint** test coins with known puzzle hashes
3. **Build** spend bundles with `SpendContext`
4. **Validate** via both `Simulator::new_transaction()` and `dig_clvm::validate_spend_bundle()`
5. **Verify** resulting coin states: additions, removals, fees

The simulator runs full consensus validation including CLVM execution, signature
aggregation, and announcement matching — the same code path as Chia L1.

### 2. Parity Tests (MUST for PAR-* requirements)

Tests MUST demonstrate identical behavior to Chia L1 by:
- Using known puzzle hashes from `chia-puzzles`
- Verifying against Chia L1 test vectors where available
- Round-tripping through both Simulator and `validate_spend_bundle()`

### 3. Required Test Infrastructure

```toml
# Cargo.toml [dev-dependencies]
chia-sdk-test = "0.30"
hex-literal = "0.4"
rand = "0.8"
```

```rust
use chia_sdk_test::{Simulator, BlsPair};
use chia_sdk_driver::SpendContext;
use dig_clvm::{validate_spend_bundle, ValidationContext, ValidationConfig};
use dig_constants::DIG_TESTNET;
```

---

## Master Spec Reference

All requirements trace back to the SPEC:
[SPEC.md](../resources/SPEC.md)
