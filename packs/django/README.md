# codelaw-rules-django

codelaw's Django rules — one of codelaw's first-party rule packs
([SR-RULE-PACKS](../../records/0101_rule-packs.md)). It ships on PyPI only, as
`codelaw-rules-django` (Django is a Python framework). Its rules use the namespace
`django`.

```toml
# codelaw.toml
extends = ["codelaw:py/recommended", "codelaw:django/recommended"]
preview = true   # every rule is `preview` until a corpus run promotes it
```

Installing the pack is the opt-in: `codelaw init` does not add it yet.

## Presets

<!-- docs-sync:presets -->

| preset | rules | ported from |
|---|---|---|
| `django/recommended` | 7 | flake8-django, through Ruff's `DJ` rules (Ruff 0.16.9): the ones Ruff calls stable, at `warn` |

<!-- /docs-sync:presets -->

## The fix ladder

Each rule's fix class ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md)): `manual` rules by their named exception, and the most this pack may ship (`packs/fix-ladder.toml`). `codelaw rules --fix-stats` prints the same table.

<!-- rule-docs:fix-ladder -->

| auto | suggest | manual | value | restructure | authored | security | waiting to move up | ceiling |
|---|---|---|---|---|---|---|---|---|
| 0 | 0 | 7 | 2 | 1 | 1 | 0 | 3 | 7 |

<!-- /rule-docs:fix-ladder -->

Every rule is **reimplemented**, not vendored: the matchers, messages and docs
are written fresh, and each rule's `ported_from:` names the Ruff code.
