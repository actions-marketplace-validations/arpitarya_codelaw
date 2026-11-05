---
id: py/invalid-all-format
title: "__all__ is a list or tuple"
languages: [python]
fix: manual
fix_reason: "the names to export are the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0605]
since: 0.5.0
---

`__all__` is assigned a list or tuple of strings, not a string or other value.

**Why:** A string is iterated character by character, so `from m import *` imports each letter.

```codelaw
match:
  kind: assignment
  all:
  - has: {field: left, regex: ^__all__$}
  - has:
      field: right
      any:
      - {kind: string}
      - {kind: integer}
      - {kind: dictionary}
      - {kind: set}
      - {kind: none}
message: '`__all__` must be a list or tuple of names'
```

## How to fix

1. Write `__all__ = ["name", ...]`.

## Examples

### Python

#### Don't

```python
__all__ = "load"
```

#### Do

```python
__all__ = ["load"]
```
