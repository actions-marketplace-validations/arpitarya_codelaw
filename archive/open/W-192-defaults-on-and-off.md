---
type: Handoff
name: W-192
description: "Arpit, 2026-11-09 (voice docket): one place that lists every rule on and off by default, per pack and per preset — with W-139's preset and promotion leftovers."
item: W-192
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-24
updated_by: "Claude Code (Sonnet 5.5): the 23 added; closed"
ball: none
---

# W-192 — What is on and what is off when codelaw is installed

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Generate a page: every pack, every preset, each rule on or off by default | the defaults reviewed in one place | ✅ 2026-11-17 |
| 2 | Build the presets ruled and not yet made: `py/pylint`, `py/strict`, a pydocstyle preset | the opt-in presets that were promised | ✅ 2026-11-17 |
| 3 | Put the 23 Oxlint correctness ports with no corpus finding to the decider for `recommended` | a ruling on what turns on by default | ✅ ruled 2026-11-24: all 23 |
| 4 | Add the 23 to their `recommended` presets and regenerate the docs | the ruling in the packs | ✅ 2026-11-24 |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* *"add one more item to the list what all things are by default turned on and turned off when code law is installed"*. Filed on *"file it."*

**Model:** Sonnet (Claude Code): a generator over the packs, then three presets.

## Goal

A page, generated from the packs and checked in CI, that says what a fresh install turns on. W-139's DoD 2 (presets) and 2d (Oxlint correctness rules proposed for `recommended`) live here now.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · page | generated, with `--check` in CI so it cannot drift | ✅ `docs/rules/defaults.md` and `docs/rules/defaults/<pack>.md`, written by `scripts/rule-docs.py`, held by its `--check` (CI's `lint` job) |
| 2 · presets | `py/pylint` (W-144 R7), `py/strict` and a pydocstyle preset (B-048) — or each one's reason it is not built | ✅ `py/strict` (622 rules with `py/recommended`), `py/pylint` (245), `py/pydocstyle` (39), opt in; `codelaw test packs/python` 889 rules · 0 failed |
| 3 · 2d | the 23 rules' corpus counts put to the decider | ✅ ruled 2026-11-24: all 23, below |
| 5 · built | the 23 in `js/recommended`, `react/recommended`, `ts/recommended` at `error`; `rule-docs.py --write`, `docs-sync.py --write`; `codelaw test` on the three packs | ✅ 2026-11-24 — `codelaw test`: js 970 rules, react 163, ts 67, 0 failed |
| 4 · ledger | `work/proposals/rule-parity/method/classify.py` learns the classes ruled 2026-11-09 (the proposal's §14), so a regenerated CSV agrees | ✅ its `RULED` table; `classify.py` then `tables.py` regenerates exactly the committed CSVs |

## The decider's ruling — the 23 Oxlint correctness ports (DoD 3)

**Ruled — Arpit, 2026-11-24 (voice docket, Cowork):** asked *all 23, all but `no-eval` and `react/no-unsafe`, or none?* — *"Go with the recommendation."* So **all 23**, those two included. Filed on *"file it."*

**Question:** should these 23 rules join their pack's `recommended` preset? Each is Oxlint's `correctness` category, none is in `recommended` today, each had **0 findings** on the corpus run (`packs/*/corpus.toml`), and each is `preview` — so joining `recommended` changes nothing for a project until it sets `preview = true`, and nothing is on by default until a promotion makes the rule `stable` ([SR-RULE-PACKS](../../records/0101_rule-packs.md) decision 9; the [page of defaults](../../docs/rules/defaults.md) shows both gates).

**Recommended: yes, all 23.** A zero count on the corpus is the evidence [SR-RULE-AUTHORING](../../records/0113_rule-authoring.md) decision 6a counts. Held back on purpose, and not in the 23: `js/no-erasing-op` (6 findings on the corpus, every one a deliberate `get(dep) * 0` in jotai's tests) and `react/a11y-control-has-associated-label` (2, not reviewed).

| pack, preset it would join | rules | fix |
|---|---|---|
| `js` → `js/recommended` | `js/no-caller`, `js/no-eval`, `js/no-iterator`, `js/no-useless-rename`, `js/no-array-method-on-arguments`, `js/no-char-at-multichar-compare`, `js/match-all-needs-global`, `js/replace-all-needs-global`, `js/number-arg-out-of-range`, `js/no-uninvoked-array-callback` | `suggest` · `manual` · `auto` |
| `react` → `react/recommended` | `react/a11y-lang`, `react/a11y-no-aria-hidden-on-focusable`, `react/a11y-prefer-tag-over-role`, `react/forward-ref-uses-ref`, `react/jsx-props-no-spread-multi`, `react/no-did-mount-set-state`, `react/no-did-update-set-state`, `react/no-this-in-sfc`, `react/no-unsafe`, `react/no-will-update-set-state`, `react/void-dom-elements-no-children` | `suggest` · `manual` |
| `ts` → `ts/recommended` | `ts/no-unnecessary-parameter-property-assignment`, `ts/no-useless-empty-export` | `auto` |

`js/no-eval` is already in `js/sonarjs` and `js/no-char-at-multichar-compare` in `js/strict`; `react/no-unsafe` and `no-eval` are the two that flag code a project may keep on purpose, so the decider may want them left out.

**If yes:** add the 23 to those three presets at `error`, rerun `python3 scripts/rule-docs.py --write` and `docs-sync.py --write`, and close this item. **If some:** the same for those. **If no:** close this item as is.

## Blockers

None. Ruled 2026-11-24; what is left (DoD 5) is the *If yes* step above.

## Record affected

SR-RULE-PACKS (the python presets, and the page of defaults in decision 6); SR-WORK-REGISTRY (the `docs/rules/` row).

## Hazards

- Name what "on" means: on in a preset that `init` turns on, not only in a preset a project may choose.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (DoD 2, 2d; *Remaining — strand W-139-native*)
