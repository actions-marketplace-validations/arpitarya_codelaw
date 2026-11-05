---
id: py/call-datetime-utcfromtimestamp
title: "fromtimestamp(ts, tz=utc), not utcfromtimestamp(ts)"
languages: [python]
fix: suggest
fix_reason: "the result becomes timezone-aware, which changes comparisons with naive datetimes"
stability: preview
ported_from: [ruff/DTZ004]
since: 0.5.0
---

`datetime.datetime.utcfromtimestamp(ts)` is written `datetime.datetime.fromtimestamp(ts, tz=datetime.timezone.utc)`.

**Why:** `utcfromtimestamp` returns a naive datetime that most code then treats as local time; it is deprecated.

```codelaw
match:
  pattern: datetime.datetime.utcfromtimestamp($T)
message: "Use `fromtimestamp($T, tz=datetime.timezone.utc)`"
fix: "datetime.datetime.fromtimestamp($T, tz=datetime.timezone.utc)"
```

## Examples

### Python

#### Don't

```python
when = datetime.datetime.utcfromtimestamp(ts)
```

#### Do

```python
when = datetime.datetime.fromtimestamp(ts, tz=datetime.timezone.utc)
```
