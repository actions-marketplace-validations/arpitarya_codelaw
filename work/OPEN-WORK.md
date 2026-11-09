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

*Empty since 2026-11-09 — the next gate: W-105's RC re-pin, 2026-11-10.*

---

## Open items (31)

### build (21)

🟣 **W-105** `agent` Type-aware TypeScript: nine rules; helper batched, version watch proven (PR #7, issue #6). `check_types_*` budgets ruled 11-07: left unset (reported only). Next: the RC re-pin (11-10). Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-105-type-aware-engine.md)


🟢 **W-172** `agent` L8: time side built (11-08). `peak_memory_10k` ruled 11-09: amend L8 and raise it 72 → 90 MiB. Next: amend SR-LAW-8, set `perf/budgets.toml`; CI's next run proves the time side. Filed by Claude Code (W-139). [detail](open/W-172-l8-budgets-over-on-main.md)

🟢 **W-174** `agent` Python inference: SR-PY-INFERENCE (0120); 37 of 38 built (`preview`); `boolean-datetime` ruled not-applicable 11-09; decision 8 moved to research, W-177. Next: re-class that row, then close. Filed by Claude Code (W-139). [detail](open/W-174-python-inference.md)

🟢 **W-176** `agent` Options-only rules: the 19 js/node rows that check nothing until configured (`id-match`, `id-denylist`, `no-restricted-syntax`, …), built as their own class, silent by default, out of W-139's count. Filed by Arpit (Cowork). [detail](open/W-176-options-only-rules.md)

🟢 **W-178** `agent` Build `key-name-casing` (data, off by default), `require-frontmatter-fields` (md) and `check-examples` — with the engine work to lint `@example` code. Filed by Arpit (Cowork). [detail](open/W-178-config-keys-frontmatter-examples.md)

🟢 **W-179** `agent` Formatter: check and add Ruff's docstring and shebang layout rows (D201, D202, D203/D211, D204, EXE004) and two Pylint ones; D210 already done. Filed by Arpit (Cowork). [detail](open/W-179-formatter-docstring-shebang-rows.md)

🟢 **W-180** `agent` pydocstyle's imperative-mood verb list (ratified 11-09) and Ruff D401, D421 on it. Filed by Arpit (Cowork). [detail](open/W-180-imperative-mood-verbs.md)

🟢 **W-182** `agent` A Unicode confusables data package (ratified 11-09) and Ruff RUF001–RUF003 on it. Filed by Arpit (Cowork). [detail](open/W-182-unicode-confusables.md)

🟢 **W-183** `agent` Findings gated on the project's Python version; Ruff FA100, FA102, PERF203, UP036 and two Pylint rows on it. Filed by Arpit (Cowork). [detail](open/W-183-target-gated-findings.md)

🟢 **W-184** `agent` A file's executable bit for rules; Ruff EXE001, EXE002 on it. Filed by Arpit (Cowork). [detail](open/W-184-file-mode-bits.md)

🟢 **W-185** `agent` The Python side of the project graph (imports, class bases) and six Pylint rows on it (`cyclic-import`, `import-error`, …). Filed by Arpit (Cowork). [detail](open/W-185-python-project-graph.md)

🟢 **W-187** `agent` A data package of deprecated standard-library names from CPython's docs (approved 11-09); Pylint's five `deprecated-*` rows. Filed by Arpit (Cowork). [detail](open/W-187-stdlib-deprecations.md)

🟢 **W-190** `agent` `no-using-resource-escape`, `prefer-dispose`: the pinned JS grammar (0.25.0) parses `using`, TS 0.23.2 does not — build for JS, then move TS. Filed by Arpit (Cowork). [detail](open/W-190-using-declarations.md)

🟢 **W-193** `agent` Ruff's last buildable rows: TC008, SIM905. Filed by Arpit (Cowork). [detail](open/W-193-ruff-tail.md)

🟢 **W-194** `agent` Pylint's 15 docparams rows and two `ported_from:` links. Filed by Arpit (Cowork). [detail](open/W-194-pylint-tail.md)

🟢 **W-195** `agent` JS rows with no blocker: four unicorn/SonarJS rows, `comment-type`, `process-exit-as-throw`, six second fixes. Filed by Arpit (Cowork). [detail](open/W-195-js-tail.md)

🟢 **W-196** `agent` Biome's 98 unbuilt rows (Qwik, Solid, React Native, Vue and nursery ones classed first). Filed by Arpit (Cowork). [detail](open/W-196-biome-tail.md)

🟡 **W-181** waits on **W-183** `agent` Ruff's moved typing names table (ratified 11-09) and UP035 on it. Filed by Arpit (Cowork). [detail](open/W-181-moved-typing-names.md)

🟡 **W-186** waits on **W-185** `agent` Ruff PLC2701, importing another package's private names. Filed by Arpit (Cowork). [detail](open/W-186-import-private-name.md)

🟣 **W-188** waits on **W-105** `agent` The JS value-type rows (built 11-09 on one file's facts) ask the type helper; `jsdoc-no-unnecessary-type-assertion`. Filed by Arpit (Cowork). [detail](open/W-188-js-value-rows-type-aware.md)


🟡 **W-143** waits on **W-195**, **W-196** `agent` Vue, Svelte, Astro, GraphQL as codelaw languages (L3, L7; a parser and a formatter each, grammars ratified under L11) and Biome's 89 rules for them. Filed by Arpit (Cowork). [detail](open/W-143-vue-svelte-astro-graphql.md)

### ci/cd (4)

🟢🧨 **W-169** `agent` `codelaw` segfaulted on the musl build legs now and then; cause found and fixed 11-07. Pushed 11-08: all eight build legs green in `ci` 42454141410 — 1 of 5 runs on the fix; four more needed. Filed by Arpit (Claude Code). [detail](open/W-169-rules-lint-segfault.md)

🟢 **W-126** `agent` `perf` is split and off the critical path (11-08: done 3m27 after the builds). But a Rust/pack-change run took 24m56: Windows builds 9m25, the corpus legs are the tail; goal not met, 1 of 3 runs. Filed by Arpit (Cowork). [detail](open/W-126-fast-pipeline.md)

🟢 **W-95** `agent` VS Code extension: Marketplace live (0.4.0); Open VSX by trusted publishing in `publish.yml` since 11-05, proven by the next release. Left: a VSCodium install after it (Open VSX's darwin-arm64 is 0.3.0). Filed by Arpit (Cowork). [detail](open/W-95-publish-vscode-extension.md)

🟡 **W-142** waits on **W-182**, **W-187** `arpit` Registry names for the new packs (five PyPI framework packs, npm `testing`/`nextjs`, Tailwind's) and data packages (browsers, polyfills, the two new ones), with Fux indexes; before their release. Filed by Arpit (Cowork). [detail](open/W-142-framework-pack-names.md)

### cleanup (0)

### testing (0)

### reporting and benchmarks (2)

🟡 **W-102** waits on **W-176**, **W-178**, **W-180**, **W-181**, **W-182**, **W-183**, **W-184**, **W-185**, **W-186**, **W-187**, **W-189**, **W-190**, **W-193**, **W-194**, **W-195**, **W-196** `agent` 7 tasks × 9 runs (~$65), `PREREG.md` amended first; once every rule is built. Filed by Arpit (Cowork). [detail](open/W-102-guidance-vs-check-experiment.md)

🟢 **W-192** `agent` A generated page of what is on and off by default per pack and preset; W-139's missing presets and its Oxlint promotions. Filed by Arpit (Cowork). [detail](open/W-192-defaults-on-and-off.md)

### record update (1)

🟢 **W-191** `agent` SR-PROJECT-GRAPH as the overview of every graph; the module graph, semantic model and Python flow pass each in a record. SR-CODE-PATH-GRAPH filed 11-09. Filed by Arpit (Cowork). [detail](open/W-191-graph-records.md)

### research (3)

🟢 **W-167** `cowork` Architecture design for graph generation at 100k–1M files: today's pipeline, prior art (Glean, Kythe, SCIP, salsa, Knip, Nx), 2–3 options as a proposal; step 3 reads W-168's numbers. Filed by Arpit (Cowork). [detail](open/W-167-graph-architecture-design.md)

🟢 **W-177** `cowork` Should the Python rows that need a call's return or an imported class ask the project's own type checker (`ty`, pyright, mypy), as TypeScript is asked? SR-PY-INFERENCE decision 8, as a proposal. Filed by Arpit (Cowork). [detail](open/W-177-python-type-checker-research.md)

🟢 **W-189** `cowork` A code-path graph for JS (routes through `await`, loops, `try`) for `require-atomic-updates` and `no-multiple-promise-resolver-calls`. Filed by Arpit (Cowork). [detail](open/W-189-js-code-path-graph-research.md)

---
