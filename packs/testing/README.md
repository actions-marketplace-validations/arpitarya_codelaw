# @codelaw/rules-testing

codelaw's rules for test code — Jest, Vitest, Testing Library and Playwright —
one of codelaw's first-party rule packs
([SR-RULE-PACKS](../../records/0101_rule-packs.md)). It ships on npm only, as
`@codelaw/rules-testing`: these are Node test frameworks. Its rules use the
namespace `testing`.

```toml
# codelaw.toml — switch on the presets for the frameworks the project uses
extends = ["codelaw:js/recommended", "codelaw:testing/vitest", "codelaw:testing/testing-library"]
preview = true   # every rule is `preview` until a corpus run promotes it
```

`codelaw init` adds the pack, and the preset for each framework, when
`package.json` depends on `jest`, `vitest`, any `@testing-library/*` or
`@playwright/test`.

## Presets

<!-- docs-sync:presets -->

| preset | rules | ported from |
|---|---|---|
| `testing/jest` | 17 | eslint-plugin-jest 29.17.0 `recommended`, at its severities |
| `testing/vitest` | 15 | @vitest/eslint-plugin 1.6.28 `recommended` |
| `testing/testing-library` | 22 | eslint-plugin-testing-library 7.16.2's `react` config |
| `testing/playwright` | 33 | eslint-plugin-playwright 2.12.1 `recommended` |

<!-- /docs-sync:presets -->

## The fix ladder

Each rule's fix class ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md)): `manual` rules by their named exception. `codelaw rules --fix-stats` prints the same table.

<!-- rule-docs:fix-ladder -->

| auto | suggest | manual | value | restructure | authored | security | waiting to move up |
|---|---|---|---|---|---|---|---|
| 53 | 34 | 82 | 20 | 54 | 8 | 0 | 0 |

<!-- /rule-docs:fix-ladder -->

Every rule is **reimplemented**, not vendored: the matchers, messages and docs
are written fresh, and each rule's `ported_from:` names the source rule.

## How the ports differ

| rule | difference |
|---|---|
| `max-expects`, `max-nested-describe` (and Playwright's) | the source's default limit (5), fixed; `max-expects` counts the statements directly in the test body |
| `consistent-test-it` | the source's defaults (`test` at the top level, `it` in `describe`), fixed |
| `no-large-snapshots` | inline snapshots only; `.snap` files are not read |
| `valid-expect` | accepts a message string as the second argument (Vitest, Playwright) |
| `expect-expect` | also counts `expectTypeOf`, `assertType` and `assert` |
| `prefer-lowercase-title` | leaves a title that starts with an acronym alone |
| `no-restricted-matchers` | names a matcher, not a modifier chain |
| `testing-library-consistent-data-testid` | one fixed pattern (letters, digits, `-`, `_`); runs on every file |
| `prefer-strict-equal`, `prefer-to-be-truthy`/`-falsy`, `prefer-strict-boolean-matchers`, `prefer-expect-type-of`, `playwright-prefer-web-first-assertions`, `-prefer-to-have-count`, `-require-soft-assertions` | `suggest`, not `auto` as in the source: each edit can change what the assertion accepts or when it runs (L4 decision 3) |

Each rule runs on test files only — `*.test.*`, `*.spec.*`, and files under
`__tests__/`, `test/`, `tests/` and `e2e/` — as the source plugins are
configured for test globs.

`js/no-focused-tests` and `js/skipped-test` (in `js`) already cover the
frameworks' `no-focused-tests` and `no-disabled-tests`.

There is no pinned corpus repository for test code yet, so the pack has no
`corpus.toml` counts; `scripts/corpus.py` leaves it out as it does `nextjs`.
