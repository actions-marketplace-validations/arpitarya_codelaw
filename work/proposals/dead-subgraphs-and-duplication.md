---
type: Proposal
title: Dead subgraphs and duplicated blocks — the project graph's answer to AI copy-paste
description: "graph/no-unused-* extended from single symbols to whole unreachable subgraphs (cycles included) with a why-dead explanation, and a new deterministic duplicate-block rule whose suggest fix points at the existing function."
status: proposed
timestamp: 2026-11-05T19:50:00+05:30
---

# Dead subgraphs and duplicated blocks

Filed 2026-11-05 (Cowork, Opus 5.5) from the research brief — *"file proposals
for all of the items"*. Item 5 of 7.

## 1 · In one line

**AI assistants copy instead of reuse, and leave dead code behind. Find whole
dead islands, not just one unused export, and flag a new block that already
exists elsewhere.**

## 2 · Evidence

| source | finding |
|---|---|
| GitClear, AI code quality report (2025; 211M lines) | blocks of ≥ 5 duplicated lines up **8×** in 2024; moved (refactored) lines **−39.9%**; 2024 the first year copied lines outnumbered moved |
| Meta SCARF (ESEC/FSE 2023 industry; Meta engineering, 2023) | > 100M lines deleted in > 370,000 changes over five years; **whole-graph analysis, cycles included, found nearly 50% more dead code** than symbol-by-symbol; each change says why the code is "provably dead"; some languages auto-merge |

## 3 · Today

- [SR-PROJECT-GRAPH](../../records/0103_project-graph.md) lists `graph/no-unused-exports`, `no-unused-files`, `no-cycles` and more; `codelaw graph why-unused` explains one verdict.
- A group of modules that only reference each other is not dead to a per-symbol check.
- No duplicate-code rule exists in any pack.

## 4 · The proposal

**A — dead subgraphs**
- **`graph/no-dead-subgraph`**: a strongly connected set of modules or symbols that no entry point reaches; one finding per island, on its largest member, listing the rest.
- **Fix `suggest`:** delete the island in one diff; `why-unused` output in the finding.
- **Optional usage input:** `[graph] usage = "<file>"` — a list of symbols seen at runtime, written by the project's own tooling; read offline, never fetched. An island with a runtime hit is not dead.

**B — duplicated blocks**
- **`graph/no-duplicate-block`** (`warn`): a new or edited block of ≥ N normalised tokens (identifiers and literals abstracted) that matches an existing block elsewhere in the repo.
- **On new code only by default** (the touched-file rule of the baseline), so an old codebase is not flooded.
- **Fix `suggest`** where the match is a whole function: replace the block with a call to it, imported through the graph. Otherwise `manual` with the location of the original.
- **Served before the write:** MCP `check_snippet` reports the match, so the agent reuses before it pastes.

## 5 · Laws touched

- **L4** — both rules carry a fix.
- **L5** — token hashing, no model; no network.
- **L8** — a cost on the graph stage; measured before default-on.

## 6 · Open questions for the decider

1. **N for duplicate blocks** — recommended 50 tokens, after a corpus count.
2. **Runtime usage file** — in scope now, or later?
3. **Auto-merge** — none; or a future `auto` for islands with no runtime hit, as SCARF does for some languages?

## 7 · Graduation trigger

**The decider rules §6.1 and §6.2** → a W-item builds both rules on the project graph, with corpus counts and an L8 row.

## Sources (read 2026-11-05)

- DevClass on GitClear's 2025 report — <https://devclass.com/2025/02/20/ai-is-eroding-code-quality-states-new-in-depth-report/>
- Meta — Automating dead code cleanup — <https://engineering.fb.com/2023/10/24/data-infrastructure/automating-dead-code-cleanup/>
- Dead Code Removal at Meta (ESEC/FSE 2023 industry) — <https://2023.esec-fse.org/details/fse-2023-industry/12/Dead-Code-Removal-at-Meta-Automatically-Deleting-Millions-of-Lines-of-Code-and-Petab>
