---
id: py/invalid-length-return-type
title: "__len__ returns a non-negative int"
languages: [python]
fix: manual
fix_reason: "the right length is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0303]
since: 0.5.0
---

`__len__` does not return a literal that is not a non-negative int.

**Why:** `len()` raises TypeError or ValueError on anything else.

```codelaw
match:
  kind: return_statement
  has: {any: [{kind: float}, {kind: string}, {kind: none}, {kind: 'true'}, {kind: 'false'}, {kind: list}, {kind: tuple}, {kind: dictionary}, {kind: unary_operator, regex: '^-'}]}
  inside: {kind: function_definition, stopBy: {kind: function_definition}, has: {field: name, regex: '^__len__$'}}
message: "`__len__` must return a non-negative int"
```

## How to fix

1. Return the length as a non-negative int.

## Examples

### Python

#### Don't

```python
class A:
    def __len__(self):
        return "3"
```

#### Do

```python
class A:
    def __len__(self):
        return 3
```
