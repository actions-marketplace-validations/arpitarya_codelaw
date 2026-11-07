---
type: Handoff
name: W-172
description: "main is over three set L8 budgets on the reference runner: check_cold_10k, peak_memory_10k and mcp_rules_for_path (nightly perf run 42316894936 on 3b53d1a8, found by W-139 step 3); optimise, never loosen."
item: W-172
filed: 2026-11-07
filed_by: "Claude Code (session 0a172e1f), W-139 step 3"
updated: 2026-11-07
updated_by: "Claude Code (session 0a172e1f), W-139 step 3"
ball: agent
---

# W-172 — main is over three L8 budgets

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Find which landing between `9ae07264` (last green `perf`, 2026-11-05 nightly) and `3b53d1a8` moved each number — bisect the 242 commits with `codelaw bench --only check_cold_10k,peak_memory_10k,mcp_rules_for_path` on one quiet machine | the cause, not a guess | ⏳ |
| 2 | Optimise it back under each budget | the `perf cold` and `perf warm` jobs green | ⏳ |

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

## Hazards

- L8 decision 5: a budget is never raised. The fix is in the engine or the packs, never in `perf/budgets.toml`.
- A loaded developer Mac is not the reference machine; take the numbers from `perf` runs, or from a quiet machine with both arms on one binary.

## Reference

[W-139](W-139-rule-parity-build.md) step 3 · [SR-LAW-8](../../records/0010_LAW-8-performance-budgets.md) · W-126 (the `perf` split) · W-168
