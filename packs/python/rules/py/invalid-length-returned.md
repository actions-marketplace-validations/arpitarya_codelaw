---
id: py/invalid-length-returned
title: "__len__ returns a non-negative int"
languages: [python]
fix: manual
fix_reason: "the value the method should compute is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-length-returned]
since: 0.5.0
---

`__len__` does not return a literal that is not a non-negative int: `return "3"` is reported. (Pylint infers any returned value; this rule reads literals.)

**Why:** Python calls `__len__` for a builtin operation and raises `TypeError` when the value has the wrong type.

```codelaw
match:
  kind: return_statement
  has: {nthChild: 1, any: [{kind: string, not: {has: {kind: interpolation}}}, {kind: float}, {kind: none}, {kind: list}, {kind: tuple}, {kind: dictionary}, {kind: unary_operator, regex: '^-'}]}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__len__$'}
message: "`__len__` must return a non-negative int"
```

## How to fix

1. Return a non-negative int from `__len__`.

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
        return len(self.items)
```
