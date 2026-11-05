---
type: Proposal
title: The done gate — codelaw at the agent's stop, and a measured agent-output budget
description: "A third hook moment after Gate and Enforce: the agent cannot finish (or open a PR) while its diff adds an error; and the size of what hook and MCP return to an agent gets measured and an L8-style budget."
status: proposed
timestamp: 2026-11-05T19:50:00+05:30
---

# The done gate — codelaw at the agent's stop

Filed 2026-11-05 (Cowork, Opus 5.5) from the research brief asked for that day:
*"research repo, papers case studies which could tell how codelaw could help or
how it can improve"* → *"file proposals for all of the items"*. Item 1 of 7.

## 1 · In one line

**Gate refuses a bad write, Enforce fixes after it — nothing stops an agent
from *finishing* with new errors. Add `codelaw hook --stop`, and measure how
many tokens codelaw hands an agent per call.**

## 2 · Evidence

| source | finding | what it says for codelaw |
|---|---|---|
| SWE-agent (NeurIPS 2024) | a linter that rejects a bad edit: 15.0% → 18.0% resolved on SWE-bench Lite | the Gate (`hook --pre`) is the right shape — already built |
| Static Analysis as a Feedback Loop (arXiv 2508.14419) | Bandit + Pylint fed back to GPT-4o: security issues > 40% → 13%, readability > 80% → 11% | iterating to green matters, not one pass |
| Spotify background coding agent | deterministic verifiers before an LLM judge; a Claude Code **stop hook blocks PR creation** if one fails; verifiers return only the relevant errors to save context | the missing moment is *done*; output size is a design constraint |

## 3 · Today

- **Gate** — `hook --pre` refuses an edit that adds an `error` ([SR-AGENT-SURFACES](../../records/0102_agent-surfaces.md) decision 9).
- **Enforce** — `PostToolUse` runs `codelaw hook` on the edited file.
- **No done moment.** `templates/agents/claude/` has `hook-group.json` and `pre-hook-group.json`; no `Stop` hook.
- **No measured output size.** The hook's and MCP tools' output is shaped for agents (SR-CLI-MCP decision 5), but its token cost is not measured — an `unmeasured` claim.

## 4 · The proposal

- **`codelaw hook --stop --agent <name>`** — on the agent's stop event, checks every file the session changed (`--changed-since` the session's base) and blocks the stop while a new `error` remains; the reason lists at most N findings.
  - Claude Code `Stop`; the other agents' equivalents only once a payload fixture exists (the matrix rule of decision 2).
  - **Fails open** when codelaw itself fails, like the Gate.
  - **Loop guard:** the same finding set twice in a row → let the stop through with a note (the `codelaw-fix` skill's "two honest attempts" rule, made mechanical).
- **Template** `templates/agents/claude/stop-hook-group.json`; `sync-agents` writes it beside the other two (L10).
- **Agent-output budget** — `codelaw bench` measures bytes and an approximate token count for `hook`, `hook --pre`, `hook --stop` and each MCP tool on the reference workload; the number goes to the decider as a budget, then into L8.

## 5 · Laws touched

- **L5** — none: an exact-match check at a new moment.
- **L8** — a new measured row; a budget only on the decider's ruling.
- **L10** — one new template.

## 6 · Open questions for the decider

1. **Block on new errors only, or on any error in a touched file?** Recommended: new only — a baselined file must not trap the agent.
2. **N findings in the stop reason** — recommended 10, the rest as a count.
3. **Is the output budget a law row or a context row?** Recommended: context first, law after three CI runs.

## 7 · Graduation trigger

**The decider rules on §6** → a W-item builds `hook --stop`, its template, the Claude Code fixture and the bench rows.

## Sources (read 2026-11-05)

- SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering — Table 3 — <https://arxiv.org/pdf/2405.15793>
- Static Analysis as a Feedback Loop: Enhancing LLM-Generated Code Beyond Correctness — <https://arxiv.org/abs/2508.14419>
- Spotify: building reliable background coding agents with verification loops — <https://zenml.io/llmops-database/building-reliable-background-coding-agents-with-verification-loops>
