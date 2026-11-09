---
type: Handoff
name: W-195
description: "Arpit, 2026-11-09 (voice docket): W-139's JavaScript leftovers with no blocker get one build item."
item: W-195
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-195 — The JavaScript rows and fixes with no blocker

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build the four unbuilt rows that need nothing new | the rows done | ⏳ |
| 2 | Build `js/comment-type` and decide `node/process-exit-as-throw`'s shape | the last two rows placed | ⏳ |
| 3 | The second fixes: react's five rules, `ts/no-import-type-side-effects` past one specifier | fixes for every shape | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* Unblocked rows go to build items: *"Only the ones that are blocked."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): native rules on the scope model.

## Goal

Rows: unicorn `js/no-unnecessary-parameters`, `js/prefer-abort-signal-any`, `js/prefer-iterator-zip`; SonarJS `js/parameterized-tests`; TSLint `js/comment-type` (the taste preset, W-144 R8); eslint-plugin-n `node/process-exit-as-throw` (it reports nothing itself; it changes how other rules read `process.exit`). Fixes: a second fix for `react/button-has-type`, `iframe-missing-sandbox`, `jsx-curly-brace-presence`, `jsx-no-useless-fragment`, `self-closing-comp`; `ts/no-import-type-side-effects` for more than one specifier.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rows | the six, each with cases | ⏳ |
| 2 · fixes | the six second fixes | ⏳ |
| 3 · ledger | the rows built or classed | ⏳ |

## Blockers

None.

## Record affected

no SR affected (SR-RULE-FORMAT if a per-finding fix needs a format change).

## Hazards

- `packs/js` is shared with W-176 and W-196: claim only these files.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-js, second pass*; *Needs a native rule — strand W-139-react-ts*)
