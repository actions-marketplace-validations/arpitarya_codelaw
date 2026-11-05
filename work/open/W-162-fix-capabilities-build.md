---
type: Handoff
name: W-162
description: "Arpit, 2026-11-05 (voice): build the fix capabilities W-161 designs and move the rules waiting on them up the fix ladder."
item: W-162
filed: 2026-11-05
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Cowork (Opus 5.5)"
ball: agent
---

# W-162 — Build: the six fix capabilities, and the rules waiting on them

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | W-161's designs and build order | what to build, in what order | 🟡 waits on W-161 |
| 2 | Build each capability in W-161's order | the engine can make the fix | ⏳ |
| 3 | Move each waiting rule up, proven by its own examples (`codelaw test`) | agents get a diff, not steps | ⏳ |
| 4 | Close when no `fix_pending:` names W-162 | L4 holds without a stand-in | ⏳ |

**Arpit, 2026-11-05 (voice):** *"So we'll do research on each one of these items So create a research work item so that we can figure out how these can be implemented."* · *"We can close it but once the research item is done it needs to be implemented. … go ahead and create one more build item blocked on research work item."*; then (text) *"file it."*

**Model:** Opus (Claude Code): engine changes under a law (L4) and the verifier.

## Goal

Every rule whose `fix_pending:` names W-162 reaches its proposed rung, or is re-judged `manual` with a `fix_exception` ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decision 5).

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · capabilities | each of W-161's designs built and tested, within L8 | ⏳ |
| 2 · rules | no `fix_pending:` names W-162; `codelaw rules --fix-stats --check` passes | ⏳ |

## Blockers

W-161 (its designs).

## Record affected

SR-LAW-4 decision 5 (the stand-in ends), and the records W-161's designs name.

## Hazards

- W-156's moves touch the same packs while it is open: claim per pack.

## Reference

[W-161](W-161-fix-capabilities-research.md) · [W-156](W-156-fix-ladder.md) B4
