---
id: py/invalid-bytes-returned
title: "__bytes__ returns bytes"
languages: [python]
fix: manual
fix_reason: "the value the method should compute is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-bytes-returned]
since: 0.5.0
---

`__bytes__` does not return a literal that is not bytes: `return "data"` is reported. (Pylint infers any returned value; this rule reads literals.)

**Why:** Python calls `__bytes__` for a builtin operation and raises `TypeError` when the value has the wrong type.

```codelaw
match:
  kind: return_statement
  has: {nthChild: 1, any: [{kind: integer}, {kind: float}, {kind: 'true'}, {kind: 'false'}, {kind: none}, {kind: list}, {kind: dictionary}, {kind: tuple}, {kind: string, not: {regex: '^[bB]'}}]}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__bytes__$'}
message: "`__bytes__` must return bytes"
```

## How to fix

1. Return bytes from `__bytes__`.

## Examples

### Python

#### Don't

```python
class A:
    def __bytes__(self):
        return "data"
```

#### Do

```python
class A:
    def __bytes__(self):
        return b"data"
```
