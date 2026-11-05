---
id: py/property-without-return
title: "A property returns a value"
languages: [python]
fix: manual
fix_reason: "what the property returns is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/RUF066]
since: 0.5.0
---

A function decorated `@property` contains a `return` with a value, a `raise` or a `yield` (stubs whose body is `...` or `pass` are left alone).

**Why:** Without one, every read of the attribute gives `None`.

```codelaw
match:
  kind: function_definition
  all:
    - not: {has: {stopBy: end, any: [{kind: return_statement, regex: '^return\s+\S'}, {kind: raise_statement}, {kind: yield}]}}
    - has: {field: body, kind: block, not: {regex: '^(\.\.\.|pass)$'}}
  inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(\w+\.)*(property|cached_property)$'}}
message: "This property never returns a value"
```

## How to fix

1. Return the value the attribute should have.

## Examples

### Python

#### Don't

```python
class A:
    @property
    def size(self):
        self._size
```

#### Do

```python
class A:
    @property
    def size(self):
        return self._size
```
