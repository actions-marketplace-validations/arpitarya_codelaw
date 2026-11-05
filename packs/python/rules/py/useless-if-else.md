---
id: py/useless-if-else
title: "No conditional whose branches are equal"
languages: [python]
fix: suggest
fix_reason: "the condition is no longer evaluated, which matters if it has side effects"
stability: preview
ported_from: [ruff/RUF034]
since: 0.5.0
---

`a if cond else a` is written `a`.

**Why:** Both branches give the same value, so the condition decides nothing.

```codelaw
match:
  pattern: $A if $C else $A
message: "Both branches are `$A`"
fix: "$A"
```

## Examples

### Python

#### Don't

```python
size = limit if big else limit
```

#### Do

```python
size = limit
```
