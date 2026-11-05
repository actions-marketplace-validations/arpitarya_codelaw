---
id: py/call-datetime-fromtimestamp
title: "fromtimestamp() with a time zone"
languages: [python]
fix: suggest
fix_reason: "UTC is suggested; review that the code meant UTC, and that comparisons with naive datetimes are updated"
stability: preview
ported_from: [ruff/DTZ006]
since: 0.5.0
---

`datetime.datetime.fromtimestamp(ts)` is given a time zone.

**Why:** Without one it returns a naive local time.

```codelaw
match:
  kind: call
  pattern: datetime.datetime.fromtimestamp($T)
  all:
  - has: {field: function, regex: ^datetime\.datetime\.fromtimestamp$}
  - has:
      field: arguments
      not:
        has:
          any:
          - {nthChild: 2}
          - kind: keyword_argument
            has: {field: name, regex: ^tz$}
message: 'Pass a time zone: `fromtimestamp(ts, tz=...)`'
fix: datetime.datetime.fromtimestamp($T, tz=datetime.timezone.utc)
```

## Examples

### Python

#### Don't

```python
when = datetime.datetime.fromtimestamp(ts)
```

#### Do

```python
when = datetime.datetime.fromtimestamp(ts, tz=datetime.timezone.utc)
```
