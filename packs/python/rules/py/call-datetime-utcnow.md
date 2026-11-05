---
id: py/call-datetime-utcnow
title: "datetime.now(timezone.utc), not utcnow()"
languages: [python]
fix: suggest
fix_reason: "the result becomes timezone-aware, which changes comparisons with naive datetimes"
stability: preview
ported_from: [ruff/DTZ003]
since: 0.5.0
---

`datetime.datetime.utcnow()` is written `datetime.datetime.now(datetime.timezone.utc)`.

**Why:** `utcnow()` returns a naive datetime that most code then treats as local time; it is deprecated.

```codelaw
match:
  pattern: datetime.datetime.utcnow()
message: "Use `datetime.datetime.now(datetime.timezone.utc)`"
fix: "datetime.datetime.now(datetime.timezone.utc)"
```

## Examples

### Python

#### Don't

```python
stamp = datetime.datetime.utcnow()
```

#### Do

```python
stamp = datetime.datetime.now(datetime.timezone.utc)
```
