---
id: py/return-in-init
title: "No return value in __init__"
languages: [python]
fix: suggest
fix_reason: "the value is dropped; it was never usable, since returning one raises TypeError"
stability: preview
ported_from: [ruff/PLE0101]
since: 0.5.0
---

`__init__` does not return a value.

**Why:** Returning anything but `None` from `__init__` raises TypeError.

```codelaw
match:
  kind: return_statement
  regex: '^return\s+\S'
  not: {regex: '^return\s+None$'}
  inside:
    kind: function_definition
    stopBy: {kind: function_definition}
    has: {field: name, regex: '^__init__$'}
message: "`__init__` must return None"
fix: "return"
```

## Examples

### Python

#### Don't

```python
class A:
    def __init__(self, x):
        self.x = x
        return self
```

#### Do

```python
class A:
    def __init__(self, x):
        self.x = x
        return
```
