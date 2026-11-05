---
type: Handoff
name: W-160
description: "Worktrees left by session 91fa9b84-153a-455b-834e-9548467dc213 would lose data if removed"
item: W-160
filed: 2026-11-05
filed_by: "now-ledger.py (Claude Code)"
updated: 2026-11-05
updated_by: "now-ledger.py (Claude Code)"
ball: agent
---

# W-160 — Worktrees left by session 91fa9b84-153a-455b-834e-9548467dc213 would lose data if removed

## At a glance

| what | state |
|---|---|
| Worktrees left by session 91fa9b84-153a-455b-834e-9548467dc213 would lose data if removed | 🟡 ruled **keep** both; they land with W-156 |

Filed at the end of session `91fa9b84-153a-455b-834e-9548467dc213`.

These worktrees were left untouched. Rule per worktree: **keep** (merge it or commit the files; the next cleanup removes it) or **drop** (only then may an agent force the removal and delete the unmerged branch).

- `/Users/arpitarya/my_programs/codelaw/.claude/worktrees/W-156-a6` — HEAD 54b4660b is not in main; 503 modified, staged, deleted or untracked files
- `/Users/arpitarya/my_programs/codelaw/.claude/worktrees/W-156-moves` — 511 modified, staged, deleted or untracked files

## Ruling — 2026-11-05

**Keep both.** `W-156-a6` (per-project lowering) and `W-156-moves` are the live work of W-156's agent, stopped by the 5-hour usage limit while winding down, under Arpit's standing direction (30 Oct) to land what is done. They land through lockmaster with W-156; the next cleanup removes them. W-160 closes when both have landed.

## Record affected

No SR affected — the rule is SR-WORK-CLEANUP.
