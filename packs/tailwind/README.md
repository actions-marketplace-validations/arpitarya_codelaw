# @codelaw/rules-tailwind

codelaw's Tailwind CSS rules — one of codelaw's first-party rule packs
([SR-RULE-PACKS](../../records/0101_rule-packs.md)). It ships on npm as
`@codelaw/rules-tailwind` and on PyPI as `codelaw-rules-tailwind`, the same
files on both. Its rules use the namespace `tailwind`.

```toml
# codelaw.toml
extends = ["codelaw:js/recommended", "codelaw:tailwind/recommended"]
preview = true   # every rule is `preview` until a corpus run promotes it
```

`codelaw init` adds it when `package.json` depends on `tailwindcss` (or
`@tailwindcss/vite`, `/postcss`, `/cli`), or the repo holds a
`tailwind.config.*` or a stylesheet that imports "tailwindcss".

## Its provider

Three rules — `no-unknown-classes`, `no-conflicting-classes` and
`enforces-canonical-classname` — and the formatter's class sorting read a class
table. The pack holds no copy of Tailwind's classes or order: the table is built
from **the project's own `tailwindcss`** by `@codelaw/provider-tailwind`, an npm
package this pack depends on (pinned to the same version) and names in its
manifest (`providers`). codelaw runs it with Node and keeps the table in its
cache until Tailwind, its stylesheet or the provider changes.

Without the provider, or without `node_modules/tailwindcss` (a PyPI install with
Tailwind's standalone CLI, a CI image with no Node), those rules and the sorting
are skipped with one note; `[providers] on_missing = "error"` stops the run
instead. The other five rules read only the classes as written and always run.

## Presets

<!-- docs-sync:presets -->

| preset | rules | ported from |
|---|---|---|
| `tailwind/recommended` | 5 | eslint-plugin-better-tailwindcss and eslint-plugin-tailwindcss — their bug-catching rules; `no-conflicting-classes` at `error`, the rest at `warn` |

<!-- /docs-sync:presets -->

## The fix ladder

Each rule's fix class ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md)): `manual` rules by their named exception. `codelaw rules --fix-stats` prints the same table.

<!-- rule-docs:fix-ladder -->

| auto | suggest | manual | value | restructure | authored | security | waiting to move up |
|---|---|---|---|---|---|---|---|
| 3 | 4 | 1 | 1 | 0 | 0 | 0 | 0 |

<!-- /rule-docs:fix-ladder -->

`tailwind/no-arbitrary-value`, `tailwind/no-restricted-classes` and
`tailwind/enforces-canonical-classname` are in no preset: switch them on in
`[rules]`. The last makes the provider ask Tailwind for canonical spellings,
which Tailwind takes seconds to index the first time in each run.

Every rule is **reimplemented**, not vendored: the matchers, messages and docs
are written fresh, and each rule's `ported_from:` names the source rule. Class
order is the formatter's ([SR-FORMATTER](../../records/0105_formatter.md)
decision 9), so the upstream ordering rules have no port here.
