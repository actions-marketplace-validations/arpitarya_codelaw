---
type: Queue
description: "The single live work queue, three lanes, plus the Blocked-on-the-decider inbox. The list only — its rules live in the record below."
governed_by: SR-WORK-OPEN-QUEUE
governed_by_path: records/0051_WORK-open-queue.md
---

# OPEN-WORK — what is still open

**This file is the LIST, plus the legend below.** Everything else — what the
queue is, an item's lifecycle, the shape of a row, ball precedence, ordering,
how the inbox works, and what every session owes — is stated once in
**[SR-WORK-OPEN-QUEUE](../records/0051_WORK-open-queue.md)** and is not repeated
here. Read that record before changing anything below it.

**A ball on every row, always one:** 🔴 blocked on the decider, directly or through another item · 🟣 gated on a named date, directly or through another item · 🟡 waiting on another item, named right after the id — `waits on **W-mm**`, or `**<project> W-mm**` for another project's · 🟢 no blockers.
**Optional, after the ball:** 🧨 broken or getting worse · 🔺 do first (the decider only). Full legend: [SR-WORK-OPEN-QUEUE](../records/0051_WORK-open-queue.md) rules 20–34.

---

## Blocked on the decider

| what the decider decides | filed |
|---|---|
| 🔴 **W-192** put the 23 Oxlint `correctness` ports with no corpus finding into their pack's `recommended` preset (10 in `js/recommended`, 11 in `react/recommended`, 2 in `ts/recommended`)? Each is `preview`, so a project sees one only with `preview = true`; each had 0 findings on the corpus. Recommended: yes, all 23. [detail](open/W-192-defaults-on-and-off.md) | 2026-11-17 |
| ↳ **blocks:** W-192 (its last step) | |
| 🔴 **W-142** bind the registry names for the packs and data packages before their release: five PyPI pending publishers, the npm first publishes, the two data packages; Claude fills the forms in Chrome, you check and submit. Every package is built now. [detail](open/W-142-framework-pack-names.md) | 2026-10-30 |
| ↳ **blocks:** nothing else in the queue | |


---

## Open items (18)

### build (9)

🟣 **W-105** `agent` Type-aware TypeScript: nine rules; helper batched, version watch proven (PR #7, issue #6). `check_types_*` budgets ruled 11-07: left unset (reported only). Next: the RC re-pin; no 7.1 RC on npm yet (11-15). Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-105-type-aware-engine.md)


🟢 **W-172** `agent` L8: time side built (11-08); `peak_memory_10k` raised 72 → 90 MiB by ruling, L8 decision 5a (11-15). Next: CI's next run on a pushed `main` proves `check_cold_10k` and `lsp_after_edit`; close then. Filed by Claude Code (W-139). [detail](open/W-172-l8-budgets-over-on-main.md)







🟢 **W-190** `agent` `no-using-resource-escape`, `prefer-dispose`: the pinned JS grammar (0.25.0) parses `using`, TS 0.23.2 does not — build for JS, then move TS. Filed by Arpit (Cowork). [detail](open/W-190-using-declarations.md)

🟢 **W-195** `agent` JS rows with no blocker: four unicorn/SonarJS rows, `comment-type`, `process-exit-as-throw`, six second fixes. Filed by Arpit (Cowork). [detail](open/W-195-js-tail.md)

🟢 **W-196** `agent` Biome's 98 unbuilt rows (Qwik, Solid, React Native, Vue and nursery ones classed first). Filed by Arpit (Cowork). [detail](open/W-196-biome-tail.md)

🟢 **W-181** `agent` Ruff's moved typing names table (ratified 11-09) and UP035 on it. Filed by Arpit (Cowork). [detail](open/W-181-moved-typing-names.md)

🟢 **W-186** `agent` Ruff PLC2701, importing another package's private names. Filed by Arpit (Cowork). [detail](open/W-186-import-private-name.md)

🟣 **W-188** waits on **W-105** `agent` The JS value-type rows (built 11-09 on one file's facts) ask the type helper; `jsdoc-no-unnecessary-type-assertion`. Filed by Arpit (Cowork). [detail](open/W-188-js-value-rows-type-aware.md)


🟡 **W-143** waits on **W-195**, **W-196** `agent` Vue, Svelte, Astro, GraphQL as codelaw languages (L3, L7; a parser and a formatter each, grammars ratified under L11) and Biome's 89 rules for them. Filed by Arpit (Cowork). [detail](open/W-143-vue-svelte-astro-graphql.md)

### ci/cd (4)

🟢🧨 **W-169** `agent` `codelaw` segfaulted on the musl build legs now and then; cause found and fixed 11-07. Pushed 11-08: all eight build legs green in `ci` 42454141410 — 1 of 5 runs on the fix; four more needed. Filed by Arpit (Claude Code). [detail](open/W-169-rules-lint-segfault.md)

🟢 **W-126** `agent` `perf` is split and off the critical path (11-08: done 3m27 after the builds). But a Rust/pack-change run took 24m56: Windows builds 9m25, the corpus legs are the tail; goal not met, 1 of 3 runs. Filed by Arpit (Cowork). [detail](open/W-126-fast-pipeline.md)

🟢 **W-95** `agent` VS Code extension: Marketplace live (0.4.0); Open VSX by trusted publishing in `publish.yml` since 11-05, proven by the next release. Left: a VSCodium install after it (Open VSX's darwin-arm64 is 0.3.0). Filed by Arpit (Cowork). [detail](open/W-95-publish-vscode-extension.md)

🔴 **W-142** `arpit` Registry names for the new packs (five PyPI framework packs, npm `testing`/`nextjs`, Tailwind's) and data packages (browsers, polyfills, the two new ones), with Fux indexes; before their release. Filed by Arpit (Cowork). [detail](open/W-142-framework-pack-names.md)

### cleanup (0)

### testing (0)

### reporting and benchmarks (2)

🟡 **W-102** waits on **W-181**, **W-186**, **W-189**, **W-190**, **W-195**, **W-196** `agent` 7 tasks × 9 runs (~$65), `PREREG.md` amended first; once every rule is built. Filed by Arpit (Cowork). [detail](open/W-102-guidance-vs-check-experiment.md)

🔴 **W-192** `agent` The page of defaults (`docs/rules/defaults.md`) and `py/strict`, `py/pylint`, `py/pydocstyle` are built (11-17). Left: the 23 Oxlint ports for `recommended`, in the inbox. Filed by Arpit (Cowork). [detail](open/W-192-defaults-on-and-off.md)

### record update (0)

### research (3)

🟢 **W-167** `cowork` Architecture design for graph generation at 100k–1M files: today's pipeline, prior art (Glean, Kythe, SCIP, salsa, Knip, Nx), 2–3 options as a proposal; step 3 reads W-168's numbers. Filed by Arpit (Cowork). [detail](open/W-167-graph-architecture-design.md)

🟢 **W-177** `cowork` Should the Python rows that need a call's return or an imported class ask the project's own type checker (`ty`, pyright, mypy), as TypeScript is asked? SR-PY-INFERENCE decision 8, as a proposal. Filed by Arpit (Cowork). [detail](open/W-177-python-type-checker-research.md)

🟢 **W-189** `cowork` A code-path graph for JS (routes through `await`, loops, `try`) for `require-atomic-updates` and `no-multiple-promise-resolver-calls`. Filed by Arpit (Cowork). [detail](open/W-189-js-code-path-graph-research.md)

---
