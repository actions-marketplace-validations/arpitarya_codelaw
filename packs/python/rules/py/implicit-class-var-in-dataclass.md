---
id: py/implicit-class-var-in-dataclass
title: "A dataclass's unannotated attribute is not a field"
languages: [python]
fix: manual
fix_reason: "the field's type, or a `ClassVar` annotation, is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/RUF045]
since: 0.5.0
---

An assignment without an annotation in a dataclass body is reported (`_`-prefixed and dunder names are left alone).

**Why:** It becomes a class attribute, not a field: it is missing from `__init__`, `__eq__` and `__repr__`, which is rarely what was meant.

```codelaw
match:
  kind: assignment
  all:
    - not: {has: {field: type, kind: type}}
    - has: {field: left, kind: identifier, not: {regex: '^_'}}
    - inside: {kind: expression_statement, inside: {kind: block, inside: {kind: class_definition, inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(dataclasses\.)?dataclass\b'}}}}}
message: "This is a class attribute, not a field; annotate it (or mark it `ClassVar`)"
```

## How to fix

1. Add a type annotation to make it a field, or annotate it `ClassVar[...]` if it is meant to be shared.

## Examples

### Python

#### Don't

```python
@dataclass
class A:
    count = 0
```

#### Do

```python
@dataclass
class A:
    count: int = 0
```
