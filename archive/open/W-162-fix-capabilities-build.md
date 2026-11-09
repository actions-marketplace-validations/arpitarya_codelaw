---
type: Handoff
name: W-162
description: "Arpit, 2026-11-05 (voice): build the fix capabilities W-161 designs and move the rules waiting on them up the fix ladder."
item: W-162
filed: 2026-11-05
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-08
updated_by: "Claude Code (Opus 5.5), W-162 change 9"
ball: agent
---

# W-162 — Build: the six fix capabilities, and the rules waiting on them

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 0 | Re-judge the 37 waiting rules whose reason is an author's decision: `manual` with their exception ([proposal](../../work/proposals/fix-capabilities.md) §3) | the waiting count falls by a quarter | ✅ 2026-11-05, by W-156 step 4 |
| 1 | W-161's designs and build order | what to build, in what order | ✅ 2026-11-05 — [`fix-capabilities.md`](../../work/proposals/fix-capabilities.md) §2, §4 |
| 1a | The decider rules whether a fix may edit several files (L4 decisions 10, 12; proposal §5) | the 22 rename rules' path | ✅ 2026-11-05 — allowed; SR-LAW-4 decisions 10 and 12 amended |
| 2 | Build each capability in the proposal's order (§4): computed edits, scope rename, add-import, purity, types, cross-file read, cross-file write, action SHAs | the engine can make the fix | ✅ 2026-11-08 — every capability but type information, which is W-105's (SR-TYPES): its 6 rules' markers moved there |
| 2a.1 | Computed edits, tiers 1–2: ast-grep's `transform:` beside `match:`; filters `map`, `number`, `escape`, `unescape-octal`, `count`, `upper`, `lower`, `upper-first`; a refusing filter offers no fix ([SR-RULE-FORMAT](../../records/0112_rule-format.md) 3b) | 19 `js/*` rules move: `eqeqeq`, `operator-assignment`, `logical-assignment-operators`, `no-double-comparison`, `consistent-existence-index-check`, `no-implicit-coercion`, `no-zero-fractions`, `numeric-separators-style`, `use-simple-number-keys`, `prefer-numeric-literals`, `no-octal-escape`, `escape-case`, `no-div-regex`, `prefer-string-starts-ends-with`, `no-console-spaces`, `no-extra-label`, `dom-node-dataset`, `prefer-query-selector`, `no-unneeded-ternary` | ✅ 2026-11-05 — `codelaw test packs/js` 582 rules, 0 failed; `rules --fix-stats --check` passes; js auto 147 → 157, suggest 150 → 159, manual 285 → 266 |
| 2a.2 | Computed edits, tier 3: `edit-text` (SR-RULE-FORMAT 3c), regex edits inside one node, chained per target, emitted as one minimal edit | 8 `js/jsdoc-*` rules move: `no-multi-asterisks`, `multiline-blocks` (auto); `check-types`, `no-types`, `check-syntax`, `no-defaults`, `convert-to-jsdoc-comments`, `escape-inline-tags` (suggest) | ✅ 2026-11-07 — `codelaw test packs/js` 582 rules, 0 failed; `rules --fix-stats --check` passes; js auto 157 → 159, suggest 159 → 165, manual 266 → 258 |
| 2a.3 | Computed edits, tier 3: `unwrap-block` and `move` (SR-RULE-FORMAT 3c) | 3 `js/*` rules move: `no-else-return` (suggest), `no-inline-comments` (suggest), `no-useless-spread` (auto, a template; an empty spread offers no fix) | ✅ 2026-11-07 — `codelaw test packs/js` 582 rules, 0 failed; `rules --fix-stats --check` passes; js auto 159 → 160, suggest 165 → 167, manual 258 → 255 |
| 2a.3b | Re-judge (L4 decision 5): rules filed under computed-edit whose `fix_reason:` is an author's decision with no closed set of edits — 12 `security` (the `sec-*` injection, traversal, cleartext and secret rules), 4 `restructure` (`promise-prefer-await-to-then`, `no-find-dom-node`, `no-legacy-create-store`, `prefer-hooks-api`), 16 `value` (the five data schema rules, `jsdoc-require-param-name`, `-returns-check`, `-yields-check`, `no-string-case-mismatch`, `no-with`, `py/no-tunable-literals`, `a11y-lang`, `a11y-no-static-element-interactions`, `a11y-prefer-tag-over-role`, `jsx-key`, `no-suspicious-semicolon-in-jsx`), 1 `authored` (`a11y-img-redundant-alt`). Kept pending where a closed set of choices exists (`valid-typeof`, the aria rules, `google-font-display`, …) | the waiting count falls to 40 | ✅ 2026-11-07 — `codelaw test` on the seven packs 0 failed; `rules --fix-stats --check` passes |
| 2a.4 | Computed edits, the rest: `merge-into`, `move: { after }` (whole lines, blank lines kept apart), `insert-* line: true`, filters `nearest`, `arrow-body`, `style-object`, a choice with nothing to edit left out, the first native fix (`js/prefer-template`, `rewrites.rs`) | 34 rules move — auto: `no-duplicate-imports`, `default-case-last`, `no-multi-str`, `no-useless-concat`, `react/no-unescaped-entities`, `react/display-name`; suggest: the other 28 (`prefer-single-call`, `prefer-template`, `prefer-arrow-callback`, `no-for-each`, `valid-typeof`, the three a11y name rules, `google-font-display`, the Python and Django four, …). 6 re-judged `manual`: `jsdoc-empty-tags` (authored), `jsdoc-prefer-import-tag`, `py/max-depth`, `ts/parameter-properties` (restructure), `no-anonymous-default-export`, `import-no-absolute-path` (value) | ✅ 2026-11-07 — `codelaw test` on js, react, nextjs, node, python, django, ts 0 failed; `rules --fix-stats --check` passes; `cargo test -p codelaw-fix` 34 passed; whole tree auto 566 → 572, suggest 732 → 760, manual 872 → 838; no rule names computed-edit |
| 2b–2d | Scope rename (`rename-symbol`, `$FRESH`, capture-named `edit-text` patterns), purity (the native `pure` reused for `js/no-unused-vars`; unused private members removed), add-import (`ensure-import`) | moved: `no-shadow-restricted-names`, `promise-param-names`, `no-unused-vars`, `no-unused-private-class-members` (auto); `no-caller`, `prefer-for-of`, `next/no-assign-module-variable`, the seven `py/*` import rules, `next/no-head-element`, `no-html-link-for-pages` (suggest). Re-judged `manual`: `no-ex-assign`, `no-const-assign` (value); `prefer-rest-params`, `react/no-string-refs`, `ts/no-this-alias`, `next/next-script-for-ga` (restructure); `sec-py/no-reflected-xss` (security) | ✅ 2026-11-07 — `codelaw test` js, ts, react, nextjs, python, security-python 0 failed; `rules --fix-stats --check` passes; `cargo test -p codelaw-fix` 39 passed; auto 593 → 597, suggest 807 → 819, manual 1043 → 1027 |
| 2e | The ten `py/*` rules W-139 filed under W-162: `unwrap-block` on a Python `block`, `ensure-import` gated by a capture | `superfluous-else-{return,break,continue,raise}`, `useless-try-except`, `{raw-string,f-string,dot-format}-in-exception`, `pytest-composite-assertion`, `iter-method-return-iterable` to suggest | ✅ 2026-11-07 — `codelaw test packs/python` 706 rules, 0 failed; `rules --fix-stats --check` passes; suggest 819 → 829, manual 1027 → 1017 |
| 2f | Action SHAs: `deps refresh` records `actions` (`owner/repo@ref` → commit) in the snapshot; `yaml/gha-pin-sha` native (`rewrites.rs`) | `yaml/gha-pin-sha` to suggest | ✅ 2026-11-07 — `codelaw test packs/data` 0 failed; `cargo test -p codelaw-native` 275 passed; `rules --fix-stats --check` passes |
| 2g | Cross-file read and write ([SR-MULTI-FILE-FIXES](../../records/0119_multi-file-fixes.md)): `FileEdit` on a finding and a choice; the multi-file pass of `codelaw fix` (one fix at a time; every file parses, the finding is gone, no new rule fires; `codelaw_graph::dangling` before and after; `write_all_or_nothing`); `multi_file` per project and per rule; JSON/MCP `fix.files`, SARIF per-file `artifactChanges`, the editor's `WorkspaceEdit`, the agent and hook lines; never on save or from the hook; `strip_bom`; the renames of `codelaw-native` `multi_file.rs` | the 36 cross-file rules can move | ✅ 2026-11-08 — `cargo test -p codelaw-fix -p codelaw-engine -p codelaw-lsp -p codelaw-mcp` and `-p codelaw-cli --test multi_file` (7) pass; clippy clean for the new code |
| 3 | Move each waiting rule up, proven by its own examples (`codelaw test`) | agents get a diff, not steps | ✅ 2026-11-08 — the 36 cross-file rules: 29 up (8 `auto`: `scss/dollar-variable-pattern`, `at-mixin-pattern`, `html/no-utf8-bom`, `react/hook-prefix`, `no-unnecessary-use-prefix`, `py/no-star-import`, `nested-star-import`, `star-import-usage`; 21 `suggest`: the 7 `css/*` name patterns, `scss/percent-placeholder-pattern`, `css-modules/class-naming`, `no-undefined-class`, `js/new-cap`, `no-default-export`, `no-default-import`, `no-undef`, `react/jsx-pascal-case`, `jsx-no-undef`, the 4 `node/no-{missing,extraneous}-*`, `py/module-import-not-at-top`), 7 re-judged `manual` (`html/valid-id`, `stencil/tag-prefix`, `ban-prefix`, `data/codeowners-coverage`: `value`; `scss/at-extend-no-missing-placeholder`, `spell/known-typo-key`, `ts/triple-slash-reference`: `restructure`); `codelaw test` on css, scss, css-modules, html, js, react, node, python, spell, stencil, ts, data 0 failed; `rules --fix-stats --packs packs --check` exit 0; whole tree `auto` 657, `suggest` 947, `manual` 1012, 7 waiting (all W-105) |
| 4 | Close when no `fix_pending:` names W-162 | L4 holds without a stand-in | ✅ 2026-11-08 — none does; the 6 type-information rules (`js/no-plusplus`, `js/no-void-type-return`, `ts/ban-ts-comment`, `ts/no-implicit-any-let`, `ts/explicit-module-boundary-types`, `ts/explicit-function-return-type`) now name W-105 |

**Arpit, 2026-11-05 (voice):** *"So we'll do research on each one of these items So create a research work item so that we can figure out how these can be implemented."* · *"We can close it but once the research item is done it needs to be implemented. … go ahead and create one more build item blocked on research work item."*; then (text) *"file it."*

**Arpit, 2026-11-05 (voice), refining it:** *"Create an SR for this."* · *"and make it configurable"* · *"it does and that's all"*; then (text) *"file it."* The multi-file rules are their own record, [SR-MULTI-FILE-FIXES](../../records/0119_multi-file-fixes.md): the five safeguards are never configurable; `[fix] multi_file` and per rule take `auto`, `suggest` or `off` (SR-CONFIG decision 11g).

**Arpit, 2026-11-05 (Claude Code, multiple choice):** *"Allow multi-file (Recommended)"* — a fix may edit several files: verified per file plus a graph check for new unresolved references, written all-or-nothing, `auto` only when every reference resolves statically, never on save or from the per-edit hook. Written into [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decisions 10 and 12 (ruled, not built).

**Model:** Opus (Claude Code): engine changes under a law (L4) and the verifier.

## Goal

Every rule whose `fix_pending:` names W-162 reaches its proposed rung, or is re-judged `manual` with a `fix_exception` ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decision 5).

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · capabilities | each of W-161's designs built and tested, within L8 | ✅ 2026-11-08 (type information is W-105's; a multi-file fix reads the project only when a finding offers one, and the pass runs only when one is wanted) |
| 2 · rules | no `fix_pending:` names W-162; `codelaw rules --fix-stats --check` passes | ✅ 2026-11-08 |

## Blockers

None. The multi-file call was ruled 2026-11-05 (allowed, above); W-161's designs are done.

## Record affected

SR-LAW-4 decision 5 (the stand-in ends); SR-MULTI-FILE-FIXES and SR-CONFIG 11g (built: no → yes); SR-RULE-FORMAT 3a–3c; SR-TYPES 4; SR-PROJECT-GRAPH; SR-CLI-AND-MCP; SR-CACHE.

## Hazards

- W-156's moves touch the same packs while it is open: claim per pack.

## Reference

[W-161](../../archive/open/W-161-fix-capabilities-research.md) · [proposal](../../work/proposals/fix-capabilities.md) · [W-156](../../archive/open/W-156-fix-ladder.md) B4
