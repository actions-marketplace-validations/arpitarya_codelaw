---
type: Handoff
name: W-167
description: "Arpit, 2026-11-05 (Cowork): design how codelaw builds its project graph so it holds at 100k–1M files — a proposal, measured against W-168's numbers."
item: W-167
filed: 2026-11-05
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Cowork (Opus 5.5)"
ball: cowork
---

# W-167 — Architecture design for graph generation

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Write down how the graph is built today, stage by stage: walk, extract facts per file, resolve, link, query, cache | one picture of the current design, with each stage's cost named | ⏳ |
| 2 | Read how large code graphs are built elsewhere (Meta Glean, Google Kythe, Sourcegraph SCIP, rust-analyzer's salsa, Knip, Nx/Turborepo project graphs, TypeScript project references) | the options that work at 100k–1M files | ⏳ |
| 3 | Put W-168's numbers beside each stage: where time and memory go at 10k → 1M | which stage breaks first, measured, not guessed | ⏳ (needs W-168's runs) |
| 4 | A proposal: 2–3 designs, each with its cost, its risk and what it changes in the records | a shape the decider can pick | ⏳ |
| 5 | The decider picks a design | a build item, with its budgets | 🔴 decider decides |

**Arpit, 2026-11-05:** *"add few work items — 1. architecture design for graph generation"* — after *"how does the graph work in code law?"* and *"is it performant for very large repo??"*

**Model:** Cowork: reading and design, no code (SR-WORK-OPEN-QUEUE rule 37a).

## Goal

A design for building the project graph that stays fast and bounded in memory from 10,000 to 1,000,000 files, written as a proposal the decider can rule on.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · today | `work/proposals/graph-architecture.md` §2 describes today's pipeline from the code (`crates/codelaw-graph`, `crates/codelaw-engine/src/project.rs`), not from the record alone — ground truth over prose | ⏳ |
| 2 · prior art | each system read: what it stores, how it stays incremental, its memory model, its licence (L2) — sources dated | ⏳ |
| 3 · the numbers | W-168's table per stage (extract, resolve, link, query, cache read/write) at each size; the first stage to break named | ⏳ |
| 4 · designs | 2–3 options with what each changes — at least: (a) today's design, tuned; (b) a persisted, linked graph reused across runs (not only per-file facts); (c) sharding by workspace package, linked through package boundaries | ⏳ |
| 5 · per option | time and memory expected at 100k and 1M, hook latency for one edited file, cold vs warm, what the cache holds on disk (L9), laws touched (L5 pure function, L8, L11 — no new dependency without a ruling) | ⏳ |
| 6 · questions | the decider's questions in a §, each with a recommendation | ⏳ |
| 7 · filed | the proposal and its `B-nnn` row in the same change; this item closes when the decider picks | ⏳ |

## Topics the design must answer

- **Data layout:** interned paths and names; symbol ids; adjacency as flat arrays, not maps of maps.
- **Parallel link:** can resolution and linking run per package in parallel, and merge?
- **What is cached:** today per-file facts by content hash; should the linked graph itself persist, and how is it invalidated?
- **Incremental relink:** one edited file → which edges change; cost independent of repo size?
- **Memory ceiling:** the whole graph in memory per project root (SR-ARCHITECTURE decision 17) — what is it at 1M files; is a memory-mapped or on-disk graph needed?
- **Monorepo:** `root = true` splits a repo into separate graphs today; should packages inside one root be shards?
- **Watch mode:** more than 256 changed files reloads the project in full — keep or relink?

## Blockers

- **Step 3 needs W-168's runs.** Steps 1, 2 and the option sketches can start now.

## Record affected

**No SR affected by this item** — it produces a proposal. The build item that follows updates [SR-PROJECT-GRAPH](../../records/0103_project-graph.md) decisions 1 and 5 and [SR-ARCHITECTURE](../../records/0108_architecture.md) decision 17.

## Hazards

- **A design on guesses.** Without step 3 the proposal ranks options by intuition; the numbers decide which stage matters.
- **A dependency by the back door.** Prior-art libraries (an embedded database, a graph crate) need an L11 ruling each; a design that needs one says so as its own question.
- **L5:** the graph must stay a pure function of files and config; a persisted graph must give byte-identical results to a cold build (the existing test).

## Reference

- [SR-PROJECT-GRAPH](../../records/0103_project-graph.md) · [SR-ARCHITECTURE](../../records/0108_architecture.md) decision 17 · [SR-CACHE](../../records/0109_cache.md) · [L8](../../records/0010_LAW-8-performance-budgets.md) · [L9](../../records/0011_LAW-9-machine-budget.md)
- [W-168](W-168-graph-scale-measurement.md) — the numbers this design reads
