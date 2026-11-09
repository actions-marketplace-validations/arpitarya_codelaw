---
id: airflow/airflow3-incompatible-function-signature
title: "A call or a definition whose signature changed in Airflow 3"
languages: [python]
fix: manual
fix_reason: "the keyword names to pass, and the shape of a `get_link`, are the author's"
fix_exception: value
stability: preview
ported_from: [ruff/AIR303]
since: 0.5.0
---

Three shapes whose signature changed in Airflow 3.0 are reported:

- `HookLineageCollector().create_asset(…)`, or `collector.create_asset(…)` on a variable assigned that call, with a positional argument: pass them as keywords (`create_asset(uri=…)`);
- `Dataset(name, {…})`, `datasets.Dataset(…)` or `Asset(name, {…})` with a dict as the second positional argument: pass it as `extra=`;
- `get_link` in a subclass of `BaseOperatorLink` that is not `def get_link(self, operator, *, ti_key)` or `def get_link(self, operator, ti_key)`.

**Why:** Each of these runs on Airflow 2 and breaks, or is read differently, on 3.0.

```codelaw
native: airflow/airflow3-incompatible-function-signature
```

## How to fix

1. Call `create_asset(uri=…)` with keywords.
2. Pass the dict as `Asset(name=…, uri=…, extra={…})`.
3. Write `get_link` as `def get_link(self, operator, *, ti_key)`.


## Examples

### Python

#### Don't

```python
from airflow.sdk import Asset

asset = Asset("orders", {"owner": "data"})
```

#### Do

```python
from airflow.sdk import Asset

asset = Asset("orders", extra={"owner": "data"})
```

## Differences from the source

A `BaseOperatorLink` base is any name under `airflow.models` or `airflow.sdk` ending in it, or a class of the same file that derives from one.
