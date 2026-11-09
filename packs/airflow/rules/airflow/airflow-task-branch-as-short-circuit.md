---
id: airflow/airflow-task-branch-as-short-circuit
title: "A branch that is a short-circuit"
languages: [python]
fix: manual
fix_reason: "the condition has to be lifted out of the branching `return`s by the author"
fix_exception: restructure
stability: preview
ported_from: [ruff/AIR004]
since: 0.5.0
---

A `@task.branch` function, or the callable of a `BranchPythonOperator`, that has at least two `return` statements of which exactly one returns a non-empty list is a short-circuit: it returns a list of task ids, or nothing. `@task.short_circuit` / `ShortCircuitOperator` says so directly.

**Why:** A branch decides *which* downstream tasks run; a function that either continues or stops everything downstream is a short-circuit, and the short-circuit form returns the condition itself, which is simpler and what the Airflow UI labels it as.

```codelaw
native: airflow/airflow-task-branch-as-short-circuit
```

## How to fix

1. Replace `@task.branch` with `@task.short_circuit` (or `BranchPythonOperator` with `ShortCircuitOperator`).
2. Return the condition instead of the list of task ids.


## Examples

### Python

#### Don't

```python
from airflow.decorators import task


@task.branch
def check():
    if ready():
        return ["load"]
    return []
```

#### Do

```python
from airflow.decorators import task


@task.short_circuit
def check():
    return ready()
```

## Differences from the source

`@task` is `airflow.decorators.task` or `airflow.sdk.task`; `BranchPythonOperator` is one of the three `airflow.operators.python`, `…python_operator` and `airflow.providers.standard.operators.python` names. The operator form follows `python_callable=<name>` to a function defined once in the same file.
