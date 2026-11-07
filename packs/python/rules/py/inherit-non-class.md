---
id: py/inherit-non-class
title: "A class inherits only from classes"
languages: [python]
fix: manual
fix_reason: "which base was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/inherit-non-class]
since: 0.5.0
---

A class's bases are not literals — a number, a string, `None`, a list.

**Why:** Python raises `TypeError` when the class is created.

```codelaw
match:
  any: [{kind: integer}, {kind: float}, {kind: string}, {kind: none}, {kind: 'true'}, {kind: 'false'}, {kind: list}, {kind: dictionary}, {kind: tuple}]
  inside: {kind: argument_list, inside: {kind: class_definition, field: superclasses}}
message: "A base must be a class"
```

## How to fix

1. Name the base class itself, not a string or a value.

## Examples

### Python

#### Don't

```python
class A("Base"):
    pass
```

#### Do

```python
class A(Base):
    pass
```
