---
id: py/call-datetime-today
title: "No datetime.today()"
languages: [python]
fix: suggest
fix_reason: "UTC is suggested; review that the code meant UTC, and that comparisons with naive datetimes are updated"
stability: preview
ported_from: [ruff/DTZ002]
since: 0.5.0
---

`datetime.datetime.today()` is replaced by `datetime.datetime.now(tz=...)`.

**Why:** `today()` returns a naive local time.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^datetime\.datetime\.today$'}
message: "Use `datetime.now(tz=...)`"
fix: 'datetime.datetime.now(tz=datetime.timezone.utc)'
```

## Examples

### Python

#### Don't

```python
now = datetime.datetime.today()
```

#### Do

```python
now = datetime.datetime.now(tz=datetime.timezone.utc)
```
