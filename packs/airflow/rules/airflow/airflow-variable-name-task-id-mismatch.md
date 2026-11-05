---
id: airflow/airflow-variable-name-task-id-mismatch
title: "A task's variable is named like its task_id"
languages: [python]
fix: suggest
fix_reason: "the `task_id` changes, which renames the task in the UI and its history"
stability: preview
ported_from: [ruff/AIR001]
since: 0.5.0
---

`load = PythonOperator(task_id="extract", ...)` uses the same name for the variable and the `task_id`.

**Why:** Two names for one task make the DAG file hard to match with the UI.

```codelaw
match:
  kind: assignment
  all:
  - has: {field: left, kind: identifier, pattern: $N}
  - has:
      field: right
      kind: call
      all:
      - has: {field: function, regex: Operator$|Sensor$}
      - has:
          field: arguments
          has:
            kind: keyword_argument
            all:
            - has: {field: name, regex: ^task_id$}
            - has:
                field: value
                kind: string
                pattern: $S
                has:
                  kind: string_content
                  not: {pattern: $N}
  - inside:
      kind: module
      stopBy: end
      has:
        any:
        - {kind: import_statement}
        - {kind: import_from_statement}
        regex: ^(import|from)\s+airflow\b
message: Name the task `"$N"`, like its variable
fix:
- replace: {target: $S, with: '"$N"'}
```

## Examples

### Python

#### Don't

```python
from airflow.operators.python import PythonOperator
load = PythonOperator(task_id="extract", python_callable=run)
```

#### Do

```python
from airflow.operators.python import PythonOperator
load = PythonOperator(task_id="load", python_callable=run)
```
