---
type: Handoff
name: W-190
description: "Arpit, 2026-11-09 (voice docket): keep the two rows on JavaScript's `using` declaration in the queue, not the backlog."
item: W-190
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-19
updated_by: "Claude Code (Sonnet 5.5): built and closed"
ball: none
---

# W-190 — The `using` declaration rows: `no-using-resource-escape` and `prefer-dispose`

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Check that the pinned JavaScript grammar (0.25.0) parses `using`, and build both rules for `.js` if it does | the rules for JavaScript files | ✅ it does (`using_declaration`, `await using`); both rules built |
| 2 | Find a TypeScript grammar release that parses `using` (0.23.2 does not) | the version to move to | ✅ none: 0.23.2 (2024-11-11) is the newest official release; the gap is named |
| 3 | Move the TypeScript grammar (L11 ratifies a new version by name) and run every pack's tests | the rules for TypeScript files too | ✅ not moved (step 2); `ts`, `react`, `node` packs pass unchanged |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the `using` pair: *"Keep it in the list."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): a grammar check, two rules, a grammar bump.

## Goal

W-139 recorded that the pinned grammars do not parse `using`. tree-sitter-javascript 0.25.0, the pinned version, defines `using_declaration` (its `grammar.js`); tree-sitter-typescript 0.23.2 does not. So the JavaScript side may be buildable now.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · js | both rules on `.js`/`.jsx`, with cases | ✅ |
| 2 · ts | the TypeScript grammar moved, or the gap named | ✅ gap named, see *Built* |
| 3 · tests | every JS and TS pack 0 failed after the move | ✅ no move; `js`, `ts`, `react`, `node` 0 failed |

## Built (2026-11-19)

`crates/codelaw-native/src/js_using.rs`, `javascript` and `jsx`, `preview`, in no preset. **`js/no-using-resource-escape`** (`manual`, `restructure`): the resource, or a function that captures it, returned or exported from its scope, also inside arrays, objects, conditionals, `&&`/`||`/`??` and a sequence's last operand, and through a local function the scope declares. **`js/prefer-dispose`** (`suggest`): a `try`/`finally`, no `catch`, whose `finally` only calls `close`, `dispose`, `destroy`, `end`, `release`, `disconnect`, `unlock` or `free` on `const` resources declared just before it, in reverse order, becomes a `using` / `await using` block; not reported when a name is read after the `try`. Specs from eslint-plugin-unicorn 79.0.0's documentation (read outside the engine, L5), written fresh.

**The TypeScript gap:** `tree-sitter-typescript` 0.23.2 (2024-11-11) is the newest official release and does not parse `using`; `tree-sitter-typescript-next` 0.25.3 is an unofficial fork that L11 has not ratified, and its support was not checked. The two rules are `javascript`/`jsx` until a release or the decider naming the fork. `for (using x of y)` parses in 0.25.0; the classic `for (using …;;)` head does not.

**Not run:** the corpus (its checkouts are not on this machine; `corpus.toml` says 0 for both, to be rewritten by CI's `--update-snapshot` if it differs) and `perf`.

## Blockers

None for step 1.

## Record affected

SR-LAW-11 (the TypeScript grammar's row, if it moves).

## Hazards

- Every JS and TS rule sits on these grammars: a version move needs the full test run and the corpus.
- tree-sitter-javascript 0.25.0 still does not parse `using` as the head of a classic `for (;;)` loop.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-js, second pass*) · `Cargo.lock` (tree-sitter-javascript 0.25.0, tree-sitter-typescript 0.23.2)
