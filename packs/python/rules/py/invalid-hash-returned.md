---
id: py/invalid-hash-returned
title: "__hash__ returns an int"
languages: [python]
fix: manual
fix_reason: "the value the method should compute is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-hash-returned]
since: 0.5.0
---

`__hash__` does not return a literal that is not an int: `return "x"` is reported. (Pylint infers any returned value; this rule reads literals.)

**Why:** Python calls `__hash__` for a builtin operation and raises `TypeError` when the value has the wrong type.

```codelaw
match:
  kind: return_statement
  has: {nthChild: 1, any: [{kind: string, not: {has: {kind: interpolation}}}, {kind: float}, {kind: none}, {kind: list}, {kind: tuple}, {kind: dictionary}]}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__hash__$'}
message: "`__hash__` must return an int"
```

## How to fix

1. Return an int from `__hash__`.

## Examples

### Python

#### Don't

```python
class A:
    def __hash__(self):
        return "x"
```

#### Do

```python
class A:
    def __hash__(self):
        return hash(self.n)
```
