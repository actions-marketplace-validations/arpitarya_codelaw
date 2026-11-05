---
id: py/invalid-index-return-type
title: "__index__ returns an int"
languages: [python]
fix: manual
fix_reason: "the right index is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0305]
since: 0.5.0
---

`__index__` does not return a literal that is not an int.

**Why:** Python raises TypeError on anything else.

```codelaw
match:
  kind: return_statement
  has: {any: [{kind: float}, {kind: string}, {kind: none}, {kind: list}, {kind: tuple}, {kind: dictionary}]}
  inside: {kind: function_definition, stopBy: {kind: function_definition}, has: {field: name, regex: '^__index__$'}}
message: "`__index__` must return an int"
```

## How to fix

1. Return an int.

## Examples

### Python

#### Don't

```python
class A:
    def __index__(self):
        return "1"
```

#### Do

```python
class A:
    def __index__(self):
        return 1
```
