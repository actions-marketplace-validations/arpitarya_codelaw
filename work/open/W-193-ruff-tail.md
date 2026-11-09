---
type: Handoff
name: W-193
description: "Arpit, 2026-11-09 (voice docket): the Ruff rows with no blocker are built — TC008 and SIM905."
item: W-193
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-193 — The last two Ruff rows: TC008 and SIM905

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build TC008 and SIM905 | Ruff's buildable rows done | ✅ 2026-11-09 — `py/quoted-type-alias` (`py_ruff_tc.rs`) and `py/split-static-string` (`py_ruff_fmt.rs`), `preview`, W-139-pydoc, slice 37 |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* Unblocked rows go to build items; only blocked ones were discussed: *"Only the ones that are blocked."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): two rules.

## Goal

`TC008` (quotes removed from a `TypeAlias` value; needs Ruff's runtime-availability check and a target) and `SIM905` (`"a b".split()` to a list).

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rules | both, with cases and fix classes | ✅ both `auto` (TC008 `fix_target` 3.10); `codelaw test packs/python` 0 failed |

## Blockers

None.

## Record affected

no SR affected.

## Hazards

- TC008 is preview upstream; leave it `preview`.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-ruff, round 4*) · `work/proposals/rule-parity/ruff.csv`
