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
| 🔴 **W-168: which graph sizes get an L8 budget?** ([results](../experiments/graph-scale/RESULTS.md)) Measured on the loaded Mac: the one-file hook is 2.0 s at 10k (SR-PROJECT-GRAPH's target is 300 ms — not met: hook mode rebuilds the whole graph), full `check` 6.3 s cold / 2.3 s warm at 10k, 56 s / 14 s at 100k, ~20 min at 1M, where the 512 MB cache default makes warm nearly cold. **Pick:** (1) *recommended*: no budget yet — the six `graph_*` rows stay context until W-167's design is built, then budget 10k and 100k on CI; (2) budget 10k now from CI's numbers (decision 4's method), 100k+ context; (3) another set of sizes. | 2026-11-07 |
| ↳ **blocks:** W-168 | |

---

## Open items (13)

### build (6)

🟢 **W-171** `agent` Six worktrees (W-139, W-141, W-156, W-162, W-168, W-169) hold unlanded work of agents stopped by the usage limit 11-05. Ruled 11-07: keep; each lands with its item. Filed by now-ledger.py (Claude Code). [detail](open/W-171-session-cleanup.md)

🟢 **W-105** `agent` Type-aware TypeScript: nine rules; helper batched, version watch proven 11-05 (PR #7, issue #6). Budgets wait on three `perf` runs, skipped until W-126's `9b8ca7b7` is pushed; next the RC (11-10). Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-105-type-aware-engine.md)

🟢 **W-139** `agent` Rule parity: js, Ruff, Pylint, testing, nextjs, 5 PyPI packs. 11-05: manual ceiling removed, the 231 held ports landed. Next: step 3 — corpus counts, L8 budgets. Filed by Arpit (Cowork). [detail](open/W-139-rule-parity-build.md)


🟡 **W-143** waits on **W-139** `agent` Vue, Svelte, Astro, GraphQL as codelaw languages (L3, L7; a parser and a formatter each, grammars ratified under L11) and Biome's 89 rules for them. Filed by Arpit (Cowork). [detail](open/W-143-vue-svelte-astro-graphql.md)

🟢 **W-156** `agent` Fix ladder (L4). Engine and lowering built; 168 capability-blocked rules marked (W-162). The `match:` moves done; native moves under way (ts landed): 176 rules left to move. Next: the next pack's native fixes. Filed by Arpit (Claude app). [detail](open/W-156-fix-ladder.md)

🟢 **W-162** `agent` Build the fix capabilities W-161 designed ([proposal](proposals/fix-capabilities.md)). Computed edits: filters, `transform:` and `edit-text` built (27 `js/*` moved; 76 still wait). Next: `unwrap-block`, `move`, native fixes, then scope rename. Filed by Arpit (Cowork). [detail](open/W-162-fix-capabilities-build.md)



### ci/cd (4)

🟢🧨 **W-169** `agent` `codelaw rules lint` segfaults in CI now and then (arm64-musl on js, x64 on react; the same commit passes on rerun). Next: a core dump and backtrace from CI, then the fix. Filed by Arpit (Claude Code). [detail](open/W-169-rules-lint-segfault.md)

🟢 **W-126** `agent` Ruled 10-30: `perf` split, nothing waits on it (until a budget is set, L8); release 3m50 accepted. Cached CI 4m11–4m25 ✅. `perf` matrix built 11-05; first runs skipped it (arm64-musl red). Next: three runs once pushed. Filed by Arpit (Cowork). [detail](open/W-126-fast-pipeline.md)

🟢 **W-95** `agent` VS Code extension: Marketplace live (0.4.0); Open VSX by trusted publishing in `publish.yml` since 11-05, proven by the next release. Left: a VSCodium install after it (Open VSX's darwin-arm64 is 0.3.0). Filed by Arpit (Cowork). [detail](open/W-95-publish-vscode-extension.md)

🟡 **W-142** waits on **W-139** `arpit` New packs' registry names: five PyPI framework packs (pending publishers), npm `testing`/`nextjs` (was W-147), Tailwind's and `data-browsers` (W-166), with Fux indexes; before their release. Filed by Arpit (Cowork). [detail](open/W-142-framework-pack-names.md)

### cleanup (0)

### testing (0)

### reporting and benchmarks (2)

🔴 **W-168** `agent` Steps 1–5 done 11-07: `codelaw bench --graph`, per-stage `--timings`, 10k–1M measured ([results](../experiments/graph-scale/RESULTS.md)) — first to break: the one-file hook (2.0 s at 10k vs 300 ms), then the 512 MB cache default past ~200k. Waits on step 6 (budgets). Filed by Arpit (Cowork). [detail](open/W-168-graph-scale-measurement.md)

🟡 **W-102** waits on **W-139** `agent` Ruled 11-05: the 7 tempting tasks at 9 runs per arm (~$65), `PREREG.md` amended first — and run only once every rule is built (voice). Rework, turns and tokens: B-072. Filed by Arpit (Cowork). [detail](open/W-102-guidance-vs-check-experiment.md)

### record update (0)

### research (1)

🟢 **W-167** `cowork` Architecture design for graph generation at 100k–1M files: today's pipeline, prior art (Glean, Kythe, SCIP, salsa, Knip, Nx), 2–3 options as a proposal; step 3 reads W-168's numbers. Filed by Arpit (Cowork). [detail](open/W-167-graph-architecture-design.md)


---
