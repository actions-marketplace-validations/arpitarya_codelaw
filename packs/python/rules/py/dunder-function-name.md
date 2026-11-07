---
id: py/dunder-function-name
title: "No dunder names for plain functions"
languages: [python]
fix: manual
fix_reason: "the function's new name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/N807]
since: 0.5.0
---

A function that is not a method is not named with double underscores at both ends, as `__helper__`. A module's `__getattr__` and `__dir__` (PEP 562) are not reported.

**Why:** Names like `__x__` are reserved for Python's own protocols; a plain function named so reads as one.

```codelaw
match:
  kind: identifier
  regex: '^__\w+__$'
  inside: {kind: function_definition, field: name}
  all:
    - not: {regex: '^__(getattr|dir)__$'}
    - not:
        any:
          - inside: {kind: function_definition, inside: {kind: block, inside: {kind: class_definition}}}
          - inside: {kind: function_definition, inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition}}}}
message: "A function that is not a method is not named `__…__`"
```

## How to fix

1. Rename the function without the surrounding double underscores, and update its callers.

## Examples

### Python

#### Don't

```python
def __setup__():
    pass
```

#### Do

```python
def setup():
    pass
```
