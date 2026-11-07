---
id: py/invalid-index-returned
title: "__index__ returns an int"
languages: [python]
fix: manual
fix_reason: "the value the method should compute is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-index-returned]
since: 0.5.0
---

`__index__` does not return a literal that is not an int: `return "2"` is reported. (Pylint infers any returned value; this rule reads literals.)

**Why:** Python calls `__index__` for a builtin operation and raises `TypeError` when the value has the wrong type.

```codelaw
match:
  kind: return_statement
  has: {nthChild: 1, any: [{kind: string, not: {has: {kind: interpolation}}}, {kind: float}, {kind: none}, {kind: list}, {kind: tuple}, {kind: dictionary}]}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__index__$'}
message: "`__index__` must return an int"
```

## How to fix

1. Return an int from `__index__`.

## Examples

### Python

#### Don't

```python
class A:
    def __index__(self):
        return "2"
```

#### Do

```python
class A:
    def __index__(self):
        return self.n
```
