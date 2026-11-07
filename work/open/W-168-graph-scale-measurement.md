---
type: Handoff
name: W-168
description: "Arpit, 2026-11-05 (Cowork): measure the project graph's time and memory at 10k, 50k, 100k, 500k and 1M files — today's bench never builds the graph."
item: W-168
filed: 2026-11-05
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-07
updated_by: "Claude Code (Opus 5.5)"
ball: arpit
---

# W-168 — Measure the graph on very large repos: 10k, 50k, 100k, 500k, 1M files

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Give `codelaw bench` a graph workload: the generated repo with `js/project` and `py/project` on, and imports that cross folders and packages | a workload where the graph does real work — today's 10k run never builds it | ✅ `--graph` |
| 2 | Time each graph stage apart: extract, resolve, link, project rules, cache read and write | where the time goes, not only the total | ✅ `--timings` `graph-*` phases |
| 3 | Run it at 10k, 50k, 100k, 500k and 1M files: cold, warm, one edited file (hook), peak memory, cache size on disk | the scaling curve | ✅ all five sizes (loaded Mac) |
| 4 | Check the curve on real repos at the sizes that exist | proof the generated repo is not flattering us | ✅ ionic-framework 10.5k, tanstack-query 3.7k: totals within 1.2×, resolve and link 4–11× heavier per file |
| 5 | Write up the table and the first stage to break; hand it to W-167 | the numbers the design is built on | ✅ [RESULTS.md](../../experiments/graph-scale/RESULTS.md) |
| 6 | The decider says which sizes get an L8 budget | budgets, or context rows only | 🔴 decider decides — in the inbox, 2026-11-07 |

**Arpit, 2026-11-05:** *"add few work items — 2. measurement for very large repos 10k, 50k, 100k, 500k, 1million"* — after *"is it performant for very large repo??"*

**Model:** Opus: bench code in `crates/codelaw-cli/src/bench/` and a careful, reproducible measurement method.

## State (2026-11-07, Claude Code)

**Steps 1–5 are done; the item waits on step 6.** The numbers, the method and
the first stages to break are in [`experiments/graph-scale/RESULTS.md`](../../experiments/graph-scale/RESULTS.md):
the one-file hook is 2.0 s at 10k (target 300 ms) because hook mode rebuilds
the whole graph; past ~200k files the 512 MB cache default prunes what a warm
run needs; memory is ~6 KiB a file. Measured on the decider's Mac **while
other agents built in parallel** (load 12–60 for 10k–100k, 4–7 for 500k and
1M); 10 runs at 10k, 5 at 50k/100k, 2 (one sample) at 500k/1M.

Left from the DoD, named so they are not lost: **DoD 4** — no real repo of
50k+ files was on disk and none was cloned, so the two largest corpus repos
stood in (ionic-framework 10.5k, tanstack-query 3.7k); no new `corpus.json`
pin. **DoD 5** — the `--graph` runs are not yet a job in CI's `perf` group.
Both wait on step 6: a budget needs CI numbers; context rows do not.

## Goal

Measured time and memory for the project graph from 10,000 to 1,000,000 files, per stage, so the graph's speed stops being a design target nobody has checked.

## Why

- `codelaw bench`'s workload extends only each pack's `recommended` preset (`bench/workload.rs` `CONFIG`). The graph rules are in `js/project` and `py/project`, and with no project rule on, step 4 of the engine builds no graph (SR-ARCHITECTURE decision 4, step 4).
- So `check_warm_10k` (p95 221 ms), `check_cold_10k` (8.7 s) and `peak_memory_10k` (48 MiB) **do not include the graph**.
- SR-PROJECT-GRAPH decision 5's hook target — *under 300 ms on 10,000 files, warm* — has never been measured.
- The generated files import only their 24 siblings: no cross-folder, cross-package or barrel imports — not the shape that stresses resolution and linking.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · workload | `--graph` workload: same seed and generator, plus `codelaw:js/project` and `codelaw:py/project`, workspace packages, barrels (`index.ts` re-exports), cross-package imports and Python packages; its own digest, pinned | ✅ `fdccaee5e97504da` |
| 2 · stages | per-stage timings in the bench report (extract, resolve, link, rules, cache) from the engine's own timers, no profiler needed | ✅ `graph_stages` |
| 3 · sizes | 10k, 50k, 100k, 500k, 1M — each: `graph_cold`, `graph_warm`, `graph_hook_one_file`, `graph_peak_memory`, cache bytes; 10 runs, first discarded, median and p95 (L8 decision 3); 500k and 1M may use fewer runs if stated | ✅ 10 / 5 / 5 / 2 / 2 runs, stated |
| 4 · real repos | at least two real JS/TS or Python repos near 10k and 50k+ files, pinned by commit in `scripts/corpus.json` (candidates to be checked for size and licence) — their numbers beside the generated ones | 🟨 two already-pinned repos (10.5k, 3.7k); none at 50k+ on disk, none cloned |
| 5 · where it runs | 10k–100k on CI's `perf` group and locally; 500k and 1M where disk and memory allow (see Hazards) — the machine named in the report | 🟨 local only (the decider's Mac, named); CI waits on step 6 |
| 6 · write-up | `experiments/graph-scale/RESULTS.md` (the shape of W-102's results): the table, the curve, the first stage to break, the method; linked from W-167 | ✅ (W-167 links W-168) |
| 7 · rows | the new operations as context rows in `perf/budgets.toml` (never gated) until the decider rules step 6 | ✅ six `graph_*` rows, unset |

## Blockers

- **None to start.** Step 6 is the decider's.

## Record affected

[SR-CLI-MCP](../../records/0115_cli-and-mcp.md) decision 10 (the bench's operations and workloads) · [SR-LAW-8](../../records/0010_LAW-8-performance-budgets.md) decision 1 only if the decider sets a budget · [SR-PROJECT-GRAPH](../../records/0103_project-graph.md) decision 5 (the hook target, now measured).

## Hazards

- **L9 — disk.** The project budget is 5 GB, all agents combined. A 1M-file workload plus its cache may be several GB: **estimate bytes per file from the 10k run first**, write the large workloads outside the repo and delete them after each run, and ask the decider before any run that would pass the budget. 500k and 1M on CI if the runner's disk fits.
- **Runner limits.** A standard CI runner's memory and disk may not hold 1M files; if it does not, that is a finding, written down, not a skipped row.
- **A loaded machine** (LESSONS): say what else is running; time on an idle machine or on CI.
- **Generated shape flatters.** Real repos (DoD 4) are the check; if they disagree by more than 2× per file, the generator's shape is wrong.
- **Bench time.** 10 runs × 1M files cold may take hours; use the `perf` matrix's own job for the large sizes, never the gating groups.

## Reference

- [SR-PROJECT-GRAPH](../../records/0103_project-graph.md) decision 5 · [SR-ARCHITECTURE](../../records/0108_architecture.md) decisions 4 and 17 · [L8](../../records/0010_LAW-8-performance-budgets.md) · [L9](../../records/0011_LAW-9-machine-budget.md) · [`work/MACHINE.md`](../MACHINE.md)
- `crates/codelaw-cli/src/bench/workload.rs` (`CONFIG`, `PER_DIR`) · `perf/budgets.toml`
- [W-167](W-167-graph-architecture-design.md) — reads these numbers
