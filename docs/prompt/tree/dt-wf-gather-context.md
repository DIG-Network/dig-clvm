# dt-wf-gather-context — Workflow: Gather Context

MUST use all three tools during context gathering. This step ensures you have complete understanding before writing any code.

## Step 0: SocratiCode Search First

Before reading any files, search for related code:

```
codebase_search { query: "requirement topic or key concept" }
```

Then understand the dependency structure of relevant files:

```
codebase_graph_query { filePath: "src/consensus/relevant_file.rs" }
```

Search for related schemas, configs, and API patterns:

```
codebase_context_search { query: "related concept or type name" }
```

## Step 1: Repomix Pack

Pack the scope you are about to work on:

```bash
# Pack the implementation scope
npx repomix@latest src/consensus -o .repomix/pack-consensus.xml

# Pack tests for pattern reference
npx repomix@latest tests -o .repomix/pack-tests.xml

# Pack the domain requirements
npx repomix@latest docs/requirements/domains/<domain> -o .repomix/pack-<domain>-reqs.xml
```

## Step 2: Requirements Trace

Read the full requirements chain for the selected requirement:

1. **NORMATIVE.md** — Read `#{id}` section for the authoritative requirement statement
2. **specs/{id}.md** — Read the detailed specification (acceptance criteria, API surface, error handling)
3. **Source citations** — Follow links to relevant SPEC.md sections
4. **VERIFICATION.md** — Understand the expected test approach
5. **TRACKING.yaml** — Understand current status (gap, partial, implemented, verified)

## Step 3: Cross-References

- Check the `References` section in the dedicated spec for related requirement IDs
- Use SocratiCode to find code that implements those related requirements:
  ```
  codebase_search { query: "related requirement ID or function name" }
  ```
- Understand how your requirement fits into the larger validation pipeline

## Step 4: Existing Code Patterns

- Search for similar implementations already in `src/consensus/`:
  ```
  codebase_search { query: "similar function or pattern" }
  ```
- Check test patterns in `tests/` to match existing style:
  ```
  codebase_search { query: "test pattern for similar requirement" }
  ```

## Authority Order

When sources give conflicting information:

1. NORMATIVE.md (highest)
2. Dedicated spec (specs/PREFIX-NNN.md)
3. Source citations (SPEC.md sections)
4. Chia L1 source code
5. Existing code in this crate (lowest)

---

Navigation: Prev < [dt-wf-select.md](dt-wf-select.md) | Next > [dt-wf-test.md](dt-wf-test.md)
