---
id: py/complex-assignment-in-stub
title: "A stub assigns one plain name at a time"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "each name gets its own declaration"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI017]
since: 0.5.0
---

A stub does not chain assignments (`a = b = 1`), unpack (`a, b = 1, 2`) or assign to an attribute or item.

**Why:** Type checkers read only a simple `name: type` or `name = value` in a stub.

```codelaw
match:
  kind: assignment
  any:
    - has: {field: right, kind: assignment}
    - has: {field: left, any: [{kind: pattern_list}, {kind: tuple_pattern}, {kind: list_pattern}, {kind: attribute}, {kind: subscript}]}
  not: {inside: {kind: assignment}}
message: "Declare one name at a time"
```

## How to fix

1. Write each name as its own declaration, `name: type`.

## Examples

### Python

#### Don't

```python path=api.pyi
a = b = 1
```

#### Do

```python path=api.pyi
a: int
b: int
```
