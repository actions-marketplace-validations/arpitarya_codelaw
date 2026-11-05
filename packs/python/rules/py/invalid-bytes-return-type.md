---
id: py/invalid-bytes-return-type
title: "__bytes__ returns bytes"
languages: [python]
fix: manual
fix_reason: "the right bytes are the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0308]
since: 0.5.0
---

`__bytes__` does not return a literal that is not bytes.

**Why:** `bytes()` raises TypeError on anything else.

```codelaw
match:
  kind: return_statement
  has: {any: [{kind: integer}, {kind: float}, {kind: none}, {kind: 'true'}, {kind: 'false'}, {kind: list}, {kind: tuple}, {kind: dictionary}, {kind: string, not: {regex: '^[rR]?[bB]'}}]}
  inside: {kind: function_definition, stopBy: {kind: function_definition}, has: {field: name, regex: '^__bytes__$'}}
message: "`__bytes__` must return bytes"
```

## How to fix

1. Return a bytes value.

## Examples

### Python

#### Don't

```python
class A:
    def __bytes__(self):
        return "x"
```

#### Do

```python
class A:
    def __bytes__(self):
        return b"x"
```
