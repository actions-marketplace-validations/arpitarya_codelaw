---
id: py/yield-in-init
title: "No yield in __init__"
languages: [python]
fix: manual
fix_reason: "where the generator belongs is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLE0100]
since: 0.5.0
---

`__init__` does not `yield`.

**Why:** A generator `__init__` returns a generator, and creating the object raises TypeError.

```codelaw
match:
  kind: yield
  inside:
    kind: function_definition
    stopBy: {kind: function_definition}
    has: {field: name, regex: '^__init__$'}
message: "`__init__` cannot be a generator"
```

## How to fix

1. Move the generator into its own method.

## Examples

### Python

#### Don't

```python
class A:
    def __init__(self):
        yield 1
```

#### Do

```python
class A:
    def items(self):
        yield 1
```
