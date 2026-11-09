---
id: airflow/airflow-task-implicit-multiple-outputs
title: "`multiple_outputs` is not left to inference"
languages: [python]
fix: suggest
fix_reason: "the keyword is added as Airflow infers it today; a function returning a dict that is not meant to split into XComs takes `multiple_outputs=False`, and that is the author's call"
stability: preview
ported_from: [ruff/AIR202]
since: 0.5.0
---

A function decorated with `@task` or one of its variants (`@task.python`, `.virtualenv`, `.external_python`, `.branch`, `.branch_virtualenv`, `.branch_external_python`, `.short_circuit`, `.docker`, `.kubernetes`, `.pyspark`) states `multiple_outputs` when it returns a dictionary: its return annotation is a mapping type (`dict`, `Mapping`, a `TypedDict` of the file …) or one of its `return` statements returns a dict display, a dict comprehension or `dict(…)`. `@task.sensor` is left out because it fixes the value itself.

**Why:** Airflow decides whether to split the returned dictionary into one XCom per key from the return annotation, and that inference has changed between versions. Written down, the behaviour is the same on every version.

```codelaw
native: airflow/airflow-task-implicit-multiple-outputs
```


## Examples

### Python

#### Don't

```python
from airflow.decorators import task


@task
def extract() -> dict[str, int]:
    return {"a": 1}
```

#### Do

```python
from airflow.decorators import task


@task(multiple_outputs=True)
def extract() -> dict[str, int]:
    return {"a": 1}
```

#### Fixed

```python
from airflow.decorators import task


@task(multiple_outputs=True)
def extract() -> dict[str, int]:
    return {"a": 1}
```

## Differences from the source

The fix adds `multiple_outputs=True` when the annotation is a mapping and `multiple_outputs=False` when only the body returns a dict, as Ruff does; Ruff's fix is also unsafe. `@task` is `airflow.decorators.task` or `airflow.sdk.task`. A `TypedDict` class is found through the classes of the same file.
