# @codelaw/rules-js · codelaw-rules-js

codelaw's core JavaScript rules, which also run on TypeScript, JSX and TSX — one of codelaw's first-party rule packs
([SR-RULE-PACKS](../../records/0101_rule-packs.md)). The
same files ship on npm as `@codelaw/rules-js` and on PyPI as `codelaw-rules-js`.

```toml
# codelaw.toml
extends = ["codelaw:js/recommended"]
preview = true   # every rule is `preview` until a corpus run promotes it
```

## Presets

<!-- docs-sync:presets -->

| preset | rules | ported from |
|---|---|---|
| `js/recommended` | 151 | ESLint `js.configs.recommended`, `@eslint/js` 10.0.1 — all 64 rules — plus `js/no-console` at `warn`, which ESLint leaves out, and `js/use-instead` at `warn`, inert until `replacements` is set (W-73); a team turns it to `error` in `[profiles.ci]`; eslint-plugin-promise 7.3.0's `recommended` — all 10 `js/promise-*` rules, at its severities; and the 16 JavaScript rules Biome 2.6.4 recommends that no other tool has, the two unused-name rules at `warn` (W-139) |
| `js/typescript` | 4 | the four core rules typescript-eslint 8.71.2's `eslint-recommended` switches on for TypeScript: `js/no-var`, `js/prefer-const`, `js/prefer-rest-params`, `js/prefer-spread`; use it with `codelaw:ts/recommended` |
| `js/conventions` | 3 | codelaw's starter catalogue: `js/exports-at-end`, `js/import-groups`, `js/no-default-export` — opt in |
| `js/project` | 6 | `graph/no-unused-exports`, the rule that reads the whole repo, from knip's `exports` and `types` — opt in |
| `js/size` | 5 | size limits at `warn`, with ESLint's defaults: `js/max-lines` (300 per file), `js/max-lines-per-function` (50) and `js/max-params` (3) — opt in |
| `js/markers` | 3 | TODO comments with a ticket (and a date when asked), no focused tests, skipped tests with a reason and an expiry; opt in (SR-CONFIG decision 11a) |
| `js/jsdoc` | 36 | eslint-plugin-jsdoc 65.4.2 recommended, reimplemented as js/jsdoc-* rules at the plugin's warn; opt in (W-139, ruled 2026-10-30 on W-144: R7) |
| `js/strict` | 306 | eslint-plugin-unicorn 79.0.0 recommended, reimplemented: the stricter, opinionated rules; opt in (W-139, ruled 2026-10-30 on W-138: R2) |
| `js/taste` | 15 | ESLint's style-only rules and two TSLint ones, which no defect stands behind; opt in (W-139, ruled 2026-10-30 on W-138: R1) |
| `js/sonarjs` | 158 | eslint-plugin-sonarjs 4.2.2 recommended, reimplemented from its documented behaviour (LGPL-3.0: no code, text or tests used); opt in (W-139, ruled 2026-10-30 on W-144: R7) |
| `js/regexp` | 58 | eslint-plugin-regexp 3.3.2's recommended rules, reimplemented on codelaw's own regular-expression parser (W-139); `js/recommended` extends it |

<!-- /docs-sync:presets -->

## The fix ladder

Each rule's fix class ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md)): `manual` rules by their named exception. `codelaw rules --fix-stats` prints the same table.

<!-- rule-docs:fix-ladder -->

| auto | suggest | manual | value | restructure | authored | security | waiting to move up |
|---|---|---|---|---|---|---|---|
| 264 | 281 | 362 | 136 | 141 | 42 | 41 | 2 |

<!-- /rule-docs:fix-ladder -->

Rules that typescript-eslint's `eslint-recommended` switches off for TypeScript
(`no-undef`, `no-redeclare`, `no-dupe-class-members` and the rest, which the
compiler checks) declare only `javascript` and `jsx` here. The four rules in
`js/typescript` declare all four languages, so they also run on JavaScript
files when that preset is on.

Every rule is **reimplemented**, not vendored: the matchers, messages and docs
are written fresh, and each rule's `ported_from:` names the source rule.

## How the ports differ

Every rule matches its source's documented behaviour on the source's own
examples. Where codelaw differs, the rule's `## Differences from the source`
section says so. The larger ones:

| rule | difference |
|---|---|
| `js/constructor-super`, `js/no-this-before-super` | follow the constructor's statements in straight-line order; a `super()` on any branch counts |
| `js/no-constant-condition` | literal conditions only; ESLint also folds `typeof x`, `!1` and similar |
| `js/no-extra-boolean-cast` | `!!x` in a condition; `Boolean(x)` in a condition is not checked yet |
| `js/no-invalid-regexp` | structural problems only (brackets, groups, quantifiers, flags) |
| `js/no-useless-assignment` | dead stores a straight-line reading proves; ESLint follows every code path |
| `js/no-useless-escape` | strings and templates; escapes inside regular expressions are not checked yet |
| `js/no-undef` | environments come from the `env` option (`shared-node-browser` by default), using the `globals` data tables |
| `js/prefer-spread` | only `f.apply(null, args)` on a plain function name |
| `js/no-var` | `suggest`, not `auto`: `let` is block-scoped, so the swap can change meaning |
| `js/promise-always-return`, `js/promise-no-multiple-resolved` | read the callback's statements, not each code path: a `return` anywhere counts, and two settling calls are reported only one after the other in one block |
| `js/promise-no-nesting` | every nested chain; the source allows one whose callback uses the outer callback's parameters |
| `js/promise-prefer-catch` | `suggest`, not `auto`: `.catch(b).then(a)` passes the handler's result on to `a` |
| `js/no-unused-function-parameters` | only the last parameter, since an earlier one is kept by those after it |
| `js/use-simple-number-keys`, `js/no-string-case-mismatch`, `js/no-misrefactored-shorthand-assign` | `manual`: Biome's fix computes a new literal or picks an operator, which a fix template cannot |

Nothing from `js.configs.recommended` or eslint-plugin-promise's `recommended` is left out, and of Biome's own recommended rules only `noConstantMathMinMaxClamp` is not built yet (it compares two numbers, which a matcher cannot); the plugin's seven other rules are `preview` and in no preset.

## Data

`data/globals/*.json` — the globals of each environment `js/no-undef`'s `env`
option names, from the `globals` package 17.13.0 (MIT). See `NOTICE`.
