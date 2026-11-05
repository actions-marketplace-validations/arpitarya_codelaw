---
id: py/class-with-mixed-type-vars
title: "No Generic[...] base beside type parameters"
languages: [python]
fix: suggest
fix_reason: "the `Generic[...]` base is removed; review that its type variables are the class's parameters"
stability: preview
ported_from: [ruff/RUF053]
since: 0.5.0
---

A class with PEP 695 type parameters (`class A[T]:`) does not also list `Generic[...]` as a base.

**Why:** Mixing the two styles is a TypeError at class creation.

```codelaw
match:
  kind: subscript
  regex: '^(typing\.)?Generic\['
  inside:
    kind: argument_list
    inside: {kind: class_definition, has: {kind: type_parameter}}
message: "Remove `Generic[...]`; the type parameters already make the class generic"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
class Box[T](Base, Generic[T]):
    pass
```

#### Do

```python
class Box[T](Base):
    pass
```
