---
type: Handoff
name: W-183
description: "Arpit, 2026-11-09 (voice docket): build the engine change that turns a finding on or off by the project's target Python version — six Ruff and Pylint rows wait on it."
item: W-183
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-183 — Findings that depend on the project's Python version

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Let a rule say which target versions its finding applies to | the finding follows the project's Python version | ⏳ |
| 2 | Build the four Ruff rows and two Pylint rows on it | six rules that stay quiet on targets they do not concern | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the target-gated group, after the recommendation to build it: *"Then we build it."* Filed on *"file it."*

**Model:** Opus (Claude Code): a rule-format change, then six rules.

## Goal

Today `fix_target:` gates a fix, not a finding (SR-RULE-FORMAT decision 2c). This adds a finding gate read from the target codelaw already takes from `requires-python`. Rows: Ruff `FA100`, `FA102`, `PERF203`, `UP036`; Pylint `using-exception-groups-in-unsupported-version` (3.11), `using-generic-type-syntax-in-unsupported-version` (3.12).

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · format | a finding gate in the rule format, with its record decision | ⏳ |
| 2 · rules | the six, each with cases on both sides of its version | ⏳ |
| 3 · reuse | UP046 and UP047 (built reporting on every target) checked against the gate | ⏳ |

## Blockers

None.

## Record affected

SR-RULE-FORMAT decision 2c; SR-CONFIG (the target).

## Hazards

- A project with no `requires-python`: say what the gate does then, the same with or without an agent (L5).

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-ruff, round 4*, *Slice 35*) · `work/proposals/rule-parity/ruff.csv`, `pylint.csv`
