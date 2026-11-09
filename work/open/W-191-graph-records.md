---
type: Handoff
name: W-191
description: "Arpit, 2026-11-09 (voice docket): SR-PROJECT-GRAPH becomes the overview of every graph codelaw builds, and each graph gets its own record."
item: W-191
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-191 — Records: one overview of the graphs, one record per graph

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Write SR-PROJECT-GRAPH as the overview: each graph, what it answers, which record owns it | one place to see them all | ⏳ |
| 2 | Move the module graph's decisions into their own record | the module graph in one record | ⏳ |
| 3 | Move the per-file semantic model (SR-ARCHITECTURE decision 15a) and the Python flow pass (SR-PY-INFERENCE) into records of their own | each graph in one record | ⏳ |
| 4 | SR-CODE-PATH-GRAPH filed as `proposed`, naming W-189 | the code-path graph has its record | ✅ 2026-11-09 |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the graphs: *"All the graphs that we have over here create SR for individual graphs."* · *"maybe create an empty SR for code path for now. And the SR project graph, it could be more like an overview of all the graphs and then individual graphs could have their own SRs."* — told the register allows no placeholder files, so the code-path record is a short `proposed` one: *"Perfect, let's go with it."* Filed on *"file it."*

**Model:** Opus (Claude Code): moving decisions between records without losing or restating one (SR-LAW-0).

## Goal

Each graph has one record; SR-PROJECT-GRAPH names them all and holds no decision another record owns.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · overview | SR-PROJECT-GRAPH rewritten as the overview | ⏳ |
| 2 · records | the module graph, the semantic model and the Python flow pass each in its own record; `owns:` moved with them; the register and its Ownership table updated | ⏳ |
| 3 · cites | every citation of a moved decision re-pointed | ⏳ |

## Blockers

None.

## Record affected

SR-PROJECT-GRAPH, SR-ARCHITECTURE decision 15a, SR-PY-INFERENCE, the register.

## Hazards

- A decision moves, it is not copied: one home each (SR-LAW-0).
- `scripts/sr-guard.sh` reads `owns:` — move ownership in the same change as the code's record.

## Reference

[SR-PROJECT-GRAPH](../../records/0103_project-graph.md) · [SR-CODE-PATH-GRAPH](../../records/0121_code-path-graph.md) · [SR-PY-INFERENCE](../../records/0120_python-inference.md)
