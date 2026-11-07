---
type: Handoff
name: W-172
description: "main is over three set L8 budgets on the reference runner: check_cold_10k, peak_memory_10k and mcp_rules_for_path (nightly perf run 42316894936 on 3b53d1a8, found by W-139 step 3); optimise, never loosen."
item: W-172
filed: 2026-11-07
filed_by: "Claude Code (session 0a172e1f), W-139 step 3"
updated: 2026-11-07
updated_by: "Claude Code (session 0a172e1f), W-172 steps 1–2"
ball: agent
---

# W-172 — main is over three L8 budgets

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Find what moved each number between `9ae07264` (last green `perf`) and `3b53d1a8` | the cause, not a guess | ✅ 11-07 — a per-rule cost, paid for every rule a project's packs ship (below) |
| 2 | Make a rule nobody turns on cheap: packs kept in memory, dormant matchers dropped, the settings lookup skipped, `where:` before captures | `mcp_rules_for_path` back under; peak memory down by about half its rise | ✅ 11-07 — built, tests green; numbers below |
| 3 | Bring `peak_memory_10k` and `check_cold_10k` under: what is left is the rules that are **on** — W-139 turned about 130 more on in the recommended presets | both budgets green | ⏳ next — see *What is left* |
| 4 | CI's proof: the `perf cold` / `perf warm` jobs on a pushed `main` | the reference runner's numbers | ⏳ waits on the decider pushing `main` |

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

- L8 decision 5: a budget is never raised. The fix is in the engine or the packs, never in `perf/budgets.toml`.
- A loaded developer Mac is not the reference machine; take the numbers from `perf` runs, or from a quiet machine with both arms on one binary.
- `git stash` is one stack for every worktree (LESSONS 2026-11-07).

## Reference

[W-139](W-139-rule-parity-build.md) step 3 · [SR-LAW-8](../../records/0010_LAW-8-performance-budgets.md) · [SR-ARCHITECTURE](../../records/0108_architecture.md) decision 15 · [SR-CACHE](../../records/0109_cache.md) decision 8 · W-126 (the `perf` split) · W-168
