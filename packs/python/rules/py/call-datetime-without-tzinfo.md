---
id: py/call-datetime-without-tzinfo
title: "datetime() with a tzinfo"
languages: [python]
fix: suggest
fix_reason: "UTC is suggested; review that the code meant UTC, and that comparisons with naive datetimes are updated"
stability: preview
ported_from: [ruff/DTZ001]
since: 0.5.0
---

`datetime.datetime(...)` is given a `tzinfo`.

**Why:** A naive datetime has no zone; comparing or converting it silently assumes local time.

```codelaw
match:
  kind: call
  pattern: datetime.datetime($$$A)
  all:
  - has: {field: function, regex: ^datetime\.datetime$}
  - has:
      field: arguments
      has: {nthChild: 1}
      not:
        has:
          any:
          - kind: keyword_argument
            has: {field: name, regex: ^tzinfo$}
          - {nthChild: 8}
          - {kind: list_splat}
          - {kind: dictionary_splat}
message: Pass `tzinfo=` to make this datetime aware
fix: datetime.datetime($$$A, tzinfo=datetime.timezone.utc)
```

## Examples

### Python

#### Don't

```python
start = datetime.datetime(2026, 1, 1)
```

#### Do

```python
start = datetime.datetime(2026, 1, 1, tzinfo=datetime.timezone.utc)
```
