---
id: airflow/airflow3-dag-dynamic-value
title: "No changing values in a DAG's arguments"
languages: [python]
fix: manual
fix_reason: "the fixed value is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/AIR304]
since: 0.5.0
---

A DAG's `start_date` and similar arguments are not computed from the current time.

**Why:** The file is parsed again and again; a `start_date` of `datetime.now()` moves each time, so the DAG never runs as scheduled.

```codelaw
match:
  kind: keyword_argument
  all:
  - has: {field: name, regex: ^(start_date|end_date|dag_id|schedule)$}
  - has: {field: value, regex: '(datetime\.)?(now|utcnow|today)\(\)|pendulum\.(now|today)\('}
  - inside:
      kind: call
      stopBy: end
      has: {field: function, regex: '^(airflow\.)?(DAG|dag)$'}
  - inside:
      kind: module
      stopBy: end
      has:
        any:
        - {kind: import_statement}
        - {kind: import_from_statement}
        regex: ^(import|from)\s+airflow\b
message: Use a fixed date; a value computed from the current time moves on every parse
```

## How to fix

1. Use a fixed date, for example `pendulum.datetime(2026, 1, 1, tz='UTC')`.

## Examples

### Python

#### Don't

```python
from airflow import DAG
from datetime import datetime
with DAG(dag_id='etl', start_date=datetime.now(), schedule='@daily'):
    pass
```

#### Do

```python
from airflow import DAG
from datetime import datetime
with DAG(dag_id='etl', start_date=datetime(2026, 1, 1), schedule='@daily'):
    pass
```
