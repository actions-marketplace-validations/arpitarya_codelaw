---
type: Handoff
name: W-190
description: "Arpit, 2026-11-09 (voice docket): keep the two rows on JavaScript's `using` declaration in the queue, not the backlog."
item: W-190
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-190 — The `using` declaration rows: `no-using-resource-escape` and `prefer-dispose`

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Check that the pinned JavaScript grammar (0.25.0) parses `using`, and build both rules for `.js` if it does | the rules for JavaScript files | ⏳ |
| 2 | Find a TypeScript grammar release that parses `using` (0.23.2 does not) | the version to move to | ⏳ |
| 3 | Move the TypeScript grammar (L11 ratifies a new version by name) and run every pack's tests | the rules for TypeScript files too | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the `using` pair: *"Keep it in the list."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): a grammar check, two rules, a grammar bump.

## Goal

W-139 recorded that the pinned grammars do not parse `using`. tree-sitter-javascript 0.25.0, the pinned version, defines `using_declaration` (its `grammar.js`); tree-sitter-typescript 0.23.2 does not. So the JavaScript side may be buildable now.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · js | both rules on `.js`/`.jsx`, with cases | ⏳ |
| 2 · ts | the TypeScript grammar moved, or the gap named | ⏳ |
| 3 · tests | every JS and TS pack 0 failed after the move | ⏳ |

## Blockers

None for step 1.

## Record affected

SR-LAW-11 (the TypeScript grammar's row, if it moves).

## Hazards

- Every JS and TS rule sits on these grammars: a version move needs the full test run and the corpus.
- tree-sitter-javascript 0.25.0 still does not parse `using` as the head of a classic `for (;;)` loop.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-js, second pass*) · `Cargo.lock` (tree-sitter-javascript 0.25.0, tree-sitter-typescript 0.23.2)
