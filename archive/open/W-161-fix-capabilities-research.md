---
type: Handoff
name: W-161
description: "Arpit, 2026-11-05 (voice): research how the engine can make the six kinds of fix it cannot make yet, so the ~119 rules waiting on them can move up the fix ladder; W-162 builds the result."
item: W-161
filed: 2026-11-05
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Cowork (Opus 5.5)"
ball: done
---

# W-161 — Research: the six fixes the engine can't make yet

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Count the rules waiting on each capability, from their `fix_pending: "…; needs <capability>"` lines | the real demand per capability | ✅ 2026-11-05 |
| 2 | For each capability, read how the tools the rules came from do it, and what codelaw's engine already has | a design grounded in prior art | ✅ 2026-11-05 |
| 3 | Write one design per capability: the engine change, how the fix is verified, its L8 cost, any L11 dependency | W-162 builds without guessing | ✅ 2026-11-05 |
| 4 | Rank the six for W-162 by rules unlocked per unit of work | the build's order | ✅ 2026-11-05 |

**Arpit, 2026-11-05 (voice):** *"So we'll do research on each one of these items So create a research work item so that we can figure out how these can be implemented."* · *"We can close it but once the research item is done it needs to be implemented. … go ahead and create one more build item blocked on research work item."*; then (text) *"file it."*

**Done 2026-11-05 (Cowork):** [`work/proposals/fix-capabilities.md`](../../work/proposals/fix-capabilities.md). **151** rules wait on a capability on `main` (`d7ec95ae`), not 119. **112** move once W-162 builds six things (computed edits 57, cross-file 34, add-import 7, scope rename 7, type information 4, purity 2, action-tag SHAs 1). **37** wait on an author's decision and are re-judged `manual` with their exception as W-162's first step. **2** belong to W-163 and W-105. One question for the decider, multi-file fixes amending L4 decisions 10 and 12, went to the inbox on W-162. Closed and archived.

**Model:** Cowork: research only (SR-WORK-OPEN-QUEUE rule 37a), no code.

## Goal

A buildable design for each capability W-156's B4 row names: **cross-file edits** (69 rules), **type information** (17), **adding an import** (16), **scope rename** (6), **computed edits** (6), **purity** (5).

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · inventory | every `needs` value under `packs/` counted, with its rules | ✅ 2026-11-05 |
| 2 · designs | one section per capability: what it does, the engine change, how a fix using it passes L4's verifier (decisions 9–10: parses, idempotent, the rule no longer fires, bytes outside the edit unchanged), the L8 cost, L5 (no network, no model) and L11 (no new dependency, or the one to put to the decider) | ✅ 2026-11-05 |
| 3 · order | the six ranked for W-162 | ✅ 2026-11-05 |
| 4 · handoff | W-162's file links each design and carries the order | ✅ 2026-11-05 |

## Blockers

None.

## Record affected

**No SR affected** by the research; each design names the records W-162 will change.

## Hazards

- Type information already has a path — W-105, the project's own TypeScript through `codelaw-types` ([SR-TYPES](../../records/0117_types.md)). Design on it, not beside it.
- Cross-file edits meet L4 decision 10 ("fixes touch only their own range") as written: the design says how verification spans files.

## Reference

[W-156](../../work/open/W-156-fix-ladder.md) B4 · [W-162](W-162-fix-capabilities-build.md) · [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decisions 5, 9, 10 · [`work/proposals/fix-ladder.md`](../../work/proposals/fix-ladder.md)
