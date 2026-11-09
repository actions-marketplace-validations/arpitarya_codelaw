---
type: Handoff
name: W-189
description: "Arpit, 2026-11-09 (voice docket): a research item to design a code-path graph — the routes through a function across await, loops and try — for the two JS rows that need one."
item: W-189
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: cowork
---

# W-189 — Research: a code-path graph for JavaScript

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Read prior art: ESLint's code-path analysis, codelaw's Python flow pass | what such a graph holds | ⏳ |
| 2 | Propose two or three shapes, each with its cost against L8 | a choice the decider can make | ⏳ |
| 3 | The decider picks a shape | SR-CODE-PATH-GRAPH accepted | 🔴 decider decides |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the pair: *"Okay, let's put it as a separate research work item for code path graph."* Filed on *"file it."*

**Model:** Cowork: research and a proposal, no code (SR-WORK-OPEN-QUEUE rule 37a).

## Goal

A proposal for the graph `js/require-atomic-updates` and `js/no-multiple-promise-resolver-calls` need: which statements a value or a resolver call reaches across `await`, loops and `try`.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · prior art | ESLint's code-path analysis, the Python flow pass (`crates/codelaw-semantic/src/py_flow.rs`), what others ship | ⏳ |
| 2 · proposal | a proposal in `work/proposals/`, two or three options with their cost | ⏳ |
| 3 · record | SR-CODE-PATH-GRAPH filled in on the decider's pick | ⏳ |

## Blockers

None to start.

## Record affected

[SR-CODE-PATH-GRAPH](../../records/0121_code-path-graph.md) (proposed, filed 2026-11-09).

## Hazards

- One function at a time, or across calls: say which, and what each costs.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-js, second pass*) · [SR-CODE-PATH-GRAPH](../../records/0121_code-path-graph.md)
