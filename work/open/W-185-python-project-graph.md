---
type: Handoff
name: W-185
description: "Arpit, 2026-11-09 (voice docket): one item to build the Python project graph (import resolution, a class's bases across files), with six Pylint rows on it."
item: W-185
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-185 — The Python side of the project graph

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Resolve Python's absolute and relative imports across the project | which file each import names | ⏳ |
| 2 | Carry each class's bases across files | a class's ancestors | ⏳ |
| 3 | Build the six Pylint rows on graph queries | the cross-file Python rules | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the recommendation (one item for the Python graph, the six rows on it): *"Yes."* Filed on *"file it."*

**Model:** Opus (Claude Code): graph design on SR-PROJECT-GRAPH, as W-175 did for JavaScript.

## Goal

The project graph answers for Python what it answers for JavaScript. Rows: Pylint `cyclic-import`, `import-error`, `no-name-in-module`, `relative-beyond-top-level`, `unused-wildcard-import`, `too-many-ancestors`. Ruff `PLC2701` is W-186, on this graph.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · graph | Python import resolution and class bases in the graph, with tests | ⏳ |
| 2 · rules | the six, in an opt-in preset as `js/project` is | ⏳ |
| 3 · L8 | the graph's cost measured; nothing on by default that a cold check pays for | ⏳ |

## Blockers

None.

## Record affected

SR-PROJECT-GRAPH.

## Hazards

- W-177 asks whether the project's own type checker should answer some cross-file rows; read it before giving the graph class facts.
- Namespace packages, `src/` layouts and `sys.path` changes: say which are read.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Slice 35*) · [W-175](../../archive/open/W-175-js-module-graph-rules.md) (the JavaScript side) · `work/proposals/rule-parity/pylint.csv`
