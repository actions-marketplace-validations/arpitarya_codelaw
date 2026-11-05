---
type: Handoff
name: W-165
description: "Twelve local branches not merged into main — ruled keep by the decider, 2026-11-05"
item: W-165
filed: 2026-11-05
filed_by: "Claude Code (session 0a172e1f)"
updated: 2026-11-05
updated_by: "Claude Code (session 0a172e1f)"
ball: done
---

# W-165 — Twelve unmerged local branches, kept

## At a glance

| what | state |
|---|---|
| Twelve local branches with commits not in `main` ([SR-WORK-CLEANUP](../../records/0062_WORK-cleanup.md) decision 3) | ✅ ruled **keep** all twelve, 2026-11-05 |

No worktree holds any of them; `lm/W-74` (fully in `main`) was deleted with `git branch -d`.
Each branch's added lines were checked against `main`, archive copies included: what main lacks is
superseded text — old status lines, `updated_by` metadata, 0.3.0 badges — or content never meant for main.

- `demo/W-45-annotations-v2` — the W-45 PR-annotations demo (also on `origin`)
- `keep/W-70-fux-index-2026-10-27`, `keep/W-71-fux-index-2026-10-27`, `keep/W-73-fux-index-2026-10-27` — regenerated `.fux` indexes
- `lm/W-82` — OPEN-WORK's open-count change, landed; two lines differ
- `lm/handoff-20261029-191320`, `lm/handoff-20261029-191828` — W-137 drafts, since rewritten and archived
- `lm/handoff-20261030-043224`, `lm/handoff-20261030-044012`, `lm/handoff-20261030-044726`, `lm/handoff-20261030-150228` — Cowork queue states of 10-30, since superseded
- `lm/save-20261029-214816` — README/USAGE at 0.3.0

## Ruling — 2026-11-05

**Arpit (Claude Code, asked: tag-then-drop, drop, or keep): "Keep them."** The branches stay as they are; no agent deletes them
with `-D` without a new ruling. `lockmaster doctor` will keep listing the `lm/*` ones as kept cleanups — expected, not a fault.
