---
id: airflow/airflow-variable-get-outside-task
title: "Variable.get inside a task"
languages: [python]
fix: manual
fix_reason: "moving the read into the task or a template changes the DAG's code"
fix_exception: restructure
stability: preview
ported_from: [ruff/AIR003]
since: 0.5.0
---

`Variable.get` is called inside a task, not at the top level of a DAG file.

**Why:** Top-level code runs every time the scheduler parses the file, so each read hits the metadata database over and over.

```codelaw
match:
  pattern: Variable.get($$$A)
  not: { inside: { kind: function_definition, stopBy: end } }
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+airflow\b' } }
message: "Read the Variable inside the task, or use a `{{ var.value… }}` template"
```

## How to fix

1. Move the call into the task function, or use the `{{ var.value.name }}` template.

## Examples

### Python

#### Don't

```python
from airflow.models import Variable
bucket = Variable.get('bucket')
```

#### Do

```python
from airflow.models import Variable

def load():
    bucket = Variable.get('bucket')
```
