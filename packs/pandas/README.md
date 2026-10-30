# codelaw-rules-pandas

codelaw's pandas rules — one of codelaw's first-party rule packs
([SR-RULE-PACKS](../../records/0101_rule-packs.md)). It ships on PyPI only, as
`codelaw-rules-pandas` (pandas is a Python framework). Its rules use the namespace
`pandas`.

```toml
# codelaw.toml
extends = ["codelaw:py/recommended", "codelaw:pandas/recommended"]
preview = true   # every rule is `preview` until a corpus run promotes it
```

Installing the pack is the opt-in: `codelaw init` does not add it yet.

## Presets

<!-- docs-sync:presets -->

| preset | rules | ported from |
|---|---|---|
| `pandas/recommended` | 12 | pandas-vet, through Ruff's `PD` rules (Ruff 0.16.9): the ones Ruff calls stable, at `warn` |

<!-- /docs-sync:presets -->

Every rule is **reimplemented**, not vendored: the matchers, messages and docs
are written fresh, and each rule's `ported_from:` names the Ruff code.
