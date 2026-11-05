---
id: py/if-expr-with-false-true
title: "not c, not False if c else True"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/SIM211]
since: 0.5.0
---

`False if c else True` is written `not c`.

**Why:** The conditional restates the negation.

```codelaw
match:
  pattern: False if $C else True
message: "Write `not $C`"
fix: "not $C"
```

## Examples

### Python

#### Don't

```python
empty = False if items else True
```

#### Do

```python
empty = not items
```
