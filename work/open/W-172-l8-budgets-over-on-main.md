---
type: Handoff
name: W-172
description: "main is over three set L8 budgets on the reference runner: check_cold_10k, peak_memory_10k and mcp_rules_for_path (nightly perf run 42316894936 on 3b53d1a8, found by W-139 step 3); optimise, never loosen."
item: W-172
filed: 2026-11-07
filed_by: "Claude Code (session 0a172e1f), W-139 step 3"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5): the value ruled — peak_memory_10k to 90 MiB"
ball: agent
---

# W-172 — main is over three L8 budgets

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Find what moved each number between `9ae07264` (last green `perf`) and `3b53d1a8` | the cause, not a guess | ✅ 11-07 — a per-rule cost, paid for every rule a project's packs ship (below) |
| 2 | Make a rule nobody turns on cheap: packs kept in memory, dormant matchers dropped, the settings lookup skipped, `where:` before captures | `mcp_rules_for_path` back under; peak memory down by about half its rise | ✅ 11-07 — built, tests green; numbers below |
| 3 | Bring `peak_memory_10k`, `check_cold_10k` and `lsp_after_edit` under: what is left is the rules that are **on** — W-139 turned about 130 more on in the recommended presets | all three green | 🔶 11-08 second pass: time side built (below); `peak_memory_10k` still over on the Mac — *What is left* |
| 4 | CI's proof: the `perf cold` / `perf warm` jobs on a pushed `main` | the reference runner's numbers | ⏳ waits on the decider pushing `main` |
| 5 | Amend L8 so `peak_memory_10k` is raised to a value the decider names, instead of optimising the last ~14 MiB | the memory budget met by ruling, not by trade-offs against time | ⏳ ruled 11-09: **90 MiB** — the agent amends L8 and sets it |

## Ruling, 2026-11-09 — amend L8 for `peak_memory_10k`

**Arpit (voice, Cowork docket walk-through), 2026-11-09**, after the memory levers in *What it would take to reach 72* were read to him: *"Update the budget. This looks good."* Told that L8 says a budget is never loosened, so raising it means amending L8 itself: *"Amend the [L8]."* (the transcript reads *"alert"*; it answered the question whether to amend L8).

- **The value — 90 MiB.** Arpit (voice, Cowork docket, 2026-11-09), offered 86 (as measured, 86.1 p95) or 90 (headroom against run-to-run noise, recommended): *"Let's go with 90."* Filed on *"file it."*
- **Next (agent):** amend [SR-LAW-8](../../records/0010_LAW-8-performance-budgets.md) with this ruling named in it (how a budget may be raised: by the decider's ruling, by name, per budget), set `perf/budgets.toml`, and update CLAUDE.md's L8 line (SR-WORK-DOCS decisions 6–8). The time budgets (`check_cold_10k`, `lsp_after_edit`) are not covered by this ruling: they still wait on step 4's CI run.
- The memory levers below stay recorded as options; none is required once the budget is raised.

## What is known

The `ubuntu-24.04` reference runner ([SR-LAW-8](../../records/0010_LAW-8-performance-budgets.md) decision 2), p95:

| operation | budget | run 41945322410 on `9ae07264` (11-05 nightly) | run 42316894936 on `3b53d1a8` (11-07 nightly) |
|---|--:|--:|--:|
| `check_cold_10k` | 13,118 ms | 6,715 ms | **13,465 ms — over** |
| `peak_memory_10k` | 72 MiB | 60.2 MiB | **124.2 MiB — over** |
| `mcp_rules_for_path` | 65 ms | 40.5 ms | **128.7 ms — over** |
| `check_warm_10k` | 331 ms | 191.7 ms | 252.9 ms |
| `format_warm_10k` | 2,379 ms | 1,132 ms | 1,436 ms |
| `lsp_after_edit` | 5 ms | 2.2 ms | 3.8 ms |

`3b53d1a8` is the push of 2026-11-05 15:19 UTC that carried most of W-139's rules (689 → 2,122 across the packs), W-156's fix ladder, W-166 and W-162. `mcp_rules_for_path` and peak memory roughly doubling suggests a per-rule cost, paid for every installed rule, not just the enabled ones. That is a hypothesis for step 1.

**Not the 233 rules of 2026-11-05 alone:** W-139 step 3 benched today's packs against the same packs without those 233 rules, on one release binary on a loaded Mac (load average 21–67). The arms did not differ beyond noise: cold 6.83 s against 6.74 s in the quietest pair, and peak memory 127–160 MiB in both arms.

## Step 1 — the cause (2026-11-07)

**No engine commit to bisect to.** The 11-05 and 11-07 engines do not load each other's packs (122 and 615 rule-file problems), so a commit bisect mixes two arms. Measured instead with one binary per arm, on the reference workload (seed 20261018, 10,000 files), a Mac (10 cores) with the load average printed beside each run:

- **Rules in no preset cost as much as rules that run.** The workload's packs load 1,581 rules; 474 are on, 1,107 off (11-05: 443 loaded, 344 on). One `main` binary, the same packs with the 908 rules no preset names removed: `rules --for` 110 → 70 MiB and 0.41 → 0.22 s.
- **Where the memory went** (`heap --sortBySize`, `malloc_history -callTree -invert` on a live `codelaw mcp`): 94 MB of heap, ~60 MB of it `regex` automata — 3,350 regexes, every `regex:` of every loaded rule compiled once per grammar (a JS rule has three) — the rest the parsed rule files (twice: each was copied into the rule set).
- **Where `mcp_rules_for_path` went** (`sample`): every call re-read and re-parsed all ~2,200 pack files (YAML, `open`, blake3), then copied each rule.
- **Where `check_cold_10k` went** (`--timings`, `--threads 4`): `rules` 8–10 s of CPU on 11-05, 17–18 s now — the newly-on rules (`js/promise-param-names`, `js/no-unused-function-parameters`, `js/promise-valid-params`, `js/no-redeclare`, `js/no-unused-imports`, …); `compile` 30 → 150–940 ms, one thread.

## Step 2 — what changed (2026-11-07)

- **Packs stay loaded** in a process, keyed by every read file's size and mtime (SR-CACHE decision 8), and a rule set shares the pack's rules instead of copying them (`codelaw-rules`: `load_pack_cached`, `RulePack.rules: Vec<Arc<RuleFile>>`).
- **A dormant rule keeps no matcher** (SR-ARCHITECTURE decision 15): every rule is still compiled at load, so a broken one still refuses the run (SR-RULE-PACKS decision 13); only a rule a preset turns on keeps its matchers, any other is compiled again the first time a file's settings turn it on (`MatchSet`, a `OnceLock`). Compiling is one rule at a time — in parallel each thread kept its own leftovers (10 MB more).
- **A file asks only the rules that can be on** — a preset default or a layer naming it — for globs and settings (`lint.rs`).
- **`where:` and `auto_when:` before captures** (`codelaw-match`): a node the predicate turns down copies no capture text. `js/no-redeclare` 375 → 149 ms.
- Smaller: no slack in a rule's example lists, the parsed block moved not cloned, no buffer of whole rules kept.

**Before/after, `codelaw bench --only check_cold_10k,peak_memory_10k,mcp_rules_for_path --runs 6`, the same packs (`7cdb480b`'s), both binaries release, interleaved:**

| operation | budget | `main` (`7cdb480b`) median / p95 | W-172 median / p95 | load |
|---|--:|--:|--:|--:|
| `mcp_rules_for_path` | 65 ms | 112.1 / 115.9 ms | **10.8 / 12.4 ms** | 6–7 |
| `peak_memory_10k` | 72 MiB | 158.6 / 166.8 MiB | 106.9 / 108.7 MiB | 6–7 |
| `peak_memory_10k` (round 2) | | 166.7 / 168.5 MiB | 103.3 / 111.7 MiB | 11–15 |
| `check_cold_10k` | 13,118 ms | 3,639 / 3,851 ms | 3,881 / 4,810 ms | 6–7 |

`check_cold_10k` on this Mac is noise-bound (three more interleaved rounds at load 15–40: 8.1–12.8 s against 7.3–10.6 s, either way), and both arms are far under its budget here, so the Mac does not decide it. At `--threads 4` (the runner's 4 vCPUs), peak footprint is 110–121 MB on `main` and 62–64 MB with W-172, 32–43 MB on 11-05. `codelaw test` on js, ts, react, python, data, css, html, scss and security-js: 0 failed; `cargo test --release -p codelaw-rules -p codelaw-match -p codelaw-engine` and eleven `codelaw-cli` test files green (`warm`'s socket test timed out once at load 130 and passes alone).

## What is left (step 3)

**`peak_memory_10k` is still over on this Mac** (103–112 MiB against 72), and a two-point fit of the 11-05 and 11-07 runner numbers to the Mac's footprint puts the runner near 80 MiB. Measured floors (one loaded run each): dropping every dormant rule outright saves ~5–7 MB more, clearing every rule's docs fields ~2–4 MB. What remains is the **rules that are on**: their regexes (~12 MB, ast-grep builds each per grammar) and matchers. `check_cold_10k` is the same: the newly-on rules' cost is their matchers — most are anchored on `identifier` and ask `inside:`, and tree-sitter's `parent()` walks down from the root (`ts_node_child_with_descendant` tops a `sample`). Next, in order:

1. Push `main` (the decider) and read the runner's three numbers — the fit may be wrong either way.
2. Re-anchor the hot newly-on `js/*` rules on their parent kinds with `has:` instead of `identifier` + `inside:` (packs/js — W-139-js and W-162 hold those files; `codelaw test packs/js` proves each).
3. A rule's regex compiled once, not once per grammar, needs ast-grep's matcher built from a shared `Regex` — not possible through its public API (0.51); a dormant rule's body could be parsed only when asked for.

## Hazards

- L8 decision 5: a budget is never raised. The fix is in the engine or the packs, never in `perf/budgets.toml` — the one exception, once SR-LAW-8 is amended, is `peak_memory_10k` to 90 MiB (the ruling above); every other budget is unchanged.
- A loaded developer Mac is not the reference machine; take the numbers from `perf` runs, or from a quiet machine with both arms on one binary.
- `git stash` is one stack for every worktree (LESSONS 2026-11-07).

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) step 3 · [SR-LAW-8](../../records/0010_LAW-8-performance-budgets.md) · [SR-ARCHITECTURE](../../records/0108_architecture.md) decision 15 · [SR-CACHE](../../records/0109_cache.md) decision 8 · W-126 (the `perf` split) · W-168

## 2026-11-08 — step 3 landed by the coordinator (agent stopped: disk full)

The agent's in-progress change — each distinct `regex:` compiled once and shared across rules and grammars (codelaw-match), rule examples and cases read on demand (codelaw-rules), packs kept per process from their second load and identified by a stamp hash — was verified before landing: `cargo build -p codelaw-cli`; `codelaw test` 0 failed on every pack; `cargo test` codelaw-cache 7, -engine 74, -match 43, -rules 30, -test 8, -config 29; codelaw-cli `--test laws --test commands`. **Not re-measured** — a debug build's numbers would mislead and a release build did not fit the disk that day. Next: `codelaw bench --only peak_memory_10k,check_cold_10k,mcp_rules_for_path` on a release build, then CI once `main` is pushed.

## 2026-11-08 — second pass: where the time went, and what was left in `main`

**CI's fact (run 42454141410 on pushed `main` `b20e1a1f`, ubuntu-24.04, 4 threads, p95):** `check_cold_10k` 13,388 ms against 13,118 (over), `peak_memory_10k` 86.1 MiB against 72 (over), `lsp_after_edit` 6.3 ms against 5 (over; 3.8 ms on 11-07). `mcp_rules_for_path` 22.7 ms, `check_warm_10k`, `format_warm_10k`, `startup`: fine. **That run is before step 3's landing** (`5e03af7a`, which is after `b20e1a1f`), so it measures none of step 3.

**A leftover in step 3 as landed.** `crates/codelaw-cli/src/main.rs` carried an experiment (`mod xx_alloc // XX-EXPERIMENT`): a counting `#[global_allocator]` with an atomic add and an atomic max on every allocation of every thread, behind `XX_TRACE` / `XX_ALLOC`. It was never meant to land and would have cost every run on the next push. Removed; `main.rs` is byte-identical to `b20e1a1f`'s again.

**The regression of `lsp_after_edit` and most of `check_cold_10k`: tree-sitter's `parent()`.** `sample` of a running `codelaw lsp` (1,500 edits of the bench's file): about 40 % of the CPU of an edit is `ts_node_parent` → `ts_node_child_with_descendant` → `ts_node_child_iterator_next` — tree-sitter finds a node's parent by walking down from the root, `O(depth × siblings)` per call, and `nthChild:` and `inside:` ask for it. `--timings` over the 10,000 files (4 threads) put four rules on top, all anchored on `identifier` or `call_expression` and asking for parents on every such node: `js/promise-param-names` (368,118 calls, 2.8 s of CPU), `js/no-unused-function-parameters` (311,732; 1.6 s), `js/promise-valid-params` (1.3 s), `js/no-unused-imports`; then two natives: `js/no-useless-assignment` (9,760 calls, 1.4 s: it asked every ancestor for its parent, `O(depth²)`) and `html/valid-id` / `html/unrecognized-char-ref` (0.8 s each: a unicode regex compiled, and an entity set of 2,231 names built, **for every file**).

**What was built (engine-level; no pack file touched):**

- `codelaw-match` `order` — a `match:` object's keys tried cheapest first (SR-LINTER 2026-11-08): `js/promise-valid-params` went from 1.3 s to under 0.2 s because its `has: { field: function, regex }` is now tried before its `matches: two`.
- `codelaw-match` `gate` + `codelaw_lang::preorder_parents` + `lint.rs` — a rule whose every alternative needs a given parent kind (`inside:` at `stopBy: neighbor`) is not asked about a node with another parent (SR-ARCHITECTURE decision 15 note): `js/promise-param-names` 368,118 → 90,913 calls, 2.6 s → 0.46–0.75 s; `js/no-unused-function-parameters` 311,732 → 89,209 calls, 1.3 s → 0.4–0.6 s. Findings over the whole workload are identical, line for line, before and after (1,520).
- `js/no-useless-assignment` walks the ancestors of a node once (1.4 s → 0.6–1.0 s); the three html natives build their regex (and the entity set) once (0.8 s each → below the top twenty).
- The per-node rule list reuses one buffer (it was a `Vec` clone per node).
- Test proof: `codelaw test` on all 27 packs, 0 failed; `cargo test -p codelaw-lang -p codelaw-match -p codelaw-engine -p codelaw-native -p codelaw-lsp`; clippy `-D warnings` on those and `codelaw-rules`.

**Measured** (`codelaw bench --only check_cold_10k,peak_memory_10k,lsp_after_edit`, one Mac of 10 cores, **load average 14–35 the whole time** — other agents building — so a number here is a direction, not a result; the ratio of the runner to this Mac is about 2 for `lsp_after_edit` (CI 6.3 ms, this Mac 3.1–3.4 on `main` with the counting allocator)):

| | `main` + counting allocator (before) | after, allocator gone | load |
|---|--:|--:|--:|
| `check_cold_10k` median | 9.6 / 11.1 s | 6.3–7.0 s | 14–35 |
| `lsp_after_edit` median | 3.1 / 3.4 ms | 2.7 ms (1.9–2.3 before the allocator was removed, on a quieter minute) | 14–35 |
| `peak_memory_10k` median | 87.8 / 86.9 MiB | 84–88 MiB | 14–35 |

**`peak_memory_10k` is not fixed.** The Mac's 84–88 MiB is the runner's 86.1 (the two agree to 1 MiB), so it is still about 14 MiB over 72. Where it is (`MallocStackLogging` on a live `codelaw lsp`, `/usr/bin/time -l`): a one-file project with the nine recommended presets is 50 MiB of RSS (34 MiB footprint) — **the fixed cost of loading packs and compiling the rules that are on**, 21.7 MiB of live heap: pack load 10.3 (rule files 4.3, data tables 3.3 — `serde_json` trees of ~400 KB of JSON —, one `Arc<RuleFile>` box per rule 2.1), compile 10.3 (regexes 4.2, `match:` cores 1.8, `target:` rules 1.8). A 10,000-file run adds ~18 MiB on 4 threads (~2.3 MiB a thread, ~7 MiB for the file list and results). What is left, by size: data tables kept as `serde_json` trees (3.3 MiB; a compact or lazily parsed table), the regexes of the rules that are on (4.2 MiB; ast-grep builds each with `Regex::new`, no builder), `RuleFile`'s inline size (2.1 MiB), per-thread allocator arenas on glibc (not measurable on a Mac).

**Left:** CI on the next push is the proof for all three numbers; do not call a budget met before a reference-runner run says so. If `lsp_after_edit` is still over, the next lever is the same one at the pack level — re-anchor `js/promise-param-names` and `js/no-unused-function-parameters` on `new_expression` / `function` kinds with `has:` (packs/js; W-139-js holds those files). Memory: the three items above.

## 2026-11-09 — round 2: memory, honestly short of 72 MiB

**Baseline** (one release build of `main` `7b4832de`, copied away before measuring; Mac, load 9–32): `peak_memory_10k` 84.6 MiB median (79.9–91.7), 88.6 / 85.8 in later runs; a one-file project 47 MiB RSS; `main` is still ~14 MiB over 72 and the Mac still matches the runner's 86.1.

**Where the RSS is** (`vmmap`, `malloc_history`, `/usr/bin/time -l`, mid-run at 4 threads): resident 81 MiB = **malloc zone 38 MiB dirty (26 live + 12.6 fragmentation)** + **code 23 MiB** (`__TEXT`: 22 MiB of code in a 39 MB binary, paged in as languages are first used — the RSS grows through the run as html, css, md, py files bring in their grammars and rules; one thread: 56 MiB, four: ~67, eight: ~79) + ~3 MiB other. Live heap is only a third of RSS. Live heap of the fixed part (21.1 MiB, `codelaw lsp` on a one-file project): pack load 10.3 (rule files 4.2, data tables 3.3 — `serde_json` trees of ~400 KB of JSON —, one `Arc<RuleFile>` box per rule 2.1), compile 10.3 (shared regexes 4.2 — 1.6 of it the one-pass DFA each regex builds, `regex` has no switch for it through ast-grep —, `match:` cores 1.8, `target:` schemas 1.8 of which 0.8 regexes).

**Built (small, measured by live heap, deterministic):** a `schema:` predicate compiles each pattern on first use (−1.8 MiB); `graph:`/`target:` blocks held as text (−0.9 MiB parsed trees); `malloc_trim` after compile (glibc only: the loading thread's arena holes are not reused by the workers — a hypothesis for the Linux runner, no effect measured on macOS). **Result: live heap 21.1 → 19.3 MiB; RSS unchanged within the Mac's ±5 MiB run-to-run noise.** `codelaw test` 0 failed on data, deps, css, html, js, python, react, security-js, ts, testing.

**What it would take to reach 72** (each is a larger change than this item's brief; ordered by size): data tables parsed on first use or compacted (~2–3 MiB; every crate reads `Arc<serde_json::Value>`, the js_parity strand is editing them); rule docs (`summary`, `why`, `steps`) read on demand like examples, and cold `RuleFile` fields boxed (~2–3 MiB; lsp, cli and the docs commands read them); a smaller binary or fewer pages touched (code 23 MiB: `opt-level = "s"` for cold crates, `panic = "abort"` — each trades time budgets); glibc arena limits (`mallopt(M_ARENA_MAX)`; needs a Linux run to price the contention). None is likely to give 14 MiB alone. **The budget is not met and nothing here says it will be; the next push's CI run is the only proof of either number.**
