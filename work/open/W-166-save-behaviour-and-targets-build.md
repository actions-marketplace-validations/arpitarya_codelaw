---
type: Handoff
name: W-166
description: "Arpit, 2026-11-05: build the ruled save-and-targets design — `on_edit` per rule with its defaults, `[targets]` for Python and browsers, codelaw's own browser-support table, the lru_cache rules target-gated."
item: W-166
filed: 2026-11-05
filed_by: "Arpit (Claude Code, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-166 — Build: fixes that wait for `codelaw fix`, and target versions

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | The `on_edit` rule key, its config (`[rules]`, scopes, profiles, `[fix] on_edit`), save and the agent hook honouring it, and the 14 rules' `false` defaults | a save or an agent edit never deletes work in progress | ⏳ |
| 2 | `[targets]`: Python from `requires-python`'s lower bound, explicit browser minimums, per scope and per rule; `config --print` and `rules --why` show them | fixes know which versions the project supports | ⏳ |
| 3 | The `fix_target` key on the twelve Python rules, `py/lru-cache-with-maxsize-none` and `py/lru-cache-without-parameters` among them | `auto` only where the target allows it | ⏳ |
| 4 | codelaw's own browser-support table, `packs/css/data/support.json`, starting with what the corpus finds; the nine CSS rules read it | CSS prefix fixes go `auto` for a project that states its browsers | ⏳ |

**Ruled 2026-11-05 (Arpit, Claude Code, multiple choice)** on W-163's proposal, accepted as presented: *"Own table (Recommended)"* · *"Target-gated (Recommended)"*. The design: [`save-behaviour-and-targets.md`](../proposals/save-behaviour-and-targets.md) §1–3, §5. W-163's file carries the full ruling: [W-163](../../archive/open/W-163-save-behaviour-and-targets.md).

**Model:** Opus (Claude Code): config loader, the LSP and hook surfaces, rule files and a cited data table.

## Goal

The records' "Built: no" lines below become built: `on_edit`, `[targets]`, `fix_target` and the browser table, each tested.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · on_edit | key loaded and traced; save and `codelaw hook` skip `on_edit: false`; `fix`, pre-commit and `fix --check` still apply it; the 14 defaults set | ⏳ |
| 2 · targets | `[targets]` in the loader and `codelaw.schema.json`, §1's map of SR-CONFIG; the PEP 440 lower bound shared with `[deps.runtime]`; unknown target → `suggest` with the line that would make it `auto` | ⏳ |
| 3 · rules | the twelve Python rules carry `fix_target`, proven on both sides by `codelaw test` | ⏳ |
| 4 · table | `support.json` with a source per row and its NOTICE; the nine CSS rules gated by it | ⏳ |

## Blockers

None.

## Record affected

[SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decision 12; [SR-CONFIG](../../records/0110_config.md) decisions 11e–11f; [SR-RULE-FORMAT](../../records/0112_rule-format.md) decisions 2b–2c; [SR-ARCHITECTURE](../../records/0108_architecture.md) (`[fix] on_edit`); [SR-RULE-PACKS](../../records/0101_rule-packs.md) (the data table); [SR-AGENT-SURFACES](../../records/0102_agent-surfaces.md) (the hook).

## Hazards

- W-156 and W-162 move rules to `auto` in the same packs: a move in an `on_edit` kind sets `on_edit: false` in the same change (SR-RULE-FORMAT decision 2b).
- W-158 is changing `crates/codelaw-config` now: claim after it lands, or claim per file.

## Reference

[W-163](../../archive/open/W-163-save-behaviour-and-targets.md) · [proposal](../proposals/save-behaviour-and-targets.md) · [W-156](W-156-fix-ladder.md) · [W-162](W-162-fix-capabilities-build.md)
