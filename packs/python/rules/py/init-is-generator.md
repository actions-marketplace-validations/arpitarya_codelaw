---
id: py/init-is-generator
title: "__init__ is not a generator"
languages: [python]
fix: manual
fix_reason: "what the yield was for moves to another method"
fix_exception: restructure
stability: preview
ported_from: [pylint/init-is-generator]
since: 0.5.0
---

`__init__` does not contain `yield`.

**Why:** A generator `__init__` returns a generator, and Python raises `TypeError` when the class is called.

```codelaw
match:
  kind: yield
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__init__$'}
message: "`__init__` cannot yield"
```

## How to fix

1. Move the generator into a method of its own and call it from `__init__` or from the caller.

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
    def __init__(self):
        self.n = 1
```
