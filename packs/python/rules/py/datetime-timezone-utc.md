---
id: py/datetime-timezone-utc
title: "datetime.UTC, not datetime.timezone.utc"
languages: [python]
fix: suggest
fix_reason: "`datetime.UTC` needs Python 3.11"
stability: preview
ported_from: [ruff/UP017]
since: 0.5.0
---

`datetime.timezone.utc` is written `datetime.UTC` (Python 3.11+).

**Why:** The alias is shorter and says the same.

```codelaw
match:
  kind: attribute
  regex: '^datetime\.timezone\.utc$'
message: "Write `datetime.UTC`"
fix: "datetime.UTC"
```

## Examples

### Python

#### Don't

```python
now = datetime.datetime.now(datetime.timezone.utc)
```

#### Do

```python
now = datetime.datetime.now(datetime.UTC)
```
