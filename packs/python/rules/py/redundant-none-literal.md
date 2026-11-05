---
id: py/redundant-none-literal
title: "None, not Literal[None]"
languages: [python]
fix: suggest
fix_reason: "the annotation text changes; the type is the same"
stability: preview
ported_from: [ruff/PYI061]
since: 0.5.0
---

`Literal[None]` is written `None`.

**Why:** `None` is already its own type.

```codelaw
match:
  kind: generic_type
  regex: '^(typing\.)?Literal\[\s*None\s*\]$'
message: "Write `None`"
fix: "None"
```

## Examples

### Python

#### Don't

```python
def f(x: Literal[None]) -> None: ...
```

#### Do

```python
def f(x: None) -> None: ...
```
