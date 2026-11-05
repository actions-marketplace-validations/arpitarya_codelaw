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
| 0 | `[fix] on_edit` and `[fix] multi_file` in the loader and the schema — main was red on SR-CONFIG's veto test | main green | ✅ 2026-11-05 |
| 1 | The `on_edit` rule key, its config (`[rules]`, scopes, profiles, `[fix] on_edit`), save and the agent hook honouring it, and the 14 rules' `false` defaults | a save or an agent edit never deletes work in progress | ✅ 2026-11-05 |
| 2 | `[targets]`: Python from `requires-python`'s lower bound, explicit browser minimums, per scope and per rule; `config --print` and `rules --why` show them | fixes know which versions the project supports | ✅ 2026-11-05 |
| 3 | The `fix_target` key on the twelve Python rules, `py/lru-cache-with-maxsize-none` and `py/lru-cache-without-parameters` among them | `auto` only where the target allows it | ✅ the two lru_cache rules; the other ten are `suggest` and take `fix_target:` as W-156 moves them |
| 4 | codelaw's own browser-support table as its own data package, `@codelaw/data-browsers` / `codelaw-data-browsers` (L6 decision 15), starting with what the corpus finds; the nine CSS rules read it | CSS prefix fixes go `auto` for a project that states its browsers | ⏳ |
| 5 | A weekly CI workflow: check the browsers' releases, open a pull request naming the rows to review | the table stays current; the engine never goes online | ⏳ |

**Ruled 2026-11-05 (Arpit, Claude Code, multiple choice)** on W-163's proposal, accepted as presented: *"Own table (Recommended)"* · *"Target-gated (Recommended)"*. The design: [`save-behaviour-and-targets.md`](../proposals/save-behaviour-and-targets.md) §1–3, §5. W-163's file carries the full ruling: [W-163](../../archive/open/W-163-save-behaviour-and-targets.md).

**Arpit, 2026-11-05 (voice), refining the table:** the table *"can we have it as a separate package"* · *"let's have a CI/CD pipeline to keep a check on it"* · on PyPI *"yeah i do"*; then (text) *"file it."* The table is its own data package on npm and PyPI ([L6](../../records/0008_LAW-6-python-and-node-packages.md) decision 15), with a weekly CI check (step 5). Its names wait on [W-142](W-142-framework-pack-names.md) before its first release.

**Model:** Opus (Claude Code): config loader, the LSP and hook surfaces, rule files and a cited data table.

## Goal

The records' "Built: no" lines below become built: `on_edit`, `[targets]`, `fix_target` and the browser table, each tested.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · on_edit | key loaded and traced; save and `codelaw hook` skip `on_edit: false`; `fix`, pre-commit and `fix --check` still apply it; the 14 defaults set | ✅ `tests/on_edit.rs`; multi-file forced `false` waits for W-162's multi-file fixes |
| 2 · targets | `[targets]` in the loader and `codelaw.schema.json`, §1's map of SR-CONFIG; the PEP 440 lower bound shared with `[deps.runtime]`; unknown target → `suggest` with the line that would make it `auto` | ✅ `tests/targets.rs` |
| 3 · rules | the twelve Python rules carry `fix_target`, proven on both sides by `codelaw test` | ⏳ |
| 4 · table | the data package on both registries' layouts, a source per row and its NOTICE, its own version line named by `--version --json` and the cache key; the nine CSS rules gated by it | ⏳ |
| 5 · ci | a scheduled weekly workflow checks the browsers' releases and opens a pull request naming the rows to review; no network in the engine | ⏳ |

## Blockers

None.

## Record affected

[SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decision 12; [SR-CONFIG](../../records/0110_config.md) decisions 11e–11f; [SR-RULE-FORMAT](../../records/0112_rule-format.md) decisions 2b–2c; [SR-ARCHITECTURE](../../records/0108_architecture.md) (`[fix] on_edit`); [SR-RULE-PACKS](../../records/0101_rule-packs.md) (the data table); [SR-AGENT-SURFACES](../../records/0102_agent-surfaces.md) (the hook).

## Hazards

- **Publishing needs the names:** `@codelaw/data-browsers` and `codelaw-data-browsers` are reserved before the release that first carries them ([W-142](W-142-framework-pack-names.md), the decider's hands).
- W-156 and W-162 move rules to `auto` in the same packs: a move in an `on_edit` kind sets `on_edit: false` in the same change (SR-RULE-FORMAT decision 2b).
- W-158 is changing `crates/codelaw-config` now: claim after it lands, or claim per file.

## Reference

[W-163](../../archive/open/W-163-save-behaviour-and-targets.md) · [proposal](../proposals/save-behaviour-and-targets.md) · [W-156](W-156-fix-ladder.md) · [W-162](W-162-fix-capabilities-build.md)
