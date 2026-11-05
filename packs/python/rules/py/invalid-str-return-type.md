---
id: py/invalid-str-return-type
title: "__str__ returns a str"
languages: [python]
fix: manual
fix_reason: "the right text is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0307]
since: 0.5.0
---

`__str__` does not return a literal that is not a str.

**Why:** `str()` raises TypeError on anything else.

```codelaw
match:
  kind: return_statement
  has: {any: [{kind: integer}, {kind: float}, {kind: none}, {kind: 'true'}, {kind: 'false'}, {kind: list}, {kind: tuple}, {kind: dictionary}, {kind: string, regex: '^[bB]'}]}
  inside: {kind: function_definition, stopBy: {kind: function_definition}, has: {field: name, regex: '^__str__$'}}
message: "`__str__` must return a str"
```

## How to fix

1. Return a str.

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
        return "1"
```
