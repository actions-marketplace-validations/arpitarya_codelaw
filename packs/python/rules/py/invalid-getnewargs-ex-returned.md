---
id: py/invalid-getnewargs-ex-returned
title: "__getnewargs_ex__ returns a (tuple, dict) pair"
languages: [python]
fix: manual
fix_reason: "the value the method should compute is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-getnewargs-ex-returned]
since: 0.5.0
---

`__getnewargs_ex__` does not return a literal that is not a (tuple, dict) pair: `return [1]` is reported. (Pylint infers any returned value; this rule reads literals.)

**Why:** Python calls `__getnewargs_ex__` for a builtin operation and raises `TypeError` when the value has the wrong type.

```codelaw
match:
  kind: return_statement
  has: {nthChild: 1, any: [{kind: string, not: {has: {kind: interpolation}}}, {kind: integer}, {kind: float}, {kind: none}, {kind: list}, {kind: dictionary}]}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__getnewargs_ex__$'}
message: "`__getnewargs_ex__` must return a (tuple, dict) pair"
```

## How to fix

1. Return a (tuple, dict) pair from `__getnewargs_ex__`.

## Examples

### Python

#### Don't

```python
class A:
    def __getnewargs_ex__(self):
        return [1]
```

#### Do

```python
class A:
    def __getnewargs_ex__(self):
        return ((1,), {})
```
