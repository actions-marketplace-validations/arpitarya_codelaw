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

*Empty since 2026-11-08 — next gate: the TypeScript 7.1 RC, 2026-11-10 (W-105).*

---

## Open items (11)

### build (5)

🟣 **W-105** `agent` Type-aware TypeScript: nine rules; helper batched, version watch proven (PR #7, issue #6). `check_types_*` budgets ruled 11-07: left unset (reported only). Next: the RC re-pin (11-10). Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-105-type-aware-engine.md)

🟢 **W-139** `agent` Rule parity: js, Ruff, Pylint, testing, nextjs, 5 PyPI packs. DoD 1 — js 71 rows left, py 164. 11-09: tail built (15 rules); 25 layout rows and the Airflow tables → re-rule list (item, *Slice 28*). Next: js JSDoc rows, py natives. Filed by Arpit (Cowork). [detail](open/W-139-rule-parity-build.md)

🟢 **W-172** `agent` L8 budgets: 11-08 time side built (allocator leftover removed, key order, parent gate); `peak_memory_10k` still ~14 MiB over. Next: push `main` for CI. Filed by Claude Code (W-139). [detail](open/W-172-l8-budgets-over-on-main.md)

🟢 **W-174** `agent` Python inference: SR-PY-INFERENCE (0120); 30 of 38 rows built (`preview`, counted 11-09), 8 left with their question. Decider (not blocking): a project type checker for the rest, decision 8. Next: the 8 in-file rows. Filed by Claude Code (W-139). [detail](open/W-174-python-inference.md)


🟡 **W-143** waits on **W-139** `agent` Vue, Svelte, Astro, GraphQL as codelaw languages (L3, L7; a parser and a formatter each, grammars ratified under L11) and Biome's 89 rules for them. Filed by Arpit (Cowork). [detail](open/W-143-vue-svelte-astro-graphql.md)

### ci/cd (4)

🟢🧨 **W-169** `agent` `codelaw` segfaulted on the musl build legs now and then; cause found and fixed 11-07. Pushed 11-08: all eight build legs green in `ci` 42454141410 — 1 of 5 runs on the fix; four more needed. Filed by Arpit (Claude Code). [detail](open/W-169-rules-lint-segfault.md)

🟢 **W-126** `agent` `perf` is split and off the critical path (11-08: done 3m27 after the builds). But a Rust/pack-change run took 24m56: Windows builds 9m25, the corpus legs are the tail; goal not met, 1 of 3 runs. Filed by Arpit (Cowork). [detail](open/W-126-fast-pipeline.md)

🟢 **W-95** `agent` VS Code extension: Marketplace live (0.4.0); Open VSX by trusted publishing in `publish.yml` since 11-05, proven by the next release. Left: a VSCodium install after it (Open VSX's darwin-arm64 is 0.3.0). Filed by Arpit (Cowork). [detail](open/W-95-publish-vscode-extension.md)

🟡 **W-142** waits on **W-139** `arpit` New packs' registry names: five PyPI framework packs (pending publishers), npm `testing`/`nextjs` (was W-147), Tailwind's, `data-browsers` (W-166) and `data-polyfills` (W-175), with Fux indexes; before their release. Filed by Arpit (Cowork). [detail](open/W-142-framework-pack-names.md)

### cleanup (0)

### testing (0)

### reporting and benchmarks (1)

🟡 **W-102** waits on **W-139** `agent` Ruled 11-05: the 7 tempting tasks at 9 runs per arm (~$65), `PREREG.md` amended first — and run only once every rule is built (voice). Rework, turns and tokens: B-072. Filed by Arpit (Cowork). [detail](open/W-102-guidance-vs-check-experiment.md)

### record update (0)

### research (1)

🟢 **W-167** `cowork` Architecture design for graph generation at 100k–1M files: today's pipeline, prior art (Glean, Kythe, SCIP, salsa, Knip, Nx), 2–3 options as a proposal; step 3 reads W-168's numbers. Filed by Arpit (Cowork). [detail](open/W-167-graph-architecture-design.md)

---
