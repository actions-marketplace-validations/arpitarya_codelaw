---
id: py/any-eq-ne-annotation
title: "object, not Any, for __eq__'s other"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PYI032]
since: 0.5.0
---

The `other` parameter of `__eq__` and `__ne__` is annotated `object`, not `Any`.

**Why:** `object` is what the protocol accepts; `Any` turns type checking off for the argument.

```codelaw
match:
  kind: type
  regex: '^(typing\.)?Any$'
  inside: {kind: typed_parameter, inside: {kind: parameters, inside: {kind: function_definition, has: {field: name, regex: '^__(eq|ne)__$'}}}}
message: "Annotate `object`"
fix: "object"
```

## Examples

### Python

#### Don't

```python
class A:
    def __eq__(self, other: Any) -> bool: ...
```

#### Do

```python
class A:
    def __eq__(self, other: object) -> bool: ...
```
