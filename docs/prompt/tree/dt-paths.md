# dt-paths — Path Conventions

## Project Layout

```
dig-clvm/
├── docs/
│   ├── resources/
│   │   └── SPEC.md                          # Master specification
│   ├── requirements/
│   │   ├── SCHEMA.md                        # Data model and conventions
│   │   ├── README.md                        # Requirements system overview
│   │   ├── REQUIREMENTS_REGISTRY.yaml       # Domain registry
│   │   ├── IMPLEMENTATION_ORDER.md          # Phased checklist
│   │   └── domains/{domain}/               # Per-domain artifacts
│   │       ├── NORMATIVE.md                 # Authoritative requirement statements
│   │       ├── VERIFICATION.md              # QA approach and status
│   │       ├── TRACKING.yaml               # Machine-readable status
│   │       └── specs/                       # Per-requirement specs
│   │           └── PREFIX-NNN.md            # Detailed specification
│   └── prompt/                              # This workflow system
│       ├── prompt.md
│       ├── start.md
│       ├── tree/                            # Decision tree files (you are here)
│       └── tools/                           # Tool documentation
├── src/
│   ├── lib.rs                               # Re-exports ONLY
│   └── consensus/
│       ├── validate.rs                      # validate_spend_bundle() orchestration
│       ├── block.rs                         # build_block_generator(), validate_block()
│       ├── context.rs                       # ValidationContext struct
│       ├── config.rs                        # ValidationConfig, cost constants
│       ├── result.rs                        # SpendResult, BlockGeneratorResult
│       ├── cache.rs                         # BlsCache integration helpers
│       └── error.rs                         # ValidationError enum
├── tests/
│   ├── validation_tests.rs
│   ├── block_tests.rs
│   ├── parity_tests.rs
│   └── vv_req_{prefix}_{nnn}.rs            # Per-requirement VV tests
├── Cargo.toml
└── .repomix/                                # Ephemeral context packs (gitignored)
```

## Sibling Crate

```
../dig-constants/                            # Network parameters (separate crate)
```

## Key Paths to Remember

| Artifact | Path |
|----------|------|
| Master spec | `docs/resources/SPEC.md` |
| Implementation order | `docs/requirements/IMPLEMENTATION_ORDER.md` |
| Domain requirements | `docs/requirements/domains/{domain}/NORMATIVE.md` |
| Requirement spec | `docs/requirements/domains/{domain}/specs/PREFIX-NNN.md` |
| Main entry | `src/lib.rs` |
| Core logic | `src/consensus/*.rs` |
| Tests | `tests/*.rs` |

---

Navigation: Next > [dt-role.md](dt-role.md)
