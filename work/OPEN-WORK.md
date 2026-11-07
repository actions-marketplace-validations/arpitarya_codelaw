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
| 🔴 **`check_types_cold` / `check_types_warm` L8 budgets: 14,602 ms and 8,663 ms?** ([W-105](open/W-105-type-aware-engine.md)): p95 on three `perf` runs on `main` of the batched helper (`ci` 42316894936 on `3b53d1a8`, its nightly and two re-runs of the job; ubuntu-24.04, 2,200 files, TypeScript 7.1.0-beta): cold 9,734.3 / 5,185.9 / 6,010.8 ms, warm 5,775.1 / 5,286.0 / 5,005.8 ms. Your method (L8 decision 4): 1.5 × the worst → **cold 14,602 ms, warm 8,663 ms**; no design target, so no guard. The cold spread is the runner (same commit, same binary). **Set them** (a p95 over fails CI) **or leave them unset** (reported only). | 2026-11-07 |
| ↳ **blocks:** W-105 | |

---

## Open items (13)

### build (6)

🔴 **W-105** `arpit` Type-aware TypeScript: nine rules; helper batched, version watch proven (PR #7, issue #6). `check_types_*` budgets in the inbox (cold 14,602, warm 8,663 ms); next the RC re-pin (11-10). Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-105-type-aware-engine.md)

🟢 **W-139** `agent` Rule parity: js, Ruff, Pylint, testing, nextjs, 5 PyPI packs. 11-07: steps 3–4 done; L8 → W-172. DoD 1 under way: js slices 18–19 (104 rules), py slices 19–20 (101). Next: more js and py rows. Filed by Arpit (Cowork). [detail](open/W-139-rule-parity-build.md)

🟢 **W-172** `agent` main over three L8 budgets on the reference runner (11-07): `check_cold_10k` 13,465 > 13,118 ms, `peak_memory_10k` 124 > 72 MiB, `mcp_rules_for_path` 129 > 65 ms; green 11-05. Next: bisect, optimise. Filed by Claude Code (W-139). [detail](open/W-172-l8-budgets-over-on-main.md)

🟡 **W-143** waits on **W-139** `agent` Vue, Svelte, Astro, GraphQL as codelaw languages (L3, L7; a parser and a formatter each, grammars ratified under L11) and Biome's 89 rules for them. Filed by Arpit (Cowork). [detail](open/W-143-vue-svelte-astro-graphql.md)

🟢 **W-156** `agent` Fix ladder (L4). Engine and lowering built; 139 capability-blocked rules marked (W-162). The `match:` moves done; native moves under way (css landed): 60 rules left to move. Next: the next pack's native fixes. Filed by Arpit (Claude app). [detail](open/W-156-fix-ladder.md)

🟢 **W-162** `agent` Build the fix capabilities W-161 designed ([proposal](proposals/fix-capabilities.md)). Computed edits: 30 `js/*` moved, 33 re-judged `manual`; 40 still wait. Next: `merge-into` and native fixes, then scope rename. Filed by Arpit (Cowork). [detail](open/W-162-fix-capabilities-build.md)



### ci/cd (4)

🟢🧨 **W-169** `agent` `codelaw` segfaulted on the musl build legs now and then. Cause found 11-07 from CI's core: tree-sitter's name lookup + an over-reading `strncmp`; fixed, with tests. Next: five green build runs, once Arpit pushes. Filed by Arpit (Claude Code). [detail](open/W-169-rules-lint-segfault.md)

🟢 **W-126** `agent` Ruled 10-30: `perf` split, nothing waits on it (until a budget is set, L8); release 3m50 accepted. Cached CI 4m11–4m25 ✅. `perf` matrix built; `graph` group added 11-07 (`perf-plan` was red). Next: three runs once pushed. Filed by Arpit (Cowork). [detail](open/W-126-fast-pipeline.md)

🟢 **W-95** `agent` VS Code extension: Marketplace live (0.4.0); Open VSX by trusted publishing in `publish.yml` since 11-05, proven by the next release. Left: a VSCodium install after it (Open VSX's darwin-arm64 is 0.3.0). Filed by Arpit (Cowork). [detail](open/W-95-publish-vscode-extension.md)

🟡 **W-142** waits on **W-139** `arpit` New packs' registry names: five PyPI framework packs (pending publishers), npm `testing`/`nextjs` (was W-147), Tailwind's and `data-browsers` (W-166), with Fux indexes; before their release. Filed by Arpit (Cowork). [detail](open/W-142-framework-pack-names.md)

### cleanup (1)

🟢 **W-171** `agent` Six worktrees of agents stopped by the usage limit 11-05, ruled keep 11-07: five landed and removed (W-139, W-141, W-162, W-168, W-169); W-156's is left, its agent still at work. Filed by now-ledger.py (Claude Code). [detail](open/W-171-session-cleanup.md)

### testing (0)

### reporting and benchmarks (1)

🟡 **W-102** waits on **W-139** `agent` Ruled 11-05: the 7 tempting tasks at 9 runs per arm (~$65), `PREREG.md` amended first — and run only once every rule is built (voice). Rework, turns and tokens: B-072. Filed by Arpit (Cowork). [detail](open/W-102-guidance-vs-check-experiment.md)

### record update (0)

### research (1)

🟢 **W-167** `cowork` Architecture design for graph generation at 100k–1M files: today's pipeline, prior art (Glean, Kythe, SCIP, salsa, Knip, Nx), 2–3 options as a proposal; step 3 reads W-168's numbers. Filed by Arpit (Cowork). [detail](open/W-167-graph-architecture-design.md)


---
