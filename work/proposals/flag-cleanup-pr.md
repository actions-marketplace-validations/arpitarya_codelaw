---
type: Proposal
title: One cleanup PR per decided flag, to its owner — Piranha's loop on codelaw's flag registry
description: "The scheduled auto-fix workflow gains a flags mode: one PR per flag whose registry entry says keep, reviewer the flag's owner; keep-branch learns && / ! simplification so fewer reads fall to manual."
status: proposed
timestamp: 2026-11-05T19:50:00+05:30
---

# One cleanup PR per decided flag

Filed 2026-11-05 (Cowork, Opus 5.5) from the research brief — *"file proposals
for all of the items"*. Item 6 of 7.

## 1 · In one line

**codelaw already knows each flag's owner, expiry and kept branch. Close the
loop: a weekly job opens one PR per decided flag, assigned to its owner.**

## 2 · Evidence

| source | finding |
|---|---|
| Uber Piranha (2019; open source) | ~2,000 stale flags removed; a **weekly pipeline** opens one diff per stale flag with **the flag's author as reviewer**, who accepts, edits or marks it not stale; a bot nudges open ones; boolean checks simplified, tests tied to the flag cleaned |
| Google, AI for internal code migrations (arXiv 2501.06972) | in flag cleanup, checks confirm every instance is gone and build and tests pass |

## 3 · Today

- `js/flag-expiry`, `py/flag-expiry` — registry `.codelaw/flags.toml` with `owner`, `expires`, `keep`; fix `keep-branch` (`suggest`) ([SR-PROJECT-GRAPH](../../records/0103_project-graph.md) decision 11).
- `keep-branch` refuses a read inside a larger condition (`&&`, `!`, `await`) → `manual`.
- `init --ci … --autofix daily|weekly [--unsafe]` opens **one** PR with every fix ([SR-CLI-MCP](../../records/0115_cli-and-mcp.md) decision 12); it has never run for real (B-053).

## 4 · The proposal

- **`--autofix weekly --flags`** — a second workflow from its own template: for each registry entry with `keep` set, run `codelaw fix --unsafe --only-flag <name>` on a branch `codelaw/flag-<name>`, open or update one PR, **request review from `owner`**, link `ticket`.
- **`codelaw fix --only-flag <name>`** — applies `keep-branch` for that flag's reads only, then the follow-on `auto` fixes it unlocks (the marker and the registry entry removed).
- **`keep-branch` gains boolean simplification:** `flag && x` → `x` / `false`, `!flag` → constant, `flag ? a : b` inside a larger expression — each a rewrite with the same no-side-effect rule.
- **Tests tied to the flag:** a test file whose only flag read is this flag, under `keep = "off"`, is reported, not deleted (`manual`).
- **Follow-on dead code** is the next run's `graph/no-unused-*` findings, not this PR's.

## 5 · Laws touched

- **L4** — fewer `manual` findings (the boolean cases).
- **L5** — the job runs in the consumer's CI; codelaw itself stays offline.
- **L10** — one new workflow template per CI.

## 6 · Open questions for the decider

1. **Expired without `keep`** — open an issue to the owner, or only the CI finding? Recommended: finding only; codelaw opens nothing it cannot fill.
2. **Owner as GitHub reviewer** — `owner` is an email today; map through CODEOWNERS or a `[flags] reviewers` table?
3. **Order against B-053** — prove the plain auto-fix PR first? Recommended: yes.

## 7 · Graduation trigger

**B-053 proven, then the decider rules §6.1–6.2** → a W-item: `--only-flag`, the boolean rewrites, the flags workflow templates.

## Sources (read 2026-11-05)

- Uber — Introducing Piranha — <https://www.uber.com/blog/piranha/>
- How is Google using AI for internal code migrations? — <https://arxiv.org/abs/2501.06972>
