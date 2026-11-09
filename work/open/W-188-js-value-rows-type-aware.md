---
type: Handoff
name: W-188
description: "Arpit, 2026-11-09 (voice docket): the JS rows that need a value's type get their own item, waiting on W-105's type helper."
item: W-188
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-188 — The JavaScript rows that need a value's type, on the type helper

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | When W-105's helper lands, let these rules ask it a value's type | findings on values one file cannot place | ⏳ |
| 2 | Build `js/jsdoc-no-unnecessary-type-assertion` on it | the one row not built at all | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* *"the 16 rows which wait on value type."* — a separate item. Filed on *"file it."*

**Model:** Sonnet (Claude Code): each rule gains a type question; one new rule.

## Goal

Fifteen of the sixteen rows were built on 2026-11-09 (W-139 slice 29) from what one file states — a constructor, an annotation, a `const` alias — and report nothing for a value they cannot place; with the type helper they also answer for those. Rows: `js/no-collection-bracket-access`, `no-invalid-argument-count`, `no-nonstandard-builtin-properties`, `no-object-methods-with-collections`, `no-unused-iterator-helper`, `no-useless-iterator-to-array`, `no-useless-set-construction`, `prefer-array-iterable-methods`, `prefer-iterator-concat`, `prefer-iterator-helpers`, `prefer-iterator-to-array`, `prefer-iterator-to-array-at-end`, `prefer-object-iterable-methods`, `prefer-temporal-conversion`, `prefer-type-guard`. The sixteenth, `prefer-iterator-zip`, is not built and needs no types: W-195. New here: `js/jsdoc-no-unnecessary-type-assertion`.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rules | each rule asks the type helper when a TypeScript program is loaded, and keeps its syntax answer when none is | ⏳ |
| 2 · new | `js/jsdoc-no-unnecessary-type-assertion` | ⏳ |

## Blockers

🟣 waits on **W-105** (its RC re-pin, 2026-11-10).

## Record affected

SR-TYPES (the type questions asked).

## Hazards

- L5: the answer with a type program and without one is stated on each rule page.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-js, second pass*, slice 29) · [W-105](W-105-type-aware-engine.md)
