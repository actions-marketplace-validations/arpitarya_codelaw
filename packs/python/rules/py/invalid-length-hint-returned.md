---
id: py/invalid-length-hint-returned
title: "__length_hint__ returns a non-negative int"
languages: [python]
fix: manual
fix_reason: "the value the method should compute is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-length-hint-returned]
since: 0.5.0
---

`__length_hint__` does not return a literal that is not a non-negative int: `return -1` is reported. (Pylint infers any returned value; this rule reads literals.)

**Why:** Python calls `__length_hint__` for a builtin operation and raises `TypeError` when the value has the wrong type.

```codelaw
match:
  kind: return_statement
  has: {nthChild: 1, any: [{kind: string, not: {has: {kind: interpolation}}}, {kind: float}, {kind: none}, {kind: list}, {kind: tuple}, {kind: dictionary}, {kind: unary_operator, regex: '^-'}]}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__length_hint__$'}
message: "`__length_hint__` must return a non-negative int"
```

## How to fix

1. Return a non-negative int from `__length_hint__`.

## Examples

### Python

#### Don't

```python
class A:
    def __length_hint__(self):
        return -1
```

#### Do

```python
class A:
    def __length_hint__(self):
        return 0
```
