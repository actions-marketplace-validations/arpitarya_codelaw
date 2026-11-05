---
type: Handoff
name: W-164
description: "Arpit, 2026-11-05 (voice, Claude app): rewrite the README's opening to the positioning ruled that day — the agent problem first, deterministic and AI-free second."
item: W-164
filed: 2026-11-05
filed_by: "Arpit (Claude app, Opus 5.5)"
updated: 2026-11-05
updated_by: "Arpit (Claude app, Opus 5.5)"
ball: agent
---

# W-164 — Rewrite the README opening around the new positioning

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Rewrite the README's top: badge line, the one-line pitch and the first paragraph of *The story* | a pitch that leads with the pain only codelaw solves | ⏳ |
| 2 | Lead line: *"Your team's rules, enforced on every AI edit."* | the agent problem first, not "another linter" | ⏳ |
| 3 | Second line: deterministic — no AI in the engine, the same verdict in CI, the editor and pre-commit, with or without an agent | trust, as reassurance rather than the headline | ⏳ |
| 4 | `docs-sync.py` and the README's other sections still agree | no drift | ⏳ |

**Arpit, 2026-11-05 (voice):** on *"Your team's rules, enforced on every AI edit"* with ESLint, Biome and Ruff as the engine underneath — *"It is [too narrow]. You can actually lint it even without AI. Should we call it out?"*; on calling it out as the second line, not the headline — *"Perfect. Let's go with it."*; then *"Create a work item for readme opening."*

**Model:** Opus (Claude Code or Cowork): a short piece of writing that has to stay faithful to what the binary does today.

## Goal

The README's opening says what codelaw is for in one line a team feels: their own rules, held on every edit an AI agent makes. The second line says it is deterministic and runs the same with no AI — the reason a team can trust it in CI. The rest of the README (four moments, comparison, packs) follows unchanged unless it contradicts the new opening.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · lead | the first line under the title is the ruled lead, or a wording Arpit approves | ⏳ |
| 2 · second line | deterministic, no AI in the engine, same result with or without an agent ([SR-LAW-5](../../records/0007_LAW-5-no-model-in-the-engine.md)) | ⏳ |
| 3 · honest | no claim the binary does not back today; no number from W-102 until it has run | ⏳ |
| 4 · checks | `python3 scripts/docs-sync.py --check` green | ⏳ |

## Blockers

None.

## Record affected

**No SR affected** — README copy. README and CHANGELOG by [SR-WORK-DOCS](../../records/0055_WORK-docs.md) decision 10.

## Hazards

- **W-102's step 4** decides the README's "rules it can't skip" line from data. Don't remove or harden that line here; W-102 owns it.
- The screen (W-102, 2026-10-30) found a frontier agent already follows a convention it can see in a helper. The lead claims enforcement, which the hook does; it must not claim agents *need* it until W-102 says so.
- `docs-sync.py` fills the pack and fix-class tables, the counts and the roadmap between `<!-- docs-sync:… -->` markers; edit outside them.

## Reference

- [`README.md`](../../README.md) — *The story*, *One set of rules, four moments*.
- [W-102](W-102-guidance-vs-check-experiment.md) — the experiment behind the pitch.
