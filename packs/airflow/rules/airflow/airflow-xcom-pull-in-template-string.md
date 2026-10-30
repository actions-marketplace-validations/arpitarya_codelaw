---
id: airflow/airflow-xcom-pull-in-template-string
title: "task.output, not xcom_pull in a template"
languages: [python]
fix: manual
fix_reason: "the operator's argument becomes a reference to the upstream task"
stability: preview
ported_from: [ruff/AIR201]
since: 0.5.0
---

An operator argument does not pull an XCom through a `{{ ti.xcom_pull(...) }}` template string.

**Why:** `upstream.output` passes the value and records the dependency between the tasks; the template does neither visibly.

```codelaw
match:
  kind: string
  regex: '\{\{\s*(ti|task_instance)\.xcom_pull\('
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+airflow\b' } }
message: "Pass the upstream task's `.output` instead of an `xcom_pull` template"
```

## How to fix

1. Pass `upstream_task.output` (or `upstream_task.output['key']`) as the argument.

## Examples

### Python

#### Don't

```python
from airflow.operators.bash import BashOperator
BashOperator(task_id='b', bash_command="echo {{ ti.xcom_pull(task_ids='a') }}")
```

#### Do

```python
from airflow.operators.bash import BashOperator
BashOperator(task_id='b', bash_command=a.output)
```
