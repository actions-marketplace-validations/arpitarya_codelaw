---
id: py/call-date-fromtimestamp
title: "No date.fromtimestamp()"
languages: [python]
fix: suggest
fix_reason: "UTC is suggested; review that the code meant UTC, and that comparisons with naive datetimes are updated"
stability: preview
ported_from: [ruff/DTZ012]
since: 0.5.0
---

`datetime.date.fromtimestamp(ts)` is replaced by `datetime.datetime.fromtimestamp(ts, tz=...).date()`.

**Why:** It uses the machine's local zone, so the date depends on where the code runs.

```codelaw
match:
  kind: call
  pattern: datetime.date.fromtimestamp($T)
  has: {field: function, regex: '^datetime\.date\.fromtimestamp$'}
message: "Use `datetime.fromtimestamp(ts, tz=...).date()`"
fix: 'datetime.datetime.fromtimestamp($T, tz=datetime.timezone.utc).date()'
```

## Examples

### Python

#### Don't

```python
day = datetime.date.fromtimestamp(ts)
```

#### Do

```python
day = datetime.datetime.fromtimestamp(ts, tz=datetime.timezone.utc).date()
```
