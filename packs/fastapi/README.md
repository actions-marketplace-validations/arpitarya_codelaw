# codelaw-rules-fastapi

codelaw's FastAPI rules — one of codelaw's first-party rule packs
([SR-RULE-PACKS](../../records/0101_rule-packs.md)). It ships on PyPI only, as
`codelaw-rules-fastapi` (FastAPI is a Python framework). Its rules use the namespace
`fastapi`.

```toml
# codelaw.toml
extends = ["codelaw:py/recommended", "codelaw:fastapi/recommended"]
preview = true   # every rule is `preview` until a corpus run promotes it
```

Installing the pack is the opt-in: `codelaw init` does not add it yet.

## Presets

<!-- docs-sync:presets -->

| preset | rules | ported from |
|---|---|---|
| `fastapi/recommended` | 2 | Ruff's `FAST` rules (Ruff 0.16.9): the ones Ruff calls stable, at `warn` |

<!-- /docs-sync:presets -->

Every rule is **reimplemented**, not vendored: the matchers, messages and docs
are written fresh, and each rule's `ported_from:` names the Ruff code.
