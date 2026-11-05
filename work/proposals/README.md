---
type: Index
description: "Index of parked, undecided ideas, each with a graduation trigger."
---

# `work/proposals/` — parked ideas

Ideas worth keeping that are **not being built now** and **not yet decided**.
Every proposal names its **graduation trigger**. Every file here also carries
one `B-nnn` row in [`../BACKLOG.md`](../BACKLOG.md), filed and deleted in the
same change as the proposal.

Frontmatter:

```yaml
---
type: Proposal
title: <idea>
description: <one line>
status: proposed        # proposed | graduated | rejected
timestamp: <ISO 8601>
---
```

| doc | idea | graduation trigger | status |
|-----|------|--------------------|--------|
| [write-time-guidance](write-time-guidance.md) | `hook --pre` for every agent with a pre-write hook; the brief cut by what is specific to the repo (W-05) | ruled 2026-10-27 → built by W-93 | graduated |
| [linter-for-ai-positioning](linter-for-ai-positioning.md) | codelaw beside ESLint, Prettier, Biome, Ruff, oxc: claims, README changes, gaps G1–G8 (W-36) | ruled 2026-10-27 → W-94–W-102 | graduated |
| [guidance-experiment-redesign](guidance-experiment-redesign.md) | W-102's experiment redesigned: a no-codelaw baseline, tasks screened for temptation by the agent, the check off the guidance arms and on as arm D, ≥ 5 runs per cell, a pre-registered bound instead of "zero in 60" (W-118) | ruled 2026-10-30 → W-102 reruns | graduated |
| [type-aware-rules](type-aware-rules.md) | type-aware TS rules and TS errors through the project's own TypeScript 7.1 API, a Node helper (W-101) | ruled 2026-10-27 → W-104–W-107 | graduated |
| [deps-flags-markers-reviews](deps-flags-markers-reviews.md) | the `deps` pack (vulnerable from merged on/off advisory sources, outdated, licence, lockfile sync, drift, runtime), flag expiry, TODO/skip markers, complexity and CODEOWNERS, rules from review comments | ruled 2026-10-29/30 → W-128–W-133 | graduated |
| [rule-parity](rule-parity.md) | every rule of ESLint, typescript-eslint, TSLint, Ruff, Biome, stylelint, html-validate, markdownlint and the plugins, counted and matched: 3,431 upstream, 653 covered, 1,690 to port in tranches A–E; the formatter measured against Prettier (W-138) | round 1 ruled 2026-10-30 (build all) → W-139; round 2 (W-144) ruled 2026-10-30 (`testing` and `nextjs` npm packs; SonarJS, Pylint, jsdoc opt-in; regexp, promise on) → W-139; Oxlint (§12) → W-148, B-066 | graduated |
| [fix-ladder](fix-ladder.md) | every rule's fix, automatic first: 810 rules inventoried, the 717 non-`auto` reviewed one by one — `manual` 577 → 147; the engine capabilities that unlock it, choices first (W-156) | ruled 2026-10-30 → W-156, L4 rewritten | graduated |
| [agent-done-gate](agent-done-gate.md) | `codelaw hook --stop`: the agent cannot finish with a new error; agent-output tokens measured (research brief, 2026-11-05) | the decider rules its §6 | proposed |
| [conventions-to-rules](conventions-to-rules.md) | every `AGENTS.md` convention names its rule or says unchecked; a skill drafts the rule (research brief, 2026-11-05) | the decider rules its §6 | proposed |
| [security-pack-benchmark](security-pack-benchmark.md) | a published detection rate for the security packs on LLM-written vulnerable code; security fixes to `suggest` (research brief, 2026-11-05) | the decider rules its §6 | proposed |
| [hallucinated-packages-and-api-evolution](hallucinated-packages-and-api-evolution.md) | `deps/unknown-package`, `deps/new-package`; version-gated library deprecation packs (research brief, 2026-11-05) | the decider rules its §6 | proposed |
| [dead-subgraphs-and-duplication](dead-subgraphs-and-duplication.md) | dead islands (cycles included) and duplicated blocks on the project graph (research brief, 2026-11-05) | the decider rules its §6 | proposed |
| [flag-cleanup-pr](flag-cleanup-pr.md) | one weekly cleanup PR per decided flag, its owner as reviewer (research brief, 2026-11-05) | the decider rules its §6 | proposed |
| [suppression-gate-and-rule-usefulness](suppression-gate-and-rule-usefulness.md) | refuse an agent's new suppression; MCP `report_conflict`; a local usefulness count per rule (research brief, 2026-11-05) | the decider rules its §6 | proposed |
