---
id: py/call-date-today
title: "No date.today()"
languages: [python]
fix: suggest
fix_reason: "UTC is suggested; review that the code meant UTC, and that comparisons with naive datetimes are updated"
stability: preview
ported_from: [ruff/DTZ011]
since: 0.5.0
---

`datetime.date.today()` is replaced by `datetime.datetime.now(tz=...).date()`.

**Why:** `today()` uses the machine's local zone, so the date depends on where the code runs.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^datetime\.date\.today$'}
message: "Use `datetime.now(tz=...).date()`"
fix: 'datetime.datetime.now(tz=datetime.timezone.utc).date()'
```

## Examples

### Python

#### Don't

```python
day = datetime.date.today()
```

#### Do

```python
day = datetime.datetime.now(tz=datetime.timezone.utc).date()
```
