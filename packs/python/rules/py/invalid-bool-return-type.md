---
id: py/invalid-bool-return-type
title: "__bool__ returns a bool"
languages: [python]
fix: manual
fix_reason: "the right truth value is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0304]
since: 0.5.0
---

`__bool__` does not return a literal that is not `True` or `False`.

**Why:** `bool()` raises TypeError on anything else.

```codelaw
match:
  kind: return_statement
  has: {any: [{kind: integer}, {kind: float}, {kind: string}, {kind: none}, {kind: list}, {kind: tuple}, {kind: dictionary}]}
  inside: {kind: function_definition, stopBy: {kind: function_definition}, has: {field: name, regex: '^__bool__$'}}
message: "`__bool__` must return True or False"
```

## How to fix

1. Return `True` or `False` (or `bool(...)`).

## Examples

### Python

#### Don't

```python
class A:
    def __bool__(self):
        return 1
```

#### Do

```python
class A:
    def __bool__(self):
        return True
```
