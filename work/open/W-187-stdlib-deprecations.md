---
type: Handoff
name: W-187
description: "Arpit, 2026-11-09 (voice docket): approved — a data package of deprecated Python standard-library names, generated from CPython's docs, for Pylint's five deprecated-* rows."
item: W-187
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-187 — A table of deprecated standard-library names for Pylint's deprecated-* rows

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build the data package from CPython's docs, generated and checked in CI | which names are deprecated in which version | ⏳ |
| 2 | Enter it in L11's list with source, licence and pin | the dependency on record | ⏳ |
| 3 | Build the five rows on it | the rules ship | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the recommendation (approve the table, as its own item): *"Yes."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): a generator script, as `data-polyfills`, and five rules.

## Goal

Pylint `deprecated-argument`, `deprecated-attribute`, `deprecated-class`, `deprecated-method`, `deprecated-module`, built on a table keyed by the Python version that deprecates each name.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · package | generated from CPython's documentation, `--check` in CI (L6 decision 15) | ⏳ |
| 2 · L11 | its D-nn row | ⏳ |
| 3 · rules | the five, with cases | ⏳ |

## Blockers

None: approved 2026-11-09. The rules read the target; W-183's finding gate is the way to do it.

## Record affected

SR-LAW-11 (its D-nn row); L6 decision 15.

## Hazards

- CPython's docs are PSF-licensed: check L2 before generating.
- The package's registry names go to W-142 before its first release.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Slice 35*) · `work/proposals/rule-parity/pylint.csv`
