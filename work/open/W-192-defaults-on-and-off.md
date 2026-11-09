---
type: Handoff
name: W-192
description: "Arpit, 2026-11-09 (voice docket): one place that lists every rule on and off by default, per pack and per preset — with W-139's preset and promotion leftovers."
item: W-192
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-192 — What is on and what is off when codelaw is installed

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Generate a page: every pack, every preset, each rule on or off by default | the defaults reviewed in one place | ⏳ |
| 2 | Build the presets ruled and not yet made: `py/pylint`, `py/strict`, a pydocstyle preset | the opt-in presets that were promised | ⏳ |
| 3 | Put the 23 Oxlint correctness ports with no corpus finding to the decider for `recommended` | a ruling on what turns on by default | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* *"add one more item to the list what all things are by default turned on and turned off when code law is installed"*. Filed on *"file it."*

**Model:** Sonnet (Claude Code): a generator over the packs, then three presets.

## Goal

A page, generated from the packs and checked in CI, that says what a fresh install turns on. W-139's DoD 2 (presets) and 2d (Oxlint correctness rules proposed for `recommended`) live here now.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · page | generated, with `--check` in CI so it cannot drift | ⏳ |
| 2 · presets | `py/pylint` (W-144 R7), `py/strict` and a pydocstyle preset (B-048) — or each one's reason it is not built | ⏳ |
| 3 · 2d | the 23 rules' corpus counts put to the decider | ⏳ |
| 4 · ledger | `work/proposals/rule-parity/method/classify.py` learns the classes ruled 2026-11-09 (the proposal's §14), so a regenerated CSV agrees | ⏳ |

## Blockers

None.

## Record affected

SR-RULE-PACKS (presets); SR-WORK-DOCS if a new maintained page is added.

## Hazards

- Name what "on" means: on in a preset that `init` turns on, not only in a preset a project may choose.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (DoD 2, 2d; *Remaining — strand W-139-native*)
