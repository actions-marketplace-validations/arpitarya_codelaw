---
id: py/datetime-min-max
title: "No naive datetime.min or datetime.max"
languages: [python]
fix: suggest
fix_reason: "UTC is suggested; review that the code meant UTC, and that comparisons with naive datetimes are updated"
stability: preview
ported_from: [ruff/DTZ901]
since: 0.5.0
---

`datetime.datetime.min` and `.max` are given a zone (`.replace(tzinfo=...)`) before use.

**Why:** They are naive; converting `datetime.min` to another zone can overflow.

```codelaw
match:
  kind: attribute
  regex: '^datetime\.datetime\.(min|max)$'
  not: {inside: {kind: attribute, has: {field: attribute, regex: '^replace$'}}}
message: "This is naive; attach a zone with `.replace(tzinfo=...)`"
fix: '$$MATCH.replace(tzinfo=datetime.timezone.utc)'
```

## Examples

### Python

#### Don't

```python
start = datetime.datetime.min
```

#### Do

```python
start = datetime.datetime.min.replace(tzinfo=datetime.timezone.utc)
```
