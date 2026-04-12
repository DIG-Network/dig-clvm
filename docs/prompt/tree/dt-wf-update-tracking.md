# dt-wf-update-tracking — Workflow: Update Tracking Artifacts

After validation passes, update all three tracking artifacts for the completed requirement.

## 1. TRACKING.yaml

File: `docs/requirements/domains/{domain}/TRACKING.yaml`

Update the entry for the completed requirement:

```yaml
PREFIX-NNN:
  status: implemented    # was: gap
  tests:
    - vv_req_prefix_nnn  # test file name (without .rs)
  notes: "Brief description of what was implemented"
```

### Status Values

| Status | Meaning |
|--------|---------|
| `gap` | Not started |
| `partial` | Some work done, not complete |
| `implemented` | Code written, tests pass |
| `verified` | Tests pass AND cargo clippy/fmt clean |

Set `implemented` when tests pass. Set `verified` when the full validation suite (cargo test + clippy + fmt) is clean.

## 2. VERIFICATION.md

File: `docs/requirements/domains/{domain}/VERIFICATION.md`

Update the row for the completed requirement:

| Before | After |
|--------|-------|
| Status: --- | Status: ✅ (verified) or ⚠️ (partial) |
| Verification Approach: TBD | Verification Approach: describe what was tested |

Example update:
```markdown
| VAL-001 | ✅ | Simulator test: valid standard spend succeeds; invalid signature returns error |
```

## 3. IMPLEMENTATION_ORDER.md

File: `docs/requirements/IMPLEMENTATION_ORDER.md`

Check off the completed requirement:

```markdown
# Before
- [ ] VAL-001 — validate_spend_bundle entry point

# After
- [x] VAL-001 — validate_spend_bundle entry point
```

Only mark `[x]` when the requirement is fully verified on main (tests pass, clippy clean, fmt clean).

## Tracking Update Checklist

Before proceeding to commit, verify:

- [ ] TRACKING.yaml updated with correct status, test names, and notes
- [ ] VERIFICATION.md row updated with status and verification approach
- [ ] IMPLEMENTATION_ORDER.md checkbox changed from `[ ]` to `[x]`
- [ ] No other requirement's tracking was accidentally modified

---

Navigation: Prev < [dt-wf-validate.md](dt-wf-validate.md) | Next > [dt-wf-commit.md](dt-wf-commit.md)
