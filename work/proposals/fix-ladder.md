---
type: Proposal
title: The fix ladder — every rule's fix, automatic first
description: "W-156's research: all 810 first-party rules inventoried by fix class, the 717 that aren't `auto` reviewed one by one, and a rung proposed for each — manual falls from 577 to 147; L4 rewritten to bind the order."
status: graduated
timestamp: 2026-10-30T21:30:00+05:30
---

# The fix ladder — every rule's fix, automatic first

**Ruled 2026-10-30 (Arpit):** *"Whenever a rule gets created, there has to be
a fix for it. The first priority is fixing it automatically. The second
priority is suggesting it so that AI agent can fix it. The last one is manual,
and only in exceptional cases we go with manual. Research all the rules and
propose how it can be fixed."* The law is [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md);
[W-156](../../archive/open/W-156-fix-ladder.md) builds this.

Every row is in [`fix-ladder/rules.csv`](fix-ladder/rules.csv): rule, pack,
current class, proposed rung, the safe subset, the edit, the exception, the
engine capability it needs, confidence, the upstream tool's fix, and the rule's
current `fix_reason`.

## 1 · In one line

Today 71% of the rules are `manual`. **18%** would be, if every proposal here
is built: 254 rules fix themselves (112 always, 142 for a stated safe subset),
409 hand an agent a diff (155 of them a short menu of diffs), and 147 stay
`manual` for one of four named reasons.

## 2 · Today and proposed

| | rules | `auto` | `auto` for a subset | `suggest` | `suggest` with choices | `manual` |
|---|---|---|---|---|---|---|
| **today** | 810 | 93 (11%) | — | 140 (17%) | — | **577 (71%)** |
| **proposed** | 810 | 112 | 142 | 254 | 155 | **147 (18%)** |

How the 717 non-`auto` rules move:

| from \ to | `auto` | `auto` subset | `suggest` | choices | `manual` |
|---|---|---|---|---|---|
| `manual` (577) | 16 | 93 | 181 | 140 | 147 |
| `suggest` (140) | 3 | 49 | 73 | 15 | — |

The 147 that stay `manual`, by exception: **restructure 57** (cross-file or
architectural: split a module, promisify callers, add an error boundary),
**value 56** (a name, value or text the code lacks: the expected exception, an
allowed origin, an env-var name, a ticket), **authored 31** (alt text, labels,
JSDoc, a test's expectation), **security 3** (any edit would create false
safety).

## 3 · By pack

| pack | rules | today auto · suggest · manual | proposed auto · subset · suggest · choices · manual |
|---|---|---|---|
| css | 73 | 13 · 20 · 40 | 13 · 20 · 18 · 20 · **2** |
| css-modules | 5 | 0 · 1 · 4 | 0 · 0 · 2 · 2 · **1** |
| data | 42 | 9 · 5 · 28 | 9 · 6 · 6 · 15 · **6** |
| deps | 4 | 0 · 2 · 2 | 0 · 0 · 2 · 0 · **2** |
| html | 74 | 10 · 11 · 53 | 12 · 10 · 12 · 20 · **20** |
| js | 162 | 21 · 20 · 121 | 29 · 40 · 34 · 33 · **27** |
| node | 10 | 0 · 1 · 9 | 0 · 2 · 0 · 8 · **0** |
| python | 109 | 17 · 19 · 73 | 20 · 23 · 30 · 20 · **15** |
| react | 73 | 2 · 3 · 68 | 4 · 16 · 24 · 15 · **14** |
| redux-toolkit | 12 | 0 · 0 · 12 | 0 · 0 · 6 · 1 · **5** |
| scss | 16 | 4 · 2 · 10 | 4 · 5 · 4 · 2 · **1** |
| security-html | 19 | 1 · 6 · 12 | 1 · 0 · 15 · 1 · **2** |
| security-js | 66 | 0 · 17 · 49 | 0 · 4 · 34 · 6 · **22** |
| security-python | 61 | 0 · 17 · 44 | 0 · 1 · 40 · 4 · **16** |
| security-ts | 16 | 0 · 3 · 13 | 0 · 0 · 10 · 1 · **5** |
| spell | 7 | 3 · 2 · 2 | 3 · 2 · 2 · 0 · **0** |
| stencil | 18 | 4 · 0 · 14 | 6 · 2 · 5 · 1 · **4** |
| tanstack-query | 13 | 3 · 5 · 5 | 3 · 2 · 7 · 0 · **1** |
| ts | 30 | 6 · 6 · 18 | 8 · 9 · 3 · 6 · **4** |

`graph/no-unused-exports` is counted once in `js` and once in `python`: each
pack ships a copy.

## 4 · What the engine needs, by rules unlocked

| capability | rules | what it is |
|---|---|---|
| **choices** | 157 | several labelled edit sets per finding (keep first or last key; `return` or `raise`; the nearest valid property, element, attribute or keyword). LSP and SARIF already model several fixes. |
| none | 424 | the edit is expressible today, in a `fix:` step or a native rule's edit |
| **cross-file edits** | 69 | renaming every use of a name; an edit to `package.json` beside the source edit; the project graph's references |
| **type information** | 17 | the safe subset of `== None`, `x++`, `parseInt` and the TS rules needs to know a value is a number, not an object with `__eq__` |
| add an import | 16 | a suggested edit that also adds `import secrets` or `textContent`'s helper |
| browser targets | 9 | vendor prefixes, colour notation, alias properties: safe only against a declared target (a new config input) |
| scope rename | 6 | rename a local, safe when no `locals()`/`eval` and no closure sees it |
| computed edit | 6 | text computed from the matched text: `\012` → `\x0a`, joining string continuations, canonical number keys |
| purity | 5 | "this expression has no side effect", to drop an initialiser |
| target Python | 3 | the project's Python version, for `strict=` defaults |

Choices first: it moves the most rules (140 from `manual` alone), and every
other capability is narrower.

## 5 · Upstream fixes that aren't safe

27 rules have an upstream autofix that this review keeps out of `auto`, because
the edit can change behaviour. Examples:

- oxlint `no-new-wrappers`, `valid-typeof`, `use-isnan`, `no-unsafe-negation`;
  Biome `no-prototype-builtins`, `no-case-declarations`; eslint-plugin-promise
  `prefer-catch`.
- react-hooks `exhaustive-deps`: adding a dependency changes when the effect
  runs.
- Ruff's "safe" `== None` → `is None`: numpy and pandas override `__eq__`.
- stylelint's duplicate-property, `:not()` notation, unspaced `calc` operator
  and vendor-prefix fixes: each can change rendering.

So the law says it: upstream's fix is evidence, never the reason (SR-LAW-4
decision 3).

## 6 · Patterns

1. **Spelling out a default is `auto`.** `except:` → `except BaseException:`,
   `stacklevel=1`, `strict=False`: the same behaviour, now visible.
2. **Code that doesn't compile has no behaviour to protect.** One sensible
   edit is `auto` (a late `__future__` import); a few are choices (`break`
   outside a loop: `return` or `raise`).
3. **Exact duplicates are `auto` for a subset;** different values are choices
   (keep first, keep last). JSON, YAML, TOML keys; CSS custom properties; Python
   dict keys; Sass variables with no read between.
4. **Closed lists become choices.** A misspelt CSS property, at-rule, element,
   attribute or keyword: the nearest valid names from the dictionary.
5. **Edits that keep behaviour but erase the evidence of a bug stay
   `suggest`:** deleting a dead duplicate case, adding `return undefined`, a
   `void` on a floating promise. They silence, they don't fix.
6. **Security fixes are `suggest`, not `auto`:** `innerHTML` → `textContent`,
   `yaml.load` → `yaml.safe_load`, dropping `verify=False`. Their whole point
   is to change behaviour. What stays `manual` is policy: origins, hosts,
   limits, stored-format migrations.
7. **Renames to a pattern aren't `value` cases:** the new name is computed from
   the old one (kebab-case, camelCase). They need cross-file edits, not a
   person.
8. **Edits on erased TypeScript are safe:** type-only renames, modifiers on
   Stencil decorators, `any` → `unknown` in a parameter no one reads.

## 7 · Open questions for the decider

1. **Save is not `codelaw fix`.** Some `auto` edits are behaviour-safe but
   destroy work in progress when applied on every save: removing a whole unused
   import, unreachable code, a focused test. Proposal: a rule may say
   `on_save: false`; `codelaw fix` and the agent hook still apply it, save does
   not.
2. **Two new config inputs:** browser targets (9 CSS rules) and the target
   Python version (3 rules). Without them those rules stay `suggest`.
3. **The port plan** ([rule-parity](rule-parity.md), W-139): 1,499 planned
   ports are `manual` only because upstream has no fix. Re-judge each against
   SR-LAW-4 decisions 3–5 as it is ported, not after.

## 8 · Method and limits

- **Inventory:** every `packs/*/rules/**/*.md` (not `.cases.md`), parsed for
  `fix`, `fix_reason`, the rule kind (native, match, graph, target) and
  `ported_from`. Joined to each upstream tool's fix ability from
  [`rule-parity/`](rule-parity/), which was read from each tool's own package.
- **Review:** the 717 non-`auto` rules were split by pack into five groups and
  reviewed in parallel against one rubric. The rubric set the ladder, required
  a stated safe subset for `auto` on part of the findings, and rejected any
  suggestion with a placeholder. Each rule's frontmatter, source, steps and
  examples were read. Confidence: 340 high, 333 medium, 44 low.
- **Corrections after review:** three proposed `auto` fixes change behaviour
  by design, so they were moved to `suggest`: `sec-js/express-hardening`,
  `sec-py/no-assert-for-auth` and `sec-html/angularjs-url-expression`.
- **Not yet proven.** A proposal is a claim. Each rule that moves is proven
  when it is built, by its own examples (SR-LAW-4 decision 11). The 44 rows
  marked low confidence are checked first.

## References

- [SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) · [W-156](../../archive/open/W-156-fix-ladder.md) · [rule-parity](rule-parity.md)
- [`fix-ladder/rules.csv`](fix-ladder/rules.csv) — every reviewed rule.
- `packs/*/rules/**/*.md` — the rules, as of `main` on 2026-10-30.
