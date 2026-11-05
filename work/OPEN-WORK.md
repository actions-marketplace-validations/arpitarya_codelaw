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
| 🔴 **`spell`'s L8 budget: 2,152 ms?** ([W-103](open/W-103-spell-pack.md)): the `spell` operation's p95 on the three `perf` runs on `main` is 1,383.3 / 1,434.2 / 1,429.5 ms (`ci` 41005346138, 41006814384, 41007205048; 1,004 files, 12 findings each). Your method (L8 decision 4): 1.5 × the worst, 1,434.2 → **2,152 ms**; no design target, so no guard. **Set it** (then a `spell` p95 over 2,152 ms fails CI) **or leave it unset** (reported only). | 2026-11-05 |
| ↳ **blocks:** W-103 | |
| 🔴 **W-162: may a fix edit several files?** ([proposal](proposals/fix-capabilities.md) §5): 22 rules rename across files (CSS classes, ids, custom properties, SCSS members, exports and their importers) or add a dependency to `package.json`. That amends L4 decisions 10 ("only their own range") and 12 (the hook). **Pick:** (1) *recommended*: allow it — verified per file plus a graph check for new unresolved references, written all-or-nothing, `auto` only when every reference resolves statically, never on save or from the per-edit hook; (2) keep fixes single-file, and the 22 become `manual` `restructure`. | 2026-11-05 |
| ↳ **blocks:** W-162 | |
| 🔴 **W-163: rule on the save-and-targets proposal** ([proposal](proposals/save-behaviour-and-targets.md)): `on_edit` per rule (14 `auto` rules default `false`; a move to `auto` in those kinds sets it), `[targets]` with Python from `requires-python`'s lower bound and explicit browser minimums, overridable per scope and rule. **Two calls:** (1) *recommended*: codelaw writes its own browser-support table (~250 cited rows, starting with what the corpus finds), since caniuse (CC-BY) and MDN (CC0) are refused under L2 — or no table, and nine CSS rules stay `suggest`; (2) *recommended*: `py/lru-cache-with-maxsize-none` (3.9) and `-without-parameters` (3.8), `auto` today, become target-gated as the ruling says — or assume Python ≥ 3.9, as Ruff does. | 2026-11-05 |
| ↳ **blocks:** W-163 | |

---

## Open items (14)

### build (7)

🟢 **W-105** `agent` Type-aware TypeScript: nine rules and the version watch built (its dispatch proof left); next profiling, corpus counts, the RC (11-10). Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-105-type-aware-engine.md)

🟢 **W-139** `agent` Rule parity, wound down 11-05: js 345/828, Ruff 349/708, Pylint 108/298, testing 169/182, nextjs, 5 PyPI packs. Ruled 11-05: no manual ceiling. Next: remove `fix-ladder.toml`, land the 231 held ports. Filed by Arpit (Cowork). [detail](open/W-139-rule-parity-build.md)

🟢 **W-141** `agent` Formatter vs Prettier. Built 10-30: CSS comment, HTML ` />` (HTML 18% → 69%), four Prettier keys, `format-corpus.py`. Oxc's formatter crates ratified 11-05 (L11, D-52); next: build them behind the formatter and remeasure. Filed by Arpit (Cowork). [detail](open/W-141-formatter-parity.md)

🟡 **W-143** waits on **W-139** `agent` Vue, Svelte, Astro, GraphQL as codelaw languages (L3, L7; a parser and a formatter each, grammars ratified under L11) and Biome's 89 rules for them. Filed by Arpit (Cowork). [detail](open/W-143-vue-svelte-astro-graphql.md)

🟢 **W-156** `agent` Fix ladder (L4). Engine, exceptions and lowering built (11-05); 533 rules `fix_pending:`. Ruled 11-05: closes once only the capability-blocked rules are left, their markers naming W-162. Next: the moves, pack by pack. Filed by Arpit (Claude app). [detail](open/W-156-fix-ladder.md)

🔴 **W-162** `arpit` Build the fix capabilities W-161 designed ([proposal](proposals/fix-capabilities.md)): re-judge 37 rules first, then computed edits (57), scope rename, add-import, purity, types, cross-file. Next: Arpit's multi-file call (inbox). Filed by Arpit (Cowork). [detail](open/W-162-fix-capabilities-build.md)

🟢 **W-158** `agent` `.codelawignore` covers every rule — built 10-31. Ruled 11-05: an optional `reason` on a rule set to `"off"` in `[rules]` or a scope, plus `[rules] require_reason` (default `false`); left: build it. Filed by Arpit (Claude app). [detail](open/W-158-codelawignore-every-rule.md)


### ci/cd (3)

🟢 **W-126** `agent` Ruled 10-30: `perf` split across parallel jobs, nothing waits on its verdict (until a budget is set, L8); release 3m50 accepted. Cached CI 4m11–4m25 ✅. `perf` matrix built 11-05 (seven groups, the build's binary). Next: measurement runs. Filed by Arpit (Cowork). [detail](open/W-126-fast-pipeline.md)

🟢 **W-95** `agent` VS Code extension: Marketplace live (0.4.0); Open VSX by trusted publishing in `publish.yml` since 11-05, proven by the next release. Left: a VSCodium install after it (Open VSX's darwin-arm64 is 0.3.0). Filed by Arpit (Cowork). [detail](open/W-95-publish-vscode-extension.md)

🟡 **W-142** waits on **W-139** `arpit` New packs' registry names: five PyPI framework packs (pending publishers), npm `testing`/`nextjs` (was W-147) and Tailwind's, with Fux indexes; before their release. Merged 10-30. Filed by Arpit (Cowork). [detail](open/W-142-framework-pack-names.md)

### cleanup (0)

### testing (0)

### reporting and benchmarks (2)

🔴 **W-103** `arpit` Spell pack done; the `spell` p95s read from the three `perf` runs (1,383–1,434 ms); blocked on Arpit's ruling on the budget, 2,152 ms by L8's method (inbox). Filed by Arpit (Cowork). [detail](open/W-103-spell-pack.md)

🟡 **W-102** waits on **W-139** `agent` Ruled 11-05: the 7 tempting tasks at 9 runs per arm (~$65), `PREREG.md` amended first — and run only once every rule is built (voice). Rework, turns and tokens: B-072. Filed by Arpit (Cowork). [detail](open/W-102-guidance-vs-check-experiment.md)

### record update (0)

### research (1)

🔴 **W-163** `arpit` Researched ([proposal](proposals/save-behaviour-and-targets.md)): `on_edit` per rule (14 default `false`), `[targets]` for 21 rules. Next: Arpit rules on it (inbox), then its build item. Filed by Arpit (Cowork). [detail](open/W-163-save-behaviour-and-targets.md)

---
