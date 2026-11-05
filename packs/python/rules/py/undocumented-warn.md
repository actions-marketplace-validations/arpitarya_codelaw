---
id: py/undocumented-warn
title: "logging.WARNING, not logging.WARN"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/LOG009]
since: 0.5.0
---

`logging.WARN` is written `logging.WARNING`.

**Why:** `WARN` is an undocumented alias.

```codelaw
match:
  kind: attribute
  regex: '^logging\.WARN$'
message: "Write `logging.WARNING`"
fix: "logging.WARNING"
```

## Examples

### Python

#### Don't

```python
logger.setLevel(logging.WARN)
```

#### Do

```python
logger.setLevel(logging.WARNING)
```
