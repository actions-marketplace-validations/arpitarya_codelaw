---
id: py/invalid-bool-returned
title: "__bool__ returns a bool"
languages: [python]
fix: manual
fix_reason: "the value the method should compute is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-bool-returned]
since: 0.5.0
---

`__bool__` does not return a literal that is not a bool: `return 1` is reported. (Pylint infers any returned value; this rule reads literals.)

**Why:** Python calls `__bool__` for a builtin operation and raises `TypeError` when the value has the wrong type.

```codelaw
match:
  kind: return_statement
  has: {nthChild: 1, any: [{kind: string, not: {has: {kind: interpolation}}}, {kind: integer}, {kind: float}, {kind: none}, {kind: list}, {kind: tuple}, {kind: dictionary}]}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__bool__$'}
message: "`__bool__` must return a bool"
```

## How to fix

1. Return a bool from `__bool__`.

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
        return self.n > 0
```
