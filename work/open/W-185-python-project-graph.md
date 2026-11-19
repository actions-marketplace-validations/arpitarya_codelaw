---
type: Handoff
name: W-185
description: "Arpit, 2026-11-09 (voice docket): one item to build the Python project graph (import resolution, a class's bases across files), with six Pylint rows on it."
item: W-185
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-17
updated_by: "Claude Code (Sonnet 5.5)"
ball: agent
---

# W-185 — The Python side of the project graph

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Resolve Python's absolute and relative imports across the project | which file each import names | ✅ (the resolver was there; the queries and the facts they read are new) |
| 2 | Carry each class's bases across files | a class's ancestors | ✅ |
| 3 | Build the six Pylint rows on graph queries | the cross-file Python rules | ✅ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the recommendation (one item for the Python graph, the six rows on it): *"Yes."* Filed on *"file it."*

**Model:** Opus (Claude Code): graph design on SR-MODULE-GRAPH, as W-175 did for JavaScript.

## Goal

The project graph answers for Python what it answers for JavaScript. Rows: Pylint `cyclic-import`, `import-error`, `no-name-in-module`, `relative-beyond-top-level`, `unused-wildcard-import`, `too-many-ancestors`. Ruff `PLC2701` is W-186, on this graph.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · graph | Python import resolution and class bases in the graph, with tests | ✅ `crates/codelaw-graph/src/pynames.rs`, `pynames_tests.rs` (15 tests on virtual repos) |
| 2 · rules | the six, in an opt-in preset as `js/project` is | ✅ `py/project` (7 rules with `graph/no-unused-exports`), all `preview` |
| 3 · L8 | the graph's cost measured; nothing on by default that a cold check pays for | ✅ measured, see *What was built* |

## What was built (closed 2026-11-17)

- **Facts** (`FileFacts::py`, `ImportFact::{guarded, type_checking, stmt}`; `FACTS_VERSION` 8): names bound at a module's top level, a literal `__all__`, whether the module may bind names the facts cannot see, a `sys.path` touch, each class with its bases, a file's free names (only with a `from m import *`), the statement and guard of each import. The semantic model keeps `py_all` / `py_all_dynamic`.
- **Queries** (SR-MODULE-GRAPH decision 12): `cycle:` reads Python files; `python: { imports: unresolved | unexported | beyond_top | wildcard_unused }`; `python: { classes: ancestors }`; `[graph] max_ancestors` (default 7).
- **Rules** in `packs/python/rules/py/`, `preview`, `manual`, in `py/project`: `py/cyclic-import`, `py/import-error`, `py/no-name-in-module`, `py/relative-beyond-top-level`, `py/unused-wildcard-import`, `py/too-many-ancestors`.
- **The hazards, answered.** Namespace packages are never judged by `unresolved` and `unexported`; a `src/` layout and each `pyproject.toml` / `setup.py` folder are roots; a `sys.path` change is not followed (a file that touches it is not judged by `unresolved`); nothing reads `site-packages`, so Pylint's `import-error` for an uninstalled package is not built. W-177 (should the project's own type checker answer the rows that need an imported class's methods) is untouched: the class facts here carry bases for a count only.
- **Cost (L8).** `<<L8>>`

## Blockers

None.

## Record affected

SR-MODULE-GRAPH (decision 12, written in this item), SR-SEMANTIC-MODEL (decision 5 names `py_all`).

## Hazards

- W-177 asks whether the project's own type checker should answer some cross-file rows; read it before giving the graph class facts.
- Namespace packages, `src/` layouts and `sys.path` changes: say which are read.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Slice 35*) · [W-175](../../archive/open/W-175-js-module-graph-rules.md) (the JavaScript side) · `work/proposals/rule-parity/pylint.csv`
