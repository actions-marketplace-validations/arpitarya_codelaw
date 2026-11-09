---
type: Handoff
name: W-186
description: "Arpit, 2026-11-09 (voice docket): PLC2701 gets its own item, waiting on the Python project graph."
item: W-186
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-186 — Ruff PLC2701: importing another package's private names

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build PLC2701 on the Python graph | an import of a `_private` name from another package is reported | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* *"PLC 2701 have a separate work item for it."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): one graph rule.

## Goal

`PLC2701` (`import-private-name`) needs the package root and the imported module's own names across files.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rule | PLC2701 with cases, on W-185's queries | ⏳ |

## Blockers

🟡 waits on **W-185** (the Python project graph).

## Record affected

SR-PROJECT-GRAPH (a query, if one is added).

## Hazards

- Upstream treats a package's own internal imports as allowed: read the package root the same way.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-ruff, round 4*) · `work/proposals/rule-parity/ruff.csv`
