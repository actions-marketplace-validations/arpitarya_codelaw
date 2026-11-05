---
type: Queue
description: "The single live work queue, three lanes, plus the Blocked-on-the-decider inbox. The list only — its rules live in the record below."
governed_by: SR-WORK-OPEN-QUEUE
governed_by_path: records/0051_WORK-open-queue.md
---

# OPEN-WORK — what is still open

**This file is the LIST, plus the legend below.** Everything else — what the
queue is, an item's lifecycle, the shape of a row, ball precedence, ordering,
how the inbox works, and what every session owes — is stated once in
**[SR-WORK-OPEN-QUEUE](../records/0051_WORK-open-queue.md)** and is not repeated
here. Read that record before changing anything below it.

**A ball on every row, always one:** 🔴 blocked on the decider, directly or through another item · 🟣 gated on a named date, directly or through another item · 🟡 waiting on another item, named right after the id — `waits on **W-mm**`, or `**<project> W-mm**` for another project's · 🟢 no blockers.
**Optional, after the ball:** 🧨 broken or getting worse · 🔺 do first (the decider only). Full legend: [SR-WORK-OPEN-QUEUE](../records/0051_WORK-open-queue.md) rules 20–34.

---

## Blocked on the decider

| what the decider decides | filed |
|---|---|

*Empty since 2026-11-05 — next gate: the TypeScript 7.1 RC, 2026-11-10 (W-105).*

---

## Open items (15)

### build (8)

🟢 **W-105** `agent` Type-aware TypeScript: nine rules and the version watch built (its dispatch proof left); next profiling, corpus counts, the RC (11-10). Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-105-type-aware-engine.md)

🟢 **W-139** `agent` Rule parity, wound down 11-05: js 345/828, Ruff 349/708, Pylint 108/298, testing 169/182, nextjs, 5 PyPI packs. Ruled 11-05: no manual ceiling. Next: remove `fix-ladder.toml`, land the 231 held ports. Filed by Arpit (Cowork). [detail](open/W-139-rule-parity-build.md)

🟢 **W-141** `agent` Formatter vs Prettier. Built 10-30: CSS comment, HTML ` />` (HTML 18% → 69%), four Prettier keys, `format-corpus.py`. Oxc's formatter crates ratified 11-05 (L11, D-52); next: build them behind the formatter and remeasure. Filed by Arpit (Cowork). [detail](open/W-141-formatter-parity.md)

🟡 **W-143** waits on **W-139** `agent` Vue, Svelte, Astro, GraphQL as codelaw languages (L3, L7; a parser and a formatter each, grammars ratified under L11) and Biome's 89 rules for them. Filed by Arpit (Cowork). [detail](open/W-143-vue-svelte-astro-graphql.md)

🟢 **W-156** `agent` Fix ladder (L4). Engine, exceptions and lowering built (11-05); 533 rules `fix_pending:`. Ruled 11-05: closes once only the capability-blocked rules are left, their markers naming W-162. Next: the moves, pack by pack. Filed by Arpit (Claude app). [detail](open/W-156-fix-ladder.md)

🟡 **W-162** waits on **W-161** `agent` Build the six fix capabilities W-161 designs (cross-file edits, type information, adding an import, scope rename, computed edits, purity) and move the ~119 rules waiting on them up the ladder. Filed by Arpit (Cowork). [detail](open/W-162-fix-capabilities-build.md)

🟢 **W-158** `agent` `.codelawignore` covers every rule — built 10-31. Ruled 11-05: an optional `reason` on a rule set to `"off"` in `[rules]` or a scope, plus `[rules] require_reason` (default `false`); left: build it. Filed by Arpit (Claude app). [detail](open/W-158-codelawignore-every-rule.md)

🟢 **W-164** `agent` README opening, ruled 11-05: lead *"Your team's rules, enforced on every AI edit"*; second line: deterministic, no AI in the engine, same result with or without an agent. Nothing written. Filed by Arpit (Claude app). [detail](open/W-164-readme-opening.md)

### ci/cd (3)

🟢 **W-126** `agent` Ruled 10-30: `perf` split across parallel jobs, nothing waits on its verdict (until a budget is set, L8); release 3m50 accepted. Cached CI 4m11–4m25 ✅. Next: the `perf` matrix, then the measurement runs. Filed by Arpit (Cowork). [detail](open/W-126-fast-pipeline.md)

🟢 **W-95** `agent` VS Code extension: Marketplace live (0.4.0); Open VSX verified, trusted publisher (env `vscode`) registered 11-05. Left: `publish.yml`'s Open VSX step to trusted publishing, then a VSCodium check. Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-95-publish-vscode-extension.md)

🟡 **W-142** waits on **W-139** `arpit` New packs' registry names: five PyPI framework packs (pending publishers), npm `testing`/`nextjs` (was W-147) and Tailwind's, with Fux indexes; before their release. Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-142-framework-pack-names.md)

### cleanup (0)

### testing (0)

### reporting and benchmarks (2)

🟢 **W-103** `agent` Spell pack done; the `spell` bench operation has its three CI `perf` runs on `main` (10-30: 41005346138, 41006814384, 41007205048). Next: read their p95s, then the budget to Arpit. Filed by Arpit (Cowork). [detail](open/W-103-spell-pack.md)

🟡 **W-102** waits on **W-139** `agent` Ruled 11-05: the 7 tempting tasks at 9 runs per arm (~$65), `PREREG.md` amended first — and run only once every rule is built (voice). Rework, turns and tokens: B-072. Filed by Arpit (Cowork). [detail](open/W-102-guidance-vs-check-experiment.md)

### record update (0)

### research (2)

🟢 **W-161** `cowork` Research: how the engine can make the six kinds of fix it cannot make yet — cross-file edits (69 rules), type information (17), adding an import (16), scope rename (6), computed edits (6), purity (5); one design each, for W-162. Filed by Arpit (Cowork). [detail](open/W-161-fix-capabilities-research.md)

🟢 **W-163** `cowork` Research: per-rule save and agent-hook behaviour (`on_edit`) and target versions (Python, browsers) — each configurable per rule and per project, with a default for every rule. Filed by Arpit (Cowork). [detail](open/W-163-save-behaviour-and-targets.md)

---
