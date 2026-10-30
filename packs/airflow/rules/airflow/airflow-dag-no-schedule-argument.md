---
id: airflow/airflow-dag-no-schedule-argument
title: "A DAG names its schedule"
languages: [python]
fix: manual
fix_reason: "the schedule is the author's"
stability: preview
ported_from: [ruff/AIR002]
since: 0.5.0
---

A `DAG(...)` or `@dag(...)` passes `schedule` (or `schedule_interval`/`timetable`, before Airflow 3).

**Why:** The default schedule changed between Airflow versions (daily in 2, none in 3); stating it keeps the DAG running when it should.

```codelaw
match:
  kind: call
  has: { field: function, regex: '^(airflow\.)?(DAG|dag)$' }
  not: { has: { field: arguments, has: { kind: keyword_argument, regex: '^(schedule|schedule_interval|timetable)\s*=' } } }
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+airflow\b' } }
message: "Pass `schedule=` to this DAG"
```

## How to fix

1. Add `schedule=` with a cron string, a timetable or `None`.

## Examples

### Python

#### Don't

```python
from airflow import DAG
with DAG(dag_id='etl', start_date=start):
    pass
```

#### Do

```python
from airflow import DAG
with DAG(dag_id='etl', start_date=start, schedule='@daily'):
    pass
```
