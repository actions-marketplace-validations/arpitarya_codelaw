---
id: py/invalid-all-object
title: "__all__ holds names as strings"
languages: [python]
fix: suggest
fix_reason: "the name becomes its string; review that the object's name is the one exported"
stability: preview
ported_from: [ruff/PLE0604]
since: 0.5.0
---

Every item of `__all__` is a string, not the object itself.

**Why:** `from module import *` raises TypeError on a non-string item.

```codelaw
match:
  kind: identifier
  pattern: $N
  inside:
    any: [{kind: list}, {kind: tuple}]
    inside: {kind: assignment, has: {field: left, regex: '^__all__$'}}
message: "Write `\"$N\"` as a string"
fix: '"$N"'
```

## Examples

### Python

#### Don't

```python
__all__ = ["load", save]
```

#### Do

```python
__all__ = ["load", "save"]
```
