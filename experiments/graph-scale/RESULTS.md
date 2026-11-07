# The project graph at 10k to 1M files — results (W-168)

Measured 2026-11-05 22:13 → 2026-11-07 10:34 by `codelaw bench --graph`
(a release build of 0.4.3 at the W-168 branch), on the decider's Mac:
macOS on Apple silicon, 10 threads, 16 GiB of memory.
**The machine was loaded.** The 10k–100k runs ran while other agents built in
parallel (load average 12–60); the 500k and 1M runs ran at load 4–7. Read
every number as an upper bound from a shared developer machine, never as an
L8 budget, which is measured on CI's ubuntu-24.04 runner (L8 decision 2).
The raw reports are the `graph-<n>.json` files beside this one.

Read by [W-167](../../work/open/W-167-graph-architecture-design.md) (step 3).
Ruled 2026-11-07 (Arpit, W-168 step 6): *"No budget yet (Recommended)"* — context-only until W-167's design is built; then 10k and 100k get budgets from CI runs.

## Method

- **Workload** — `codelaw bench --graph --files <n>`: the reference generator
  (seed 20261018), then every TypeScript and JavaScript folder a workspace
  package with an `index` barrel, a root `package.json` with workspaces and
  a `tsconfig.json` `@ts/*` alias. Each script module imports a package by
  name through its barrel, one in two a `@ts/*` alias, one in four a sibling
  folder by `../`; each Python module imports another folder's module by
  its dotted path (`services/py<n>/` are namespace packages: the reference
  generator writes each folder's `__init__.py` to `services/s<n>/`, so no
  Python barrel re-exports — a gap in the shape, named here). Only each module's helper and
  the one-in-eight finding stay exported, so about one export in ten is
  unused, as in a real repo. `codelaw.toml` extends only `js/project` and
  `py/project` (preview). The 10k digest is pinned: `fdccaee5e97504da`.
- **Rows** — `graph_cold` (empty cache, `check --timings --format json`),
  `graph_warm` (`--prefer-warm`, the daemon), `graph_warm_no_daemon` (primed
  cache, own process), `graph_hook_one_file` (`codelaw hook <file>` after
  adding an export to one module), `graph_peak_memory` (`ru_maxrss` of the
  cold runs), `graph_cache_size` (the cache folder's allocated bytes).
- **Runs** — 10 at 10k, 5 at 50k and 100k, 2 at 500k and 1M; the first run
  is always discarded, so 500k and 1M are **one sample each**.
- **Stages** — the engine's own `--timings` phases (new in W-168):
  `graph-read`, `graph-facts` (wall), `graph-extract`, `graph-cache-read`
  and `graph-cache-write` (summed over the 10 threads), `graph-index`,
  `graph-resolve`, `graph-link`, `graph-rules`. `read` and `format` are the
  file stage's (every file is read and format-checked by `check`).
- **Disk** — each size was generated under the session scratchpad, outside
  the repo, measured, and deleted before the next. The 1M workload was
  4.1 GB plus its caches; it fit in the free disk (23 GB at the start).

## The table

Wall times in seconds (median; p95 in brackets where there were 4+ samples).

| files | workload | cold | warm (daemon) | warm (own process) | hook, one file | peak memory | cache on disk | findings |
|---|---|---|---|---|---|---|---|---|
| 10,370 | 58 MiB | 6.3 (6.9) | 1.9 (2.5) | 2.3 (3.0) | **2.0** (4.5) | 226 MiB | 70 MiB | 3,133 |
| 51,842 | 220 MiB | 52.2 (55.8) | 8.2 (9.9) | 18.5 (31.9) | 7.6 (8.1) | 708 MiB | 349 MiB | 15,620 |
| 103,682 | 422 MiB | 56.3 (71.3) | 13.9 (15.6) | 13.9 (15.5) | 12.1 (12.4) | 1,405 MiB | 698 MiB | 31,144 |
| 518,402 | 2.0 GiB | 377 | 255 | 182 | 197 | 3,828 MiB | 1,312 MiB | 155,851 |
| 1,036,802 | 4.0 GiB | 1,186 | 1,016 | 955 | 236 | 6,126 MiB | 1,285 MiB | 311,355 |

Stage medians, cold, in seconds (`extract`, `cache-write`, `cache-read` and
`format` summed over threads; the rest wall):

| files | graph-read | graph-facts (wall) | extract | cache-write | cache-read | index | resolve | link | rules |
|---|---|---|---|---|---|---|---|---|---|
| 10k | 0.52 | 1.9 | 11.4 | 7.1 | 0.38 | 0.005 | 0.05 | 0.13 | 0.48 |
| 50k | 6.7 | 13.8 | 77.7 | 55.9 | 3.0 | 0.04 | 0.65 | 1.8 | 5.5 |
| 100k | 7.6 | 16.2 | 79.6 | 75.7 | 4.0 | 0.06 | 0.43 | 1.1 | 3.5 |
| 500k | 38.5 | 77.0 | 306 | 432 | 27.4 | 0.24 | 1.8 | 4.8 | 11.9 |
| 1M | 105 | 251 | 721 | 1,510 | 271 | 0.69 | 5.4 | 14.6 | 46.5 |

Warm, own process (`graph_warm_no_daemon`):

| files | graph-read | graph-facts (wall) | cache-read | extract | index + resolve + link | rules |
|---|---|---|---|---|---|---|
| 10k | 0.44 | 0.21 | 2.0 | — | 0.14 | 0.39 |
| 50k | 4.7 | 1.4 | 13.4 | — | 1.7 | 3.4 |
| 100k | 2.5 | 1.7 | 17.0 | — | 1.0 | 2.0 |
| 500k | 40.2 | 24.1 | 111 | **63** | 6.7 | 12.0 |
| 1M | 83 | 158 | 373 | **313** | 16.3 | 25.4 |

## Real repos (step 4)

No real repo of 50k+ files is on disk and none was cloned. The two largest
pinned corpus repos (`scripts/corpus.json`, already checked out under
`target/corpus/repos/`) were copied to the scratchpad with the same
`codelaw.toml` and timed with `check --timings`, 5 runs after one discarded:

| repo (commit) | files | cold | warm (own process) | peak memory | resolve / link, cold |
|---|---|---|---|---|---|
| ionic-framework (`25a114f`) | 10,524 | 5.7 s | 2.7 s | 207 MiB | 0.33 / 0.56 s |
| tanstack-query (`cb3c3d5`) | 3,701 | 3.0 s | 1.4 s | 203 MiB | 0.18 / 0.09 s |
| generated, 10k | 10,370 | 6.3 s | 2.3 s | 226 MiB | 0.05 / 0.13 s |

Per file, the totals agree within 1.2×. **Resolve and link do not:** ionic's
are 7× and 4× the generated workload's per file (tanstack-query's resolve
11× per file) — real imports are deeper (`exports` maps, `paths`, many
specifiers per file). That is past the item's 2× hazard, so the generated
shape **flatters resolve and link**; at 1M their generated 20 s could be
80–150 s on a real monorepo. Every other stage agrees.

## What breaks first

1. **The one-file hook, already at 10k.** SR-PROJECT-GRAPH decision 5's
   target is under 300 ms on 10,000 files, warm; the measured median is
   **2.0 s** (p95 4.5 s), 7× over, and 12 s at 100k. The hook rebuilds the
   whole graph: it reads every file again (`graph-read`), asks the cache for
   every file's facts, links every module and runs every project rule — the
   decision's "re-extract that file, relink only its edges" is not built.
   At 10k warm the parts are `graph-read` 0.44 s, every cache lookup 2.0 s
   (summed; 0.21 s wall), link 0.14 s, rules 0.39 s, plus the file stage.
2. **The cache's default size, between 100k and 500k.** `[cache] max_size`
   defaults to 512 MB (SR-CACHE); a 100k run fills 698 MiB on disk, and at
   500k and 1M the cache is pruned below what one run wrote (336k and 329k
   of the entries kept). **Warm runs stop being warm**: at 500k a primed run
   re-extracts (63 s summed) and re-writes; at 1M warm (955 s) is 80 % of
   cold (1,186 s). Every stage reads per-file cache entries, so the cache's
   file count (one file per key) is also the cold run's largest cost:
   `graph-cache-write` 1,510 s summed at 1M, more than extraction.
3. **Memory grows linearly at ~6 KiB a file**: 226 MiB at 10k, 1.4 GiB at
   100k, 6.1 GiB at 1M — a third of this machine's 16 GiB.
4. **Not first, but superlinear here:** `graph-read` (0.5 s → 105 s, 200×
   for 100× files) and `graph-rules` (one `LineIndex` built per finding);
   linking, resolving and indexing stay small and roughly linear on the
   generated shape (but see the real-repo caveat above).

So: the hook is past its target at every size measured; full `check` holds
to 100k in about a minute cold and 14 s warm; past ~200k the cache ceiling
makes every run nearly cold, and 1M takes 20 minutes either way.
