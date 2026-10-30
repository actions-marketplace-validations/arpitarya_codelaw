---
type: Handoff
name: W-156
description: "Arpit, 2026-10-30: every rule fixes automatically first, suggests a diff for the agent second, and is manual only by exception — L4 rewritten; build the engine pieces and move the 717 non-auto rules up the ladder."
item: W-156
filed: 2026-10-30
filed_by: "Arpit (Claude app, Opus 5.5)"
updated: 2026-10-30
updated_by: "Arpit (Claude app, Opus 5.5)"
ball: agent
---

# W-156 — Every rule fixes itself first, suggests second, and is manual only by exception

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Count every rule's fix class and review the 717 that aren't automatic | a proposed rung for each rule | ✅ 2026-10-30 |
| 2 | Rewrite L4 as the ladder: automatic, then suggested, then manual by a named exception | the rule every future rule follows | ✅ 2026-10-30 |
| 3 | Teach the engine per-finding fallback, choices, `fix_exception`, the manual ceiling and `--fix-stats` | the law becomes checkable | ⏳ |
| 4 | Move the rules up, pack by pack, choices first: 577 manual → 147 | agents get diffs, not prose | ⏳ |
| 5 | The decider rules on save vs `fix`, and on two new config inputs | the last 21 moves and safe saves | 🔴 decider decides |

**Arpit, 2026-10-30:** *"Whenever a rule gets created, there has to be a fix for it. The first priority is fixing it automatically. The second priority is suggesting it so that AI agent can fix it. The last one is manual, and only in exceptional cases we go with manual. Research all the rules and propose how it can be fixed. The priority, create a law SR for it."*

**Model:** Opus for slice A (the engine's fix model touches a law and every output format); Sonnet for the per-pack moves, where each rule's examples prove the move.

## Goal

Build [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decisions 2, 4, 5, 6, 7 and 15, then move each rule to the rung [`fix-ladder/rules.csv`](../proposals/fix-ladder/rules.csv) proposes.

## Definition of done

| DoD | what | state |
|---|---|---|
| A1 · per-finding rung | a finding drops below its rule's class when its own edit can't be proved safe (decision 2); `Diagnostic` reports the finding's rung | ⏳ |
| A2 · choices | `Diagnostic` holds several labelled edit sets; text, JSON, SARIF, MCP, LSP quick fixes and the agent hook show all; `fix --unsafe` applies only a `default` choice; `codelaw test` proves each against `Fixed (<label>)` (decisions 4, 11) | ⏳ |
| A3 · `fix_exception` | the rule schema gains it; the loader refuses a `manual` rule without one of `value`, `restructure`, `authored`, `security` (decision 5); `ENGINE_RULES` carry one (decision 15). Lands with B0, so nothing fails to load | ⏳ |
| A4 · ceiling | `packs/fix-ladder.toml` at today's counts; a `laws.rs` test fails a pack over its ceiling (decision 6) | ⏳ |
| A5 · `--fix-stats` | `codelaw rules --fix-stats [--check]`, text and JSON; `scripts/rule-docs.py` writes each pack README's table (decision 7) | ⏳ |
| B0 · exceptions | the 147 rules staying `manual` get their `fix_exception` from the CSV | ⏳ |
| B1 · choices rules | the 155 `suggest-choices` rows, pack by pack, each with its `Fixed (<label>)` examples | ⏳ |
| B2 · subset `auto` | the 142 `auto+` rows: the subset condition in `fix_reason`, a `Don't` example on each side | ⏳ |
| B3 · the rest | 16 + 3 → `auto`; 181 → `suggest`; ceilings lowered as each pack lands | ⏳ |
| B4 · needs | cross-file (69), type info (17), add-import (16), scope rename (6), computed edits (6), purity (5): each row whose capability is missing stays at its current rung until that capability lands | ⏳ |
| C · ports | W-139's ports are judged against decisions 3–5 when built; none is `manual` only because its upstream has no fix | ⏳ |

## Open questions — the decider

1. **Save vs `fix`.** Some `auto` edits keep behaviour but destroy work in progress on every save: a whole unused import, unreachable code, a focused test. Recommended: a rule may say `on_save: false`. `codelaw fix` and the agent hook still apply it; save does not.
2. **Browser targets** (9 CSS rules) and **the target Python version** (3 rules) as config inputs. Without them those rules stay `suggest`.

## Blockers

None for A and B. Open questions 1 and 2 hold only their own rows.

## Record affected

SR-LAW-4 (rewritten 2026-10-30); SR-RULE-FORMAT (the `fix_exception` key, choices in `fix:` steps, `Fixed (<label>)` blocks); SR-ARCHITECTURE (the fix verifier for choices); SR-EDITOR and SR-CLI-MCP (choices on every surface).

## Hazards

- **A3 and B0 land together,** or every `manual` rule stops loading.
- **Low confidence first.** Check the 44 rows marked low before building them. `flask-settings` and `django-settings` were reviewed from a truncated source.
- **Subset conditions must be exact.** An `auto` edit that fires outside its subset changes behaviour silently, which is the worst failure this law allows.
- **Lockmaster.** The packs are shared with W-139. Claim one pack per change.

## Reference

- [`work/proposals/fix-ladder.md`](../proposals/fix-ladder.md) and [`fix-ladder/rules.csv`](../proposals/fix-ladder/rules.csv)
- `crates/codelaw-core/src/diagnostic.rs` `FixClass`, `Diagnostic`
- `crates/codelaw-rules/src/rule.rs`: the frontmatter keys
