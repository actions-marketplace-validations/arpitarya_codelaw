# codelaw-rules-airflow

codelaw's Apache Airflow rules — one of codelaw's first-party rule packs
([SR-RULE-PACKS](../../records/0101_rule-packs.md)). It ships on PyPI only, as
`codelaw-rules-airflow` (Apache Airflow is a Python framework). Its rules use the namespace
`airflow`.

```toml
# codelaw.toml
extends = ["codelaw:py/recommended", "codelaw:airflow/recommended"]
preview = true   # every rule is `preview` until a corpus run promotes it
```

Installing the pack is the opt-in: `codelaw init` does not add it yet.

## Presets

<!-- docs-sync:presets -->

| preset | rules | ported from |
|---|---|---|
| `airflow/recommended` | 1 | Ruff's `AIR` rules (Ruff 0.16.9): the ones Ruff calls stable, at `warn` |

<!-- /docs-sync:presets -->

## The fix ladder

Each rule's fix class ([SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md)): `manual` rules by their named exception, and the most this pack may ship (`packs/fix-ladder.toml`). `codelaw rules --fix-stats` prints the same table.

<!-- rule-docs:fix-ladder -->

| auto | suggest | manual | value | restructure | authored | security | waiting to move up | ceiling |
|---|---|---|---|---|---|---|---|---|
| 0 | 1 | 4 | 3 | 1 | 0 | 0 | 0 | 4 |

<!-- /rule-docs:fix-ladder -->

**Preview upstream, in no preset:** `airflow/airflow-variable-get-outside-task`, `airflow/airflow-xcom-pull-in-template-string`, `airflow/airflow3-dag-dynamic-value` — Ruff still calls them preview; turn them on under `[rules]`.

Every rule is **reimplemented**, not vendored: the matchers, messages and docs
are written fresh, and each rule's `ported_from:` names the Ruff code.
