---
id: py/call-datetime-strptime-without-zone
title: "strptime with %z, or a zone attached"
languages: [python]
fix: suggest
fix_reason: "UTC is suggested; review that the code meant UTC, and that comparisons with naive datetimes are updated"
stability: preview
ported_from: [ruff/DTZ007]
since: 0.5.0
---

`datetime.datetime.strptime` with a format lacking `%z` is followed by `.replace(tzinfo=...)` or `.astimezone(...)`.

**Why:** Otherwise the parsed datetime is naive.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^datetime\.datetime\.strptime$}
  - has:
      field: arguments
      has:
        kind: string
        nthChild: 2
        not: {regex: '%z'}
  - not:
      inside:
        kind: attribute
        has: {field: attribute, regex: ^(replace|astimezone)$}
message: The parsed datetime is naive; add `%z` or attach a zone
fix: $$MATCH.replace(tzinfo=datetime.timezone.utc)
```

## Examples

### Python

#### Don't

```python
when = datetime.datetime.strptime(text, "%Y-%m-%d")
```

#### Do

```python
when = datetime.datetime.strptime(text, "%Y-%m-%d").replace(tzinfo=datetime.timezone.utc)
```
