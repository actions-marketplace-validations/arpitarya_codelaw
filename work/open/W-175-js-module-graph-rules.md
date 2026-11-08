---
type: Handoff
name: W-175
description: "The project-graph queries the W-139 js rows need — another module's exports, re-exports, cycles, deprecations and unused modules — built, with the 9 eslint-plugin-import/import-x rows (+ the cross-module half of import/export); unicorn's polyfill row waits on a data-package ruling."
item: W-175
filed: 2026-11-08
filed_by: "Claude Code (session 0a172e1f), W-139 strand W-139-js"
updated: 2026-11-08
updated_by: "Arpit (Claude Code, Sonnet 5.5)"
ball: arpit
---

# W-175 — Graph queries for the W-139 js rows that read other modules

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Write the queries into the project-graph record, with their cost | the design is on record before the code | ✅ |
| 2 | Build the queries in the graph crate, with tests on small made-up repos | a rule can ask what another file exports, who imports whom, what is deprecated, what is never imported | ✅ |
| 3 | Write the nine import rules (and the cross-file half of `import/export`) from the plugins' behaviour, with cases | nine more rows of W-139 closed: 891 js rules, 0 failing | ✅ |
| 4 | Build the polyfill rule | the tenth row, which needs a table of which browser has which feature | 🔴 Arpit: may codelaw add a data package for that table? |

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
| 1 | Amend SR-PROJECT-GRAPH decision 6 with the queries (`import:` `names`/`default`/`namespace`/`default_name`/`default_member`/`deprecated`, `cycle:`, `export:`, `module: { exports }`) and the one fact (`default_name`); measure the facts' cost against L8 | ✅ 2026-11-08 — decision 6's *Queries on what another module exports*; the graph is built only for a project rule and each query runs only for a rule that names it; the one fact is read for a module that has a default export (an 800-statement file: 3.3–6.2 s for 50 extractions with it, 3.3–3.9 s without, a debug build on a loaded Mac — inside the noise); `FACTS_VERSION` 7 |
| 2 | Build the queries in `crates/codelaw-graph/src/names.rs` and their compile checks in `query.rs`, with graph tests on virtual repos | ✅ 2026-11-08 — `cargo test -p codelaw-graph`: 74 passed (13 new, `names_tests.rs`); clippy clean; no engine change (`codelaw-rules`' compile path is the graph crate's `compile`) |
| 3 | Write each row fresh from the pinned upstream source (eslint-plugin-import 2.32.0's packed `lib/` and docs, import-x 4.17.1's packed `lib/`; read, never copied), its cases (`files` fences) and fix class | ✅ 2026-11-08 — 10 `preview` rules: `js/import-named`, `import-namespace` (`suggest`: `nearest-export`), `import-no-unused-modules` (`suggest`: strip the export keyword or delete), `import-default`, `import-no-named-as-default-member`, `import-no-cycle`, `import-export-star` (`manual`, `restructure`), `import-no-named-as-default`, `import-no-rename-default`, `import-no-deprecated` (`manual`, `value`); `codelaw test packs/js` 891 rules · 21,909 checks · 0 failed; `rules lint packs/js` 0 problems |
| 4 | `no-unnecessary-polyfills`: propose its table (source, licence, refresh), then build it | 🔴 gated — the proposal is below; one question to Arpit |

## What was built, and what it leaves out

Nine of the ten rows, and the cross-module half of `import/export` (as `js/import-export-star`, so `js/import-export` keeps its one-file half). The five tranche A rows (`named`, `default`, `namespace`, `no-named-as-default`, `no-named-as-default-member`) are in the opt-in `js/project` preset at the plugin's severities, **not in `js/recommended`** as W-139's method says: every one is a project rule, and a project rule builds the graph, which `check_cold_10k` would pay for on every JavaScript repo (SR-PROJECT-GRAPH decision 5 measured 6.3 s cold at 10k files; no budget is set for it). The rest are `preview` in no preset. What each page says it leaves out: computed namespace access (`allowComputed`) and assignment to a namespace member; a namespace read two levels deep; a module-level `@deprecated`; `no-unused-modules`' `src`/`ignoreExports` (the graph's entry and keep settings stand in, and every module of a package whose entries cannot be mapped to source is skipped); `no-cycle`'s options and `require()`; `no-rename-default`'s options; import-named's report on `export … from` is at the statement, not the specifier.

Checked on two real repositories on this machine (jotai, zustand; the js corpus checkouts are being re-fetched by another agent): `import-named` found one true positive (`atomFamily` is no longer exported by `jotai/utils`), `import-no-deprecated` six uses of names jotai marks `@deprecated`, nothing else. The pack's `corpus.toml` has no counts for the ten rules yet: `packs/js/corpus.toml` is W-169's refresh; one `scripts/corpus.py run --pack js --update` after both land adds them (preview rules need none to pass `codelaw test`).

## Step 4 — `no-unnecessary-polyfills`: the proposal

unicorn 79.0.0's rule reports an import of a polyfill (`core-js/…`, `array.prototype.flat`, `object-assign`, …) for a feature every target the project supports already has. It needs two tables and a target:

| need | proposal |
|---|---|
| **feature → polyfill modules** (`es.array.flat` ↔ `core-js/modules/es.array.flat`, `core-js/features/array/flat`, `array.prototype.flat`, `es-shims`' package names, …) | a new **data package** in the shape of `@codelaw/data-browsers` (L6 decision 15): `@codelaw/data-polyfills` / `codelaw-data-polyfills`, data only (JSON, its schema, its `NOTICE`), no code and no dependency, optional (a rule that does not find it stays silent), versioned by the date of its last change |
| **feature → first version per engine** (`chrome`, `edge`, `firefox`, `safari`, `ios_saf`, `samsung`, `node`) | rows derived from **core-js-compat's `data.json`** (MIT; its own sources are the engines' release notes, MDN's browser-compat-data (CC0) and kangax's compat-table (MIT)), by a generator run in CI outside the engine (L5): `scripts/data-polyfills-check.py --refresh` (networked, like `codelaw deps refresh`), a weekly workflow that opens a pull request naming the rows to review, as `data-browsers.yml` does. The shipped table keeps each row's source, and `NOTICE` carries core-js-compat's MIT notice. caniuse is not used (CC-BY-4.0) |
| **the project's targets** | read, with no `browserslist` library: `package.json`'s `engines.node`, its `browserslist` field and `.browserslistrc` when they spell versions out (`chrome >= 100`, `node >= 18`, `last 2 chrome versions` is **not** resolved), and the rule's `targets` option (unicorn's own); a query the reader cannot resolve without caniuse-lite's data means the rule stays silent for that target and says so on its page |
| **fix class** | `suggest` where the import is a bare `import "polyfill"` (delete the statement), `manual`/`restructure` where the binding is used |

Licence check (L2): core-js-compat MIT, compat-table MIT, MDN BCD CC0, the vendors' notes facts; nothing copyleft, nothing paid. L11: the data package is not a dependency of either `codelaw` package (L6 decision 15: optional, data only); the generator reads core-js-compat's published JSON in CI and does not install it — **no dependency is added anywhere**. Cost: one more package pair in the release (npm and PyPI), one more weekly workflow.

**The one question for Arpit:** may codelaw add a data package `data-polyfills` (npm `@codelaw/data-polyfills`, PyPI `codelaw-data-polyfills`; data only, derived in CI from core-js-compat's MIT data, refreshed weekly like `data-browsers`), resolving only browserslist queries that spell out versions? Yes builds `js/no-unnecessary-polyfills` and files the registry names under W-142; no leaves the row `class = port` in W-139's DoD 1 as the one js row this item did not close.

## Blockers

Step 4 waits on the ruling above (L11, L6 decision 15). Nothing else.
