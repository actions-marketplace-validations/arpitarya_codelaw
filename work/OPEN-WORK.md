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

*Empty since 2026-11-07 — next gate: the TypeScript 7.1 RC, 2026-11-10 (W-105).*

---

## Open items (13)

### build (7)

🟢 **W-171** `agent` Six worktrees (W-139, W-141, W-156, W-162, W-168, W-169) hold unlanded work of agents stopped by the usage limit 11-05. Ruled 11-07: keep; each lands with its item. Filed by now-ledger.py (Claude Code). [detail](open/W-171-session-cleanup.md)

🟢 **W-105** `agent` Type-aware TypeScript: nine rules; helper batched, version watch proven 11-05 (PR #7, issue #6). Budgets wait on three `perf` runs, skipped until W-126's `9b8ca7b7` is pushed; next the RC (11-10). Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-105-type-aware-engine.md)

🟢 **W-139** `agent` Rule parity: js, Ruff, Pylint, testing, nextjs, 5 PyPI packs. 11-07: steps 3–4 done — 27 packs 0 failed, corpus counted, 2 rules narrowed, proposal §13 (1,548 of 2,590 built). L8 → W-172. Next: DoD 1. Filed by Arpit (Cowork). [detail](open/W-139-rule-parity-build.md)

🟢 **W-172** `agent` main over three L8 budgets on the reference runner (11-07): `check_cold_10k` 13,465 > 13,118 ms, `peak_memory_10k` 124 > 72 MiB, `mcp_rules_for_path` 129 > 65 ms; green 11-05. Next: bisect, optimise. Filed by Claude Code (W-139). [detail](open/W-172-l8-budgets-over-on-main.md)

🟡 **W-143** waits on **W-139** `agent` Vue, Svelte, Astro, GraphQL as codelaw languages (L3, L7; a parser and a formatter each, grammars ratified under L11) and Biome's 89 rules for them. Filed by Arpit (Cowork). [detail](open/W-143-vue-svelte-astro-graphql.md)

🟢 **W-156** `agent` Fix ladder (L4). Engine and lowering built; 138 capability-blocked rules marked (W-162). The `match:` moves done; native moves under way (stencil landed): 150 rules left to move. Next: the next pack's native fixes. Filed by Arpit (Claude app). [detail](open/W-156-fix-ladder.md)

🟢 **W-162** `agent` Build the fix capabilities W-161 designed ([proposal](proposals/fix-capabilities.md)). Computed edits: filters, `transform:`, `edit-text`, `unwrap-block`, `move` built (30 `js/*` moved; 73 still wait). Next: `merge-into`, native fixes, then scope rename. Filed by Arpit (Cowork). [detail](open/W-162-fix-capabilities-build.md)



### ci/cd (4)

🟢🧨 **W-169** `agent` `codelaw rules lint` segfaults in CI now and then (arm64-musl on js, x64 on react; the same commit passes on rerun). Next: a core dump and backtrace from CI, then the fix. Filed by Arpit (Claude Code). [detail](open/W-169-rules-lint-segfault.md)

🟢 **W-126** `agent` Ruled 10-30: `perf` split, nothing waits on it (until a budget is set, L8); release 3m50 accepted. Cached CI 4m11–4m25 ✅. `perf` matrix built 11-05; first runs skipped it (arm64-musl red). Next: three runs once pushed. Filed by Arpit (Cowork). [detail](open/W-126-fast-pipeline.md)

🟢 **W-95** `agent` VS Code extension: Marketplace live (0.4.0); Open VSX by trusted publishing in `publish.yml` since 11-05, proven by the next release. Left: a VSCodium install after it (Open VSX's darwin-arm64 is 0.3.0). Filed by Arpit (Cowork). [detail](open/W-95-publish-vscode-extension.md)

🟡 **W-142** waits on **W-139** `arpit` New packs' registry names: five PyPI framework packs (pending publishers), npm `testing`/`nextjs` (was W-147), Tailwind's and `data-browsers` (W-166), with Fux indexes; before their release. Filed by Arpit (Cowork). [detail](open/W-142-framework-pack-names.md)

### cleanup (0)

### testing (0)

### reporting and benchmarks (1)

🟡 **W-102** waits on **W-139** `agent` Ruled 11-05: the 7 tempting tasks at 9 runs per arm (~$65), `PREREG.md` amended first — and run only once every rule is built (voice). Rework, turns and tokens: B-072. Filed by Arpit (Cowork). [detail](open/W-102-guidance-vs-check-experiment.md)

### record update (0)

### research (1)

🟢 **W-167** `cowork` Architecture design for graph generation at 100k–1M files: today's pipeline, prior art (Glean, Kythe, SCIP, salsa, Knip, Nx), 2–3 options as a proposal; step 3 reads W-168's numbers. Filed by Arpit (Cowork). [detail](open/W-167-graph-architecture-design.md)


---
