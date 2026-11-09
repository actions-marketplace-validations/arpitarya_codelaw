---
type: Handoff
name: W-179
description: "Arpit, 2026-11-09 (voice docket): Ruff's six layout rows are the formatter's — check which the Python formatter already does and add the rest."
item: W-179
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-179 — Formatter: the docstring and shebang layout rows

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Check which rows the Python formatter already does, one fixture each | what is already covered | ⏳ |
| 2 | Add the rest to the formatter | the layout handled where it belongs | ⏳ |
| 3 | Mark every row formatter-owned in the parity ledger | none counted as a missing lint rule | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On Ruff's layout group: *"So whatever is handled on formatter side, they are not again needed in the linter side."* · *"All these look like format formatters to me."* · *"Not in backlog. Put it in the work item list."* — *"All six."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): a fixture per row and a formatter change each.

## Goal

Ruff `D201`, `D202`, `D203` or `D211` (they contradict: one is chosen), `D204` and `EXE004` are done by `codelaw format`, not by a lint rule. `D210` the formatter already does. Pylint's `inconsistent-quotes` and `implicit-str-concat`, classed formatter-owned on the recommendation the same day, are checked the same way.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · check | a fixture per row: formatted output before and after | ⏳ |
| 2 · formatter | the rows it lacks added, matching Black and Ruff's formatter where they agree | ⏳ |
| 3 · ledger | the rows formatter-owned in `ruff.csv`, `pylint.csv` | ⏳ |

## Blockers

None.

## Record affected

SR-FORMATTER.

## Hazards

- A formatter change moves every Python file's output: run the formatter corpus before and after (L7).
- `D203` against `D211`: pick the one Black and Ruff's formatter follow, and say why.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-ruff, round 4*) · `work/proposals/rule-parity/ruff.csv`
