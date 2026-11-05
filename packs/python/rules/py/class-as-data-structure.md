---
id: py/class-as-data-structure
title: "A dataclass, not a class that only stores its arguments"
languages: [python]
fix: manual
fix_reason: "turning the class into a dataclass rewrites its body"
fix_exception: restructure
stability: preview
ported_from: [ruff/B903]
since: 0.5.0
---

A class whose only method is an `__init__` that copies its arguments onto `self` is written as a `@dataclass`.

**Why:** The dataclass generates `__init__`, `__repr__` and `__eq__` from the fields, and states the shape.

```codelaw
match:
  kind: class_definition
  not: {has: {field: superclasses, kind: argument_list}}
  has:
    field: body
    kind: block
    not: {has: {kind: function_definition, nthChild: 2}}
    has:
      kind: function_definition
      nthChild: 1
      has: {field: name, regex: '^__init__$'}
      not: {has: {field: body, has: {any: [{kind: if_statement}, {kind: for_statement}, {kind: while_statement}, {kind: try_statement}, {kind: with_statement}, {kind: return_statement}, {kind: raise_statement}, {kind: expression_statement, not: {has: {kind: assignment, regex: '^self\.\w+\s*=\s*\w+$'}}}]}}}
message: "This class only stores its arguments; make it a dataclass"
```

## How to fix

1. Decorate the class `@dataclass`, declare each attribute as an annotated field, and delete `__init__`.

## Examples

### Python

#### Don't

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
```

#### Do

```python
@dataclass
class Point:
    x: int
    y: int
```
