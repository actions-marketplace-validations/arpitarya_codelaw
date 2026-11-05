---
type: Handoff
name: W-163
description: "Arpit, 2026-11-05 (voice): research which automatic fixes skip save and the agent hook, and target versions as inputs to fixes — both configurable per rule and per project, with a default for every rule."
item: W-163
filed: 2026-11-05
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Cowork (Opus 5.5)"
ball: cowork
---

# W-163 — Research: fixes that wait for `codelaw fix`, and target versions

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | List every `auto` rule whose edit throws away work in progress (an unused import, code after a debugging `return`, a `.only` test) and propose each rule's default | safe saves, safe agent edits | ⏳ |
| 2 | List every rule whose fix is safe only from a version up (9 CSS, 3 Python today, and any others) and the version each needs | which fixes a target unlocks | ⏳ |
| 3 | Propose the keys: the rule's setting, the project's override per rule and per scope, the targets in `codelaw.toml` | the config shape | ⏳ |
| 4 | Put the proposal to the decider, then file the build item | a ruled design | ⏳ |

**Arpit, 2026-11-05 (voice):** *"Probably research all these items and make it configurable against each and every rule."* · *"come up with default values as well for these rules."* · *"Let's go with it."* · *"But make it configurable as well."*; then (text) *"file it."*

**The direction, ruled 2026-11-05:**
- **Save behaviour:** a per-rule setting (working name `on_edit`) covering save **and** the agent hook — an agent adds an import one edit before its use. `codelaw fix` and pre-commit still apply the fix. Each rule has a default; a project switches it per rule.
- **Python target:** read from `requires-python` in `pyproject.toml`; codelaw's config can override it, also per rule.
- **Browser targets:** explicit minimum versions only (`chrome >= 110`), in codelaw's config or a browserslist entry that lists versions. No bundled caniuse data: it is CC-BY-4.0, and [SR-RULE-PACKS](../../records/0101_rule-packs.md) already refuses non-OSI data licences (MDN's CC0 data).
- **Default with no target:** the fix stays `suggest`, never `auto` — an `auto` fix must never change behaviour (L4 decision 3).

**Model:** Cowork: research and a config design (SR-WORK-OPEN-QUEUE rule 37a), no code.

## Goal

A ruled design for both settings, each rule's default, and the build item filed.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · save | every `auto` rule judged, its default named, the reason in one line | ⏳ |
| 2 · targets | every target-dependent rule, its version threshold, how the target is read and overridden | ⏳ |
| 3 · config | the keys, their precedence (rule default < project < scope < rule entry), `config --print` and `rules --why` showing them | ⏳ |
| 4 · ruled | the decider rules on the proposal; the build item is filed | ⏳ |

## Blockers

None.

## Record affected

**No SR affected** by the research; the build changes [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decision 12 (what the agent hook applies), [SR-CONFIG](../../records/0110_config.md) and [SR-RULE-FORMAT](../../records/0112_rule-format.md).

## Hazards

- A default that is too eager breaks old setups; the ruled default is the safe one.

## Reference

[W-156](W-156-fix-ladder.md) (where the two questions came from) · [SR-LAW-5](../../records/0007_LAW-5-no-model-in-the-engine.md) (no network) · [SR-LAW-2](../../records/0004_LAW-2-foss-dependencies.md)
