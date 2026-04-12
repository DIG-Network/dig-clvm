# dig-clvm Requirements

This directory contains the formal requirements for the dig-clvm crate,
following the same two-tier requirements structure as chia-l2-consensus
with full traceability.

## Quick Links

- [SCHEMA.md](SCHEMA.md) — Data model and conventions
- [REQUIREMENTS_REGISTRY.yaml](REQUIREMENTS_REGISTRY.yaml) — Central domain registry
- [domains/](domains/) — All requirement domains

## Structure

```
requirements/
├── README.md                    # This file
├── SCHEMA.md                    # Data model and conventions
├── REQUIREMENTS_REGISTRY.yaml   # Central registry
└── domains/
    ├── validation/              # VAL-* Spend bundle validation
    ├── block_generator/         # BLK-* Block generator construction + validation
    ├── bls_cache/               # BLS-* Signature caching
    ├── parity/                  # PAR-* Chia L1 parity
    ├── crate_api/               # API-* Public API, types, re-exports
    └── constants/               # CON-* DIG network constants
```

## Three-Document Pattern

Each domain contains:

| File | Purpose |
|------|---------|
| `NORMATIVE.md` | Authoritative requirement statements (MUST/SHOULD/MAY) |
| `VERIFICATION.md` | QA approach and status per requirement |
| `TRACKING.yaml` | Machine-readable status, tests, and notes |

## Specification Files

Individual requirement specifications are in each domain's `specs/` subdirectory:

```
domains/
├── validation/specs/       # VAL-001.md through VAL-015.md
├── block_generator/specs/  # BLK-001.md through BLK-009.md
├── bls_cache/specs/        # BLS-001.md through BLS-005.md
├── parity/specs/           # PAR-001.md through PAR-011.md
├── crate_api/specs/        # API-001.md through API-008.md
└── constants/specs/        # CON-001.md through CON-007.md
```

## Reference Document

All requirements are derived from:
- [SPEC.md](../resources/SPEC.md) — dig-clvm specification

## Requirement Count

| Domain | Prefix | Count |
|--------|--------|-------|
| Spend Validation | VAL | 15 |
| Block Generator | BLK | 9 |
| BLS Cache | BLS | 5 |
| Chia L1 Parity | PAR | 11 |
| Crate API | API | 8 |
| Network Constants | CON | 7 |
| **Total** | | **55** |
