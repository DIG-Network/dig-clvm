# dt-tools — Tool Integration

Three tools are deeply integrated into the dig-clvm workflow. Each has a HARD RULE governing when it MUST be used.

## SocratiCode — Semantic Codebase Intelligence

**HARD RULE:** MUST use before reading files. Search first, read targeted.

### Commands

| Command | Purpose |
|---------|---------|
| `codebase_status {}` | Check index status |
| `codebase_index {}` | Index or reindex the codebase |
| `codebase_search { query: "..." }` | Hybrid semantic + keyword search |
| `codebase_graph_query { filePath: "..." }` | Show imports and dependents for a file |
| `codebase_graph_circular {}` | Detect circular dependencies |
| `codebase_context_search { query: "..." }` | Search schemas, APIs, configs |

### Use Cases

- Find related code before implementation
- Understand dependency chains between modules
- Discover cross-references between requirements and code
- Detect architectural issues (circular deps)

Full docs: [../tools/socraticode.md](../tools/socraticode.md)

---

## GitNexus — Knowledge Graph Dependency Analysis

**HARD RULE:** MUST use before refactoring or renaming public symbols.

### Commands

| Command | Purpose |
|---------|---------|
| `npx gitnexus status` | Check if index is fresh |
| `npx gitnexus analyze` | Incremental index update |
| `npx gitnexus analyze --force` | Full re-index |
| `gitnexus_impact { symbol: "..." }` | What depends on this symbol? (MCP) |
| `gitnexus_rename { old: "...", new: "..." }` | Safe rename across codebase (MCP) |
| `gitnexus_detect_changes` | What changed since last analyze? (MCP) |

### Use Cases

- Check impact before renaming symbols
- Safe refactoring with dependency awareness
- Pre-commit change detection and scope verification

Full docs: [../tools/gitnexus.md](../tools/gitnexus.md)

---

## Repomix — Context Packing for LLM Consumption

**HARD RULE:** MUST pack context before starting implementation.

### Commands

```bash
# Pack implementation scope
npx repomix@latest src/consensus -o .repomix/pack-consensus.xml

# Pack tests
npx repomix@latest tests -o .repomix/pack-tests.xml

# Pack requirements for a domain
npx repomix@latest docs/requirements -o .repomix/pack-requirements.xml
```

`.repomix/` is gitignored. Pack files are ephemeral working context.

### Use Cases

- Feed codebase scope to LLM for implementation planning
- Ensure full context before coding
- Compare test patterns across domains

Full docs: [../tools/repomix.md](../tools/repomix.md)

---

## Integration Matrix

| Workflow Step | SocratiCode | GitNexus | Repomix |
|--------------|-------------|----------|---------|
| Select | `codebase_search` for existing implementations | -- | -- |
| Gather Context | `codebase_search` + `codebase_graph_query` | -- | Pack scope |
| Test | Search for test patterns | -- | Pack tests |
| Implement | Search before coding | Impact check before refactoring | Pack implementation scope |
| Validate | `codebase_graph_circular` | `detect_changes` | -- |
| Commit | -- | `npx gitnexus analyze` (update index) | -- |

---

Navigation: Prev < [dt-authoritative-sources.md](dt-authoritative-sources.md) | Next > [dt-git.md](dt-git.md)
