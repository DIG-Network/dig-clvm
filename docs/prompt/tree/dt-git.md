# dt-git — Git Workflow

## Sync Before Work

```bash
git fetch origin && git pull origin main
```

Always sync before selecting work. Treat `[x]` items as done after pull.

## Commit Format

```
type(scope): imperative subject
```

### Types

| Type | When |
|------|------|
| `feat` | New functionality (implementing a requirement) |
| `fix` | Bug fix |
| `docs` | Documentation only (tracking updates, spec corrections) |
| `chore` | Build, deps, tooling |
| `refactor` | Code restructuring without behavior change |
| `test` | Test-only changes |

### Scopes

| Scope | Maps to |
|-------|---------|
| `validation` | `src/consensus/validate.rs`, `context.rs`, `config.rs`, `result.rs`, `error.rs` |
| `block` | `src/consensus/block.rs` |
| `cache` | `src/consensus/cache.rs` |
| `parity` | Parity tests (`tests/parity_tests.rs`, PAR-* requirements) |
| `api` | `src/lib.rs` re-exports |
| `constants` | `../dig-constants/` |
| `deps` | `Cargo.toml` dependency changes |

### Examples

```
feat(validation): implement VAL-001 validate_spend_bundle entry point
fix(block): correct cost accumulation in BLK-003
test(parity): add PAR-002 signature aggregation parity test
docs(validation): update TRACKING for VAL-001 through VAL-005
chore(deps): bump chia-consensus to 0.18.0
refactor(validation): extract condition grouping into helper
```

## Push

```bash
git push origin main
```

Always push after commit. Reasons: backup, CI visibility, team sync.

## Conflict Resolution

```bash
git stash
git pull origin main
git stash pop
# Resolve conflicts manually
git add <resolved files>
git commit -m "fix(scope): resolve merge conflict in ..."
git push origin main
```

---

Navigation: Prev < [dt-tools.md](dt-tools.md) | Next > [dt-wf-select.md](dt-wf-select.md)
