---
id: py/call-datetime-now-without-tzinfo
title: "datetime.now() with a time zone"
languages: [python]
fix: suggest
fix_reason: "UTC is suggested; review that the code meant UTC, and that comparisons with naive datetimes are updated"
stability: preview
ported_from: [ruff/DTZ005]
since: 0.5.0
---

`datetime.datetime.now()` is given a time zone.

**Why:** Without one it returns a naive local time.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^datetime\.datetime\.now$}
  - has: {field: arguments, regex: '^\(\s*\)$|^\(\s*(tz\s*=\s*)?None\s*\)$'}
message: 'Pass a time zone: `now(tz=...)`'
fix: datetime.datetime.now(tz=datetime.timezone.utc)
```

## Examples

### Python

#### Don't

```python
now = datetime.datetime.now()
```

#### Do

```python
now = datetime.datetime.now(tz=datetime.timezone.utc)
```
