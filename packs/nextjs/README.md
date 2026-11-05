# @codelaw/rules-nextjs

codelaw's Next.js rules — one of codelaw's first-party rule packs
([SR-RULE-PACKS](../../records/0101_rule-packs.md)). It ships on npm only, as
`@codelaw/rules-nextjs`: Next.js is a Node framework. Its rules use the
namespace `next`.

```toml
# codelaw.toml
extends = ["codelaw:js/recommended", "codelaw:react/recommended", "codelaw:next/recommended"]
preview = true   # every rule is `preview` until a corpus run promotes it
```

`codelaw init` adds it when `package.json` depends on `next`.

## Presets

<!-- docs-sync:presets -->

| preset | rules | ported from |
|---|---|---|
| `next/recommended` | 22 | @next/eslint-plugin-next 16.4.3 `recommended` — all 22 rules; its `error` rules at `error`, the rest at `warn` |

<!-- /docs-sync:presets -->

## The fix ladder

Each rule's fix class ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md)): `manual` rules by their named exception. `codelaw rules --fix-stats` prints the same table.

<!-- rule-docs:fix-ladder -->

| auto | suggest | manual | value | restructure | authored | security | waiting to move up |
|---|---|---|---|---|---|---|---|
| 0 | 0 | 22 | 2 | 8 | 0 | 0 | 12 |

<!-- /rule-docs:fix-ladder -->

Every rule is **reimplemented**, not vendored: the matchers, messages and docs
are written fresh, and each rule's `ported_from:` names the source rule.

## How the ports differ

| rule | difference |
|---|---|
| `next/no-html-link-for-pages` | every root-relative `href` counts as a page; the source reads the `pages` folder |
| `next/no-typos` | names that differ only in case or a missing final `s`; the source catches other one-letter typos |
| `next/no-unwanted-polyfillio` | every polyfill.io script, since the domain was taken over in 2024; the source reports only features Next.js already polyfills |
| `next/inline-script-id`, `next/no-script-component-in-head` | `<Script>` is matched by name in a file that imports `next/script` |

Rules about `pages/_document` or a page carry `files:` and `exclude:` globs for
those paths, as the source checks the file's path.

There is no pinned corpus repository for Next.js yet, so the pack has no
`corpus.toml` counts; `scripts/corpus.py` leaves it out as it does
`redux-toolkit`.
