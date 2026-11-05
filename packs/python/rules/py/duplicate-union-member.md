---
id: py/duplicate-union-member
title: "No type twice in a union"
languages: [python]
fix: suggest
fix_reason: "the union text changes; the type is the same"
stability: preview
ported_from: [ruff/PYI016]
since: 0.5.0
---

A union does not list the same member twice.

**Why:** The duplicate adds nothing and usually hides a typo.

```codelaw
match:
  kind: binary_operator
  pattern: $A | $A
message: "`$A` is listed twice"
fix: "$A"
```

## Examples

### Python

#### Don't

```python
def f(x: int | int) -> None: ...
```

#### Do

```python
def f(x: int) -> None: ...
```
