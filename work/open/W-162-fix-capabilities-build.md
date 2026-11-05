---
type: Handoff
name: W-162
description: "Arpit, 2026-11-05 (voice): build the fix capabilities W-161 designs and move the rules waiting on them up the fix ladder."
item: W-162
filed: 2026-11-05
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-162 — Build: the six fix capabilities, and the rules waiting on them

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 0 | Re-judge the 37 waiting rules whose reason is an author's decision: `manual` with their exception ([proposal](../proposals/fix-capabilities.md) §3) | the waiting count falls by a quarter | ⏳ |
| 1 | W-161's designs and build order | what to build, in what order | ✅ 2026-11-05 — [`fix-capabilities.md`](../proposals/fix-capabilities.md) §2, §4 |
| 1a | The decider rules whether a fix may edit several files (L4 decisions 10, 12; proposal §5) | the 22 rename rules' path | ✅ 2026-11-05 — allowed; SR-LAW-4 decisions 10 and 12 amended |
| 2 | Build each capability in the proposal's order (§4): computed edits, scope rename, add-import, purity, types, cross-file read, cross-file write, action SHAs | the engine can make the fix | ⏳ |
| 3 | Move each waiting rule up, proven by its own examples (`codelaw test`) | agents get a diff, not steps | ⏳ |
| 4 | Close when no `fix_pending:` names W-162 | L4 holds without a stand-in | ⏳ |

**Arpit, 2026-11-05 (voice):** *"So we'll do research on each one of these items So create a research work item so that we can figure out how these can be implemented."* · *"We can close it but once the research item is done it needs to be implemented. … go ahead and create one more build item blocked on research work item."*; then (text) *"file it."*

**Arpit, 2026-11-05 (voice), refining it:** *"Create an SR for this."* · *"and make it configurable"* · *"it does and that's all"*; then (text) *"file it."* The multi-file rules are their own record, [SR-MULTI-FILE-FIXES](../../records/0119_multi-file-fixes.md): the five safeguards are never configurable; `[fix] multi_file` and per rule take `auto`, `suggest` or `off` (SR-CONFIG decision 11g).

**Arpit, 2026-11-05 (Claude Code, multiple choice):** *"Allow multi-file (Recommended)"* — a fix may edit several files: verified per file plus a graph check for new unresolved references, written all-or-nothing, `auto` only when every reference resolves statically, never on save or from the per-edit hook. Written into [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decisions 10 and 12 (ruled, not built).

**Model:** Opus (Claude Code): engine changes under a law (L4) and the verifier.

## Goal

Every rule whose `fix_pending:` names W-162 reaches its proposed rung, or is re-judged `manual` with a `fix_exception` ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decision 5).

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · capabilities | each of W-161's designs built and tested, within L8 | ⏳ |
| 2 · rules | no `fix_pending:` names W-162; `codelaw rules --fix-stats --check` passes | ⏳ |

## Blockers

None. The multi-file call was ruled 2026-11-05 (allowed, above); W-161's designs are done.

## Record affected

SR-LAW-4 decision 5 (the stand-in ends); SR-MULTI-FILE-FIXES and SR-CONFIG 11g (built: no → yes); SR-RULE-FORMAT 3a–3c; SR-TYPES 4; SR-PROJECT-GRAPH; SR-CLI-AND-MCP; SR-CACHE.

## Hazards

- W-156's moves touch the same packs while it is open: claim per pack.

## Reference

[W-161](../../archive/open/W-161-fix-capabilities-research.md) · [proposal](../proposals/fix-capabilities.md) · [W-156](W-156-fix-ladder.md) B4
