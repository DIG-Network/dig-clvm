# dt-wf-commit — Workflow: Commit, Push, Loop

One requirement per commit. Include code, tests, and tracking updates together.

## Procedure

### Step 1: Update GitNexus

```bash
npx gitnexus analyze
```

Keep the knowledge graph current with your changes.

### Step 2: Stage Files

Stage exactly the files for this requirement:

```bash
git add src/consensus/validate.rs \
        tests/vv_req_val_001.rs \
        docs/requirements/domains/validation/TRACKING.yaml \
        docs/requirements/domains/validation/VERIFICATION.md \
        docs/requirements/IMPLEMENTATION_ORDER.md
```

**Include:** One requirement's implementation + tests + tracking updates.
**Exclude:** Unrelated changes, other requirement IDs, `.repomix/` files.

### Step 3: Commit

```bash
git commit -m "feat(validation): implement VAL-001 validate_spend_bundle entry point"
```

#### Commit Message Format

```
type(scope): imperative subject
```

| Type | When |
|------|------|
| `feat` | New functionality (implementing a requirement) |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `chore` | Build, deps, tooling |
| `refactor` | Restructuring without behavior change |
| `test` | Test-only changes |

| Scope | Maps to |
|-------|---------|
| `validation` | validate.rs, context.rs, config.rs, result.rs, error.rs |
| `block` | block.rs |
| `cache` | cache.rs |
| `parity` | Parity tests, PAR-* requirements |
| `api` | lib.rs re-exports |
| `constants` | dig-constants crate |
| `deps` | Cargo.toml |

### Step 4: Push

```bash
git push origin main
```

Always push after commit. Reasons:
- Backup against local machine failure
- CI pipeline starts immediately
- Team has visibility into progress

## What to Avoid

- **Mixing requirement IDs** — one commit, one requirement
- **Incomplete implementations** — do not commit partial work
- **Batching multiple requirements** — defeats traceability
- **Forgetting tracking updates** — code + tests + tracking = one atomic unit
- **Committing `.repomix/` files** — these are gitignored ephemeral context

## Loop

After pushing, return to the beginning of the workflow cycle:

**Next requirement --> [dt-wf-select.md](dt-wf-select.md)**

---

Navigation: Prev < [dt-wf-update-tracking.md](dt-wf-update-tracking.md) | Loop > [dt-wf-select.md](dt-wf-select.md)
