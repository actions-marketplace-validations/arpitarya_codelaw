---
id: py/invalid-str-returned
title: "__str__ returns a str"
languages: [python]
fix: manual
fix_reason: "the value the method should compute is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-str-returned]
since: 0.5.0
---

`__str__` does not return a literal that is not a str: `return 1` is reported. (Pylint infers any returned value; this rule reads literals.)

**Why:** Python calls `__str__` for a builtin operation and raises `TypeError` when the value has the wrong type.

```codelaw
match:
  kind: return_statement
  has: {nthChild: 1, any: [{kind: integer}, {kind: float}, {kind: 'true'}, {kind: 'false'}, {kind: none}, {kind: list}, {kind: dictionary}, {kind: tuple}, {kind: set}]}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: name, regex: '^__str__$'}
message: "`__str__` must return a str"
```

## How to fix

1. Return a str from `__str__`.

## Examples

### Python

#### Don't

```python
class A:
    def __str__(self):
        return 1
```

#### Do

```python
class A:
    def __str__(self):
        return str(self.n)
```
