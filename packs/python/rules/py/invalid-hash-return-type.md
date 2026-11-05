---
id: py/invalid-hash-return-type
title: "__hash__ returns an int"
languages: [python]
fix: manual
fix_reason: "the right hash is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0309]
since: 0.5.0
---

`__hash__` does not return a literal that is not an int.

**Why:** `hash()` raises TypeError on anything else.

```codelaw
match:
  kind: return_statement
  has: {any: [{kind: float}, {kind: string}, {kind: none}, {kind: list}, {kind: tuple}, {kind: dictionary}]}
  inside: {kind: function_definition, stopBy: {kind: function_definition}, has: {field: name, regex: '^__hash__$'}}
message: "`__hash__` must return an int"
```

## How to fix

1. Return an int, e.g. `hash((self.a, self.b))`.

## Examples

### Python

#### Don't

```python
class A:
    def __hash__(self):
        return "a"
```

#### Do

```python
class A:
    def __hash__(self):
        return hash(self.a)
```
