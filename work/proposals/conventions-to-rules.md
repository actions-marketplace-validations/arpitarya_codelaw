---
type: Proposal
title: Conventions to rules — every AGENTS.md convention mapped to a rule id, or named as unchecked
description: "A deterministic check that each convention line in a marked AGENTS.md/CLAUDE.md block names the rule that enforces it, plus a skill (beside rules-from-reviews) that drafts the missing local rule."
status: proposed
timestamp: 2026-11-05T19:50:00+05:30
---

# Conventions to rules

Filed 2026-11-05 (Cowork, Opus 5.5) from the research brief — *"file proposals
for all of the items"*. Item 2 of 7.

## 1 · In one line

**Prose says *why*, a rule enforces *how*. codelaw reports every convention in
the repo's agent files that no rule backs, and a skill drafts the rule.**

## 2 · Evidence

| source | finding |
|---|---|
| Factory, *Using linters to direct agents* (2025) | AGENTS.md holds the "why", lint rules the enforceable "how"; **each guideline maps to a rule id**; cycle: observe drift → codify a rule (LLM drafts it) → surface → remediate with agents → prevent regressions. Rule kinds: grep-ability, file layout, layer boundaries, security, colocated tests, logging, docs |
| W-102's screen (2026-10-30, this repo) | arm 0 broke a convention only where **the compliant idiom was not visible in the code** (inline styles, a column-name SQL parameter, copied `== None`); where a helper existed the agent used it 15 of 15 times |
| Evaluating AGENTS.md (ICLR 2026), IFScale (NeurIPS 2025) | prose instructions degrade as their count grows — already in [guidance-experiment-redesign](guidance-experiment-redesign.md) §3 |

**Read together:** prose is enough where the code shows the way; a rule earns its
place exactly where it does not. A repo should know which of its conventions are
prose-only.

## 3 · Today

- `codelaw-rules-from-reviews` drafts rules from repeated review comments ([SR-AGENT-SURFACES](../../records/0102_agent-surfaces.md) decision 10).
- `codelaw-rule-author` agent writes a local rule ([SR-RULE-AUTHORING](../../records/0113_rule-authoring.md)).
- Nothing reads the repo's own `AGENTS.md` / `CLAUDE.md` conventions.

## 4 · The proposal

- **A marked block**, e.g. `<!-- codelaw:conventions -->` … `<!-- /codelaw:conventions -->`, one convention per list item, each ending in a rule id (`— acme/no-inline-style`) or `— unchecked: <reason>`.
- **`docs/unmapped-convention`** (Markdown, `info` by default): an item with neither a rule id nor `unchecked:`; an id that names no rule is an `error`. Fix: `suggest` — the skill's draft.
- **`codelaw conventions`** (CLI) / `list_conventions` (MCP): the table convention → rule → state (`checked`, `unchecked`, `unmapped`), text or JSON.
- **Skill `codelaw-rules-from-conventions`** — reads unmapped items, drafts each as a local rule with cases, runs it on the repo, returns the diff. The engine only checks; drafting stays in the skill (L5).

## 5 · Laws touched

- **L3** — Markdown is a codelaw language; the block is plain Markdown.
- **L4** — the new rule has a `suggest` fix.
- **L5** — the engine never interprets prose; it checks a list item's trailing id.

## 6 · Open questions for the decider

1. **Opt-in marker block, or every list item under a `## Conventions` heading?** Recommended: the marker — no false positives on ordinary docs.
2. **Severity of `docs/unmapped-convention`** — recommended `info`, `warn` in CI profile.
3. **Should W-102's write-up cite this as the use of its result?**

## 7 · Graduation trigger

**The decider picks the block form (§6.1)** → a W-item: the rule, the command and MCP tool, the skill with evals, a fixture repo.

## Sources (read 2026-11-05)

- Factory — Using linters to direct agents — <https://factory.com/news/using-linters-to-direct-agents>
- W-102 screen results — [`experiments/guidance-vs-check/results/2026-10-w102-screen/`](../../experiments/guidance-vs-check/results/2026-10-w102-screen/RESULTS.md)
