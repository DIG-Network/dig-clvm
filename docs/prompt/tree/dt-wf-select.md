# dt-wf-select — Workflow: Select Next Requirement

## Procedure

### Step 1: Sync

```bash
git pull origin main
```

### Step 2: Open IMPLEMENTATION_ORDER.md

```
docs/requirements/IMPLEMENTATION_ORDER.md
```

### Step 3: Choose the first unchecked item

- Scan from top to bottom
- Choose the first `- [ ]` that matches your current focus or an explicit task
- **Skip every `[x]`** — those are done

### Step 4: Read the requirement

1. Follow the NORMATIVE link in IMPLEMENTATION_ORDER to `domains/{domain}/NORMATIVE.md`
2. Read the requirement statement (look for the requirement ID heading)
3. Open the dedicated spec at `domains/{domain}/specs/PREFIX-NNN.md`
4. Read the full specification

### Step 5: Confirm selection

Before proceeding, verify:
- [ ] The requirement is `[ ]` (unchecked) in IMPLEMENTATION_ORDER
- [ ] You have read NORMATIVE.md for the requirement ID
- [ ] You have read the dedicated spec
- [ ] You understand what MUST be implemented

## Phase Priority

Work phases in order. Do not skip ahead.

| Phase | Focus | Requirements |
|-------|-------|-------------|
| Phase 0 | Setup | Project structure, dependencies, configuration |
| Phase 1 | Core Validation | VAL-* (validate_spend_bundle and supporting functions) |
| Phase 2 | Block Generator | BLK-* (block generator building and validation) |
| Phase 3 | Cache | CAC-* (BLS signature cache integration) |
| Phase 4 | Parity Tests | PAR-* (prove identical behavior to Chia L1) |

---

Navigation: Prev < [dt-git.md](dt-git.md) | Next > [dt-wf-gather-context.md](dt-wf-gather-context.md)
