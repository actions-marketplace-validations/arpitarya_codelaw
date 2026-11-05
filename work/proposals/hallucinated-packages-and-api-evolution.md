---
type: Proposal
title: Packages that do not exist, and APIs that moved — two deps rules and version-gated deprecation packs
description: "deps refresh also records each declared package's existence and first-publish date, so two offline rules catch hallucinated and freshly squatted names; and library deprecations ship as data packs gated by the installed version, served to agents before they write."
status: proposed
timestamp: 2026-11-05T19:50:00+05:30
---

# Packages that do not exist, and APIs that moved

Filed 2026-11-05 (Cowork, Opus 5.5) from the research brief — *"file proposals
for all of the items"*. Item 4 of 7.

## 1 · In one line

**Agents invent package names and use APIs from before their cutoff. codelaw
already owns the offline dependency snapshot and dated deprecations — extend
both to catch these.**

## 2 · Evidence

| source | finding |
|---|---|
| *We Have a Package for You!* (USENIX Security 2025, arXiv 2406.10279) | 576,000 samples, 16 models: hallucinated packages in **5.2%** (commercial) and **21.7%** (open) of cases; **205,474 unique invented names** — attackers register them ("slopsquatting") |
| LLMs Meet Library Evolution (arXiv 2406.09834) | deprecated-API usage measured across 7 models and 8 Python libraries; prior work it reports: 25–38% |
| When LLMs Lag Behind (arXiv 2604.09515) | 270 real API updates: with only the update's description a model follows it in 74.6% of cases; with full docs 92.9% |

## 3 · Today

- `codelaw deps refresh` — the one networked command (L5 decision 3) — writes a committed snapshot; `deps/*` rules and MCP `check_dependency` read it offline; `check_dependency` already returns `known`.
- `codelaw deprecate <symbol>` + `graph/no-deprecated-usage` (planned) — a repo's **own** deprecations, dated ([SR-BASELINE](../../records/0104_baseline-and-deprecations.md)).
- Nothing flags a declared package the registry has never heard of, or one first published days ago.
- No shipped deprecation data for third-party libraries.

## 4 · The proposal

**A — two `deps` rules**
- **`deps/unknown-package`** (`error`): a manifest names a package the registry does not have. Fix `manual` (exception: no safe target exists) — the message suggests the closest known name from the snapshot, never applies it.
- **`deps/new-package`** (`warn`): first published fewer than `[deps] min_age_days` (default 30) before the as-of date, or the version itself younger than that. Fix `manual`.
- **`deps refresh`** also records `first_published` and `version_published` per package — the same registry calls it already makes.
- **MCP `check_dependency`** answers before the agent adds the line: an unknown name returns `known: false` with `next: "do not add"`.

**B — version-gated deprecation packs**
- **Pack data, not rules:** `packs/<lib>-evolution/` lists `{symbol, deprecated_in, removed_in, replacement}` per library (pandas, numpy, React, Node first).
- **Gated by the installed version**, read offline from the lockfile — a pandas 1.x project is not told about 2.x removals.
- **Served before the write:** `rules_for_path` and `brief_for_task` include the entries for imports in the file — the "update description" that took models to 74.6%.
- **Checked after:** `graph/no-deprecated-usage` reads the same data; fix `suggest` where the replacement is a rename.

## 5 · Laws touched

- **L5** — new fields come only through `deps refresh`; the engine stays offline.
- **L6** — new packs need registry names and the decider's per-registry list.
- **L11** — no new dependency.

## 6 · Open questions for the decider

1. **`min_age_days` default** — recommended 30; 0 turns the rule off.
2. **Which libraries first for B?** Recommended: React and Node on npm; pandas and numpy on PyPI (both packs exist).
3. **Who maintains evolution data** — hand-curated per release, or drafted by a skill from changelogs and reviewed?

## 7 · Graduation trigger

**A:** the decider rules §6.1 → a W-item extends `deps refresh` and adds both rules. **B:** the decider rules §6.2–6.3 → a separate W-item.

## Sources (read 2026-11-05)

- We Have a Package for You! — <https://arxiv.org/abs/2406.10279>
- LLMs Meet Library Evolution — <https://arxiv.org/abs/2406.09834>
- When LLMs Lag Behind: Knowledge Conflicts from Evolving APIs in Code Generation — <https://arxiv.org/html/2604.09515>
