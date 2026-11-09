---
type: Handoff
name: W-177
description: "Arpit, 2026-11-09 (voice docket): turn SR-PY-INFERENCE decision 8 into its own research item — should the Python rows that need a call's return or an imported class's shape ask the project's own type checker?"
item: W-177
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: cowork
---

# W-177 — Should codelaw ask the project's own Python type checker?

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | List the Python rows that stop at another file: a call's return, an imported class's bases and methods | the size of the gap | ⏳ |
| 2 | Compare asking the project's own checker (`ty`, pyright, mypy), class facts in the project graph, and leaving the rows open | two or three options, each with its cost against L5, L8 and L11 | ⏳ |
| 3 | The decider picks a shape | SR-PY-INFERENCE decision 8 answered | 🔴 decider decides |

**Arpit (voice, Cowork docket), 2026-11-09:** asked to rule decision 8 or leave the rows open: *"Open a separate research item for this."* — filed on *"file it."*

**Model:** Cowork: research and a proposal, no code (SR-WORK-OPEN-QUEUE rule 37a).

## Goal

A proposal the decider can rule in one sitting. The question is whether the Python rows W-174 could not answer inside one file should be answered by the project's own type checker, found and asked the way codelaw asks the project's TypeScript (SR-TYPES), with codelaw installing nothing (L11 decision 2).

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rows | every row that stops at an import, named with its question (from W-174: the override, abstract and `__init__` rows at an imported base; the value-type rows at a call's return) | ⏳ |
| 2 · options | each option with what it answers, its run-time cost against L8, what it needs installed, and whether it gives the same answer with or without an agent (L5) | ⏳ |
| 3 · proposal | a proposal or compare doc in `work/proposals/` or `work/compare/`, with a recommendation | ⏳ |
| 4 · ruling | the decider's pick, written into SR-PY-INFERENCE decision 8 | ⏳ |

## Blockers

None to start.

## Record affected

SR-PY-INFERENCE decision 8 (and SR-TYPES if the Python checker is asked the way TypeScript is).

## Hazards

- A Python type checker is not the project's own tool the way its TypeScript is: many projects run none, and the three disagree. The proposal says what a project without one gets.
- `archive/` is not evidence (SR-WORK-ARCHIVE).

## Reference

[W-174](W-174-python-inference.md) (where the question came from) · [SR-PY-INFERENCE](../../records/0120_python-inference.md) · [SR-TYPES](../../records/0117_types.md) · [W-105](W-105-type-aware-engine.md) (the TypeScript type program)
