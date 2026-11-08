---
type: Handoff
name: W-175
description: "The project-graph queries the W-139 js rows need and the graph does not answer yet — another module's exports, re-exports, cycles, deprecations and unused modules — for 9 eslint-plugin-import/import-x rows, plus one unicorn row that needs a polyfill data table."
item: W-175
filed: 2026-11-08
filed_by: "Claude Code (session 0a172e1f), W-139 strand W-139-js"
updated: 2026-11-08
updated_by: "Claude Code (session 0a172e1f), W-139 strand W-139-js"
ball: agent
---

# W-175 — Graph queries for the W-139 js rows that read other modules

## Why this exists

W-139's js strand labelled 29 rows "needs the project graph or the file system". Round 3 (slice 25) built every row that needs only the file's own path, whether another file exists, or the nearest `package.json` — native rules read those through `NativeContext::file` ([SR-ARCHITECTURE](../../records/0108_architecture.md) decision 15b) — and linked three to the `node/` rules the graph already runs (`graph: import: resolves | declared: false`). What is left asks about **another module's contents**: which names it exports, what its default export is called, whether it is deprecated, who imports it. The project graph ([SR-PROJECT-GRAPH](../../records/0103_project-graph.md)) holds those facts (decision 1: each module's `exports`, `imports`, `references`), but its `graph:` query language (decision 6, `crates/codelaw-graph/src/query.rs`) has no query that reports an *import* by what its target module exports — only unused exports, CSS classes, deprecated registry entries, flags and import resolution. This item is where these rows wait, so none is orphaned when W-139 closes.

## The rows, by what the graph lacks

| graph query needed | rows (eslint-plugin-import 2.32.0 unless named) |
|---|---|
| **an import's names against its target's exports** (named, default, namespace members) | `import/named` → `js/import-named`, `import/default` → `js/import-default`, `import/namespace` → `js/import-namespace`, `import/no-named-as-default` → `js/import-no-named-as-default`, `import/no-named-as-default-member` → `js/import-no-named-as-default-member` |
| **the target's default export's own name** | import-x 4.17.1 `no-rename-default` → `js/import-no-rename-default` |
| **re-exports across modules** (`export *` conflicts) | `import/export`'s cross-module half — `js/import-export` is built for one file's duplicates (slice 25) and its page says the `export *` half is this item's |
| **a cycle in the import edges** | `import/no-cycle` → `js/import-no-cycle` (SR-PROJECT-GRAPH lists `graph/no-cycles` as a future rule; one query can back both) |
| **the target's JSDoc `@deprecated` on an export** | `import/no-deprecated` → `js/import-no-deprecated` (the graph's `deprecated: registry` reads codelaw's own registry, not JSDoc tags) |
| **modules nothing imports, and exports nothing uses** | `import/no-unused-modules` → `js/import-no-unused-modules` (its `unusedExports` half is `graph/no-unused-exports`; its `missingExports` half and the module-level report are new) |
| **not a graph question: a data table** | unicorn 79.0.0 `no-unnecessary-polyfills` → `js/no-unnecessary-polyfills` needs core-js-compat's feature table and a browserslist resolver — a data package like `@codelaw/data-browsers` ([SR-RULE-FORMAT](../../records/0112_rule-format.md) decision 2c), which L11 must ratify if it is a dependency |

10 rows. Each is `class = port` in `work/proposals/rule-parity/eslint-plugin-import.csv`, `eslint-plugin-import-x.csv` or `eslint-plugin-unicorn.csv`, so W-139's DoD 1 counts it until it is built or this item re-rules it. `import/no-restricted-paths` is not here: with its default (no zones) it reports nothing, so W-139 lists it with the other default-configuration rows.

## Steps

| step | what we do | state |
|---|---|---|
| 1 | Amend SR-PROJECT-GRAPH decision 6 with an `import:` query keyed on the target's exports (`names: unexported`, `default: missing`, `default_name: differs`), a `cycle:` query and an `@deprecated` fact in the facts pass; measure the facts' cost against L8 (W-172 is profiling rule loading) | ⏳ |
| 2 | Build the queries in `crates/codelaw-graph` and their `codelaw-rules` compile checks, with graph tests on virtual repos | ⏳ |
| 3 | Write each row fresh from the pinned upstream source and tests (eslint-plugin-import at commit `01c9eb04`, import-x `v4.17.1`), its cases (`files` fences) and fix class; `codelaw test packs/js` 0 failed | ⏳ |
| 4 | `no-unnecessary-polyfills`: propose its table (source, licence, refresh), then build it | ⏳ |

## Blockers

None to start step 1. A core-js-compat or browserslist dependency (step 4) needs the decider (L11).
