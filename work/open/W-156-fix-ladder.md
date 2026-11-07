---
type: Handoff
name: W-156
description: "Arpit, 2026-10-30: every rule fixes automatically first, suggests a diff for the agent second, and is manual only by exception — L4 rewritten; build the engine pieces and move the 717 non-auto rules up the ladder."
item: W-156
filed: 2026-10-30
filed_by: "Arpit (Claude app, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-156 — Every rule fixes itself first, suggests second, and is manual only by exception

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Count every rule's fix class and review the 717 that aren't automatic | a proposed rung for each rule | ✅ 2026-10-30 |
| 2 | Rewrite L4 as the ladder: automatic, then suggested, then manual by a named exception | the rule every future rule follows | ✅ 2026-10-30 |
| 3 | Teach the engine per-finding fallback, choices, `fix_exception`, the manual ceiling and `--fix-stats` | the law becomes checkable | ✅ 2026-10-31 (lowering, A6, open) |
| 4 | Move the rules up, pack by pack, choices first: 577 manual → 147 | agents get diffs, not prose | ⏳ |
| 5 | The decider rules on save vs `fix`, the two config inputs, and the capability-blocked rules | the last moves and safe saves | ✅ ruled 2026-11-05 — W-161 and W-162 (capabilities), W-163 (save, targets) |

**Arpit, 2026-10-30:** *"Whenever a rule gets created, there has to be a fix for it. The first priority is fixing it automatically. The second priority is suggesting it so that AI agent can fix it. The last one is manual, and only in exceptional cases we go with manual. Research all the rules and propose how it can be fixed. The priority, create a law SR for it."*

**Ruled 2026-11-05. Arpit (voice):** capabilities — *"So we'll do research on each one of these items So create a research work item so that we can figure out how these can be implemented."* · *"We can close it but once the research item is done it needs to be implemented. … go ahead and create one more build item blocked on research work item."*; save and targets — *"Probably research all these items and make it configurable against each and every rule."* · *"come up with default values as well for these rules."* · *"Let's go with it."* · *"But make it configurable as well."*; then (text) *"file it."*
- **Capability-blocked rules (B4):** W-161 researches the six capabilities; W-162 (🟡 waits on W-161) builds them and moves the rules. **W-156 closes when only B4's rules are left**, each `fix_pending: "<rung>; needs <capability> (W-162)"` — [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decision 5 amended to allow it.
- **Save vs `fix`, and the targets:** out of this item, into W-163 (research): a per-rule setting, configurable per rule and per project, with a default for every rule. The 12 target rules stay `suggest` until W-163's build.
- **The manual ceiling (A4) is retired** (W-139's ruling, SR-LAW-4 decision 6); W-139 removes it. The moves no longer lower ceilings.

**Model:** Opus for slice A (the engine's fix model touches a law and every output format); Sonnet for the per-pack moves, where each rule's examples prove the move.

## Goal

Build [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decisions 2, 4, 5, 6, 7, 15 and 16, then move each rule to the rung [`fix-ladder/rules.csv`](../proposals/fix-ladder/rules.csv) proposes.

## Definition of done

| DoD | what | state |
|---|---|---|
| A1 · per-finding rung | a finding drops below its rule's class when its own edit can't be proved safe (decision 2); `Diagnostic` reports the finding's rung | ✅ `auto_when:` on a `match:` block; `NativeRule::rung_of`; `Diagnostic.fix` is the finding's rung |
| A2 · choices | `Diagnostic` holds several labelled edit sets; text, JSON, SARIF, MCP, LSP quick fixes and the agent hook show all; `fix --unsafe` applies only a `default` choice; `codelaw test` proves each against `Fixed (<label>)` (decisions 4, 11) | ✅ `fix: { choices }`, `NativeRule::choices_of`, `Diagnostic.choices`; text, JSON/MCP, SARIF, LSP (one quick fix each), agent hook; `fix --unsafe` takes the default; `codelaw test` proves each against `Fixed (<label>)` |
| A3 · `fix_exception` | the rule schema gains it; the loader refuses a `manual` rule without one of `value`, `restructure`, `authored`, `security` (decision 5); `ENGINE_RULES` carry one (decision 15). Lands with B0, so nothing fails to load | ✅ refused at load without one; `fix_pending:` stands in on a rule awaiting its move; `engine_fix_exception` for `ENGINE_RULES` |
| A4 · ceiling | `packs/fix-ladder.toml` at today's counts; a `laws.rs` test fails a pack over its ceiling (decision 6) | ✅ `packs/fix-ladder.toml` at the 2026-10-31 counts; `laws.rs` `l4_every_pack_is_under_its_manual_ceiling` |
| A5 · `--fix-stats` | `codelaw rules --fix-stats [--check]`, text and JSON; `scripts/rule-docs.py` writes each pack README's table (decision 7) | ✅ `rules --fix-stats [--packs <dir> [--check]] [--format json]`; each pack README's table from `rule-docs.py` |
| A6 · lowering | a rule entry's `fix` key in `[rules]`, scopes and profiles: lower only, raising is exit 2 naming the rule's rung; `config --print` and `rules --why` show it; schema updated (decision 16; SR-CONFIG) | ✅ 2026-11-05 — SR-CONFIG 11d; `config --print`, `rules --why`, schema; the editor completes it (W-157) |
| B0 · exceptions | the 147 rules staying `manual` get their `fix_exception` from the CSV | ✅ 223 `fix_exception` (147 from the CSV, the rest judged for rules W-139 added since) and 489 `fix_pending` on the manual rules proposed to move (see Log) |
| B1 · choices rules | the 155 `suggest-choices` rows, pack by pack, each with its `Fixed (<label>)` examples | ⏳ |
| B2 · subset `auto` | the 142 `auto+` rows: the subset condition in `fix_reason`, a `Don't` example on each side | ⏳ |
| B3 · the rest | 16 + 3 → `auto`; 181 → `suggest`; ceilings lowered as each pack lands | ⏳ |
| B4 · needs | each row whose capability is missing stays at its current rung until that capability lands; its `fix_pending:` names W-162 (ruled 2026-11-05), and W-156 closes when only these are left | ✅ 2026-11-05 — 114 marked: computed-edit 57, cross-file 34, add-import 7, scope-rename 7, type-info 4, purity 2, action-tag SHAs 1 (all W-162), browser-targets (W-166), TS code fixes (W-105); W-161 §3's 37 re-judged `manual` with their exception |
| C · ports | W-139's ports are judged against decisions 3–5 when built; none is `manual` only because its upstream has no fix | ⏳ |

## Open questions — the decider

None — all three ruled 2026-11-05 (above).

## Blockers

None for A and B. Open questions 1 and 2 hold only their own rows.

## Record affected

SR-LAW-4 (rewritten 2026-10-30; decision 16, lowering, the same day); SR-CONFIG (a rule entry's `fix` key); SR-RULE-FORMAT (the `fix_exception` key, choices in `fix:` steps, `Fixed (<label>)` blocks); SR-ARCHITECTURE (the fix verifier for choices); SR-EDITOR and SR-CLI-MCP (choices on every surface).

## Hazards

- **A3 and B0 land together,** or every `manual` rule stops loading.
- **Low confidence first.** Check the 44 rows marked low before building them. `flask-settings` and `django-settings` were reviewed from a truncated source.
- **Subset conditions must be exact.** An `auto` edit that fires outside its subset changes behaviour silently, which is the worst failure this law allows.
- **Lockmaster.** The packs are shared with W-139. Claim one pack per change.

## Reference

- [`work/proposals/fix-ladder.md`](../proposals/fix-ladder.md) and [`fix-ladder/rules.csv`](../proposals/fix-ladder/rules.csv)
- `crates/codelaw-core/src/diagnostic.rs` `FixClass`, `Diagnostic`
- `crates/codelaw-rules/src/rule.rs`: the frontmatter keys

## Log

- **2026-10-31 (Claude Code):** the engine (A1–A5) and B0 landed together. Rules W-139 added after the review were judged under decisions 3–5 and given an exception or a `fix_pending:` (the judgments are in the commit's `b0` table; 10 `ts`, 71 `js`/`node`/`ts` from strand slices 9 and later, 54 from slices 6–7). `fix_pending: "<rung>[; needs <capability>]"` is the transition marker the law's "until W-156 lands" allows; W-156 closes when none is left. **Left:** A6 (lowering, in `codelaw-config`), B1–B4 (the moves themselves, pack by pack), C.
- **2026-11-05 (Claude Code):** merged main; W-139's react rules (45, each `fix_reason:` already opening with its exception) marked; records SR-LAW-3, SR-LAW-10, SR-RULE-AUTHORING, SR-SPELL and SR-WORK-REGISTRY describe the ladder as built. First moves (B1/B3): 7 `pandas`, 2 `ts`, 1 `react`, proven by `codelaw test`; ceilings lowered. A second merge brought 100 more W-139 `js`/`py` rules (judged and marked: 48 exceptions, 52 pending). **535 rules still carry `fix_pending:`** — about 190 `match:` rules whose edit is a template, the rest native rules (Rust per rule) or rules waiting on a capability (cross-file, type info, computed edits, add-import, scope rename). The item stays open until none is left.
- **2026-11-05 (Claude Code), later:** the `spell` test's `format/unformatted` reproduced on main — the fixture's `<img …>` predates the formatter's ` />` for void elements (SR-FORMATTER as built, W-141); the fixture was wrong and is reformatted (`54a6bef7`). **A6 built:** a rule entry's `fix` lowers the rung (SR-CONFIG decision 11d), raising is exit 2.
- **2026-11-05 (Claude Code), wind-down:** `js/no-new-wrappers` and `sec-py/paramiko-host-key` moved to `suggest`. Stopped on the decider's wind-down. **What remains:** B1–B3, the moves: every rule still marked `fix_pending:` (`grep -rl '^fix_pending:' packs`, about 530). Most need more than a template — a native rule's Rust, or a fix that differs per matched pattern. B4: rows marked `fix_pending: "…; needs <capability>"` wait on cross-file edits, type info, computed edits, add-an-import or scope rename; Arpit decides whether those close the item. C: W-139's ports now carry an exception or `fix_pending:` as they land.
- **2026-11-05 (Claude Code), step 4:** B4 marked — the 114 capability-blocked rules name their item, the 37 of W-161 §3 take their exception (`security` 17, `value` 17, `restructure` 3). `fix_pending:` 533 → 496; **382 left to move** (B1–B3). Before the moves: auto 456, suggest 508, manual 1158.
- **2026-11-05 (Claude Code), step 4, security-js:** security-js: auto 0 → 0, suggest 17 → 32, manual 49 → 34 (pending 23 → 8). 15 moved to suggest (4 with labelled choices: jwt-verify, no-bad-tag-filter, no-header-injection, no-incomplete-url-check), 8 kept as needing computed edits. Left to move: 359; capability-blocked: 122.
- **2026-11-05 (Claude Code), step 4, security-python:** security-python: auto 0 → 0, suggest 18 → 29, manual 43 → 32 (pending 17 → 4). 11 moved to suggest (no-header-injection with labelled choices); 4 kept as needing computed edits, 1 an import (W-162); no-insecure-protocol takes restructure and no-path-traversal value; no-mako skips a call already passing default_filters. Left to move: 342; capability-blocked: 126.
- **2026-11-05 (Claude Code), step 4, security-ts:** security-ts: auto 0 → 0, suggest 3 → 6, manual 13 → 10 (pending 4 → 1). 3 moved to suggest (angular-native-element-html, no-query-builder-raw, typeorm-synchronize); no-typeorm-string-query kept as needing computed edits (W-162). Left to move: 338; capability-blocked: 127.
- **2026-11-05 (Claude Code), step 4, react:** react: auto 11 → 18, suggest 17 → 39, manual 123 → 94 (pending 56 → 27). 29 moved — 7 to auto with an auto_when subset (a11y-alt-text, a11y-aria-unsupported-elements, a11y-no-distracting-elements, a11y-no-redundant-roles, a11y-role-has-required-aria-props, a11y-scope, no-children-prop), the rest to suggest, choices where the review proposed them; no-deprecated, no-is-mounted and no-render-return-value to suggest, not auto (the subset isn't provable, or the edit drops a guard); 14 kept as needing computed edits or scope rename (W-162). Left to move: 295; capability-blocked: 141.
- **2026-11-05 (Claude Code), step 4, js:** js: auto 128 → 147, suggest 112 → 150, manual 342 → 285 (pending 150 → 90). moves from three slices — most suggest (choices where proposed: default-case, for-direction, guard-for-in, no-cond-assign, no-new-array, promise-always-return …), the auto subsets with auto_when (no-useless-call, no-void, no-eq-null, no-self-compare, no-sequences …); promise-no-nesting to suggest (scope not provable); 2 value exceptions (import-no-anonymous-default-export, jsdoc-require-property-name) and promise-no-callback-in-promise restructure; the rest kept as needing computed edits, scope rename or type info (W-162). Left to move: 221; capability-blocked: 155.
- **2026-11-05 (Claude Code), step 4, ts:** ts: auto 19 → 20, suggest 12 → 16, manual 34 → 29 (pending 19 → 14). 5 moved (no-confusing-non-null-assertion, no-empty-object-type and no-require-imports with labelled choices; no-non-null-assertion to suggest; no-unnecessary-parameter-property-assignment to auto with auto_when); adjacent-overload-signatures and no-this-alias kept as needing computed edits or scope rename (W-162). Left to move: 214; capability-blocked: 157.
- **2026-11-05 (Claude Code), step 4, nextjs:** nextjs: auto 0 → 0, suggest 0 → 6, manual 22 → 16 (pending 11 → 5). 6 moved to suggest (no-sync-scripts with labelled choices; google-font-preconnect, no-head-import-in-document, no-script-component-in-head, no-typos, no-unwanted-polyfillio); google-font-display kept as needing computed edits (W-162). Left to move: 207; capability-blocked: 158.
- **2026-11-05 (Claude Code), step 4, node:** node: auto 2 → 2, suggest 15 → 18, manual 18 → 15 (pending 9 → 6). 3 moved (no-process-exit with labelled choices; global-require hoisted with move-to after-imports; no-exports-assign to suggest). Left to move: 204; capability-blocked: 158.
- **2026-11-05 (Claude Code), step 4, numpy:** numpy: auto 0 → 2, suggest 1 → 1, manual 3 → 1 (pending 2 → 0). 2 moved to auto (numpy-deprecated-function; numpy2-deprecation with an auto_when on the plain aliases). Left to move: 202; capability-blocked: 158.
- **2026-11-05 (Claude Code), step 4, pandas:** pandas: auto 0 → 1, suggest 7 → 9, manual 5 → 2 (pending 3 → 0). 3 moved (pandas-use-of-dot-read-table to auto with auto_when — no sep, delimiter or second positional; nunique-constant-series-check and use-of-inplace-argument to suggest). Left to move: 199; capability-blocked: 158.
- **2026-11-05 (Claude Code), step 4, django:** django: auto 0 → 0, suggest 0 → 1, manual 7 → 6 (pending 2 → 1). django-non-leading-receiver-decorator to suggest: the two decorators swap when @receiver is second of two. Left to move: 198; capability-blocked: 158.
- **2026-11-05 (Claude Code), step 4, data:** data: auto 11 → 11, suggest 6 → 8, manual 30 → 28 (pending 22 → 20). md/fence-language moved with labelled choices (text, sh, json), reported at the opening fence; md/no-empty-links to suggest (unwrap to the text); the five schema rules kept as needing computed edits (W-162). Left to move: 191; capability-blocked: 163.
- **2026-11-05 (Claude Code), step 4, python:** python: auto 159 → 159, suggest 197 → 197, manual 150 → 150 (pending 66 → 66). py/no-tunable-literals and py/pytest-raises-ambiguous-pattern kept, now naming computed edits and add-import (W-162). Left to move: 189; capability-blocked: 165.
- **2026-11-05 (Claude Code), step 4, redux-toolkit:** redux-toolkit: auto 0 → 0, suggest 0 → 0, manual 12 → 12 (pending 7 → 7). no-legacy-create-store and prefer-hooks-api kept, now naming computed edits (W-162). Left to move: 187; capability-blocked: 167.
- **2026-11-05 (Claude Code), step 4, css-modules:** css-modules: auto 0 → 0, suggest 1 → 1, manual 4 → 4 (pending 3 → 3). no-undefined-class kept, now naming cross-file (W-162). Left to move: 186; capability-blocked: 168.
- **2026-11-05 (Claude Code), step 4 summary:** every `match:` rule waiting to move is done, one pack per change (security-js, security-python, security-ts, react, js, ts, nextjs, node, numpy, pandas, django, data, python, redux-toolkit, css-modules). Whole tree: auto 456 → 486, suggest 508 → 615 (34 with choices), manual 1158 → 1021; `fix_pending:` 533 → 354, of which **168 capability-blocked** (W-162; 54 found while writing the fixes, mostly computed edits) and **186 native rules left to move** — css 29, python 52, html 30, data 19, js 16, react 8, stencil 8, security-html 9, … each needs its fix in Rust (`NativeRule`). W-156 stays open for them. For the decider to glance at: `sec-py/no-mako` now skips a call already passing `default_filters` (it could never reach a fixpoint); `md/fence-language` reports at the opening fence; `md/no-empty-links` captures the link text. Three rules went to `suggest`, not `auto`, because their edit drops a guard or binding (`react/no-is-mounted`, `react/no-render-return-value`, `js/promise-no-nesting`); `on_edit` (W-166) can lift them. The js commit message says 60 moved; it is 57 (plus 3 exceptions).
- **2026-11-05 (Claude Code), step 4, node (native):** node: auto 2 → 3, suggest 18 → 18, manual 15 → 14 (pending 6 → 5). node/no-deprecated-api to auto for the APIs that are the same function under another name (util.isArray, util.isBuffer, os.tmpDir, os.getNetworkInterfaces, path._makeLong, crypto's randomBytes aliases), dropping an import nothing else reads; every other API manual (restructure). Left to move: 185; capability-blocked: 168.
- **2026-11-05 (Claude Code), step 4, tanstack-query (native):** tanstack-query: auto 3 → 3, suggest 5 → 9, manual 5 → 1 (pending 4 → 0). keepPreviousData: true becomes placeholderData: keepPreviousData with the import added; an updater that only sets keys of or pushes onto its value returns a copy; a rest object read only as rest.field, and a result in a dependency array read only through its fields, are destructured by name. The rest stay manual (restructure). Left to move: 181; capability-blocked: 168.
- **2026-11-05 (Claude Code), step 4, css-modules (native):** css-modules: auto 0 → 0, suggest 1 → 2, manual 4 → 3 (pending 3 → 2). css-modules/composes-valid to suggest: the classes the module defines within two edits of the missing name are labelled choices, nearest first; a name with none near, or a composes in the wrong place, is manual (value). Left to move: 180; capability-blocked: 168.
- **2026-11-05 (Claude Code), step 4, ts (native):** ts: auto 20 → 22, suggest 16 → 18, manual 29 → 25 (pending 14 → 10). no-unused-vars to auto for a name nothing reads or writes (an import with another binding left; a variable, its initialiser kept as a statement unless pure; a function, interface, type alias, catch binding), removing an import's last binding a suggestion; no-unsafe-declaration-merging to auto for an interface of property signatures, which become declare fields; no-unused-expressions with labelled choices (delete, call, assign); no-duplicate-enum-values with choices (alias, next value). ts/type-error stays with W-105. Left to move: 176; capability-blocked: 168.
- **2026-11-05 (Claude Code), step 4, scss (native):** scss: auto 18 → 20, suggest 10 → 13, manual 23 → 18 (pending 9 → 4). no-duplicate-dollar-variables and no-duplicate-mixins to auto (the earlier one goes when nothing between reads or includes it); at-rule-no-unknown and function-no-unknown with nearest-name choices; max-nesting-depth to suggest, joining a too-deep rule with a parent that holds nothing else. Left to move: 171; capability-blocked: 141.
- **2026-11-05 (Claude Code), step 4, redux-toolkit (native):** redux-toolkit: auto 0 → 0, suggest 0 → 5, manual 12 → 7 (pending 7 → 2). five to suggest: no-mutating-reducer (a push or key set followed by return state returns a copy), no-blind-payload-return (one assignment per initialState key), no-object-selector (choices: one useSelector per value, shallowEqual), no-side-effects-in-reducer (the effect moves into a prepare callback), object-shorthand-mapdispatch (the object of creators). Outside those shapes, manual. Left to move: 166; capability-blocked: 141.
